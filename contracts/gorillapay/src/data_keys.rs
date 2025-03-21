use soroban_sdk::contracttype;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Sep41,
    FeeDenominator,
    FeeNumerator,
    AdminTotalDeposit,
    AdminTotalWithdraw,
    Admin,
}