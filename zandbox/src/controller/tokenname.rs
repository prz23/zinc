use actix_web::http::StatusCode;
use actix_web::web;

use crate::database::model;
use crate::error::Error;
use crate::response::Response;

///
/// The HTTP request handler.
///
/// Sequence:
/// 1. search Contract Address.
/// 2. Return tokenname.
///
pub async fn handle_add_token(
    app_data: crate::WebData,
    query: web::Query<zinc_types::TokennameRequestQuery>,
) -> crate::Result<(), Error> {
    let query = query.into_inner();

    let swapdata = app_data
        .read()
        .expect(zinc_const::panic::SYNCHRONIZATION)
        .swapdata
        .clone();

    swapdata.add_token(&query.address,&query.tokenname);

    Ok(Response::new(StatusCode::OK))
}