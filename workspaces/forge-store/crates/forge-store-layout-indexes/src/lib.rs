#![forbid(unsafe_code)]

mod access_shape;
mod artifact_family;
mod bootstrap;
mod budget;
mod compile_fail;
mod corruption;
mod customization;
mod degraded_access;
mod execution;
mod facade;
mod handoff;
mod key_domain;
mod legacy_disposition;
mod maintenance;
mod materialization;
mod migration;
mod planning;
mod skeleton;
mod strategy;
mod strategy_registry;

pub use access_shape::{
    access_shapes, S8AccessAuthorityPosture, S8AccessLaneClassification, S8AccessShape,
    S8AccessShapeContract, S8AccessShapeDetail, S8AccessShapeUnsupportedDenial,
    S8AccessStaleDisposition, S8BatchPointBasis, S8BoundedScanBasis, S8ChunkTreeWalkBasis,
    S8CoalescedPageReadBasis, S8DegradedExactScanBasis, S8DegradedExactScanRequest,
    S8ExpectedCounterClass, S8FullDeclaredScanBasis, S8GroupedPrefixBasis, S8MaintenanceReadBasis,
    S8ManifestGraphWalkBasis, S8MultiRangeBasis, S8MutationAccessBasis, S8PrefixBasis,
    S8RangeBasis, S8SortedBatchBasis, S8StreamingContinuationBasis, S8StreamingReadBasis,
};
pub use artifact_family::{
    ArtifactAuthorityRoleWitness, ArtifactDerivedAccuracyWitness, ArtifactFamilyAccessLane,
    ArtifactFamilyAuthorityClass, ArtifactFamilyAuthorityDisposition,
    ArtifactFamilyAuthorityWitness, ArtifactFamilyClassification, ArtifactFamilyDenial,
    ArtifactFamilyInventoryRow, ArtifactFamilyLifecycleAdmission, ArtifactFamilyLifecycleClass,
    ArtifactFamilyLifecycleDisposition, ArtifactFamilyStrategyLane, ArtifactKeyScopePartition,
    ArtifactScopePartitionWitness, ArtifactTenantScopePartition, AuthorityRole,
    DerivedAccuracyClass, DurableArtifactMigrationPosture, DurableArtifactProjectionClass,
    DurableArtifactRebuildPosture, ExistingArtifactFamilySurface, PhysicalArtifactFamily,
    PhysicalArtifactFamilyDeclaration, S8ArtifactFamilyInventory,
};
pub use bootstrap::{
    bootstrap_catalog, BootstrapCatalogFacade, S8BootstrapCatalogReadAdmission,
    S8BootstrapLayoutCatalog, S8BootstrapOnlyAccessDenied, S8BootstrapOnlyAccessPath,
    S8MinimalRootDiscoveryLayout,
};
pub use budget::S8PlannedCounterEnvelope;
pub use corruption::{
    layout_corruption, LayoutCorruptionFacade, S8CorruptionDenial, S8LayoutCorruptionClass,
    S8LayoutCorruptionInput, S8LayoutCorruptionOutcome, S8LayoutQuarantineWitness,
    S8LayoutReadmissionOutcome, S8LayoutReadmissionSource, S8LayoutReadmissionWitness,
    S8NativeReadmissionInput,
};
pub use customization::{
    layout_customization_boundary, S8FutureLayoutCapabilityRequest,
    S8FutureLayoutCustomizationAdmission, S8FutureLayoutCustomizationDeferred,
    S8FutureLayoutCustomizationDenial, S8FutureLayoutCustomizationOutcome,
    S8FutureLayoutCustomizationRequest, S8FutureLayoutWorkloadEnvelope,
};
pub use execution::{
    S8AccessAttemptCostReceipt, S8AccessPathAmplificationReceipt,
    S8CostEnvelopeViolationOutcome, S8ObservedAccessPathCounters, S8ObservedCounterMetric,
    S8PlannedVsObservedCounterReceipt, S8StoreLayoutPerformanceReceipt,
};
pub use execution::{
    access_lowering, S8AccessLoweringBasis, S8AccessLoweringDeferred, S8AccessLoweringDenied,
    S8AccessLoweringOutcome, S8AccessPathCounterSnapshot, S8AccessPathKind,
    S8ExecutedCounterWitness,
    S8ExecutedAccessReceipt, S8ExecutionReadmissionWitness, S8ExecutionReadyAccessReceipt,
    S8ExecutionRebindWitness, S8LoweredAccessPayload, S8LoweredAccessReceipt,
    S8RebindRequiredAccessReceipt, S8StaleLoweredAccessReceipt,
};
pub use facade::*;
pub use key_domain::{
    CanonicalKeyBytes, CanonicalKeyEncoding, ComparatorBehavior, ComparatorLaw, CompositeKeyField,
    CompositeKeyOrderingLaw, ConcretePhysicalKeyWitness, EncodingSentinelPolicy,
    HashCollisionBehavior, HashCollisionLaw, PhysicalKeyDomain, PhysicalKeyDomainDenial,
    PhysicalKeyDomainWitness, PrefixBoundaryBehavior, PrefixLawWitness, RangeBoundBehavior,
    RangeBoundLawWitness, TenantScopedKeyDomain,
};
pub use maintenance::{
    layout_rebuild, LayoutCorruptionClassification, S8DerivedIndexCostEnvelopeParity,
    S8DerivedIndexCounterShapeParity, S8DerivedIndexCoverageParity, S8DerivedIndexIdentityParity,
    S8DerivedIndexOrderingParity, S8DerivedIndexParityBasis, S8DerivedIndexParityOutcome,
    S8DerivedIndexParityRow, S8DerivedIndexParityWitness, S8DerivedIndexPartialKeySpace,
    S8DerivedIndexRebuildDenied, S8DerivedIndexRebuildOutcome, S8DerivedIndexRebuildPlan,
    S8DerivedIndexRebuildReceipt, S8DerivedIndexRebuildRequest, S8DerivedIndexRebuildScope,
    S8DerivedIndexRebuildSourceInput, S8DerivedIndexResultIdentity,
    S8ExactPublicationAuthoritySource, S8IndexLagOutcome, S8IndexLagWitness,
    S8IndexMaintenanceFailureOutcome, S8IndexMaintenanceMode, S8IndexMaintenanceTransitionOutcome,
    S8IndexPublicationProtocol, S8LagReason, S8LayoutMutationAdmissionOutcome,
    S8LayoutMutationPlan, S8LayoutRebuildFacade, S8LiveExactMaintenanceWitness,
    S8LiveMaintenanceRequest, S8LoweredMaintenanceProtocol, S8MutationProofRequirement,
    S8PhysicalMutationShape, S8PublicationProofRequirement,
};
pub use materialization::{
    S8AbsenceAuthorityClass, S8CoverageBasisKind, S8CoverageGapClass, S8CoverageGapWitness,
    S8LayoutCoverageWitness, S8LayoutMaterializationState, S8LayoutWatermark,
    S8MaterializationCompleteness, S8MaterializationDenial, S8MaterializationStateClass,
    S8PhysicalAbsenceProof, S8PhysicalCoverageBasis, S8PrefixCompletenessWitness,
    S8RangeCompletenessWitness,
};
pub use migration::{
    LayoutBindingWitness, LayoutCompatibilityWindow, LayoutEvolutionDeclaration,
    LayoutEvolutionDenial, LayoutInterruptedMigrationDisposition, LayoutInterruptionPolicy,
    LayoutInterruptionState, LayoutMigrationFacade, LayoutMigrationOutcome, LayoutMigrationPlan,
    LayoutMigrationRequest, LayoutPlanFingerprint, LayoutReadCompatibilityPosture,
    LayoutRollbackOutcome, LayoutRollbackPlan, LayoutRollbackRequest, LayoutVersion,
    LayoutWriteCompatibilityPosture, S8LayoutRebindRequired, S8LayoutStaleBinding,
};
pub use planning::{
    S8AccessPlanCostEstimate, S8AccessPlanSelection, S8DeterministicSelectionRule,
    S8PlanFingerprint, S8PlanSelectionDenied, S8PlanningCapabilityGrant, S8SelectedAccessPlan,
    S8SelectionCandidateAudit, S8SelectionCandidateEligibility, S8SelectionCandidateOutcome,
    S8SelectionCandidateRejection,
};
pub use strategy::{
    S8BTreeCorruptionRegion, S8BTreeInvariantSuite, S8BTreeLookupBranch, S8BTreeNodeFormatLaw,
    S8BTreeRebuildMigrationLaw, S8BTreeRootPublicationLaw, S8BTreeSearchPathLaw,
    S8BTreeSeparatorLaw, S8BTreeSiblingLinkLaw, S8BTreeSplitMergeLaw, S8BTreeStableReadLaw,
    S8BTreeTombstoneLaw, S8LayoutStrategyFamily, S8LsmAdvisoryFilterLaw,
    S8LsmCompactionOrderingLaw, S8LsmInvariantSuite, S8LsmLookupDisposition, S8LsmMemtableWalLaw,
    S8LsmRunPublicationLaw, S8LsmStaleRunCleanupLaw, S8LsmTombstoneLaw, S8LsmWriteAmplificationLaw,
    S8StrategyAmplificationProfile, S8StrategyCorruptionIsolationBehavior,
    S8StrategyCounterEvidence, S8StrategyCounterProfile, S8StrategyDenial,
    S8StrategyIntegrityInvariant, S8StrategyInvariantSuite, S8StrategyLocalityProfile,
    S8StrategyLookupInvariant, S8StrategyMaterializationPosture, S8StrategyMutationInvariant,
    S8StrategyPublicationInvariant, S8StrategyRebuildSourceRequirement,
    S8StrategyRecoveryInvariant,
};

