use std::cell::RefCell;

use crate::rate::domain::ExchangeRate;

thread_local! {
    static RATES:RefCell<Vec<ExchangeRate>> = RefCell::new(Vec::new());
}

pub mod rate;
#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
