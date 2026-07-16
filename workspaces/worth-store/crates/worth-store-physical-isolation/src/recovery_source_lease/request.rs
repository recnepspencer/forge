use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct RecoverySourceLeaseRequest {
    pub(super) operation_identity: [u8; 32],
    pub(super) source_identity: [u8; 32],
    pub(super) source_root: PathBuf,
    pub(super) artifact_names: Vec<String>,
}

impl RecoverySourceLeaseRequest {
    pub fn new(
        operation_identity: [u8; 32],
        source_identity: [u8; 32],
        source_root: impl Into<PathBuf>,
        artifact_names: Vec<String>,
    ) -> Self {
        Self {
            operation_identity,
            source_identity,
            source_root: source_root.into(),
            artifact_names,
        }
    }
}
