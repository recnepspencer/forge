#![allow(dead_code)]

mod adapters;
mod admission;
mod authoritative;
mod catalog;
mod certification;
mod certification_runner;
mod decoding;
mod derived;
mod evidence;
mod manifests;
mod production;
mod restore;
mod rolling;

pub(crate) use adapters::{
    execute_declared_adapter_parity, first_ship_authoritative_adapter_edge_registry,
    first_ship_commit_envelope_adapted_lane, first_ship_commit_envelope_control_lane,
    AdapterParityLane,
};
pub(crate) use admission::check_artifact_with_read_receipt;
pub(crate) use admission::{
    plan_read_compatibility, plan_read_compatibility_for_path, plan_write_compatibility,
};
pub use admission::{
    BackwardReadCompatibilityWitness, CompatibilityAdapterCostClass, CompatibilityAdapterDigest,
    CompatibilityAdapterId, CompatibilityAdapterParityWitness, CompatibilityAdmissionBatch,
    CompatibilityAdmissionCounters, CompatibilityAdmissionPath, CompatibilityAdmissionPlan,
    CompatibilityAdmissionReceipt, CompatibilityBatchScope, CompatibilityDecision,
    CompatibilityEdgeProof, CompatibilityEdgeRegistry, CompatibilityManifestIndex,
    CompatibilityManifestIndexEntry, CompatibilityReadAdmissionOutcome, CompatibilityReadIntent,
    CompatibilityRejection, CompatibilityRejectionKind, CompatibilityRelation,
    CompatibilityWriteAdmissionOutcome, CompatibilityWriteIntent, DeclaredCompatibilityAdapter,
    DeclaredCompatibilityEdge, DerivedReuseCompatibilityReceipt, ForwardReadCompatibilityWitness,
    ReadCompatibilityReceipt, ReaderCapabilitySet, RestoreCompatibilityReceipt,
    RollingWindowCompatibilityReceipt, SemanticMeaningPreservationWitness, UpgradeAdmissionWitness,
    WriteCompatibilityReceipt, WriterCapabilitySet,
};
pub(crate) use authoritative::{
    admit_authoritative_meaning_with_parity_witness, declare_authoritative_meaning,
};
#[allow(unused_imports)]
pub use authoritative::{
    AuthoritativeAdmissionReport, AuthoritativeCompatibilityWitness,
    AuthoritativeMeaningDeclaration, AuthoritativePartialTruthRejection,
    AuthoritativeUnknownMeaning, BackwardAuthoritativeReadPlan, ForwardAuthoritativeReadPlan,
    UnsupportedAuthoritativeVersion,
};
pub use catalog::{
    AuthoritativeFamilyDeclaration, CompatibilityAuthorityClassification,
    CompatibilityFamilyDeclaration, CompatibilityFamilyKind, CompatibilityManifestDeclaration,
    CompatibilityRegistry, CompatibilityRegistrySnapshot, DerivedFamilyDeclaration,
    FIRST_SHIP_COMPATIBILITY_FAMILIES, FIRST_SHIP_COMPATIBILITY_FAMILY_COUNT,
};
#[allow(unused_imports)]
pub use certification::{
    Milestone12CertificationLaneId, Milestone12CertificationLaneInput,
    Milestone12CertificationLaneKind, Milestone12CertificationLaneOutcome,
    Milestone12CertificationLaneRejection, Milestone12CertificationLaneStatus,
    Milestone12CertificationRunSummary, Milestone12CompatibilityMatrix,
    Milestone12CompatibilityMatrixEntry, Milestone12CompatibilityMatrixStatus,
};
pub use certification_runner::{
    Milestone12ArtifactFormatEvolutionCertification, Milestone12CertificationDiagnostics,
    Milestone12CertificationDigestSet, Milestone12CertificationFixture,
    Milestone12CertificationRunner, Milestone12CertificationScenario,
};
pub use decoding::{
    CompatibilityArtifactFrameHeader, CompatibilityCheckedArtifact, FramedArtifactRecord,
    QuarantinedDecodedArtifact, RawArtifactBytes, SemanticArtifactView,
};
pub(crate) use derived::{
    admit_derived_rebuild_maintenance, plan_derived_lane_compatibility,
    prove_compatibility_maintenance_lane_admission, prove_retained_authority_for_derived_rebuild,
    require_matching_maintenance_lane,
};
pub use derived::{
    BulkResumeCompatibilityPlan, BulkResumeCompatibilityRejection, BulkResumeInterpretation,
    CompatibilityMaintenanceAdmissionWitness, CompatibilityMaintenanceLaneAdmission,
    CompatibilityMaintenanceLaneRejection, CompatibilityMaintenanceLaneRequirement,
    CompatibilityRebuildDebt, DerivedBasisCompatibilityInput, DerivedBasisCompatibilityPlan,
    DerivedBasisCompatibilityPosture, DerivedCompatibilityLane,
    DerivedCompatibilityLaneDeclaration, DerivedCompatibilityLaneKind,
    DerivedCompatibilityLaneRegistry, DerivedCompatibilityLaneSnapshot,
    DerivedCompatibilityReusePlan, DerivedCompatibilityReuseWitness, DerivedCompatibilityWitness,
    DerivedInvalidationPlan, DerivedInvalidationReason, DerivedLaneCompatibilityPlan,
    DerivedLaneCompatibilityPosture, DerivedLaneInvalidation, DerivedLaneRebuildRequirement,
    DerivedLaneRejection, DerivedLaneReuseAdmission, DerivedRebuildCompatibilityPlan,
    DerivedRebuildRequirement, DerivedReusePosture, RetainedAuthorityCompatibilityWitness,
    StaleDerivedVersionRejection, TierCompatibilityNonAuthorityPosture,
    TierManifestCompatibilityPlan, TierManifestCompatibilityRejection,
};
pub use evidence::{
    ArtifactFamilyCompatibilityIndex, ArtifactFamilyVersionSummary,
    CompatibilityAdapterCostClassReport, CompatibilityAdapterCostSummary,
    CompatibilityAdmissionReceiptSummary, CompatibilityAuditPlan, CompatibilityAuditSummary,
    CompatibilityAuditUnit, CompatibilityBatchScopeReport, CompatibilityManifestSummary,
    CompatibilityRebuildSummary, DerivedInvalidationSummary, Milestone12Phase1Evidence,
    ReaderWriterSkewSummary, RestoreCompatibilityBreadthBudget, RestoreVersionSummary,
};
#[allow(unused_imports)]
pub use manifests::{
    ArtifactCompatibilityWindow, ArtifactFamilyId, ArtifactFormatVersion, ArtifactSemanticVersion,
    AuthoritativeCompatibilityManifest, CompatibilityManifestDigest, CompatibilityManifestFrontier,
    CompatibilityManifestPublicationLedger, CompatibilityManifestPublicationReceipt,
    CompatibilityManifestPublicationRecord, CompatibilityManifestPublicationUnit,
    CompatibilityManifestRecoveryPlan, CompatibilityRecoveredManifestIndex,
    DerivedCompatibilityManifest, ManifestDigestMismatch, ManifestPublicationGap,
    ManifestPublicationWitness, ManifestRecoverySummary,
};
pub use production::{
    CompatibilityAuthoritativeAdapterOutcome, CompatibilityAuthoritativeAdapterRequest,
    CompatibilityDerivedRebuildOutcome, CompatibilityDerivedRebuildRequest,
    CompatibilityRestoreExecutionOutcome, CompatibilityRollingPublicationOutcome,
    CompatibilityRollingPublicationRequest,
};
pub(crate) use restore::execute_restore_publication;
pub(crate) use restore::plan_restore_compatibility;
#[allow(unused_imports)]
pub use restore::{
    BackupCompatibilityManifest, DisasterRecoveryCompatibilityClass,
    DisasterRecoveryCompatibilityPlan, DisasterRecoveryCompatibilityWindow, RestoreBackupScope,
    RestoreCompatibilityPlan, RestoreCompatibilityTarget, RestorePublicationConflictKind,
    RestorePublicationConflictSet, RestorePublicationConflictUnit, RestorePublicationWitness,
    RestoreVersionRejection,
};
pub(crate) use rolling::{
    first_ship_commit_rolling_edge_registry, plan_first_ship_rolling_upgrade,
};
pub use rolling::{
    MaintenanceCompatibilityPosture, MixedVersionPostureKind, MixedVersionStorePosture,
    ReplicaCompatibilityPosture, RollingCapabilityWindow, RollingUpgradeAdmissionPlan,
    RollingUpgradePolicy, RollingUpgradeRejection, RollingUpgradeWindow, UpgradeSkewRejection,
};

#[cfg(test)]
mod tests;
