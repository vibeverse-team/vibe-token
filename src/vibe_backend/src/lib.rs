/// This file contains the code of the SEG token which implements the ICRC-1
/// standard that can be found here:
/// https://github.com/dfinity/ICRC-1/blob/main/standards/ICRC-1
#[cfg(test)]
mod tests;

mod functions;

use common::*;
use functions::*;

use candid::{Nat, Principal};
use ic_cdk_macros::*;

#[update]
fn initialize(initial_owner: Principal) -> Result<Nat, &'static str> {
    do_init(initial_owner)
}

#[ic_cdk_macros::query]
fn get_initial_owner() -> Option<Principal> {
    do_get_initial_owner()
}

#[ic_cdk_macros::query]
fn icrc1_metadata() -> Vec<&'static str> {
    vec![]
}

#[ic_cdk_macros::query]
fn icrc1_name() -> &'static str {
    "VIBE token"
}

#[ic_cdk_macros::query]
fn icrc1_symbol() -> &'static str {
    "VIBE"
}

#[ic_cdk_macros::query]
fn icrc1_decimals() -> u8 {
    DECIMALS
}

#[ic_cdk_macros::query]
fn icrc1_fee() -> Nat {
    Nat::from(0)
}

#[ic_cdk_macros::query]
fn icrc1_total_supply() -> Balance {
    into_units(TOTAL_SUPPLY.into())
}

#[ic_cdk_macros::query]
fn icrc1_minting_account() -> Option<Principal> {
    None
}

#[ic_cdk_macros::query]
fn icrc1_balance_of(account: Account) -> Balance {
    do_get_balance_of(account)
}

#[update]
fn icrc1_transfer(arg: TransferArg) -> Result<Nat, TransferError> {
    let caller = ic_cdk::api::caller();
    do_transfer(caller, arg)
}

#[ic_cdk_macros::query]
fn icrc1_supported_standards() -> Vec<StandardRecord> {
    vec![StandardRecord {
        name: "ICRC-1".to_string(),
        url: "https://github.com/dfinity/ICRC-1".to_string(),
    }]
}
