use actix_web::http::StatusCode;
use actix_web::web;

use crate::database::model;
use crate::error::Error;
use crate::response::Response;

///
/// The HTTP request handler.
///
/// Sequence:
/// 1. Get swap data in db.
/// 2. Return the swap data.
///
pub async fn handle(
    app_data: crate::WebData,
    query: web::Query<zinc_types::SwapRequestQuery>,
) -> crate::Result<zinc_types::SwapResponseBody, Error> {
    let query = query.into_inner();

    let swapdata = app_data
        .read()
        .expect(zinc_const::panic::SYNCHRONIZATION)
        .swapdata
        .clone();

    let (amount,count,fee) = swapdata.find_all(&query.name);

    let response = zinc_types::SwapResponseBody::new(amount as u64,count as u64,fee as u64);

    Ok(Response::new_with_data(StatusCode::OK, response))
}