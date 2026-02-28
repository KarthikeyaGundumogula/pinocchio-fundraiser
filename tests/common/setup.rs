use std::path::PathBuf;

use litesvm::LiteSVM;
use litesvm_token::{
    CreateAssociatedTokenAccount, CreateMint, MintTo,
};
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
};

use crate::fixtures::{AMOUNT_TO_RAISE, ASSOCIATED_TOKEN_PROGRAM_ID, program_id};

pub struct TestContext {
    pub svm: LiteSVM,
    pub maker: Keypair,
    pub donar: Keypair,
    pub mint: Pubkey,
    pub maker_ata: Pubkey,
    pub donar_ata: Pubkey,
    pub fundraiser: Pubkey,
    pub contribution: Pubkey,
    pub fundraiser_bump: u8,
    pub contribution_bump: u8,
    pub vault_ata: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}

pub fn setup() -> TestContext {
    let mut svm = LiteSVM::new();
    let maker = Keypair::new();
    let donar = Keypair::new();

    svm.airdrop(&maker.pubkey(), 10 * LAMPORTS_PER_SOL)
        .expect("Airdrop failed for maker");
    svm.airdrop(&donar.pubkey(), 10 * LAMPORTS_PER_SOL)
        .expect("Airdrop failed for donar");

    // Load program SO file
    let so_path = PathBuf::from(
"target/sbpf-solana-solana/release/pinocchio_fundraiser.so"
    );
    let program_data = std::fs::read(so_path).expect("Failed to read program SO file");
    svm.add_program(program_id(), &program_data)
        .expect("Failed to add program");

    // Create mints
    let mint = CreateMint::new(&mut svm, &maker)
        .decimals(6)
        .authority(&donar.pubkey())
        .send()
        .unwrap();

    let donar_ata = CreateAssociatedTokenAccount::new(&mut svm, &donar, &mint)
        .owner(&donar.pubkey())
        .send()
        .unwrap();

    let maker_ata = CreateAssociatedTokenAccount::new(&mut svm, &donar, &mint)
        .owner(&maker.pubkey())
        .send()
        .unwrap();


    // Derive fundraise PDA and vault
    let (fundraiser, fundraiser_bump) = Pubkey::find_program_address(
        &[b"fundraiser".as_ref(), maker.pubkey().as_ref()],
        &program_id(),
    );

    // Derive contribution PDA
    let (contribution, contribution_bump) = Pubkey::find_program_address(
        &[b"contributor".as_ref(), fundraiser.as_ref(), donar.pubkey().as_ref()],
        &program_id(),
    );



    let vault_ata = spl_associated_token_account::get_associated_token_address(&fundraiser, &mint);

    // Mint tokens to donar
    MintTo::new(&mut svm, &donar, &mint, &donar_ata, 2 * AMOUNT_TO_RAISE)
        .send()
        .unwrap();

    let associated_token_program = ASSOCIATED_TOKEN_PROGRAM_ID.parse::<Pubkey>().unwrap();
    let system_program = Pubkey::from(pinocchio_system::ID);

    TestContext {
        svm,
        maker,
        donar,
        mint,
        donar_ata,
        maker_ata,
        fundraiser,
        contribution,
        fundraiser_bump,
        contribution_bump,
        vault_ata,
        associated_token_program,
        system_program,
    }
}

