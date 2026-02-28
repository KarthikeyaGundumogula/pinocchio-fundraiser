use pinocchio::{AccountView, error::ProgramError};
use wincode::SchemaRead;

#[derive(SchemaRead)]
pub struct Contribution {
    pub amount: u64,
}

impl Contribution {
    pub const LEN: usize = 8;
    pub fn from_account_info(account_info: &AccountView) -> Result<&mut Self, ProgramError> {
        let mut data = account_info.try_borrow_mut()?;
        if data.len() != Contribution::LEN {
            return Err(ProgramError::InvalidAccountData);
        }

        if (data.as_ptr() as usize) % core::mem::align_of::<Self>() != 0 {
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(unsafe { &mut *(data.as_mut_ptr() as *mut Self) })
    }
}
