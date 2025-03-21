#![no_std]

use soroban_sdk::{contracttype, Address};

trait Ownable {
    fn is_owner(&self, owner: &Address) -> bool;
}

impl Ownable for OwnableContract {
    fn is_owner(&self, owner: &Address) -> bool {
        self.owner == *owner
    }
}

fn only_owner(contract: &OwnableContract, owner: &Address) -> bool {
    contract.is_owner(owner)
}

#[contracttype]

pub struct OwnableContract {
    owner: Address,
    number: u32,
}