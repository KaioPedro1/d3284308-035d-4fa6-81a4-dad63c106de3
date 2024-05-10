use actix_web::{ post, web, HttpResponse,Result};
use serde_json::json;

use crate::models::{AccountManager, DepositRequest};

#[post("/event")]
async fn transaction_request(req_body: web::Json<DepositRequest>, acc_manager: web::Data<AccountManager>) -> Result<HttpResponse>  {
    println!("Received deposit request: {:?}", req_body);
    let response_body = json!({"message": "Deposit request received"});
    Ok(HttpResponse::Ok().json(response_body))
}