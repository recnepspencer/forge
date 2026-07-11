use forge_store_physical_isolation::ReadCopyUpdateRootPublication;
use forge_store_recovery_physics::CheckpointCutoverReceipt;

fn misuse(receipt: CheckpointCutoverReceipt) {
    let _ = ReadCopyUpdateRootPublication::publish(receipt);
}

fn main() {}
