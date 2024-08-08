use crate::ExchangeRate;
use crate::RATES;
use reqwest::Error;
use serde_json::Value;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use super::error::FetchError;

pub async fn fetch_latest_exchange_rate() -> Result<ExchangeRate, FetchError> {
    let response: Value = reqwest::get("https://api.exchangerate-api.com/v4/latest/USD")
        .await?
        .json()
        .await?;

    let rates: HashMap<String, f64> = serde_json::from_value(response["rates"].clone())?;
    let time_last_updated = response["time_last_updated"]
        .as_str()
        .unwrap_or("")
        .to_string();
    let date = response["date"].as_str().unwrap_or("").to_string();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();

    Ok(ExchangeRate::new(
        "USD".to_string(),
        date,
        time_last_updated,
        rates,
        timestamp,
    ))
}


pub async fn store_latest_exchange_rate() {
    match fetch_latest_exchange_rate().await {
        Ok(exchange_rate) => {
            RATES.with(|rates| {
                rates.borrow_mut().push(exchange_rate.clone());
            });
            ic_cdk::println!(
                "Fetched and stored latest exchange rate: {:?}",
                exchange_rate
            );
        }
        Err(e) => {
            ic_cdk::println!("Failed to fetch or store latest exchange rate: {:?}", e);
        }
    }
}

// 获取当前距离UTC时间0点的秒数
pub fn seconds_until_utc_midnight() -> u64 {
    const SECONDS_IN_A_DAY: u64 = 24 * 60 * 60;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let seconds_since_midnight = now % SECONDS_IN_A_DAY;
    SECONDS_IN_A_DAY - seconds_since_midnight
}
