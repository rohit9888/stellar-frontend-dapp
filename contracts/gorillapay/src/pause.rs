use soroban_sdk::{contracttype, Address, Env};
use crate::data_keys::DataKey;

#[contracttype]
pub enum PauseKey {
    Paused,
}

pub fn paused(env: &Env) -> bool {
    env.storage().instance().get(&PauseKey::Paused).unwrap_or(false)
}

pub fn set(env: &Env, status: bool) {
    let admin: Address = env.storage()
        .instance()
        .get(&DataKey::Admin)
        .expect("Admin not set");
    admin.require_auth(); 
    env.storage().instance().set(&PauseKey::Paused, &status);
}