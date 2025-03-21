use soroban_sdk::{contractimpl, contract, token, Address, Env, 
    symbol_short,  panic_with_error,
};
use crate::{
    data_keys::DataKey,
    pause::{paused, set},
    errors::Error,
};

#[contract]
pub struct GorillaPayContract;

#[contractimpl]
impl GorillaPayContract {
    pub fn __constructor(env: Env, admin: Address) {
        env.storage().instance().set(&DataKey::Admin, &admin);
        set(&env, false);
    }
    // -- Admin functions with authorization --
    pub fn set_sep41(env: Env, sep41_address: Address) {
        (&env.current_contract_address()).require_auth();
        env.storage().instance().set(&DataKey::Sep41, &sep41_address);
    }

    pub fn set_fees_parameter(env: Env, fee_numerator: u32, fee_denominator: u32) {
        (&env.current_contract_address()).require_auth();
        if fee_denominator == 0 {
            panic_with_error!(&env, Error::DivideByZero);
        }
        env.storage().instance().set(&DataKey::FeeNumerator, &fee_numerator);
        env.storage().instance().set(&DataKey::FeeDenominator, &fee_denominator);
    }

    pub fn get_fees_parameter(env: &Env) -> (u32, u32) {
        let fee_num = env.storage().instance()
            .get(&DataKey::FeeNumerator)
            .unwrap_or(0);
        let fee_den = env.storage().instance()
            .get(&DataKey::FeeDenominator)
            .unwrap_or(1);  
        (fee_num, fee_den)
    }

    // -- Core functionality with proper error handling --
    pub fn pay_merchant(env: &Env, merchant: Address, amount: i128, user: Address) -> Result<(), Error> {
        if paused(&env) {
            return Err(Error::Paused);
        }
        if amount <= 0 {
            return Err( Error::InvalidAmount);
        }
        // if merchant == Address::from_str(&env, "0") {
        //     panic_with_error!(&env, Error::InvalidAddress);
        // }

        user.require_auth();

        let token_address = env.storage().instance().get(&DataKey::Sep41)
            .ok_or(Error::NotFound)?;

        let token = token::Client::new(&env, &token_address);
        let contract_address = env.current_contract_address();

        // Calculate fees
        let (fee_num, fee_den) = Self::get_fees_parameter(&env);
        let fee = (amount * fee_num as i128) / (100 * fee_den as i128);
        let merchant_amount = amount - fee;

        token.transfer_from(&contract_address,&user, &contract_address, &amount);
        token.transfer(&contract_address, &merchant, &merchant_amount);

        // Update admin fees
        let mut admin_deposit: i128 = env.storage().instance()
            .get(&DataKey::AdminTotalDeposit)
            .unwrap_or(0);
        admin_deposit += fee;
        env.storage().instance().set(&DataKey::AdminTotalDeposit, &admin_deposit);

        // Emit event
        env.events().publish(
            (symbol_short!("payment"), user),
            (merchant, amount, merchant_amount)
        );

        Ok(())
    }

    pub fn withdraw_admin_fees(env: &Env, amount: i128) -> Result<(), Error> {
            let admin: Address = env.storage().instance()
                .get(&DataKey::Admin)
                .ok_or(Error::from(Error::Unauthorized))?;
            admin.require_auth();

        let token_address = env.storage().instance().get(&DataKey::Sep41)
            .ok_or(Error::NotFound)?;
        let token = token::Client::new(&env, &token_address);
        let contract_address = env.current_contract_address();

        if amount <= 0 || amount > token.balance(&contract_address) {
            panic_with_error!(&env, Error::InvalidAmount);
        }

        // Transfer first to prevent reentrancy
        token.transfer(&contract_address, &admin, &amount);

        // Update storage
        let mut total_withdrawn: i128 = env.storage().instance()
            .get(&DataKey::AdminTotalWithdraw)
            .unwrap_or(0);
        total_withdrawn += amount;
        env.storage().instance().set(&DataKey::AdminTotalWithdraw, &total_withdrawn);

        env.events().publish(
            (symbol_short!("withdraw"), admin),
            amount
        );

        Ok(())
    }
}
