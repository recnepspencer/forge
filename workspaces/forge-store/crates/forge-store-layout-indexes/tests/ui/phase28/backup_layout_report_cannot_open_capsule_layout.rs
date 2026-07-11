use forge_store_operations::{BackupLayoutEvidenceReport, CapsuleOperationLayoutReport};

fn main() {
    let backup: BackupLayoutEvidenceReport = todo!();
    let _ = CapsuleOperationLayoutReport::from_blob_capsule_readiness(&backup);
}
