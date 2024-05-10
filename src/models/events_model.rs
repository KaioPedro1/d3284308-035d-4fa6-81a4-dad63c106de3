use serde::Deserialize;


#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransactionType {
    Deposit,
    Withdraw,
    Transfer,
}

#[derive(Debug, Deserialize)]
pub struct DepositRequest {
    #[serde(rename(deserialize = "type"))]
    pub type_: TransactionType,
    pub destination: Option<String>,
    pub amount: u64,
    pub origin: Option<String>,

}