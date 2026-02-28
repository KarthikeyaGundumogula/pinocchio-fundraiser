use pinocchio::{
    AccountView, ProgramResult,
    cpi::{Seed, Signer},
    error::ProgramError,
    sysvars::{Sysvar, clock::Clock},
};
#[allow(unused)]
use pinocchio_log::log;

use crate::{constants::SECONDS_TO_DAYS, state::Fundraiser};

pub fn process_checkout(accounts: &[AccountView], _data: &[u8]) -> ProgramResult {
    let [
        maker,
        mint,
        fundrasier_acc,
        vault_ata,
        maker_ata,
        _token_program,
        _system_program,
        _associated_token_program @ ..,
    ] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let (bump, amount) = {
        let fundraiser_state = Fundraiser::from_account_info(fundrasier_acc)?;
        let maker_ata_state = pinocchio_token::state::TokenAccount::from_account_view(maker_ata)?;
        if !maker.is_signer() && fundraiser_state.maker != *maker.address().as_array() {
            return Err(ProgramError::MissingRequiredSignature);
        }
        if maker_ata_state.owner() != maker.address() {
            return Err(ProgramError::IllegalOwner);
        }
        if maker_ata_state.mint() != mint.address() {
            return Err(ProgramError::InvalidArgument);
        }

        assert!(
            u64::from_le_bytes(fundraiser_state.amount_to_raise)
                <= u64::from_le_bytes(fundraiser_state.current_amount),
            "Amount to raise not reached"
        );


        let current_time = Clock::get()?.unix_timestamp.to_le_bytes();
        assert!(
            fundraiser_state.duration
                < ((u64::from_le_bytes(current_time)
                    - u64::from_le_bytes(fundraiser_state.time_started))
                    / SECONDS_TO_DAYS) as u8,
            "Fundraise Duration is not Over"
        );
        (
            fundraiser_state.bump,
            u64::from_le_bytes(fundraiser_state.current_amount),
        )
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
        to: maker_ata,
        authority: fundrasier_acc,
        amount,
    }
    .invoke_signed(&[signer.clone()])?;

    pinocchio_token::instructions::CloseAccount {
        account: vault_ata,
        destination: maker,
        authority: fundrasier_acc,
    }
    .invoke_signed(&[signer])?;

    Ok(())
}
