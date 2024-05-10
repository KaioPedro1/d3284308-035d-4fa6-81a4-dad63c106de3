use actix_web::web::{Data, Json};

use crate::models::{AccountManager, DepositRequest, TransactionType};

fn process_request(req_body: Json<DepositRequest>, acc_manager: Data<AccountManager>){
    match req_body.type_ {
        TransactionType::Deposit => todo!(),
        TransactionType::Withdraw => todo!(),
        TransactionType::Transfer => todo!(),
    }
}
