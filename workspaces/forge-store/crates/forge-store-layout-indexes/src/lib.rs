#![forbid(unsafe_code)]

mod access;
mod access_planning;
#[cfg(test)]
mod architecture_residue_tests;
mod artifact_family;
mod blob_basis;
pub mod bootstrap;
mod catalog;
pub mod compaction_projection;
#[cfg(test)]
mod counter_authority_residue_tests;
pub mod customization;
pub mod declarations;
pub mod evolution;
mod facade;
pub mod integrity;
mod keyspace;
pub mod maintenance;
pub mod materialization;
pub mod observation;
mod planning;
mod read;
mod recovery;
mod strategy;
pub mod strategy_declarations;
#[cfg(test)]
mod topology_residue_tests;

#[cfg(test)]
pub(crate) use access::execution::access_lowering;
pub use access::execution::{
    btree_lookup_readiness_cases, degraded_scan_readiness_cases, degraded_scan_runtime,
    BTreeLookupReadinessCaseId, BTreeLookupReadinessOutcome, BTreeLookupReady,
    DegradedScanAdmissionDenied, DegradedScanExecution, DegradedScanLoweringBasis,
    DegradedScanReadinessCaseId, DegradedScanReadinessOutcome, DegradedScanReadinessView,
    DegradedScanReadmission, DegradedScanReady, DegradedScanRuntime, ExecutedLayoutOperation,
    LoweredDegradedExactScan, PhysicalDegradedExecutionDenial,
};
pub use access::execution::{
    layout_degraded_scan_runtime, DegradedExactScanExecutionDenied,
    DegradedExactScanExecutionRequest, LayoutDegradedScanRuntime,
};
pub use access::shape::DegradedExactScanRequest;
pub use access::AdmittedAccessIntent;
pub use artifact_family::AdmittedPhysicalArtifactFamily;
pub use blob_basis::{BlobGenerationBasis, BlobIdentityKeyBasis};
pub use bootstrap::{
    bootstrap_catalog, BootstrapCatalogFacade, BootstrapCatalogReadAdmission,
    BootstrapCatalogReadOutcome, BootstrapCatalogReadOutcomeView, BootstrapLayoutCatalog,
    BootstrapOnlyAccessDenied, BootstrapOnlyAccessPath, MinimalRootDiscoveryLayout,
};
pub use catalog::system_families::io_scheduler::{
    project_background_pacing, project_foreground_interference, project_scheduler_reservation,
    BackgroundPacingInterferencePosture, BackgroundPacingLayoutReport,
    ForegroundInterferenceAccessBudget, ForegroundInterferenceLayoutReport,
    ForegroundInterferencePosture, SchedulerReservationInterferencePosture,
    SchedulerReservationLayoutReport,
};
pub use catalog::system_families::offline_verifier::{
    project_offline_custody_capsule, project_offline_export_bundle,
    project_offline_repair_blast_radius, OfflineVerifierAccessShape,
    OfflineVerifierAuthorityPosture, OfflineVerifierEvidenceKind, OfflineVerifierLayoutProjection,
};
pub use facade::access_planning;
pub use keyspace::AdmittedConcretePhysicalKey;
pub use keyspace::AdmittedPhysicalKeyDomain;
pub use maintenance::{
    layout_lsm_maintenance, LayoutLsmMaintenance, LsmCompactionAdmissionRequest,
    LsmMaintenanceAdmissionDenied, LsmReplayAdmissionRequest, LsmRunPublicationAdmissionRequest,
};
pub use materialization::{
    AdmittedCoverageBasis, AdmittedLayoutMaterialization, CurrentLayoutMaterialization,
    CurrentMaterializationFrontier, ImportedBlobMaterializationSourceIdentity,
    LayoutMaterializationSourceIdentity, LayoutMaterializationSourceKind, MaterializationDenial,
    MaterializationFreshness, RestoredArtifactMaterializationSourceIdentity,
    StaleLayoutMaterialization,
};
pub use planning::{
    imported_blob_read_admission_cases, AccessPlanCostClass, AccessPlanCostDenial,
    AccessPlanCostEstimate, AccessPlanIdentity, AccessPlanSelectionDenied,
    AccessPlanSelectionOutcome, AccessPlanSelectionView, AdmittedPhysicalMutationRequest,
    AdmittedPhysicalReadRequest, AdmittedPhysicalRecoveryRequest, BTreeLookupOperation,
    ImportedBlobReadAdmissionCaseId, ImportedBlobReadAdmissionOutcome,
    ImportedBlobReadAdmissionView, PhysicalAccessRequestAdmissionDenied, SelectedBTreeLookup,
    SelectedBTreeReplayRecovery, SelectedDegradedExactScan, SelectedLsmCompaction,
    SelectedLsmLookup, SelectedLsmReplayRecovery, SelectedLsmRunPublication,
    SelectionCandidateAudit, SelectionCandidateOutcome, SelectionCandidateRejection,
    SelectionCandidateRejectionCase,
};
pub use read::{
    layout_read_runtime, LayoutReadAdmissionDenied, LayoutReadRuntime, PageLookupRequest,
    WalLookupRequest,
};
pub use recovery::{
    layout_btree_recovery, BTreeReplayDenied, BTreeReplayLocation, BTreeReplayPhysicalSource,
    BTreeReplayRequest, LayoutBTreeRecovery,
};
pub use strategy::btree::execution::{
    btree_lookup_execution_cases, btree_lookup_runtime, btree_replay_runtime,
    decode_leaf_record as decode_baseline_btree_leaf_record,
    decode_root_record as decode_baseline_btree_root_record,
    encode_leaf_record as encode_baseline_btree_leaf_record,
    encode_root_record as encode_baseline_btree_root_record, BTreeLookupExecutionCaseId,
    BTreeLookupExecutionView, BTreeLookupRuntime, BTreeReplayReady, BTreeReplayRuntime,
    BaselineBTreeCorruptionMarker, BaselineBTreeExactCounterWitness, BaselineBTreeExecutionDenial,
    BaselineBTreeExecutionWitness, BaselineBTreeLeafRecord, BaselineBTreeLookupAbsence,
    BaselineBTreeLookupAdmission, BaselineBTreeLookupBranch, BaselineBTreeLookupExecution,
    BaselineBTreeReadPreflight, BaselineBTreeReadShape, BaselineBTreeReadSource,
    BaselineBTreeReplayAdmission, BaselineBTreeReplayRecoveryExecution, BaselineBTreeRootNode,
    StableBTreeLookupExecution,
};

