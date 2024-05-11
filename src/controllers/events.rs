use std::io::ErrorKind;

use actix_web::{post, web, HttpResponse, Result};
use serde_json::json;

use crate::{
    models::{AccountManager, TransactionRequest},
    services::process_request,
};

#[post("/event")]
async fn transaction_request(
    req_body: web::Json<TransactionRequest>,
    acc_manager: web::Data<AccountManager>,
) -> Result<HttpResponse> {
    match process_request(req_body.into_inner(), acc_manager) {
        Ok(transaction_response) => {
            let response_body = json!(transaction_response);
            Ok(HttpResponse::Created().json(response_body))
        }
        Err(e) => {
            if let ErrorKind::NotFound = e {
                Ok(HttpResponse::NotFound().body("0"))
            } else {
                Ok(HttpResponse::InternalServerError().finish())
            }
        },
    }
}