#[path = "compile_fail/certification_authority.rs"]
#[doc(hidden)]
pub mod s8_certification_authority_compile_fail;
#[path = "compile_fail/compaction_family_kind_shortcut.rs"]
#[doc(hidden)]
pub mod s8_compaction_family_kind_shortcut_compile_fail;
#[path = "compile_fail/derived_authority_shortcut.rs"]
#[doc(hidden)]
pub mod s8_derived_authority_shortcut_compile_fail;
#[path = "compile_fail/facade_bypass.rs"]
#[doc(hidden)]
pub mod s8_facade_bypass_compile_fail;
#[path = "compile_fail/lifecycle_strategy_shortcut.rs"]
#[doc(hidden)]
pub mod s8_lifecycle_strategy_shortcut_compile_fail;
#[path = "compile_fail/raw_accuracy_shortcut.rs"]
#[doc(hidden)]
pub mod s8_raw_accuracy_shortcut_compile_fail;
#[path = "compile_fail/raw_construction.rs"]
#[doc(hidden)]
pub mod s8_raw_construction_compile_fail;
#[path = "compile_fail/raw_declaration_classification_shortcut.rs"]
#[doc(hidden)]
pub mod s8_raw_declaration_classification_shortcut_compile_fail;
#[path = "compile_fail/raw_hash_identity_shortcut.rs"]
#[doc(hidden)]
pub mod s8_raw_hash_identity_shortcut_compile_fail;
#[path = "compile_fail/raw_key_comparator_shortcut.rs"]
#[doc(hidden)]
pub mod s8_raw_key_comparator_shortcut_compile_fail;
#[path = "compile_fail/raw_range_bound_shortcut.rs"]
#[doc(hidden)]
pub mod s8_raw_range_bound_shortcut_compile_fail;
#[path = "compile_fail/raw_role_shortcut.rs"]
#[doc(hidden)]
pub mod s8_raw_role_shortcut_compile_fail;
#[path = "compile_fail/raw_scope_shortcut.rs"]
#[doc(hidden)]
pub mod s8_raw_scope_shortcut_compile_fail;
