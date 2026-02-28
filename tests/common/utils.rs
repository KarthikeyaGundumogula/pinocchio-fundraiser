use litesvm::LiteSVM;
use solana_sdk::clock::Clock;

pub fn set_clock(svm: &mut LiteSVM, unix_timestamp: i64) {
    let mut clock: Clock = svm.get_sysvar();
    clock.slot = 10;
    clock.epoch = 1000;
    clock.unix_timestamp = unix_timestamp;
    svm.set_sysvar(&clock);
}