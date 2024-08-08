use std::collections::HashMap;

use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct ExchangeRates{
    pub rates:Vec<ExchangeRate>
}

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct ExchangeRate {
    pub base: String,
    pub date: String,
    pub time_last_updated: String,
    pub rates: HashMap<String,f64>,
    pub timestamp: String,
}

impl ExchangeRate {
    pub fn new(base:String, date:String, time_last_updated:String, rates:HashMap<String,f64>, timestamp:String) -> Self {
        ExchangeRate {
        base:base.to_string(),
        date:date.to_string(),
        time_last_updated:time_last_updated.to_string(),
        rates:rates,
        timestamp:timestamp.to_string()
    }
}
}

impl Default for ExchangeRate {
    fn default() -> Self {
        ExchangeRate {
            base:String::new(),
            date:String::new(),
            time_last_updated:String::new(),
            rates:HashMap::new(),
            timestamp:String::new(),
        }
    }
    
}

