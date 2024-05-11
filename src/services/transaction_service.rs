use std::io::ErrorKind;

use actix_web::web::Data;

use crate::models::{
    Account, AccountManager, TransactionRequest, TransactionResponse, TransactionType,
};

pub fn process_transaction_request(
    req_body: TransactionRequest,
    acc_manager: Data<AccountManager>,
) -> Result<TransactionResponse, ErrorKind> {
    match req_body.type_ {
        TransactionType::Deposit => match process_deposit(req_body, acc_manager) {
            Ok(acc) => Ok(TransactionResponse {
                origin: None,
                destination: Some(acc),
            }),
            Err(e) => Err(e),
        },
        TransactionType::Withdraw => match process_withdraw(req_body, acc_manager) {
            Ok(acc) => Ok(TransactionResponse {
                origin: Some(acc),
                destination: None,
            }),
            Err(e) => Err(e),
        },
        TransactionType::Transfer => match process_transfer(req_body, acc_manager) {
            Ok((acc_origin, acc_destination)) => Ok(TransactionResponse {
                origin: Some(acc_origin),
                destination: Some(acc_destination),
            }),
            Err(e) => Err(e),
        },
    }
}

fn process_deposit(
    req_body: TransactionRequest,
    acc_manager: Data<AccountManager>,
) -> Result<Account, ErrorKind> {
    req_body
        .destination
        .map_or(Err(ErrorKind::InvalidInput), |dest| {
            acc_manager.deposit(dest, req_body.amount)
        })
}

fn process_withdraw(
    req_body: TransactionRequest,
    acc_manager: Data<AccountManager>,
) -> Result<Account, ErrorKind> {
    req_body
        .origin
        .map_or(Err(ErrorKind::InvalidInput), |origin| {
            acc_manager.withdraw(origin, req_body.amount)
        })
}

fn process_transfer(
    req_body: TransactionRequest,
    acc_manager: Data<AccountManager>,
) -> Result<(Account, Account), ErrorKind> {
    match (req_body.origin, req_body.destination) {
        (Some(origin), Some(destination)) => {
            acc_manager.transfer(origin, destination, req_body.amount)
        }
        _ => return Err(ErrorKind::NotFound),
    }
}
