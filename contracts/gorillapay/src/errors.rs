use soroban_sdk::{contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    Paused = 1,
    DivideByZero = 2,
    InvalidAmount = 3,
    InvalidAddress = 4,
    NotFound = 5,
    Unauthorized = 6,
}