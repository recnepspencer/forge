use forge_store_physical_certification::{OracleFamilyKind, PhysicalEvidenceBundleDenial};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlobCloseoutSourceDenial {
    MissingRequiredOracleFamily(OracleFamilyKind),
    HeavyQualificationEvidenceMissing,
    HeavyCleanupEvidenceMissing,
    HeavyPatternLaneEvidenceMissing,
    ReplayEvidenceDenied(PhysicalEvidenceBundleDenial),
}

impl From<PhysicalEvidenceBundleDenial> for BlobCloseoutSourceDenial {
    fn from(denial: PhysicalEvidenceBundleDenial) -> Self {
        Self::ReplayEvidenceDenied(denial)
    }
}
