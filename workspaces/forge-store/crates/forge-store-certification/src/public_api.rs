//! Lifecycle-ordered public facade for forge-store-certification.
//!
//! Order: authority ÃƒÂ¢Ã¢â‚¬Â Ã¢â‚¬â„¢ evidence ÃƒÂ¢Ã¢â‚¬Â Ã¢â‚¬â„¢ scenario ÃƒÂ¢Ã¢â‚¬Â Ã¢â‚¬â„¢ replay ÃƒÂ¢Ã¢â‚¬Â Ã¢â‚¬â„¢ harness ÃƒÂ¢Ã¢â‚¬Â Ã¢â‚¬â„¢ closeout ÃƒÂ¢Ã¢â‚¬Â Ã¢â‚¬â„¢ lanes.

// --- runtime matrix: courtroom-only Phase 33 completeness gate ---
pub use crate::courtroom::layout::runtime_matrix::{
    require_complete_layout_runtime_matrix, required_layout_runtime_obligations,
    LayoutRuntimeCompletenessDenial, LayoutRuntimeStrategyEquivalenceClass,
};

// --- authority: substrate certification entry points ---
pub use crate::authority::{
    certify_physical_page_segment_extent_substrate, certify_store_json_residue_inventory,
    PhysicalSubstrateCertificationDenial, StoreCertificationProgram, StoreJsonAuthorityRisk,
    StoreJsonResidueClassification, StoreJsonResidueDenial, StoreJsonResidueInventory,
    StoreJsonResidueOccurrence, StoreJsonResidueTokenKind, StoreJsonResidueZone,
};
// --- evidence: substrate evidence families ---
pub use crate::courtroom::cross_cutting::certification_matrix::S1CertificationRow;
pub use crate::courtroom::foundational::canonical_basis_source_inventory::{
    certify_scanned_store_canonical_basis_source_inventory,
    certify_store_canonical_basis_source_inventory, certify_store_canonical_basis_source_rows,
    current_store_canonical_basis_inventory, StoreCanonicalBasisInventoryDenial,
    StoreCanonicalBasisInventoryRow,
};
pub use crate::courtroom::memory::bounded_memory_closeout::{
    BoundedMemoryCloseoutDenial, BoundedMemoryCloseoutReport,
};
pub use crate::courtroom::memory::bounded_memory_residency_suite::{
    BoundedMemoryOperationKind, BoundedMemoryResidencySuite, BoundedMemoryResidencySuiteDenial,
    BoundedOperationEnvelopeCounters, BoundedOperationEnvelopeReport, S2BoundaryDenialKind,
};
pub use crate::courtroom::memory::buffer_pool_certification_bundle::{
    BufferPoolCertificationBundle, BufferPoolCertificationBundleDenial,
};
pub use crate::evidence::by_substrate::{
    certify_s0_handoff_gate_proof_evidence, offline_observer_requires_physical_references,
    AllocationEnvelopeEvidenceDenial, AllocationEnvelopeEvidenceReport,
    AllocationEnvelopeEvidenceRow, AllocationEnvelopePerformanceReceipt,
    BackgroundClassEnvelopeEvidence, BackgroundEnvelopeEvidenceBundle,
    BackgroundEnvelopeEvidenceDenial, BinaryPhysicalFormatEvidence,
    BinaryPhysicalFormatEvidenceDenial, BufferPoolProvenanceAttachment,
    CompletedResidencyBoundaryReceipt, CopyMaterializationPerformanceReceipt,
    DirtyPublicationEvidenceDenial, DirtyPublicationEvidenceReport, DirtyPublicationEvidenceRow,
    EvictionProtectionEvidenceDenial, EvictionProtectionEvidenceReport,
    EvictionProtectionEvidenceRow, FoundationalBoundaryAuthorityResult,
    FoundationalBoundaryEvidenceDenial, FoundationalEvidenceProfile, FoundationalEvidenceRichness,
    LargeStorePressureEvidenceBundle, LargeStorePressureEvidenceDenial, LargeStoreShortcutAttempt,
    MaterializationProfileReport, PhysicalComplexityEvidenceDenial,
    PhysicalComplexityEvidenceReport, PhysicalComplexityProofBundle,
    PhysicalExtentRecordFramingEvidenceDenial, PhysicalExtentRecordFramingEvidenceReport,
    PhysicalExtentRecordFramingEvidenceRow, PhysicalFoundationEvidenceBundle,
    PhysicalFoundationEvidenceBundleBuilder, PhysicalFoundationEvidenceDenial,
    PhysicalFoundationEvidenceEntry, PhysicalFoundationEvidenceIdentity,
    PhysicalHeaderDecodeEvidenceDenial, PhysicalHeaderDecodeEvidenceReport,
    PhysicalHeaderDecodeEvidenceRow, PhysicalIdentityEvidenceDenial,
    PhysicalIdentityEvidenceReport, PhysicalIdentityEvidenceRow,
    PhysicalManifestDiscoveryEvidenceDenial, PhysicalManifestDiscoveryEvidenceReport,
    PhysicalManifestDiscoveryEvidenceRow, PhysicalOfflineVerifierEvidenceDenial,
    PhysicalOfflineVerifierEvidenceReport, PhysicalOfflineVerifierEvidenceRow,
    PhysicalPageRecordFramingEvidenceDenial, PhysicalPageRecordFramingEvidenceReport,
    PhysicalPageRecordFramingEvidenceRow, PinLifecycleEvidenceDenial, PinLifecycleEvidenceReport,
    PinLifecycleEvidenceRow, PlatformPhysicalFacadeEvidenceDenial,
    PlatformPhysicalFacadeEvidenceReport, PlatformPhysicalFacadeEvidenceRow,
    ProtectedIntegrityViewEvidence, ProtectedIntegrityViewEvidenceDenial, RecordViewEvidenceDenial,
    RecordViewEvidenceReport, RecordViewEvidenceRow, RequiredInterferenceKind,
    ResidentFrameAuthorityEvidenceDenial, ResidentFrameAuthorityEvidenceReport,
    ResidentFrameAuthorityEvidenceRow, ResidentMemoryPerformanceReceipt,
    S0HandoffGateCertificationDenial, S2EntryBoundaryEvidenceDenial, S2EntryBoundaryEvidenceReport,
    S2EntryBoundaryEvidenceRow, S2ForbiddenEntryAttempt, SpeculativeWorkEvidenceDenial,
    SpeculativeWorkEvidenceReport, SpeculativeWorkEvidenceRow, ZeroCopyLayoutPostureReport,
};
pub use crate::scenario::memory::bounded_memory_harness_closeout::{
    HarnessCloseoutEvidenceReport, HarnessCloseoutTranscriptEvidence,
};
pub use crate::scenario::memory::buffer_pool_scenario_definitions::{
    LargeStoreMemoryPressureScenario, LargeStoreScenarioDenial,
};
pub use crate::scenario::memory::buffer_pool_scenario_plans::{
    BufferPoolScenarioPlan, BufferPoolScenarioPlanDenial,
};
pub use crate::scenario::memory::buffer_pool_transcripts::BufferPoolPressureTranscriptIdentity;
// --- closeout: milestone certification bundles and handoff evidence ---
pub use crate::courtroom::closeout::{
    adopt_materialized_s6_certification_evidence_for_closeout,
    assemble_physical_isolation_physical_isolation_replay_bundle,
    certify_native_blob_store_closeout, certify_security_scope_closeout,
    close_s3_physical_integrity_from_executed_evidence, evaluate_blob_closeout_request,
    materialize_s5_executed_isolation_evidence, materialize_s6_certification_evidence,
    observe_physical_isolation_physical_isolation_trace, physical_isolation_lanes,
    physical_isolation_physical_isolation_ci_certification_context_without_lane_registration,
    physical_isolation_physical_isolation_ci_certification_planning_context,
    physical_isolation_physical_isolation_context_without_lane_registration,
    physical_isolation_physical_isolation_coverage_matrix,
    physical_isolation_physical_isolation_planning_context,
    physical_isolation_required_mutation_rows, BlobCloseoutCertificationInput, BlobCloseoutDenial,
    BlobCloseoutEvidencePolicy, BlobCloseoutRequest, BlobCloseoutShortcutAttempt,
    BlobCloseoutShortcutInput, BlobCloseoutShortcutRejectionReport, BlobStoreCloseoutCertificate,
    ExecutedPhysicalIsolationEvidenceSource, ExecutedPhysicalIsolationFinding,
    ExecutedPhysicalIsolationOutcome, ExecutedPhysicalIsolationRequiredCounters,
    ExecutedPhysicalIsolationSourceBasis, ExecutedPhysicalIsolationSourceDenial,
    PhysicalIntegrityCertificationBundle, PhysicalIntegrityCloseoutDenial,
    PhysicalIntegrityCloseoutReport, PhysicalIntegrityCloseoutSuite,
    PhysicalIntegrityCloseoutSuiteEvidence, PhysicalIsolationCloseoutDenial,
    PhysicalIsolationCloseoutHandoffEvidence, PhysicalIsolationCloseoutLaneEvidence,
    PhysicalIsolationCloseoutSuite, PhysicalIsolationEvidenceProfileCounterSet,
    PhysicalIsolationMutationEvidence, PhysicalPageSegmentExtentSubstrateCloseout,
    PhysicalPageSegmentExtentSubstrateEvidence, PhysicalPageSegmentExtentSubstrateRun,
    PhysicalSubstrateCloseoutDenial, PhysicalSubstrateCloseoutStoryDenial,
    PhysicalSubstrateCloseoutStoryReport, PhysicalSubstrateCloseoutStoryRow,
    RecoveryPhysicsCertificationDenial, RecoveryPhysicsCertificationMatrix,
    RecoveryPhysicsCertificationRow, RecoveryPhysicsCounterExpectation, RecoveryPhysicsCounterKind,
    RecoveryPhysicsCrashLane, RecoveryPhysicsCrashMatrix, RecoveryPhysicsCrashMatrixBuilder,
    RecoveryPhysicsCrashMatrixDenial, RecoveryPhysicsEvidenceBundle, RecoveryPhysicsMutant,
    RecoveryPhysicsMutationFailureEvidence, RecoveryPhysicsMutationSuiteEvidence,
    RecoveryPhysicsMutationSuiteEvidenceDenial, RecoveryPhysicsMutationSuiteLaneEvidence,
    RecoveryPhysicsMutationValidationDenial, RecoveryPhysicsMutationValidationMatrix,
    RecoveryPhysicsMutationValidationRow, RecoveryPhysicsObserverKind,
    RecoveryPhysicsOracleJudgment, RecoveryPhysicsOracleKind,
    RecoveryPhysicsRoadmap2HarnessCertification, RecoveryPhysicsRoadmap2HarnessDenial,
    RecoveryPhysicsScenarioDefinition, RecoveryPhysicsScenarioDefinitionBuilder,
    RecoveryPhysicsScenarioDefinitionDenial, RecoveryPhysicsScenarioDrivers,
    RecoveryPhysicsScenarioPlan, RecoveryPhysicsScenarioPlanDenial, RecoveryPhysicsShortcutAttempt,
    RecoveryPhysicsShortcutDenialBoundary, RecoveryPhysicsShortcutDenialReason,
    RecoveryPhysicsShortcutRejection, RecoveryPhysicsTranscript, S3AcceptanceSuiteKind,
    S3CloseoutDenialBoundary, S3CloseoutEvidenceFamily, S3CloseoutExecutedOutputKind,
    S3CloseoutHarnessExecutionEvidence, S3CloseoutModuleKind, S3CloseoutSuiteHarnessSummary,
    S3CorruptionLocalizationBoundary, S3ExecutedBoundaryDenialEvidence,
    S3ExecutedCorruptionLocalizationEvidence, S3HarnessTranscriptEvidence,
    S3LineCapCompositionEvidence, S3LineCapModuleEvidence, S3OwnedCloseoutFileEvidence,
    S3S4HandoffCloseoutEvidence, S51CertificationCloseoutDenial, S51CertificationCloseoutEvidence,
    S51CertificationCloseoutInput, S51CertificationEvidencePolicy, S51CloseoutApiAdoptionEvidence,
    S51CloseoutBoundaryEvidencePublication, S51CloseoutCounterMatrix,
    S51CloseoutFoundationalBoundaryPackage, S51CloseoutFoundationalLane,
    S51CloseoutPerformanceReceipts, S51CloseoutPerformanceRows, S5CloseoutReservationSet,
    S5CloseoutReservedScope, S5ExecutedIsolationEvidenceBundle,
    S5ExecutedIsolationMaterializationDenial, S5FoundationalCanonicalBasis,
    S5FoundationalDiagnostics, S5FoundationalPerformanceReceipts, S5PhysicalIsolationHarnessLane,
    S5PhysicalIsolationProofTrace, S5PhysicalIsolationTraceFixtures, S5ProofProjectionArtifact,
    S6CertificationEvidenceAdoptionReceipt, SyntheticCloseoutRejectionDenial,
    SyntheticCloseoutShortcutAttempt, SyntheticCloseoutShortcutInput,
    SyntheticCloseoutShortcutRejectionReport,
};
// --- closeout/s6: S.6 certification materialization ---
pub use crate::courtroom::closeout::s6::{
    certify_io_pressure_backend_qualification_matrix, certify_s6_backend_capability_admission,
    certify_s6_background_pacing, certify_s6_foreground_reservation,
    publish_s6_backend_capability_readiness,
    reject_materialized_s6_certification_as_runtime_authority, IoPressureHarnessCloseoutDenial,
    IoPressureHarnessCloseoutEvidence, S6AccessPolicyEvidenceOutcomeKind,
    S6AccessPolicyEvidenceRow, S6BackendCapabilityAdmissionCertificationEvidence,
    S6BackendCapabilityReadinessPublication, S6BackendQualificationMatrixCertification,
    S6BackendQualificationRowOutcome, S6BackgroundPacingCertificationDenial,
    S6BackgroundPacingCertificationEvidence, S6BackgroundPacingOutcomeKind,
    S6CanonicalEvidenceBasis, S6CanonicalMaterializationDenial,
    S6CertificationMaterializationDenial, S6CertificationProofTrace,
    S6CertificationRuntimeAuthorityDenial, S6CertifiedQueueExecutionEvidence,
    S6CounterStrengthDeclaration, S6CounterStrengthFamily, S6FlushDurabilityEvidenceRow,
    S6ForegroundReservationCertificationDenial, S6ForegroundReservationCertificationEvidence,
    S6FoundationalAuthorityBoundary, S6FoundationalPerformanceReceipts,
    S6FoundationalProfileEvidence, S6IoQosReadinessHandoffMaterializationDenial,
    S6LatencyInterferenceCertificationDenial, S6LatencyInterferenceEvidence,
    S6MaterializedCertificationEvidenceBundle, S6MaterializedCounterStrength,
    S6PostAdmissionViolationCause, S6PostAdmissionViolationEvidenceRow,
    S6PostAdmissionViolationFamily, S6ProofProjectionArtifact, S6QueueExecutionCertificationDenial,
    S6ReclaimPolicyEvidenceOutcomeKind, S6ReclaimPolicyEvidenceRow,
    StoreOwnedS6CertificationMaterializationSources,
};
// --- harness: scenario quality and oracle surfaces ---
pub use crate::courtroom::harness::{
    PhysicalOracleDenialKind, PhysicalOracleJudgment, PhysicalOracleOutcome,
    PhysicalProofOracleKind, PhysicalProofOracleVerdict, PhysicalScenarioDriverKind,
    PhysicalScenarioDriverRequirement, PhysicalScenarioHarnessDenial, PhysicalScenarioObserverKind,
    PhysicalScenarioObserverRequirement, PhysicalScenarioQualityHarness,
};
// --- replay: observed traces and verifier comparison ---
pub use crate::courtroom::replay::{
    assemble_s8_layout_replay_bundle, FixtureAdversaryPosture, FixtureAdversaryReport,
    LargeStorePressureClass, ObservedPhysicalTrace, OfflineVerifierObserver,
    PhysicalCounterExpectationKind, PhysicalHostileScaleCondition,
    PhysicalHostileScaleFixtureDenial, PhysicalHostileScaleFixtureReport,
    PhysicalHostileScaleFixtureSource, PhysicalLayoutParity, PhysicalLayoutParityDenial,
    PhysicalLayoutParityReport, PhysicalRuntimeVerifierComparison, PhysicalScalePropertyEvidence,
    PhysicalStoryTranscript, RuntimeLayoutObserver, RuntimeVerifierComparisonClassification,
    RuntimeVerifierComparisonDenial, RuntimeVerifierComparisonReport,
    RuntimeVerifierDiagnosticDenial, RuntimeVerifierDiagnosticKind,
    RuntimeVerifierDiagnosticReport, RuntimeVerifierParityTrace, RuntimeVerifierRelationship,
    RuntimeVerifierSupportDenial, RuntimeVerifierSupportReport, S8LayoutReplayBundle,
    ScenarioCounterExpectation, ScenarioCounterObservation, ScenarioCounterTrace,
    ScenarioDenialBoundary, ScenarioDenialTrace, ScenarioObserverTrace, ShortcutRejectionTrace,
};
// --- scenario: definition, planning, and execution ---
pub use crate::courtroom::scenario::{
    certify_s8_layout_scenario, ArtifactPolicy, ExpectedPhysicalFootprint,
    PhysicalScenarioCapabilityTier, PhysicalScenarioCostClass, PhysicalScenarioDefinition,
    PhysicalScenarioDefinitionBuilder, PhysicalScenarioDefinitionDenial, PhysicalScenarioExecution,
    PhysicalScenarioExecutionReport, PhysicalScenarioPlan, PhysicalScenarioPlanDenial,
    PhysicalScenarioPlanIdentity, PhysicalScenarioPlannedWorkBoundaryReport, PhysicalStoryStep,
    S8LayoutScenarioCertificate, ScenarioLane, StorageBoundaryCrossing, WorkloadScale,
};
// --- lanes: substrate lane vocabulary ---
pub use crate::courtroom::cross_cutting::lanes::{
    LaneFamilyExtension, PhysicalSubstrateLane, RoadmapLaneFamily,
};
pub use crate::s2_acceptance_suite_transcript::S2AcceptanceSuiteKind;
