use crate::ExchangeRate;
use super::service::{store_latest_exchange_rate, seconds_until_utc_midnight};
// use ic_cdk_macros::*;
use ic_cdk::{query,update};
// use ic_cdk_macros::update;
use ic_cdk_timers::{set_timer_interval,set_timer};
use std::time::Duration;
use crate::RATES;
use ic_cdk::init;

#[init]
fn init() {
    schedule_daily_task();
}

fn schedule_daily_task() {
    let initial_delay = seconds_until_utc_midnight();
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


#[query(name = "exchange_rate/latest")]
fn get_latest_exchange_rate() -> Option<ExchangeRate> {
    RATES.with(|rates| {
        rates.borrow().last().cloned()
    })
}

#[update(name = "exchange_rate/update")]
async fn update_exchange_rate() {
    store_latest_exchange_rate().await;
}
