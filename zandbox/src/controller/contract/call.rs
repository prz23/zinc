//!
//! The contract resource POST method `call` module.
//!

use std::collections::HashMap;

use actix_web::http::StatusCode;
use actix_web::web;
use num::BigInt;
use num_old::BigUint as oldBigUint;

use crate::contract::Contract;
use crate::database::model;
use crate::error::Error;
use crate::response::Response;
use crate::storage::Storage;
use zinc_types::TransactionMsg;

use crate::database::swap_database::SwapDatabase;
use num::BigUint;
use num_traits::ToPrimitive;
///
/// The HTTP request handler.
///
/// Sequence:
/// 1. Get the contract and its data from the database.
/// 2. Extract the called method from its metadata and check if it is mutable.
/// 3. Parse the method input arguments.
/// 4. Run the method on the VM.
/// 5. Create a transactions array from the client and contract transfers.
/// 6. Send the transactions to zkSync and store its handles.
/// 7. Wait for all transactions to be committed.
/// 8. Update the contract storage state in the database.
/// 9. Send the contract method execution result back to the client.
///
pub async fn handle(
    mut app_data: crate::WebData,
    query: web::Query<zinc_types::CallRequestQuery>,
    body: web::Json<zinc_types::CallRequestBody>,
) -> crate::Result<serde_json::Value, Error> {
    let query = query.into_inner();
    let body = body.into_inner();
    let log_id = serde_json::to_string(&query.address).expect(zinc_const::panic::DATA_CONVERSION);

    let postgresql = app_data
        .read()
        .expect(zinc_const::panic::SYNCHRONIZATION)
        .postgresql
        .clone();
    let network = app_data
        .read()
        .expect(zinc_const::panic::SYNCHRONIZATION)
        .network;
    let swapdata:SwapDatabase = app_data
        .write()
        .expect(zinc_const::panic::SYNCHRONIZATION)
        .swapdata
        .clone();

    log::info!("[{}] Calling method `{}`", log_id, query.method);

    let contract = Contract::new(network, postgresql.clone(), query.address).await?;

    let method = match contract.build.methods.get(query.method.as_str()).cloned() {
        Some(method) => method,
        None => return Err(Error::MethodNotFound(query.method)),
    };
    if !method.is_mutable {
        return Err(Error::MethodIsImmutable(query.method));
    }

    let eth_address_bigint =
        BigInt::from_bytes_be(num::bigint::Sign::Plus, contract.eth_address.as_bytes());
    let mut arguments = zinc_types::Value::try_from_typed_json(body.arguments, method.input)
        .map_err(Error::InvalidInput)?;
    arguments.insert_contract_instance(eth_address_bigint.clone());

    let msg:TransactionMsg = (&body.transaction).try_to_msg(&contract.wallet)?;
    let output = contract
        .run_method(
            query.method.clone(),
            msg.clone(),
            arguments.clone(),
            postgresql.clone(),
        )
        .await?;

    let mut transactions = Vec::with_capacity(1 + output.transfers.len());
    if let zksync_types::ZkSyncTx::Transfer(ref transfer) = body.transaction.tx {
        let token = contract
            .wallet
            .tokens
            .resolve(transfer.token.into())
            .ok_or_else(|| Error::TokenNotFound(transfer.token.to_string()))?;

        log::info!(
            "[{}] Sending {} {} from {} to {} with total batch fee {} {}",
            log_id,
            zksync_utils::format_units(&transfer.amount, token.decimals),
            token.symbol,
            serde_json::to_string(&transfer.from).expect(zinc_const::panic::DATA_CONVERSION),
            serde_json::to_string(&transfer.to).expect(zinc_const::panic::DATA_CONVERSION),
            zksync_utils::format_units(&transfer.fee, token.decimals),
            token.symbol,
        );
    }
    transactions.push(body.transaction);

    let mut nonces = HashMap::with_capacity(output.storages.len());
    let mut created_instances = contract
        .execute_initial_deposits(output.initializers, &mut nonces, &mut transactions)
        .await?;
    let eth_private_keys: HashMap<zksync_types::Address, zksync_types::H256> = created_instances
        .iter()
        .map(|(address, instance)| (*address, instance.eth_private_key))
        .collect();
    contract
        .execute_main_batch(
            postgresql.clone(),
            output.transfers,
            transactions,
            nonces,
            eth_private_keys,
        )
        .await?;

    //Swap(to,min)  msg(sender,rec,token,amount)
    if query.method == "swap" {
        let div10_9 = |x:&oldBigUint| -> u64 {
            let tmp:oldBigUint = x / 10000000u64;
            tmp.to_u64().unwrap()
        };

        let arguments_flat = arguments.into_flat_values();

        let to_arg_token = arguments_flat[0].to_string();
        let from_token_name:String =
            match swapdata.tokenname_convert(&msg.token_address.to_string()){
                Some(name) => { name },
                None => { "UnknownToken".to_string() },
        };

        let pair_name = format!("{}/{}",from_token_name,to_arg_token);
        log::info!("pair_name = [{}] ", pair_name);

        swapdata.accumlate_amount(&pair_name,
                                  div10_9(&msg.amount),
                                  1,
                                  10);
    }

    let mut transaction = postgresql.new_transaction().await?;
    for (address, storage) in output.storages.into_iter() {
        let address = zinc_types::address_from_slice(address.to_bytes_be().1.as_slice());

        if let Some(instance) = created_instances.remove(&address) {
            let account_id = instance.account_id;
            let storage = Storage::from_build(storage).into_database_insert(account_id);

            postgresql
                .insert_contract(instance, Some(&mut transaction))
                .await?;
            postgresql
                .insert_fields(storage, Some(&mut transaction))
                .await?;
        } else {
            let contract = postgresql
                .select_contract(
                    model::contract::select_one::Input::new(address),
                    Some(&mut transaction),
                )
                .await?;
            let storage = Storage::from_build(storage)
                .into_database_update(contract.account_id as zksync_types::AccountId);
            postgresql
                .update_fields(storage, Some(&mut transaction))
                .await?;
        }
    }
    transaction.commit().await?;

    let response = serde_json::json!({
        "output": output.result.into_json(),
    });

    log::info!("[{}] Call finished", log_id);
    Ok(Response::new_with_data(StatusCode::OK, response))
}
