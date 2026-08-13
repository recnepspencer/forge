//! Lifecycle-ordered public facade for worth-store-certification.
//!
//! Order: authority -> evidence -> scenario -> replay -> harness -> adjudication -> lanes.

pub use crate::courtroom::foundational::{
    accept_aspect_native_boundary_handoff, reconstruct_aspect_native_boundary_verdict,
    reject_terminal_json_projection_as_boundary_handoff, AspectNativeBoundaryHandoff,
    AspectNativeBoundaryHandoffDenial, AspectNativeBoundaryHandoffVerdict,
    AspectNativeRejectedInputKind,
};
pub use crate::courtroom::layout::adjudication::{
    adjudicate_layout_courtroom, adjudicate_layout_hazards, assemble_layout_evidence_bundle,
    certify_layout_foundational_closeout, observe_layout_proof_outcomes, LayoutCompileFailBoundary,
    LayoutCourtroomDenial, LayoutCourtroomReport, LayoutCourtroomTranscriptIdentity,
    LayoutEvidenceAssemblyDenial, LayoutEvidenceBundle, LayoutFoundationalCloseoutDenial,
    LayoutFoundationalCloseoutEvidence, LayoutHazard, LayoutHazardAdjudicationDenial,
    LayoutHazardEvidencePosture, LayoutHazardInventory, LayoutHazardRow, LayoutProofOutcomeKind,
    LayoutProofOutcomeObservation,
};
pub use crate::courtroom::layout::formal_observation::{
    observe_layout_formal_model, LayoutDurableArtifactKind, LayoutDurableArtifactObservation,
    LayoutDurableOrdering, LayoutFormalInvariant, LayoutFormalObservation,
    LayoutFormalObservationDenial, LayoutFormalOwnerFamilyObservation,
};
pub use crate::courtroom::layout::owner_coverage::{
    certify_exact_owner_case_coverage, require_exact_owner_case_coverage,
    LayoutOwnerCaseDeclarations, LayoutOwnerCoverageDenial, LayoutOwnerCoverageIssue,
    LayoutOwnerCoverageReceipt, LayoutOwnerFamily, LayoutOwnerObservationLedger,
};
pub use crate::courtroom::layout::owner_evidence::{
    certify_layout_owner_execution_evidence, LayoutOwnerExecutionEvidence,
    LayoutOwnerExecutionEvidenceDenial,
};
pub use crate::courtroom::layout::owner_scenarios::{
    execute_declaration_owner_scenarios, LayoutOwnerScenarioExecutionDenial,
    LayoutOwnerScenarioTranscript,
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
pub use crate::evidence::by_substrate::{
    certify_aspect_native_boundary_audit, offline_observer_requires_physical_references,
    AspectNativeBoundaryCertificationDenial, BinaryPhysicalFormatEvidence,
    BinaryPhysicalFormatEvidenceDenial, FoundationalPerformanceEvidenceDenial,
    InMemoryPhysicalFormatModelEvidenceDenial, InMemoryPhysicalFormatModelEvidenceReport,
    InMemoryPhysicalFormatModelEvidenceRow, LargeStorePressureEvidenceBundle,
    LargeStorePressureEvidenceDenial, LargeStoreShortcutAttempt, PhysicalComplexityEvidenceDenial,
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
    PhysicalPageRecordFramingEvidenceRow,
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
    assemble_physical_isolation_replay_bundle, certify_native_blob_store_closeout,
    certify_security_scope_closeout, close_physical_integrity_from_executed_evidence,
    evaluate_blob_closeout_request, materialize_physical_isolation_executed_isolation_evidence,
    observe_physical_isolation_trace,
    physical_isolation_ci_certification_context_without_lane_registration,
    physical_isolation_ci_certification_planning_context,
    physical_isolation_context_without_lane_registration, physical_isolation_coverage_matrix,
    physical_isolation_lanes, physical_isolation_planning_context,
    physical_isolation_required_mutation_rows, BlobCloseoutCertificationInput, BlobCloseoutDenial,
    BlobCloseoutEvidencePolicy, BlobCloseoutRequest, BlobCloseoutShortcutAttempt,
    BlobCloseoutShortcutInput, BlobCloseoutShortcutRejectionReport, BlobStoreCloseoutCertificate,
    CorruptionLocalizationBoundary, ExecutedCorruptionLocalizationEvidence,
    ExecutedIntegrityBoundaryDenialEvidence, ExecutedPhysicalIsolationEvidenceSource,
    ExecutedPhysicalIsolationFinding, ExecutedPhysicalIsolationOutcome,
    ExecutedPhysicalIsolationRequiredCounters, ExecutedPhysicalIsolationSourceBasis,
    ExecutedPhysicalIsolationSourceDenial, IntegrityCloseoutDenialBoundary,
    IntegrityCloseoutEvidenceFamily, IntegrityCloseoutExecutedOutputKind,
    IntegrityCloseoutHarnessSummary, IntegrityCloseoutModuleKind, IntegrityCompositionEvidence,
    IntegrityHarnessExecutionEvidence, IntegrityHarnessTranscriptEvidence,
    IntegrityModuleCompositionEvidence, IntegrityOwnedCloseoutFileEvidence,
    IntegrityRecoveryHandoffCloseoutEvidence, PhysicalIntegrityAcceptanceSuite,
    PhysicalIntegrityCertificationBundle, PhysicalIntegrityCloseoutDenial,
    PhysicalIntegrityCloseoutReport, PhysicalIntegrityCloseoutSuite,
    PhysicalIntegrityCloseoutSuiteEvidence, PhysicalIsolationCloseoutDenial,
    PhysicalIsolationCloseoutLaneEvidence, PhysicalIsolationCloseoutSuite,
    PhysicalIsolationEvidenceProfileCounterSet, PhysicalIsolationExecutedCloseoutEvidence,
    PhysicalIsolationHarnessLane, PhysicalIsolationMutationEvidence,
    PhysicalIsolationTraceFixtures, PhysicalPageSegmentExtentSubstrateCloseout,
    PhysicalPageSegmentExtentSubstrateEvidence, PhysicalPageSegmentExtentSubstrateRun,
    PhysicalSubstrateCloseoutDenial, PhysicalSubstrateCloseoutStoryDenial,
    PhysicalSubstrateCloseoutStoryReport, PhysicalSubstrateCloseoutStoryRow,
    S51CertificationCloseoutDenial, S51CertificationCloseoutEvidence,
    S51CertificationCloseoutInput, S51CertificationEvidencePolicy, S51CloseoutApiAdoptionEvidence,
    S51CloseoutBoundaryEvidencePublication, S51CloseoutCounterMatrix,
    S51CloseoutFoundationalBoundaryPackage, S51CloseoutFoundationalLane,
    S51CloseoutPerformanceReceipts, S51CloseoutPerformanceRows, S5CloseoutReservationSet,
    S5CloseoutReservedScope, S5ExecutedIsolationEvidenceBundle,
    S5ExecutedIsolationMaterializationDenial, S5FoundationalCanonicalBasis,
    S5FoundationalDiagnostics, S5FoundationalPerformanceReceipts, S5PhysicalIsolationProofTrace,
    S5ProofProjectionArtifact, SyntheticCloseoutRejectionDenial, SyntheticCloseoutShortcutAttempt,
    SyntheticCloseoutShortcutInput, SyntheticCloseoutShortcutRejectionReport,
};
// --- closeout/s6: S.6 certification materialization ---
pub use crate::courtroom::scheduling::{
    adopt_materialized_io_qos_certification_evidence_for_closeout,
    certify_io_pressure_backend_qualification_matrix, certify_io_qos_backend_capability_admission,
    certify_io_qos_background_pacing, certify_io_qos_foreground_reservation,
    materialize_io_qos_certification_evidence, publish_io_qos_backend_capability_readiness,
    reject_materialized_io_qos_certification_as_runtime_authority, IoPressureHarnessCloseoutDenial,
    IoPressureHarnessCloseoutEvidence, S6AccessPolicyEvidenceOutcomeKind,
    S6AccessPolicyEvidenceRow, S6BackendCapabilityAdmissionCertificationEvidence,
    S6BackendCapabilityReadinessPublication, S6BackendQualificationMatrixCertification,
    S6BackendQualificationRowOutcome, S6BackgroundPacingCertificationEvidence,
    S6BackgroundPacingOutcomeKind, S6CanonicalEvidenceBasis, S6CanonicalMaterializationDenial,
    S6CertificationEvidenceAdoptionReceipt, S6CertificationMaterializationDenial,
    S6CertificationProofTrace, S6CertificationRuntimeAuthorityDenial,
    S6CertifiedQueueExecutionEvidence, S6CounterStrengthDeclaration, S6CounterStrengthFamily,
    S6FlushDurabilityEvidenceRow, S6ForegroundReservationCertificationDenial,
    S6ForegroundReservationCertificationEvidence, S6FoundationalAuthorityBoundary,
    S6FoundationalPerformanceReceipts, S6FoundationalProfileEvidence,
    S6LatencyInterferenceCertificationDenial, S6LatencyInterferenceEvidence,
    S6MaterializedCertificationAdoptionDenial, S6MaterializedCertificationAdoptionReceipt,
    S6MaterializedCertificationEvidenceBundle, S6MaterializedCounterStrength,
    S6PostAdmissionViolationCause, S6PostAdmissionViolationEvidenceRow,
    S6PostAdmissionViolationFamily, S6ProofProjectionArtifact, S6QueueExecutionCertificationDenial,
    S6ReadinessCertificationCounterEvidence, S6ReadinessCertificationCounterFamily,
    S6ReadinessCertificationCounterStrength, S6ReadinessCertificationProofSummary,
    S6ReadinessCertificationProofTopology, S6ReadinessResidualDebtEvidenceKind,
    S6ReadinessResidualDebtEvidenceRow, S6ReclaimPolicyEvidenceOutcomeKind,
    S6ReclaimPolicyEvidenceRow, StoreOwnedS6CertificationMaterializationSources,
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
    FixtureAdversaryPosture, FixtureAdversaryReport, LargeStorePressureClass,
    ObservedPhysicalTrace, OfflineVerifierObserver, PhysicalCounterExpectationKind,
    PhysicalHostileScaleCondition, PhysicalHostileScaleFixtureDenial,
    PhysicalHostileScaleFixtureReport, PhysicalHostileScaleFixtureSource, PhysicalLayoutParity,
    PhysicalLayoutParityDenial, PhysicalLayoutParityReport, PhysicalRuntimeVerifierComparison,
    PhysicalScalePropertyEvidence, PhysicalStoryTranscript, RuntimeLayoutObserver,
    RuntimeVerifierComparisonClassification, RuntimeVerifierComparisonDenial,
    RuntimeVerifierComparisonReport, RuntimeVerifierDiagnosticDenial,
    RuntimeVerifierDiagnosticKind, RuntimeVerifierDiagnosticReport, RuntimeVerifierParityTrace,
    RuntimeVerifierRelationship, RuntimeVerifierSupportDenial, RuntimeVerifierSupportReport,
    ScenarioCounterExpectation, ScenarioCounterObservation, ScenarioCounterTrace,
    ScenarioDenialBoundary, ScenarioDenialTrace, ScenarioObserverTrace, ShortcutRejectionTrace,
};
// --- scenario: definition, planning, and execution ---
pub use crate::courtroom::scenario::{
    ArtifactPolicy, ExpectedPhysicalFootprint, PhysicalScenarioCapabilityTier,
    PhysicalScenarioCostClass, PhysicalScenarioDefinition, PhysicalScenarioDefinitionBuilder,
    PhysicalScenarioDefinitionDenial, PhysicalScenarioExecution, PhysicalScenarioExecutionReport,
    PhysicalScenarioPlan, PhysicalScenarioPlanDenial, PhysicalScenarioPlanIdentity,
    PhysicalScenarioPlannedWorkBoundaryReport, PhysicalStoryStep, ScenarioLane,
    StorageBoundaryCrossing, WorkloadScale,
};
// --- lanes: substrate lane vocabulary ---
pub use crate::courtroom::cross_cutting::lanes::{
    LaneFamilyExtension, PhysicalSubstrateLane, RoadmapLaneFamily,
};
