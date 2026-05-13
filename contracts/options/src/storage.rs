//! Storage keys and helpers for the options contract.

use interfaces::OptionData;
use soroban_sdk::{contracttype, Env};

#[contracttype]
pub enum DataKey {
    NextId,
    Option(u64),
}

pub fn get_option(env: &Env, id: u64) -> OptionData {
    env.storage()
        .persistent()
        .get(&DataKey::Option(id))
        .expect("option not found")
}

pub fn option_count(env: &Env) -> u64 {
    env.storage().instance().get(&DataKey::NextId).unwrap_or(0)
}

#[allow(dead_code)]
pub fn next_id(env: &Env) -> u64 {
    let id: u64 = env.storage().instance().get(&DataKey::NextId).unwrap_or(0);
    env.storage().instance().set(&DataKey::NextId, &(id + 1));
    id
}

#[allow(dead_code)]
pub fn save_option(env: &Env, option: &OptionData) {
    env.storage()
        .persistent()
        .set(&DataKey::Option(option.id), option);
}
