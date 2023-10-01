// mod adapter;
mod pricing;
mod store;

use ic_cdk::{query, update};

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[query]
async fn get_balance() -> f64 {
    19.
}
