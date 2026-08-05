use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum CompatibilityDecision {
    Admit(CompatibilityRelation),
    Reject(CompatibilityRejection),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum CompatibilityRejectionKind {
    FamilyMismatch,
    MalformedFrame,
    TruncatedFrame,
    UndeclaredFamily,
    UnsupportedFormatVersion,
    UnsupportedSemanticVersion,
    ManifestDigestMismatch,
    MissingManifestPublication,
    RecoveredManifestDigestMismatch,
    RecoveredManifestWindowMismatch,
    MissingCompatibilityEdge,
    DeclaredIncompatibleRelation,
    AdapterHotPathRejected,
    AdapterMaintenanceRequired,
    AdapterOutOfScope,
    AdapterParityFailure,
    ReaderCapabilityUnsupported,
    WriterCapabilityUnsupported,
    ReceiptArtifactMismatch,
    ReceiptBasisMismatch,
    AuthoritativePartialTruthRejected,
    DerivedReuseIncompatible,
    DerivedRebuildIncompatible,
    DerivedBasisIncompatible,
    DerivedStaleVersion,
    DerivedRebuildAdmissionRejected,
    DerivedLaneRejected,
    BulkResumeCompatibilityRejected,
    TierManifestCompatibilityRejected,
    MaintenanceLaneMismatch,
    RollingWindowRejected,
    RollingMultiWriterRejected,
    MixedVersionSkewRejected,
    RestoreCompatibilityRejected,
    RestoreOutOfScopeScanRejected,
    RestorePublicationConflictRejected,
}
impl CompatibilityRejectionKind {
    pub fn store_error_kind(self) -> StoreErrorKind {
        match self {
            Self::FamilyMismatch => StoreErrorKind::CompatibilityArtifactFamilyUndeclared,
            Self::MalformedFrame => StoreErrorKind::CompatibilityArtifactFrameMalformed,
            Self::TruncatedFrame => StoreErrorKind::CompatibilityArtifactFrameMalformed,
            Self::UndeclaredFamily => StoreErrorKind::CompatibilityArtifactFamilyUndeclared,
            Self::UnsupportedFormatVersion => {
                StoreErrorKind::CompatibilityArtifactFormatUnsupported
            }
            Self::UnsupportedSemanticVersion => {
                StoreErrorKind::CompatibilityArtifactSemanticVersionUnsupported
            }
            Self::ManifestDigestMismatch => StoreErrorKind::CompatibilityArtifactManifestMalformed,
            Self::MissingManifestPublication => StoreErrorKind::CompatibilityManifestPublicationGap,
            Self::RecoveredManifestDigestMismatch => {
                StoreErrorKind::CompatibilityArtifactManifestMalformed
            }
            Self::RecoveredManifestWindowMismatch => {
                StoreErrorKind::CompatibilityArtifactManifestMalformed
            }
            Self::MissingCompatibilityEdge => StoreErrorKind::CompatibilityEdgeMissing,
            Self::DeclaredIncompatibleRelation => StoreErrorKind::CompatibilityEdgeMissing,
            Self::AdapterHotPathRejected => StoreErrorKind::CompatibilityAdapterParityFailure,
            Self::AdapterMaintenanceRequired => {
                StoreErrorKind::CompatibilityAuthoritativePartialTruthRejected
            }
            Self::AdapterOutOfScope => StoreErrorKind::CompatibilityAdapterParityFailure,
            Self::AdapterParityFailure => StoreErrorKind::CompatibilityAdapterParityFailure,
            Self::ReaderCapabilityUnsupported => {
                StoreErrorKind::CompatibilityArtifactSemanticVersionUnsupported
            }
            Self::WriterCapabilityUnsupported => {
                StoreErrorKind::CompatibilityArtifactSemanticVersionUnsupported
            }
            Self::ReceiptArtifactMismatch => {
                StoreErrorKind::CompatibilityAuthoritativePartialTruthRejected
            }
            Self::ReceiptBasisMismatch => {
                StoreErrorKind::CompatibilityAuthoritativePartialTruthRejected
            }
            Self::AuthoritativePartialTruthRejected => {
                StoreErrorKind::CompatibilityAuthoritativePartialTruthRejected
            }
            Self::DerivedReuseIncompatible => StoreErrorKind::CompatibilityDerivedReuseIncompatible,
            Self::DerivedRebuildIncompatible => {
                StoreErrorKind::CompatibilityDerivedRebuildIncompatible
            }
            Self::DerivedBasisIncompatible => {
                StoreErrorKind::CompatibilityDerivedRebuildIncompatible
            }
            Self::DerivedStaleVersion => StoreErrorKind::CompatibilityDerivedReuseIncompatible,
            Self::DerivedRebuildAdmissionRejected => {
                StoreErrorKind::CompatibilityDerivedRebuildIncompatible
            }
            Self::DerivedLaneRejected => StoreErrorKind::CompatibilityDerivedReuseIncompatible,
            Self::BulkResumeCompatibilityRejected => {
                StoreErrorKind::CompatibilityDerivedReuseIncompatible
            }
            Self::TierManifestCompatibilityRejected => {
                StoreErrorKind::CompatibilityDerivedReuseIncompatible
            }
            Self::MaintenanceLaneMismatch => {
                StoreErrorKind::CompatibilityDerivedRebuildIncompatible
            }
            Self::RollingWindowRejected
            | Self::RollingMultiWriterRejected
            | Self::MixedVersionSkewRejected => StoreErrorKind::CompatibilityRollingUpgradeRejected,
            Self::RestoreCompatibilityRejected | Self::RestorePublicationConflictRejected => {
                StoreErrorKind::CompatibilityRestoreRejected
            }
            Self::RestoreOutOfScopeScanRejected => {
                StoreErrorKind::CompatibilityRestoreOutOfScopeScanRejected
            }
        }
    }
}
