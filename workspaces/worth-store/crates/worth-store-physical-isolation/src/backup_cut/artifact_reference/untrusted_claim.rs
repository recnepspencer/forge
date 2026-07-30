use super::{BackupArtifactCoverage, BackupArtifactFamily};

/// Metadata claimed for an observed physical backup artifact.
///
/// This value is deliberately untrusted. It carries no backup-cut or reclaim
/// authority and becomes usable only after `BackupArtifactReference` validates
/// it against the physical observation and current-generation reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UntrustedBackupArtifactClaim {
    pub family: BackupArtifactFamily,
    pub format: worth_store_physical_format::BackupBundleArtifactFormat,
    pub identity: String,
    pub generation: u64,
    pub coverage: BackupArtifactCoverage,
}
