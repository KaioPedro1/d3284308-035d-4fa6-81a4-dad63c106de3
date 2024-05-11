use std::{collections::HashMap, io::ErrorKind, sync::Mutex};

use super::Account;

pub struct AccountManager {
    accounts: Mutex<HashMap<String, u64>>,
}

impl AccountManager {
    pub fn new() -> Self {
        AccountManager {
            accounts: Mutex::new(HashMap::new()),
        }
    }

    pub fn clear_accounts(&self) {
        self.accounts.lock().unwrap().clear();
    }

    pub fn deposit(&self, key: String, value: u64) -> Result<Account, ErrorKind> {
        let mut accounts = self.accounts.lock().unwrap();
        let new_amount = accounts.entry(key.clone()).or_insert(0);
        *new_amount += value;
        Ok(Account::new(key, *new_amount))
    }

    pub fn withdraw(&self, key: String, value: u64) -> Result<Account, ErrorKind> {
        let mut accounts = self.accounts.lock().unwrap();
        if let Some(account) = accounts.get_mut(&key) {
            if *account >= value {
                *account -= value;
                Ok(Account::new(key, *account))
            } else {
                Ok(Account::new(key, 0))
            }
        } else {
            Err(ErrorKind::NotFound)
        }
    }
    pub fn transfer(
        &self,
        origin_key: String,
        destination_key: String,
        value: u64,
    ) -> Result<(Account, Account), ErrorKind> {
        let mut accounts = self.accounts.lock().unwrap();
        let mut origin_balance = accounts.get(&origin_key).cloned().unwrap_or(0);
        let mut destination_balance = accounts.get(&destination_key).cloned().unwrap_or(0);
        if origin_balance < value {
            return Err(ErrorKind::NotFound);
        }
        origin_balance -= value;
        destination_balance += value;

        accounts.insert(origin_key.clone(), origin_balance);
        accounts.insert(destination_key.clone(), destination_balance);

        Ok((
            Account::new(origin_key.clone(), origin_balance),
            Account::new(destination_key, destination_balance),
        ))
    }

    pub fn get_balance(&self, key: String)-> Result<u64, ErrorKind>{
        let accounts = self.accounts.lock().unwrap();
        if let Some(balance) = accounts.get(&key){
            return Ok(*balance)
        }else{
            return Err(ErrorKind::NotFound) 
        }
    }
}

