use crate::icrc1_total_supply;
use common::*;

use candid::{Nat, Principal};
use std::{cell::RefCell, collections::BTreeMap};

/// Maps `Principal` to the balance of the account.
type BalanceStore = BTreeMap<Account, Balance>;

thread_local! {
    static BALANCES: RefCell<BalanceStore> = RefCell::default();
    static INITIAL_OWNER: RefCell<Option<Principal>> = RefCell::default();
}

pub fn do_get_balance_of(account: Account) -> Balance {
    BALANCES.with(|balances| -> Balance {
        if let Some(balance) = balances.borrow().get(&account) {
            balance.clone()
        } else {
            into_units(0)
        }
    })
}

pub fn do_get_initial_owner() -> Option<Principal> {
    INITIAL_OWNER.with(|initial_owner| *initial_owner.borrow())
}

pub fn do_transfer(caller: Principal, arg: TransferArg) -> Result<Nat, TransferError> {
    let from_account = Account {
        owner: caller,
        subaccount: arg.from_subaccount,
    };

    let amount = arg.amount;
    let balance = do_get_balance_of(from_account);

    if balance < amount {
        return Err(TransferError::InsufficientFunds { balance });
    }

    if amount == into_units(0) {
        return Err(TransferError::BadBurn { min_burn_amount: 1 });
    }

    let beneficiary = arg.to;
    if beneficiary == from_account {
        return Err(TransferError::GenericError {
            error_code: Nat::from(101),
            message: "Cannot transfer tokens to yourself".to_string(),
        });
    }

    BALANCES.with(|balances| -> Result<Nat, TransferError> {
        let mut balances = balances.borrow_mut();
        let sender_new_balance = balance - amount.clone();
        balances.insert(from_account, sender_new_balance);

        let beneficiary_old_balance = if let Some(balance) = balances.get(&beneficiary) {
            balance.clone()
        } else {
            into_units(0)
        };

        let beneficiary_new_balance = beneficiary_old_balance + amount;

        // This should never happen, only a defensive check.
        if beneficiary_new_balance > icrc1_total_supply() {
            return Err(TransferError::GenericError {
                error_code: Nat::from(102),
                message: "New balance cannot be greater than total supply".to_string(),
            });
        }
        balances.insert(beneficiary, beneficiary_new_balance);

        Ok(Nat::from(0))
    })
}

pub fn do_init(caller: Principal) -> Result<Nat, &'static str> {
    let creator = Account {
        owner: caller,
        subaccount: None,
    };

    INITIAL_OWNER.with(|owner| -> Result<(), &'static str> {
        let mut maybe_owner = owner.borrow_mut();
        if maybe_owner.is_some() {
            return Err("The initial owner was already set");
        }
        *maybe_owner = Some(caller);
        Ok(())
    })?;

    BALANCES.with(|balances| {
        let mut balances = balances.borrow_mut();

        balances.insert(creator, icrc1_total_supply());
    });

    Ok(Nat::from(0))
}

pub fn into_units(amount: u32) -> Balance {
    amount as u128 * 10u32.pow(DECIMALS.into()) as u128
}
