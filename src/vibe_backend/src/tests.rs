use super::*;

#[test]
fn icrc1_init_works() {
    let creator = get_creator();
    assert!(do_init(creator).is_ok());

    let creator_account = Account {
        owner: creator,
        subaccount: None,
    };

    assert_eq!(do_get_balance_of(creator_account), icrc1_total_supply());
    assert_eq!(do_get_initial_owner(), Some(creator));

    assert_eq!(do_init(creator), Err("The initial owner was already set"));
}

#[test]
fn icrc1_transfer_works() {
    let caller = get_creator();
    let _ = do_init(caller);

    let caller_account = Account {
        owner: caller,
        subaccount: None,
    };

    let beneficiary = Account {
        owner: get_default_principal(),
        subaccount: None,
    };

    let transfer_arg = TransferArg {
        from_subaccount: None,
        to: beneficiary,
        fee: None,
        created_at_time: None,
        memo: None,
        amount: into_units(1_000),
    };

    assert_eq!(do_transfer(caller, transfer_arg), Ok(Nat::from(0)));

    assert_eq!(
        do_get_balance_of(caller_account),
        icrc1_total_supply() - into_units(1_000)
    );
    assert_eq!(do_get_balance_of(beneficiary), into_units(1_000));
}

#[test]
fn icrc1_transfer_fails_when_balance_insufficient() {
    let creator = get_creator();
    let _ = do_init(creator);

    let sender = Account {
        owner: get_default_principal(),
        subaccount: None,
    };

    let beneficiary = Account {
        owner: creator,
        subaccount: None,
    };

    let transfer_arg = TransferArg {
        from_subaccount: None,
        to: beneficiary,
        fee: None,
        created_at_time: None,
        memo: None,
        amount: into_units(42_000),
    };

    assert_eq!(do_get_balance_of(sender), 0);
    assert_eq!(do_get_balance_of(beneficiary), icrc1_total_supply());

    assert_eq!(
        do_transfer(sender.owner, transfer_arg),
        Err(TransferError::InsufficientFunds {
            balance: into_units(0)
        })
    );

    assert_eq!(do_get_balance_of(sender), 0);
    assert_eq!(do_get_balance_of(beneficiary), icrc1_total_supply());
}

#[test]
fn icrc1_transfer_fails_when_zero_amount() {
    let caller = get_creator();
    let _ = do_init(caller);

    let beneficiary = Account {
        owner: get_default_principal(),
        subaccount: None,
    };

    let transfer_arg = TransferArg {
        from_subaccount: None,
        to: beneficiary,
        fee: None,
        created_at_time: None,
        memo: None,
        amount: into_units(0),
    };

    assert_eq!(
        do_transfer(caller, transfer_arg),
        Err(TransferError::BadBurn { min_burn_amount: 1 })
    );
}

#[test]
fn icrc1_transfer_fails_when_transferring_to_yourself() {
    let caller = get_creator();
    let _ = do_init(caller);

    let beneficiary = Account {
        owner: caller,
        subaccount: None,
    };

    let transfer_arg = TransferArg {
        from_subaccount: None,
        to: beneficiary,
        fee: None,
        created_at_time: None,
        memo: None,
        amount: into_units(100),
    };

    assert_eq!(
        do_transfer(caller, transfer_arg),
        Err(TransferError::GenericError {
            error_code: Nat::from(101),
            message: "Cannot transfer tokens to yourself".to_string()
        })
    );
}

fn get_creator() -> Principal {
    Principal::from_text("arlij-g2zpo-epfot-36ufg-vm4gj-3j4tj-rsjjt-fsv2m-sp4z7-nnk6b-lqe").unwrap()
}

fn get_default_principal() -> Principal {
    Principal::from_text("2vxsx-fae").unwrap()
}
