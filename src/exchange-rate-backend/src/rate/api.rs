use crate::ExchangeRate;
use super::service::{store_latest_exchange_rate};
// use ic_cdk_macros::*;
use ic_cdk::{query,update};
use crate::common::guard::admin_guard;
// use ic_cdk_macros::update;
use ic_cdk_timers::{set_timer_interval,set_timer};
use std::time::Duration;
use crate::RATES;
use ic_cdk::init;
// the code below is used to test
use std::collections::HashMap;

#[init]
fn init() {
    schedule_daily_task();
}

fn schedule_daily_task() {
    /**
     * new idea: excute store_latest_exchange_rate() as soon as the exchange rate is updated
     */
    //excute store_latest_exchange_rate() when deploy the canister
    let initial_delay = 0;
    //excute store_latest_exchange_rate() every 24 hours
    let day_duration_secs = 24 * 60 * 60;
    ic_cdk_timers::set_timer(Duration::from_secs(initial_delay), move || {
        ic_cdk::spawn(async move {
            store_latest_exchange_rate().await;
            ic_cdk_timers::set_timer_interval(Duration::from_secs(day_duration_secs), || {
                ic_cdk::spawn(store_latest_exchange_rate());
            });
        });
    });
}


// #[query(name = "exchange_rate/latest")]
// #[query(guard="admin_guard")]
// fn get_latest_exchange_rate() -> Option<ExchangeRate> {
//     RATES.with(|rates| {
//         rates.borrow().last().cloned()
//     })
// }
#[query(guard = "admin_guard")]
fn get_latest_exchange_rate() -> Result<Option<ExchangeRate>, String> {
    RATES.with(|rates| {
        let latest_rate = rates.borrow().last().cloned();
        if latest_rate.is_none() {
            return Err("No exchange rates available.".to_string());
        }
        Ok(latest_rate)
    })
}


// #[query(name = "exchange_rate/update")]
#[update(guard="admin_guard")]
async fn update_exchange_rate() {
    store_latest_exchange_rate().await;
}



// #[cfg(test)]
// mod tests {
//     // 导入模块和函数
//     use super::*;

//     // 你可能需要引入其他依赖或 mock 库，例如 `tokio` 和 `async-std` 来支持异步测试
//     use tokio;

//     #[tokio::test] // 使用 tokio 测试异步代码
//     async fn test_store_latest_exchange_rate() {
//         // 模拟 fetch_latest_exchange_rate 的返回值
//         let mock_rate = ExchangeRate {
//             base_code: "USD".to_string(),
//             time_last_update_utc: "Wed, 14 Aug 2024 00:13:01 +0000".to_string(),
//             rates: {
//                 let mut rates = HashMap::new();
//                 rates.insert("EUR".to_string(), 0.85);
//                 rates
//             },
//             timestamp: 1692039601,
//         };

//         // 模拟 fetch_latest_exchange_rate 函数
//         async fn fetch_latest_exchange_rate() -> Result<ExchangeRate, FetchError> {
//             Ok(mock_rate.clone())
//         }

//         // 执行测试
//         store_latest_exchange_rate().await;

//         // 检查 RATES 中是否存储了 mock_rate
//         RATES.with(|rates| {
//             let rates = rates.borrow();
//             assert!(rates.contains(&mock_rate));
//         });
//     }

//     // 你可以添加更多测试用例
// }


