use forge_store_blob_chunks::S7BlobChunkSecurityHandoff;
use forge_store_io_scheduler::SchedulerSecurityScopeEvidence;

fn requires_s6_handoff(_: SchedulerSecurityScopeEvidence) {}

fn main() {
    let s7_handoff: S7BlobChunkSecurityHandoff = todo!();
    requires_s6_handoff(s7_handoff);
}
