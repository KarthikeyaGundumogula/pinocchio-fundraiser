#![allow(unexpected_cfgs)]
use pinocchio::{
    AccountView, Address, ProgramResult, address::declare_id, entrypoint, error::ProgramError,
};

use crate::instructions::FundraiseInstrctions;

pub mod instructions;
pub mod state;
pub mod constants;

entrypoint!(process_instruction);

declare_id!("AZXNrDb3Ldrxxv52fWKuycxD4qQ4o5R3BvrD62aBht1S");

pub fn process_instruction(
    program_id: &Address,
    accounts: &[AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    assert_eq!(program_id, &ID);

    //get the DESCRIMINATOR from the Instruction Data
    let (descriminator, data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;
    match FundraiseInstrctions::try_from(descriminator)? {
        FundraiseInstrctions::Initialize => {
            instructions::initialize::process_initialize(accounts, data)?
        }
        // FundraiseInstrctions::Contribute => {
        //     instructions::take::process_take_instruction(accounts, data)?
        // }
        // FundraiseInstrctions::Refund => {
        //     instructions::refund::process_refund_instruction(accounts, data)?
        // }
        // FundraiseInstrctions::CheckContribution => {
        //     instructions::wincode_v2::make::process_make_v2_instruction(accounts, data)?
        // }
        _ => Err(ProgramError::InvalidInstructionData)?,
    };
    Ok(())
}
