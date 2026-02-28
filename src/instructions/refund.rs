use pinocchio::{
    AccountView, ProgramResult,
    cpi::{Seed, Signer},
    error::ProgramError,
    sysvars::{Sysvar, clock::Clock},
};
use pinocchio_log::log;
use pinocchio_pubkey::derive_address;
use wincode::SchemaRead;

use crate::{
    constants::SECONDS_TO_DAYS,
    state::{Contribution, Fundraiser},
};

#[derive(SchemaRead)]
pub struct RefundData {
    contribution_bump: u8,
}

pub fn process_refund(accounts: &[AccountView], data: &[u8]) -> ProgramResult {
    let [
        contributor,
        maker,
        mint,
        fundraiser_acc,
        contribution_acc,
        contributor_ata,
        vault_ata,
        _token_program,
        _system_program @ ..,
    ] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    let (refund_amount, bump) = {
        let contributor_pda = unsafe { contribution_acc.borrow_unchecked() };
        let contribution_data = ::wincode::deserialize::<Contribution>(contributor_pda)
            .map_err(|_| ProgramError::InvalidAccountData)?;
        let fundraise_pda = unsafe { fundraiser_acc.borrow_unchecked() };
        let fundraiser_state = wincode::deserialize::<Fundraiser>(fundraise_pda)
            .map_err(|_| ProgramError::InvalidInstructionData)?;
        let ix_data = ::wincode::deserialize::<RefundData>(data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;
        let contribution_bump = ix_data.contribution_bump;
        let contribution_bump_bytes = [contribution_bump];
        let expected_contribution = derive_address(
            &[
                b"contributor",
                fundraiser_acc.address().as_array(),
                contributor.address().as_array(),
                &contribution_bump_bytes,
            ],
            None,
            &crate::ID.to_bytes(),
        );
        if contribution_acc.address().as_array() != &expected_contribution {
          return Err(ProgramError::InvalidAccountData);
        }
        log!("passed pda check");
        if !contributor.is_signer() {
          return Err(ProgramError::MissingRequiredSignature);
        }
        let current_time = Clock::get()?.unix_timestamp.to_le_bytes();
        assert!(
          fundraiser_state.duration
          < ((u64::from_le_bytes(current_time)
          - u64::from_le_bytes(fundraiser_state.time_started))
          / SECONDS_TO_DAYS) as u8,
          "Fundraise Duration is not Over"
        );
        log!("started validating");
        
        assert!(
            u64::from_le_bytes(fundraiser_state.amount_to_raise)
                > u64::from_le_bytes(fundraiser_state.current_amount),
            "Amount is reached"
        );

        if *mint.address().as_array() != fundraiser_state.mint {
            return Err(ProgramError::InvalidAccountData);
        }

        (contribution_data.amount, fundraiser_state.bump)
    };
    let bump = [bump];
    let seed = [
        Seed::from(b"fundraiser"),
        Seed::from(maker.address().as_array()),
        Seed::from(&bump),
    ];
    let signer = Signer::from(&seed[..]);
    pinocchio_token::instructions::Transfer {
        from: vault_ata,
        to: contributor_ata,
        authority: fundraiser_acc,
        amount: refund_amount,
    }
    .invoke_signed(&[signer.clone()])?;

    let contribution_lamports = contribution_acc.lamports();
    contributor.set_lamports(contribution_lamports + contributor.lamports());
    contribution_acc.set_lamports(0);

    let close = {
        let vault_ata_state = pinocchio_token::state::TokenAccount::from_account_view(vault_ata)?;
        if vault_ata_state.amount() == 0 {
            true
        } else {
            false
        }
    };
    if close {
        pinocchio_token::instructions::CloseAccount {
            account: vault_ata,
            destination: maker,
            authority: fundraiser_acc,
        }
        .invoke_signed(&[signer])?;
        let fundraise_lamports = fundraiser_acc.lamports();
        maker.set_lamports(maker.lamports() + fundraise_lamports);
        fundraiser_acc.set_lamports(0);
    }
    Ok(())
}
