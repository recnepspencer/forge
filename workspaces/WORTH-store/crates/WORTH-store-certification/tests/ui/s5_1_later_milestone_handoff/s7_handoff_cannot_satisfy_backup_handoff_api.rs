use worth_store_blob_chunks::S7BlobChunkSecurityHandoff;
use worth_store_operations::S10BackupExportCustodyHandoff;

fn requires_backup_handoff(_: S10BackupExportCustodyHandoff) {}

fn main() {
    let s7_handoff: S7BlobChunkSecurityHandoff = todo!();
    requires_backup_handoff(s7_handoff);
}
