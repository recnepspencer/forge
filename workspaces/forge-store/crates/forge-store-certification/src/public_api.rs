//! Lifecycle-ordered public facade for forge-store-certification.
//!
//! Order: authority ÃƒÂ¢Ã¢â‚¬Â Ã¢â‚¬â„¢ evidence ÃƒÂ¢Ã¢â‚¬Â Ã¢â‚¬â„¢ scenario ÃƒÂ¢Ã¢â‚¬Â Ã¢â‚¬â„¢ replay ÃƒÂ¢Ã¢â‚¬Â Ã¢â‚¬â„¢ harness ÃƒÂ¢Ã¢â‚¬Â Ã¢â‚¬â„¢ closeout ÃƒÂ¢Ã¢â‚¬Â Ã¢â‚¬â„¢ lanes.

// --- runtime matrix: courtroom-only Phase 33 completeness gate ---
pub use crate::s8_runtime_matrix::{
    require_complete_s8_runtime_matrix, required_s8_runtime_cases, S8RuntimeMatrixDenial,
    S8RuntimeStrategyEquivalenceClass,
};

// --- authority: substrate certification entry points ---
pub use crate::authority::{
    certify_physical_page_segment_extent_substrate, certify_store_json_residue_inventory,
    PhysicalSubstrateCertificationDenial, StoreCertificationProgram, StoreJsonAuthorityRisk,
    StoreJsonResidueClassification, StoreJsonResidueDenial, StoreJsonResidueInventory,
    StoreJsonResidueOccurrence, StoreJsonResidueTokenKind, StoreJsonResidueZone,
};
// --- evidence: substrate evidence families ---
pub use crate::bounded_memory_closeout::{
    BoundedMemoryCloseoutDenial, BoundedMemoryCloseoutReport,
};
pub use crate::bounded_memory_harness_closeout::{
    HarnessCloseoutEvidenceReport, HarnessCloseoutTranscriptEvidence,
};
pub use crate::bounded_memory_residency_suite::{
    BoundedMemoryOperationKind, BoundedMemoryResidencySuite, BoundedMemoryResidencySuiteDenial,
    BoundedOperationEnvelopeCounters, BoundedOperationEnvelopeReport, S2BoundaryDenialKind,
};
pub use crate::buffer_pool_certification_bundle::{
    BufferPoolCertificationBundle, BufferPoolCertificationBundleDenial,
};
pub use crate::buffer_pool_scenario_definitions::{
    LargeStoreMemoryPressureScenario, LargeStoreScenarioDenial,
};
pub use crate::buffer_pool_scenario_plans::{BufferPoolScenarioPlan, BufferPoolScenarioPlanDenial};
pub use crate::buffer_pool_transcripts::BufferPoolPressureTranscriptIdentity;
pub use crate::canonical_basis_source_inventory::{
    certify_scanned_store_canonical_basis_source_inventory,
    certify_store_canonical_basis_source_inventory, certify_store_canonical_basis_source_rows,
    current_store_canonical_basis_inventory, StoreCanonicalBasisInventoryDenial,
    StoreCanonicalBasisInventoryRow,
};
pub use crate::certification_matrix::S1CertificationRow;
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
// --- closeout: milestone certification bundles and handoff evidence ---
pub use crate::courtroom::closeout::{
    adopt_materialized_s6_certification_evidence_for_closeout,
    assemble_s5_physical_isolation_replay_bundle, certify_s5_1_security_scope_closeout,
    certify_native_blob_store_closeout, certify_s8_layout_closeout,
    certify_s8_layout_closeout_suite, classify_s8_layout_closeout_sources,
    close_s3_physical_integrity_from_executed_evidence, evaluate_blob_closeout_request,
    materialize_s5_executed_isolation_evidence, materialize_s6_certification_evidence,
    observe_s5_physical_isolation_trace, project_s8_layout_handoff_grammar,
    s5_physical_isolation_ci_certification_context_without_lane_registration,
    s5_physical_isolation_ci_certification_planning_context,
    s5_physical_isolation_context_without_lane_registration, s5_physical_isolation_coverage_matrix,
    s5_physical_isolation_lanes, s5_physical_isolation_planning_context,
    s5_physical_isolation_required_mutation_rows, PhysicalIntegrityCertificationBundle,
    PhysicalIntegrityCloseoutDenial, PhysicalIntegrityCloseoutReport,
    PhysicalIntegrityCloseoutSuite, PhysicalIntegrityCloseoutSuiteEvidence,
    PhysicalIsolationCloseoutDenial, PhysicalIsolationCloseoutHandoffEvidence,
    PhysicalIsolationCloseoutLaneEvidence, PhysicalIsolationCloseoutSuite,
    PhysicalPageSegmentExtentSubstrateCloseout, PhysicalPageSegmentExtentSubstrateEvidence,
    PhysicalPageSegmentExtentSubstrateRun, PhysicalSubstrateCloseoutDenial,
    PhysicalSubstrateCloseoutStoryDenial, PhysicalSubstrateCloseoutStoryReport,
    PhysicalSubstrateCloseoutStoryRow, RecoveryPhysicsCertificationDenial,
    RecoveryPhysicsCertificationMatrix, RecoveryPhysicsCertificationRow,
    RecoveryPhysicsCounterExpectation, RecoveryPhysicsCounterKind, RecoveryPhysicsCrashLane,
    RecoveryPhysicsCrashMatrix, RecoveryPhysicsCrashMatrixBuilder,
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
    RecoveryPhysicsShortcutRejection, RecoveryPhysicsTranscript,
    BlobCloseoutCertificationInput, BlobCloseoutDenial, BlobCloseoutEvidencePolicy,
    BlobCloseoutRequest, BlobCloseoutShortcutAttempt, BlobCloseoutShortcutInput,
    BlobCloseoutShortcutRejectionReport, BlobStoreCloseoutCertificate,
    S3AcceptanceSuiteKind, S3CloseoutDenialBoundary,
    S3CloseoutEvidenceFamily, S3CloseoutExecutedOutputKind, S3CloseoutHarnessExecutionEvidence,
    S3CloseoutModuleKind, S3CloseoutSuiteHarnessSummary, S3CorruptionLocalizationBoundary,
    S3ExecutedBoundaryDenialEvidence, S3ExecutedCorruptionLocalizationEvidence,
    S3HarnessTranscriptEvidence, S3LineCapCompositionEvidence, S3LineCapModuleEvidence,
    S3OwnedCloseoutFileEvidence, S3S4HandoffCloseoutEvidence, S51CertificationCloseoutDenial,
    S51CertificationCloseoutEvidence, S51CertificationCloseoutInput,
    S51CertificationEvidencePolicy, S51CloseoutApiAdoptionEvidence,
    S51CloseoutBoundaryEvidencePublication, S51CloseoutCounterMatrix,
    S51CloseoutFoundationalBoundaryPackage, S51CloseoutFoundationalLane,
    S51CloseoutPerformanceReceipts, S51CloseoutPerformanceRows, S5CloseoutReservationSet,
    S5CloseoutReservedScope, S5EvidenceProfileCounterSet, S5ExecutedIsolationEvidenceBundle,
    S5ExecutedIsolationEvidenceSource, S5ExecutedIsolationFinding,
    S5ExecutedIsolationMaterializationDenial, S5ExecutedIsolationOutcome,
    S5ExecutedIsolationRequiredCounters, S5ExecutedIsolationSourceBasis,
    S5ExecutedIsolationSourceDenial, S5FoundationalCanonicalBasis, S5FoundationalDiagnostics,
    S5FoundationalPerformanceReceipts, S5PhysicalIsolationHarnessLane,
    S5PhysicalIsolationMutationEvidence, S5PhysicalIsolationProofTrace,
    S5PhysicalIsolationTraceFixtures, S5ProofProjectionArtifact,
    S6CertificationEvidenceAdoptionReceipt, S8LayoutCloseoutCertificate, S8LayoutCloseoutClassification,
    S8LayoutCloseoutDenial, S8LayoutCloseoutSources, S8LayoutCloseoutSuiteCertificate,
    S8LayoutCloseoutVerifier, S8LayoutCourtroomGrammar, SyntheticCloseoutRejectionDenial,
    SyntheticCloseoutShortcutAttempt, SyntheticCloseoutShortcutInput,
    SyntheticCloseoutShortcutRejectionReport,
};
// --- closeout/s6: S.6 certification materialization ---
pub use crate::courtroom::closeout::s6::{
    certify_s6_backend_capability_admission, certify_s6_backend_qualification_matrix,
    certify_s6_background_pacing, certify_s6_foreground_reservation,
    certify_s6_later_readiness_handoffs, publish_s6_backend_capability_readiness,
    reject_materialized_s6_certification_as_runtime_authority, S6AccessPolicyEvidenceOutcomeKind,
    S6AccessPolicyEvidenceRow, S6BackendCapabilityAdmissionCertificationEvidence,
    S6BackendCapabilityReadinessPublication, S6BackendQualificationMatrixCertification,
    S6BackendQualificationRowOutcome, S6BackgroundPacingCertificationDenial,
    S6BackgroundPacingCertificationEvidence, S6BackgroundPacingOutcomeKind,
    S6BackupExportHandoffEvidence, S6CanonicalEvidenceBasis, S6CanonicalMaterializationDenial,
    S6CertificationMaterializationDenial, S6CertificationProofTrace,
    S6CertificationRuntimeAuthorityDenial, S6CertifiedQueueExecutionEvidence,
    S6CompactionHandoffEvidence, S6CounterStrengthDeclaration, S6CounterStrengthFamily,
    S6FlushDurabilityEvidenceRow, S6ForegroundReservationCertificationDenial,
    S6ForegroundReservationCertificationEvidence, S6FoundationalAuthorityBoundary,
    S6FoundationalPerformanceReceipts, S6FoundationalProfileEvidence,
    S6IoPressureHarnessCloseoutDenial, S6IoPressureHarnessCloseoutEvidence,
    S6IoQosReadinessHandoffMaterializationDenial, S6LatencyInterferenceCertificationDenial,
    S6LatencyInterferenceEvidence, S6LaterReadinessHandoffCertification,
    S6MaterializedCertificationEvidenceBundle, S6MaterializedCounterStrength,
    S6OperatorHandoffEvidence, S6PlacementHandoffEvidence, S6PostAdmissionViolationCause,
    S6PostAdmissionViolationEvidenceRow, S6PostAdmissionViolationFamily, S6ProofProjectionArtifact,
    S6QueueExecutionCertificationDenial, S6ReclaimPolicyEvidenceOutcomeKind,
    S6ReclaimPolicyEvidenceRow, S6RepairScanHandoffEvidence, S6ResidualDebtKind,
    S6ResidualDebtLedger, S6ResidualDebtRow, StoreOwnedS6CertificationMaterializationSources,
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
pub use crate::lanes::{LaneFamilyExtension, PhysicalSubstrateLane, RoadmapLaneFamily};
pub use crate::s2_acceptance_suite_transcript::S2AcceptanceSuiteKind;
