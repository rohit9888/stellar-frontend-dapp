#![cfg(test)]
use soroban_sdk::{
    log,
    testutils::{Address as _, Ledger, LedgerInfo,},
    token::{ TokenClient, StellarAssetClient},
    Address, Env,
    Val
     };
use crate::{gorillapay::GorillaPayContract, pause};
use crate::gorillapay::GorillaPayContractClient;
use crate::errors::Error;

fn create_token_contract<'a>(env: Env, admin: Address) -> (Address, TokenClient<'a>, StellarAssetClient<'a>) {
    let token_address = env.register_stellar_asset_contract_v2(admin.clone()).address();
    let token_client = TokenClient::new(&env, &token_address);
    let asset_client = StellarAssetClient::new(&env, &token_address);

   
    (token_address, token_client,asset_client)
}

fn create_test_env() -> (Env, Address, Address, Address, TokenClient<'static>, GorillaPayContractClient<'static>, Address ) {
    let env = Env::default();
    env.mock_all_auths();
    
    env.ledger().set(LedgerInfo {
        timestamp: 12345,
        protocol_version: 22,
        sequence_number: 10,
        network_id: Default::default(),
        base_reserve: 10,
        max_entry_ttl: 100000,
        min_persistent_entry_ttl: 1000,
        min_temp_entry_ttl: 1000,
    });

    let admin = Address::generate(&env);
    let merchant = Address::generate(&env);
    let user = Address::generate(&env);
    let admin_clone = admin.clone();
    let env_clone = env.clone();
    
    // Create token
    let (token_address, token_client,asset_client) = create_token_contract(env_clone, admin_clone);
    
    // Deploy and initialize contract
    let contract_address = env.register(GorillaPayContract, (&admin,) );
    let contract_client = GorillaPayContractClient::new(&env, &contract_address);
    
    contract_client.set_sep41(&token_address);
    contract_client.set_fees_parameter(&1, &1); 
    
    asset_client.mint(&user, &1000);
    
    (env, admin, user, merchant, token_client, contract_client, contract_address)
}

// /*
#[test]
fn test_successful_payment() {
    let (env, _admin, user, merchant, token, contract_client,contract_address) = create_test_env();
        
        token.approve(&user, &contract_address, &1000, &1000);
        let (fee_num, _fee_den) = contract_client.get_fees_parameter();
        // env.logger().log("Fee Numerator:------------------------", &[Val::from(fee_num)]);
        contract_client.pay_merchant(&merchant, &100, &user);
        log!(&env,"Simulated User Address:------------------------", user);
    
    assert_eq!(token.balance(&user), 900);
    assert_eq!(token.balance(&merchant), 99);
    assert_eq!(token.balance(&contract_address), 1);
}


#[test]
#[should_panic(expected = "HostError: Error(Contract, #3)")]
fn test_payment_with_zero_amount() {
    let (_env, _admin, user, merchant, token, contract_client, contract_address) = create_test_env();
    token.approve(&user, &contract_address, &1000, &1000);

    contract_client.pay_merchant(&merchant, &0, &user);
    
}



#[test]
#[should_panic(expected = "HostError: Error(Contract, #10)")]
fn test_payment_with_insufficient_balance() {
    let (_env, _admin, user, merchant, token, contract_client,contract_address) = create_test_env();
    token.approve(&user, &contract_address, &2000, &2000);
    contract_client.pay_merchant(&merchant, &1500, &user);
}

#[test]
fn test_admin_fee_withdrawal() {
    let (_env, admin, user, merchant, token, contract_client,contract_address) = create_test_env();
    
    token.approve(&user, &contract_address, &1000, &1000);
    contract_client.pay_merchant(&merchant, &100, &user);
    
    contract_client.withdraw_admin_fees(&1);
    
    assert_eq!(token.balance(&admin), 1);
    assert_eq!(token.balance(&contract_address), 0);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #1)")]
fn test_pause_functionality() {
    let (env, _admin, user, merchant, token, contract_client,contract_address) = create_test_env();
    
    env.as_contract(&contract_address, || {
                pause::set(&env, true);
    });
    
    token.approve(&user, &contract_address, &1000, &1000);
    contract_client.pay_merchant(&merchant, &100, &user);

}
