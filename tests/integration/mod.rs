use litesvm_token::{get_spl_account, spl_token::state::Account};
use pinocchio_fundraiser::state::{Contribution, Fundraiser};
use solana_sdk::{pubkey::Pubkey, signer::Signer};

use crate::{
    fixtures::{AMOUNT_TO_RAISE, DONATION_AMOUNT, DURATION_IN_DAYS},
    instructions::{
        send_checkout_transaction, send_contribution_transaction, send_initialize_transaction,
    },
    setup,
    utils::set_clock,
};

#[test]
pub fn test_init_inx() {
    let mut ctx = setup();
    send_initialize_transaction(&mut ctx);
    let pda = ctx
        .svm
        .get_account(&ctx.fundraiser)
        .expect("Account not found");
    let fundraise_pda =
        ::wincode::deserialize::<Fundraiser>(&pda.data).expect("unable to deserialize ");
    println!("{:?}", fundraise_pda.maker);
    assert_eq!(
        ctx.maker.pubkey(),
        Pubkey::new_from_array(fundraise_pda.maker)
    );
    assert_eq!(
        AMOUNT_TO_RAISE,
        u64::from_le_bytes(fundraise_pda.amount_to_raise)
    );
}

#[test]
pub fn test_contribution_inx() {
    let mut ctx = setup();
    set_clock(&mut ctx.svm, 1000);
    send_initialize_transaction(&mut ctx);
    send_contribution_transaction(&mut ctx, DONATION_AMOUNT);
    let pda = ctx
        .svm
        .get_account(&ctx.contribution)
        .expect("account not found");
    let contributor_data =
        ::wincode::deserialize::<Contribution>(&pda.data).expect("unable to deserialize");
    assert_eq!(contributor_data.amount, DONATION_AMOUNT);
}

#[test]
pub fn test_checkout_inx() {
    let mut ctx = setup();
    set_clock(&mut ctx.svm, 1000);
    send_initialize_transaction(&mut ctx);
    send_contribution_transaction(&mut ctx, AMOUNT_TO_RAISE);
    set_clock(&mut ctx.svm, DURATION_IN_DAYS as i64 * 86_400 * 2);
    send_checkout_transaction(&mut ctx);
    let maker_ata: Account =
        get_spl_account(&ctx.svm, &ctx.maker_ata).expect("token account not found");
    assert_eq!(maker_ata.amount, AMOUNT_TO_RAISE);
}

#[should_panic]
#[test]
pub fn test_checkout_inx_fails_if_amount_not_raised() {
    let mut ctx = setup();
    set_clock(&mut ctx.svm, 1000);
    send_initialize_transaction(&mut ctx);
    send_contribution_transaction(&mut ctx, DONATION_AMOUNT);
    set_clock(&mut ctx.svm, DURATION_IN_DAYS as i64 * 86_400 * 2);
    send_checkout_transaction(&mut ctx);
}
