use candid::{CandidType, Deserialize, Nat, Principal};
use common::*;
use std::cell::RefCell;

use ic_cdk::api::call::CallResult;
use ic_cdk::call;
use ic_cdk_macros::*;

const TREAUSRY: &str = "jnk6h-xvzbx-yiiny-rlhot-sv22b-y4zfs-f42wn-lrcqv-54cbp-drvh4-wae";

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
enum WicpTxError {
    InsufficientAllowance,
    InsufficientBalance,
    ErrorOperationStyle,
    Unauthorized,
    LedgerTrap,
    ErrorTo,
    Other,
    BlockUsed,
    AmountTooSmall,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum BuyError {
    SupplyTooLow { supply: Nat },
    TransferError,
    LowAllowance { allowance: Nat },
    WicpTxError,
    WicpError,
    VibeTokenNotSet,
    WicpTokenNotSet,
}

thread_local! {
    static VIBE_TOKEN: RefCell<Option<Principal>> = RefCell::default();
    static WICP_TOKEN: RefCell<Option<Principal>> = RefCell::default();
}

#[ic_cdk_macros::query]
fn get_vibe_token() -> Option<Principal> {
    VIBE_TOKEN.with(|vibe| *vibe.borrow())
}

#[ic_cdk_macros::query]
fn get_wicp() -> Option<Principal> {
    WICP_TOKEN.with(|wicp| *wicp.borrow())
}

#[update]
fn initialize(vibe_token: Principal, wicp: Principal) -> Result<Nat, &'static str> {
    VIBE_TOKEN.with(|token| -> Result<(), &'static str> {
        let mut maybe_vibe = token.borrow_mut();
        if maybe_vibe.is_some() {
            return Err("The address of VIBE token already set");
        }
        *maybe_vibe = Some(vibe_token);
        Ok(())
    })?;

    WICP_TOKEN.with(|token| -> Result<(), &'static str> {
        let mut maybe_wicp = token.borrow_mut();
        if maybe_wicp.is_some() {
            return Err("The address of WICP already set");
        }
        *maybe_wicp = Some(wicp);
        Ok(())
    })?;

    Ok(Nat::from(0))
}

// Useful for testing.
#[update]
async fn get_allowance_of(of: Principal) -> Result<Nat, &'static str> {
    let wicp_token = WICP_TOKEN.with(|wicp| {
        let wicp = wicp.borrow();
        if wicp.is_none() {
            return Err("Wicp not set");
        }
        Ok(wicp.unwrap())
    })?;

    let ico_principal = ic_cdk::api::id();

    let maybe_allowance: CallResult<(Nat,)> =
        call(wicp_token, "allowance", (of, ico_principal)).await;
 
    match maybe_allowance {
        Ok(allowance) => Ok(allowance.0),
        Err(_) => Err("Couldn't get the allowance of the caller")
    }
}

#[update]
async fn buy(amount: u128) -> Result<Nat, BuyError> {
    let caller = ic_cdk::api::caller();

    let vibe_token = VIBE_TOKEN.with(|vibe| {
        let vibe = vibe.borrow();
        if vibe.is_none() {
            return Err(BuyError::VibeTokenNotSet);
        }
        Ok(vibe.unwrap())
    })?;

    let ico_principal = ic_cdk::api::id();
    let ico_account = Account {
        owner: ico_principal,
        subaccount: None,
    };

    let maybe_supply: CallResult<(Nat,)> =
        call(vibe_token, "icrc1_balance_of", (ico_account,)).await;

    let supply = match maybe_supply {
        Ok((supply,)) => supply,
        Err(_) => Nat::from(0),
    };

    if supply < amount {
        return Err(BuyError::SupplyTooLow { supply });
    }

    let wicp_token = WICP_TOKEN.with(|wicp| {
        let wicp = wicp.borrow();
        if wicp.is_none() {
            return Err(BuyError::WicpTokenNotSet);
        }
        Ok(wicp.unwrap())
    })?;

    let required_wicp_allowance = amount / 10;

    let maybe_allowance: CallResult<(Nat,)> =
        call(wicp_token, "allowance", (caller, ico_principal)).await;

    match maybe_allowance {
        Ok(allowance) => {
            let allowance = allowance.0;
            if allowance < required_wicp_allowance {
                return Err(BuyError::LowAllowance { allowance });
            }

            let treasuery_principal = Principal::from_text(TREAUSRY).unwrap();

            let result: CallResult<(Result<Nat, WicpTxError>,)> = call(
                wicp_token,
                "transferFrom",
                (caller, treasuery_principal, required_wicp_allowance),
            )
            .await;

            if let Err(_) = result {
                // TODO - return the wicp tx error.
                return Err(BuyError::WicpTxError);
            }
        }
        Err(_) => return Err(BuyError::WicpError),
    }

    let caller_account = Account {
        owner: caller,
        subaccount: None,
    };

    let transfer_arg = TransferArg {
        from_subaccount: None,
        to: caller_account,
        fee: None,
        created_at_time: None,
        memo: None,
        amount,
    };

    let result: CallResult<(Result<Nat, TransferError>,)> =
        call(vibe_token, "icrc1_transfer", (transfer_arg,)).await;

    match result {
        Ok(_) => Ok(Nat::from(0)),
        Err(_) => Err(BuyError::TransferError),
    }
}
