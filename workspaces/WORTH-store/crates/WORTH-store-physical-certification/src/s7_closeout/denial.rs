use crate::{OracleFamilyKind, PhysicalEvidenceBundleDenial};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum S7CloseoutSourceDenial {
    MissingRequiredOracleFamily(OracleFamilyKind),
    HeavyQualificationEvidenceMissing,
    HeavyCleanupEvidenceMissing,
    HeavyPatternLaneEvidenceMissing,
    ReplayEvidenceDenied(PhysicalEvidenceBundleDenial),
}

impl From<PhysicalEvidenceBundleDenial> for S7CloseoutSourceDenial {
    fn from(denial: PhysicalEvidenceBundleDenial) -> Self {
        Self::ReplayEvidenceDenied(denial)
    }
}
