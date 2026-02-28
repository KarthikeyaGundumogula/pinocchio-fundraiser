use solana_sdk::message::{AccountMeta, Instruction};
use solana_sdk::signer::Signer;

use crate::fixtures::TOKEN_PROGRAM_ID;
use crate::utils::send_transaction;
use crate::{
    TestContext,
    fixtures::{AMOUNT_TO_RAISE, DURATION_IN_DAYS, program_id},
};

pub fn send_initialize_transaction(ctx: &mut TestContext) {
    let amount_to_raise_bytes: [u8; 8] = {
        let mut arr = [0u8; 8];
        arr[..8].copy_from_slice(&AMOUNT_TO_RAISE.to_le_bytes());
        arr
    };

    let init_data = [
        vec![0u8],
        ctx.fundraiser_bump.to_le_bytes().to_vec(),
        amount_to_raise_bytes.to_vec(),
        DURATION_IN_DAYS.to_le_bytes().to_vec(),
    ]
    .concat();

    let init_ix = Instruction {
        program_id: program_id(),
        accounts: vec![
            AccountMeta::new(ctx.maker.pubkey(), true),
            AccountMeta::new(ctx.mint, false),
            AccountMeta::new(ctx.fundraiser, false),
            AccountMeta::new(ctx.vault_ata, false),
            AccountMeta::new(ctx.system_program, false),
            AccountMeta::new(TOKEN_PROGRAM_ID, false),
            AccountMeta::new(ctx.associated_token_program, false),
        ],
        data: init_data,
    };

    let maker_pubkey = ctx.maker.pubkey();

    for (index, account) in init_ix.accounts.iter().enumerate() {
        println!("Account {}: {}", index, account.pubkey);
    }

    send_transaction(&mut ctx.svm, init_ix, &[&ctx.maker], &maker_pubkey);
}

pub fn send_contribution_transaction(ctx: &mut TestContext, amount: u64) {
    let amount_bytes: [u8; 8] = {
        let mut arr = [0u8; 8];
        arr[..8].copy_from_slice(&amount.to_le_bytes());
        arr
    };

    let contribution_data = [
        vec![1u8],
        ctx.contribution_bump.to_le_bytes().to_vec(),
        amount_bytes.to_vec(),
    ]
    .concat();

    let contribution_ix = Instruction {
        program_id: program_id(),
        accounts: vec![
            AccountMeta::new(ctx.donar.pubkey(), true),
            AccountMeta::new(ctx.mint, false),
            AccountMeta::new(ctx.fundraiser, false),
            AccountMeta::new(ctx.donar_ata, false),
            AccountMeta::new(ctx.contribution, false),
            AccountMeta::new(ctx.vault_ata, false),
            AccountMeta::new(TOKEN_PROGRAM_ID, false),
            AccountMeta::new(ctx.system_program, false),
        ],
        data: contribution_data,
    };

    let contributor_pubkey = ctx.donar.pubkey();

    for (index, account) in contribution_ix.accounts.iter().enumerate() {
        println!("Account {}: {}", index, account.pubkey);
    }

    send_transaction(
        &mut ctx.svm,
        contribution_ix,
        &[&ctx.donar],
        &contributor_pubkey,
    );
}

pub fn send_checkout_transaction(ctx: &mut TestContext) {
    let ix_data = [vec![2u8]].concat();

    let checkout_inx = Instruction {
        program_id: program_id(),
        accounts: vec![
            AccountMeta::new(ctx.maker.pubkey(), true),
            AccountMeta::new(ctx.mint, false),
            AccountMeta::new(ctx.fundraiser, false),
            AccountMeta::new(ctx.vault_ata, false),
            AccountMeta::new(ctx.maker_ata, false),
            AccountMeta::new(TOKEN_PROGRAM_ID, false),
            AccountMeta::new(ctx.system_program, false),
            AccountMeta::new(ctx.associated_token_program, false),
        ],
        data: ix_data,
    };

    let maker_pubkey = ctx.maker.pubkey();

    for (index, account) in checkout_inx.accounts.iter().enumerate() {
        println!("Account {}: {}", index, account.pubkey);
    }

    send_transaction(&mut ctx.svm, checkout_inx, &[&ctx.maker], &maker_pubkey);
}
