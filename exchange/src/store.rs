use candid::Principal;
use std::collections::BTreeMap;
use std::{cell::RefCell, collections::BTreeSet};

pub struct TokenPool {
    name: String,
    amount: f64,
    price: f32,
}

pub struct Balance {
    total_value: f64,
    token_balance: BTreeMap<Principal, f64>,
}

pub type TokenIdToTokenPool = BTreeMap<Principal, TokenPool>;
pub type AddressToBalance = BTreeMap<Principal, Balance>;

thread_local! {
    static TOKEN_POOL: RefCell<TokenIdToTokenPool> = RefCell::default();
    static TOKEN_BALANCE: RefCell<AddressToBalance> = RefCell::default();
}
