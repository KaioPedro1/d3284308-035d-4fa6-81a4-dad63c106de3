use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransactionType {
    Deposit,
    Withdraw,
    Transfer,
}
#[derive(Debug, Serialize)]
pub struct Account {
    id: String,
    balance: u64,
}
impl Account {
    pub fn new(id: String, balance: u64) -> Self {
        Self { id, balance }
    }
}
#[derive(Debug, Deserialize)]
pub struct TransactionRequest {
    #[serde(rename(deserialize = "type"))]
    pub type_: TransactionType,
    pub destination: Option<String>,
    pub amount: u64,
    pub origin: Option<String>,
}
#[derive(Debug, Serialize)]
pub struct TransactionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Account>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<Account>,
}