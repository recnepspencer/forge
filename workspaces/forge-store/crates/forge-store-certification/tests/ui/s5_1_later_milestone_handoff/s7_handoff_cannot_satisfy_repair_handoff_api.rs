use forge_store_blob_chunks::S7BlobChunkSecurityHandoff;
use forge_store_operations::S10RepairBlastRadiusHandoff;

fn requires_repair_handoff(_: S10RepairBlastRadiusHandoff) {}

fn main() {
    let s7_handoff: S7BlobChunkSecurityHandoff = todo!();
    requires_repair_handoff(s7_handoff);
}
