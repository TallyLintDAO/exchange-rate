use crate::ExchangeRate;
use crate::RATES;
use core::str;
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, 
};
use serde_json::{self, Value};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn fetch_latest_exchange_rate() -> Result<ExchangeRate, String> {
    let host = "open.er-api.com";
    let request_headers = vec![
        HttpHeader {
            name: "Host".to_string(),
            value: format!("{host}:443"),
        },
        HttpHeader {
            name: "User-Agent".to_string(),
            value: "exchange-rate-backend".to_string(),
        },
    ];
    let request = CanisterHttpRequestArgument {
        url:format!("https://{}/v6/latest/USD",host),
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: None,
        transform: None,
        headers: request_headers,
    };
    match http_request(request, 1_603_096_400).await {
        Ok((response,)) => {
            //get value from response
            let str_body = str::from_utf8(&response.body).unwrap().to_string();
            let json_body: Value = serde_json::from_str(str_body.as_str()).unwrap();
            let rates: HashMap<String, f64> =
                serde_json::from_value(json_body["rates"].clone()).unwrap_or_default();
            let time_last_update_utc = json_body
                .get("time_last_update_utc")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string();
            let time_last_update_unix = json_body.get("time_last_update_unix").unwrap().as_u64().unwrap();
            Ok(ExchangeRate::new("USD".to_string(),  time_last_update_utc, rates, time_last_update_unix))
        }
        Err((r, m)) => {
            let message =
                format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");

            //Return the error as a string and end the method
            Err(message)
        }
    }
}

pub async fn store_latest_exchange_rate() {
    match fetch_latest_exchange_rate().await {
        Ok(exchange_rate) => {
            RATES.with(|rates| {
                let mut rates = rates.borrow_mut();
                if !rates.contains(&exchange_rate) {
                    rates.push(exchange_rate.clone());
                    ic_cdk::println!(
                        "Fetched and stored latest exchange rate: {:?}",
                        exchange_rate
                    );
                } else {
                    ic_cdk::println!("Duplicate exchange rate found, not storing");
                }
            });
        }
        Err(e) => {
            ic_cdk::println!("Failed to fetch or store latest exchange rate: {:?}", e);
        }
    }
}


// 获取当前距离UTC时间0点的秒数
// pub fn seconds_until_utc_midnight() -> u64 {
//     const SECONDS_IN_A_DAY: u64 = 24 * 60 * 60;

//     let now = SystemTime::now()
//         .duration_since(UNIX_EPOCH)
//         .expect("Time went backwards")
//         .as_secs();

//     let seconds_since_midnight = now % SECONDS_IN_A_DAY;
//     SECONDS_IN_A_DAY - seconds_since_midnight
// }
