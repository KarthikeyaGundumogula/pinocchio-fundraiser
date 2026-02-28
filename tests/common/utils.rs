use litesvm::LiteSVM;
use solana_sdk::{
    clock::Clock,
    message::{Instruction, Message},
    pubkey::Pubkey,
    signature::Keypair,
    transaction::Transaction,
};

pub fn set_clock(svm: &mut LiteSVM, unix_timestamp: i64) {
    let mut clock: Clock = svm.get_sysvar();
    clock.slot = 10;
    clock.epoch = 1000;
    clock.unix_timestamp = unix_timestamp;
    svm.set_sysvar(&clock);
}

pub fn send_transaction(svm: &mut LiteSVM, ix: Instruction, signers: &[&Keypair], payer: &Pubkey) {
    let message = Message::new(&[ix], Some(payer));
    let recent_blockhash = svm.latest_blockhash();
    let transaction = Transaction::new(signers, message, recent_blockhash);
    let tx = svm
        .send_transaction(transaction)
        .expect("Transaction should succeed");
    println!("{}",tx.pretty_logs());
    println!("CUs Consumed: {}", tx.compute_units_consumed);
}
