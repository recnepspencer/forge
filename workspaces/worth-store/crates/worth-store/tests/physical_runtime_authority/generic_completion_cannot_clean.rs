use worth_store_buffer_pool::{DirtyPhysicalFrame, PhysicalWritebackClaim};
use worth_store_physical_backend::BackendQueueExecutionCompletion;

fn bypass_physical_write(
    claim: PhysicalWritebackClaim,
    completion: BackendQueueExecutionCompletion,
) {
    let _ = claim.publish_clean(&completion);
}

fn skipped_physical_write(dirty: DirtyPhysicalFrame) {
    let _ = dirty.publish_clean();
}

fn main() {}
