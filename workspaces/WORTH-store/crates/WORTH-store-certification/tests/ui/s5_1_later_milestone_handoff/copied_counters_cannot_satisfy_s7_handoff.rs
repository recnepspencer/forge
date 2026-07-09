use worth_store_blob_chunks::S7BlobChunkSecurityHandoff;
use worth_store_security::StoreSecurityScopeAdmissionCounterSnapshot;

fn main() {
    let counters: StoreSecurityScopeAdmissionCounterSnapshot = todo!();
    let _ = S7BlobChunkSecurityHandoff::from_s5_1_readiness(counters);
}
