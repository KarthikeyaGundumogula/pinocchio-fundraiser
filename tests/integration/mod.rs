use pinocchio_fundraiser::state::Fundraiser;
use solana_sdk::{pubkey::Pubkey, signer::Signer};

use crate::{fixtures::AMOUNT_TO_RAISE, instructions::{send_contribution_transaction, send_initialize_transaction}, setup, utils::set_clock};

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
    send_contribution_transaction(&mut ctx, 500_000);
}