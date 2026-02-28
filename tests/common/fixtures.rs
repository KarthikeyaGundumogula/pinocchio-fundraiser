use litesvm_token::spl_token;
use solana_sdk::pubkey::Pubkey;

pub const ASSOCIATED_TOKEN_PROGRAM_ID: &str = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";
pub const TOKEN_PROGRAM_ID: Pubkey = spl_token::ID;
pub const AMOUNT_TO_RAISE: u64 = 100_000_000;
pub const DURATION_IN_DAYS: u8 = 5;
pub const DONATION_AMOUNT:u64 = 500_000;

pub fn program_id() -> Pubkey {
    Pubkey::from(pinocchio_fundraiser::ID)
}
