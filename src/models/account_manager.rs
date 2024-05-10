use std::{collections::HashMap, sync::Mutex};

pub struct AccountManager  {
    pub accounts: Mutex<HashMap<String, u64>>, 
}


impl AccountManager {
    pub fn clear_accounts(&self) {
        let mut accounts = self.accounts.lock().unwrap();
        accounts.clear();
    }

    pub fn new() -> Self {
        AccountManager {
            accounts: Mutex::new(HashMap::new()),
        }
    }
}