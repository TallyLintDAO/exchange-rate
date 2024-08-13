use std::collections::HashMap;
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::cmp::Eq;

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct ExchangeRates{
    pub rates:Vec<ExchangeRate>
}

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct ExchangeRate {
    pub base_code: String,
    pub time_last_update_utc: String,
    #[serde(skip)]
    pub rates: HashMap<String,f64>,
    pub time_last_update_unix: u64,
}

impl ExchangeRate {
    pub fn new(base_code:String, time_last_update_utc:String, rates:HashMap<String,f64>, time_last_update_unix:u64) -> Self {
        ExchangeRate {
        base_code:base_code.to_string(),
        time_last_update_utc:time_last_update_utc.to_string(),
        rates:rates,
        time_last_update_unix:time_last_update_unix
    }
}
}

impl Default for ExchangeRate {
    fn default() -> Self {
        ExchangeRate {
            base_code:String::new(),
            time_last_update_utc:String::new(),
            rates:HashMap::new(),
            time_last_update_unix:0,
        }
    }
    
}

impl PartialEq for ExchangeRate {
    fn eq(&self, other: &Self) -> bool {
        self.base_code == other.base_code &&
        self.time_last_update_utc == other.time_last_update_utc &&
        self.time_last_update_unix == other.time_last_update_unix 
        // && self.rates.keys().eq(other.rates.keys())  
    }
}

impl Eq for ExchangeRate {}

