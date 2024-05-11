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
    pub fn transfer(&self, origin_key: String, destination_key: String, value:u64)-> Result<(Account, Account), ErrorKind>{
        let mut accounts = self.accounts.lock().unwrap();
        let origin_balance = accounts.get(&origin_key).cloned();
        let  destination_balance  = accounts.get(&destination_key).cloned();
        match (origin_balance, destination_balance) {
            (Some(mut og_balance), Some(mut dest_balance)) => {
                if og_balance>=value{
                    og_balance-=value;
                    dest_balance+=value;
                    accounts.insert(origin_key.clone(), og_balance);
                    accounts.insert(destination_key.clone(), dest_balance);
                    return Ok((Account::new(origin_key, og_balance), Account::new(destination_key, dest_balance)))
                }else {
                    return Err(ErrorKind::NotFound)
                }
            },
            _=> return Err(ErrorKind::NotFound)
        }
    }
}
