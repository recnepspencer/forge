use worth_store::physical_runtime::{BlobPhysicalAllocation, RecoveryPhysicalAllocation};

fn consume_blob(_: BlobPhysicalAllocation) {}

fn substitute_recovery(allocation: RecoveryPhysicalAllocation) {
    consume_blob(allocation);
}

fn main() {}
