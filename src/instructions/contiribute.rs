use pinocchio::{
    AccountView, ProgramResult,
    cpi::{Seed, Signer},
    error::ProgramError,
    sysvars::{Sysvar, clock::Clock, rent::Rent},
};
use pinocchio_pubkey::derive_address;
use pinocchio_system::instructions::CreateAccount;
use wincode::SchemaRead;

use crate::{
    constants::{MAX_CONTRIBUTION_PERCENTAGE, PERCENTAGE_SCALER, SECONDS_TO_DAYS},
    state::{Contribution, Fundraiser},
};

#[derive(SchemaRead)]
pub struct ContributeData {
    bump: u8,
    contribution_bump: u8,
    amount: u64,
}

pub fn process_contribution(accounts: &[AccountView], data: &[u8]) -> ProgramResult {
    let [
        contributor,
        mint,
        fundraiser_acc,
        contributor_ata,
        contribution_acc,
        vault_ata,
        _token_program,
        _system_program,
    ] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Account validation checks
    // 1. mint matches wiht the fundraiser stored ata and contributor ata
    // 2. contributor pda matching
    // 3. vault ata owner check
    // 4. reson for not validating the fundraiser_acc its okay if the given fundraise mint is matched with the vault ata and anyways we are checking for the owner and the mint stored in that pda

    let contributor_ata_state =
        pinocchio_token::state::TokenAccount::from_account_view(contributor_ata)?;
    let vault_ata_state = pinocchio_token::state::TokenAccount::from_account_view(vault_ata)?;
    let mint_state = pinocchio_token::state::Mint::from_account_view(mint)?;

    let (fundraiser_mint, fundraiser_amount_to_raise, fundraiser_duration, fundraiser_time_started) = {
        let fundraise_data = unsafe { fundraiser_acc.borrow_unchecked() };
        let fundraise_state = ::wincode::deserialize::<Fundraiser>(fundraise_data)
            .map_err(|_| ProgramError::InvalidAccountData)?;
        (
            fundraise_state.mint,
            fundraise_state.amount_to_raise,
            fundraise_state.duration,
            fundraise_state.time_started,
        )
    }; // borrow dropped here

    if fundraiser_mint != *mint.address().as_array()
        || contributor_ata_state.mint() != mint.address()
    {
        return Err(ProgramError::InvalidArgument);
    }
    if vault_ata_state.owner() != fundraiser_acc.address() {
        return Err(ProgramError::IllegalOwner);
    }

    let ix_data = ::wincode::deserialize::<ContributeData>(data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    assert!(
        ix_data.amount > 1_u8.pow(mint_state.decimals() as u32) as u64,
        "Minimum amount Decline"
    );

    let current_time = Clock::get()?.unix_timestamp.to_le_bytes();
    assert!(
        fundraiser_duration
            <= ((u64::from_le_bytes(current_time) - u64::from_le_bytes(fundraiser_time_started))
                / SECONDS_TO_DAYS) as u8,
        "Fundraise Duration is Over"
    );

    // Derive and verify contribution PDA
    // seeds = [b"contributor", fundraiser.key().as_ref(), contributor.key().as_ref()]
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

    let contribution_signer_seeds = [
        Seed::from(b"contributor"),
        Seed::from(fundraiser_acc.address().as_array()),
        Seed::from(contributor.address().as_array()),
        Seed::from(&contribution_bump_bytes),
    ];
    let contribution_signer = Signer::from(&contribution_signer_seeds[..]);

    unsafe {
        if contribution_acc.lamports() == 0 {
            // Init if need for contributor PDA
            CreateAccount {
                from: contributor,
                to: contribution_acc,
                lamports: Rent::get()?.minimum_balance_unchecked(Contribution::LEN),
                space: Contribution::LEN as u64,
                owner: &crate::ID,
            }
            .invoke_signed(&[contribution_signer])?;
            let contribution_state = Contribution::from_account_info(contribution_acc)?;
            contribution_state.amount = ix_data.amount;
        } else if contribution_acc.owner() == &crate::ID {
            let expected_contribution = derive_address(
                &[
                    b"contributor",
                    fundraiser_acc.address().as_array(),
                    contributor.address().as_array(),
                    &contribution_bump_bytes,
                ],
                None,
                crate::ID.as_array(),
            );
            if contribution_acc.address().as_array() != &expected_contribution {
                return Err(ProgramError::InvalidAccountData);
            }
            let contribution_state = Contribution::from_account_info(contribution_acc)?;
            let new_amount = contribution_state.amount + ix_data.amount;
            assert!(
                new_amount
                    <= (u64::from_le_bytes(fundraiser_amount_to_raise))
                        * MAX_CONTRIBUTION_PERCENTAGE
                        / PERCENTAGE_SCALER,
                "new amount is exceeding the threshold"
            );
            contribution_state.amount = new_amount;
        }
    }

    pinocchio_token::instructions::Transfer {
        from: contributor_ata,
        to: vault_ata,
        authority: contributor,
        amount: ix_data.amount,
    }
    .invoke()?;

    Ok(())
}