pub(crate) use access::shape::{access_shapes, AccessLaneClassification};
pub(crate) use catalog::{
    ArtifactFamilyAuthorityWitness, ArtifactFamilyDenial, ArtifactFamilyLifecycleAdmission,
    PhysicalArtifactFamily, PhysicalArtifactFamilyDeclaration,
};
pub(crate) use integrity::{
    LayoutCorruptionOutcome, LayoutCorruptionView, LayoutQuarantineWitness,
};
pub(crate) use keyspace::{CanonicalKeyBytes, PhysicalKeyDomain, PhysicalKeyDomainWitness};
pub(crate) use maintenance::{LayoutCorruptionClassification, PhysicalMutationShape};
#[cfg(test)]
pub(crate) use materialization::LayoutMaterializationState;
pub(crate) use materialization::{LayoutCoverageWitness, MaterializationStateClass};
pub(crate) use planning::AccessPlanSelector;

// Unit tests live beside their owners but compile as one crate. Keep this
// convenience vocabulary crate-private and absent from production builds.
#[cfg(test)]
pub(crate) use access::shape::{
    AccessAuthorityPosture, AccessShapeDetail, AccessShapeUnsupportedDenial,
    AccessStaleDisposition, BoundedScanBasis, DegradedExactScanBasis, ExpectedCounterClass,
    FullDeclaredScanBasis, GroupedPrefixBasis, MaintenanceReadBasis, ManifestGraphWalkBasis,
    MultiRangeBasis, MutationAccessBasis, PrefixBasis, RangeBasis, StreamingContinuationBasis,
    StreamingReadBasis,
};
#[cfg(test)]
pub(crate) use catalog::ArtifactFamilyAccessLane;
#[cfg(test)]
pub(crate) use catalog::ArtifactScopePartitionWitness;
#[cfg(test)]
pub(crate) use evolution::{
    LayoutBindingWitness, LayoutCompatibilityWindow, LayoutEvolutionDeclaration,
    LayoutInterruptionPolicy, LayoutReadCompatibilityPosture, LayoutVersion,
    LayoutWriteCompatibilityPosture,
};
#[cfg(test)]
pub(crate) use keyspace::{CompositeKeyField, HashCollisionBehavior};
#[cfg(test)]
pub(crate) use maintenance::{
    DerivedIndexCostEnvelopeParity, DerivedIndexCounterShapeParity, DerivedIndexParityBasis,
    DerivedIndexParityRow, DerivedIndexRebuildDenied, DerivedIndexRebuildRequest,
    DerivedIndexRebuildSourceInput, DerivedIndexResultIdentity, ExactPublicationAuthoritySource,
    IndexLagOutcome, IndexLagWitness, IndexMaintenanceFailureOutcome, IndexMaintenanceMode,
    LagReason, LiveMaintenanceRequest,
};
pub use strategy::{
    baseline_lsm_lookup_admission_cases, baseline_lsm_lookup_cases, lsm_compaction_runtime,
    lsm_lookup_runtime, lsm_physical_compaction_runtime, lsm_publication_runtime,
    lsm_replay_runtime, lsm_strategy, AdmittedLsmCompactionDemand, BaselineLsmCompactionAdmission,
    BaselineLsmCompactionKeyIdentity, BaselineLsmCompactionPlan,
    BaselineLsmCompactionPublicationReceipt, BaselineLsmCompactionRecordIdentity,
    BaselineLsmCompactionRecordKind, BaselineLsmCompactionTransition,
    BaselineLsmCounterObservation, BaselineLsmExecutionAdmissionDenial, BaselineLsmLookupAbsence,
    BaselineLsmLookupAdmission, BaselineLsmLookupAdmissionCaseId,
    BaselineLsmLookupAdmissionOutcome, BaselineLsmLookupAdmissionView, BaselineLsmLookupCaseId,
    BaselineLsmLookupDisposition, BaselineLsmLookupExecution, BaselineLsmLookupSource,
    BaselineLsmLookupView, BaselineLsmManifestPublicationExecution,
    BaselineLsmMembershipObservation, BaselineLsmReplayAdmission, BaselineLsmReplayExecution,
    BaselineLsmRunIdentity, BaselineLsmRunPublicationAdmission, InterlockedLsmCompaction,
    LsmCompactionRuntime, LsmLookupRuntime, LsmPhysicalCompactionIntent,
    LsmPhysicalCompactionRuntime, LsmPublicationRuntime, LsmReplayRuntime, LsmStrategy,
    PreparedLsmCompaction, PublishedLsmCompaction,
};
#[cfg(test)]
pub(crate) use strategy::{
    BTreeCorruptionRegion, BTreeLookupBranch, LayoutStrategyFamily, StrategyAmplificationProfile,
    StrategyDenial, StrategyLocalityProfile, StrategyLookupInvariant, StrategyPublicationInvariant,
    StrategyRebuildSourceRequirement,
};

pub(crate) use declarations::layout_declarations;
