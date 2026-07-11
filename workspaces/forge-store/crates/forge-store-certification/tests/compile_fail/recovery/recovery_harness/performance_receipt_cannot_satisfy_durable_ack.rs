use forge_store_physical_backend::PosixFileFsyncDirFsyncProfile;
use forge_store_recovery_physics::{DurableAckReceipt, RecoveryCounterPerformanceReceipt};

fn requires_durable_ack(_: DurableAckReceipt<PosixFileFsyncDirFsyncProfile>) {}

fn main() {
    let receipt: RecoveryCounterPerformanceReceipt = todo!();
    requires_durable_ack(receipt);
}
