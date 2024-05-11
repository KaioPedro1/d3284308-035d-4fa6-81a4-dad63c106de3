use std::io::ErrorKind;

use actix_web::{get, post, web, HttpResponse};
use serde::Deserialize;
use serde_json::json;

use crate::{
    models::{AccountManager, TransactionRequest},
    services::process_transaction_request,
};

#[post("/event")]
async fn transaction_request(
    req_body: web::Json<TransactionRequest>,
    acc_manager: web::Data<AccountManager>,
) -> HttpResponse {
    match process_transaction_request(req_body.into_inner(), acc_manager) {
        Ok(transaction_response) => {
            let response_body = json!(transaction_response);
            HttpResponse::Created().json(response_body)
        }
        Err(e) => {
            if let ErrorKind::NotFound = e {
                HttpResponse::NotFound().body("0")
            } else {
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}

#[derive(Deserialize)]
struct BalanceQueryParam {
    account_id: u64,
}
#[get("/balance")]
async fn balance_get_request(
    query_data: web::Query<BalanceQueryParam>,
    acc_manager: web::Data<AccountManager>,
) -> HttpResponse {
    match acc_manager.get_balance(query_data.account_id.to_string()) {
        Ok(balance) => HttpResponse::Ok().body(balance.to_string()),
        Err(e) => {
            if let ErrorKind::NotFound = e {
                HttpResponse::NotFound().body("0")
            } else {
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}
