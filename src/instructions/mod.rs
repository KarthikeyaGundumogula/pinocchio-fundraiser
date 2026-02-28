pub mod checker;
pub mod contiribute;
pub mod initialize;
pub mod refund;

// pub use checker::*;
// pub use contiribute::*;
pub use initialize::*;
// pub use refund::*;

use pinocchio::error::ProgramError;

pub enum FundraiseInstrctions {
    Initialize = 0,
    Contribute = 1,
    Refund = 2,
    CheckContribution = 3,
}

impl TryFrom<&u8> for FundraiseInstrctions {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FundraiseInstrctions::Initialize),
            1 => Ok(FundraiseInstrctions::Contribute),
            2 => Ok(FundraiseInstrctions::Refund),
            3 => Ok(FundraiseInstrctions::CheckContribution),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
