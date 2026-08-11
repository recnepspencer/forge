use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QualifiedPhysicalBackendProfile([u8; 32]);

impl QualifiedPhysicalBackendProfile {
    pub(crate) fn from_report(report: &crate::RootProfileQualificationReport) -> Self {
        let mut digest = Sha256::new();
        digest.update(report.contract_version().to_le_bytes());
        digest.update(report.root_identity());
        digest.update(report.volume_identity());
        digest.update(report.profile_digest());
        digest.update(report.backend_build_identity());
        Self(digest.finalize().into())
    }
}
