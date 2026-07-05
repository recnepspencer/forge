#![forbid(unsafe_code)]

mod actors;
mod authoring;
mod closeout;
mod counters;
mod coverage;
mod drivers;
mod evidence;
mod faults;
mod fixtures;
mod observation;
mod oracles;
mod planning;
mod s45_entry;
mod s5_1_security_scope_harness;
mod s5_executed_isolation_contract;
mod s5_executed_isolation_source;
mod s5_handoff;
mod s5_physical_isolation_mutation;
mod s6_backend_qualification;
#[cfg(test)]
mod s6_backend_qualification_cross_backend_tests;
#[cfg(test)]
mod s6_backend_qualification_matrix_surface_tests;
#[cfg(test)]
mod s6_backend_qualification_negative_tests;
#[cfg(test)]
mod s6_backend_qualification_residual_debt_tests;
#[cfg(test)]
mod s6_backend_qualification_tests;
mod s6_io_pressure_coverage;
mod s6_io_pressure_execution;
mod s6_io_pressure_harness;
#[cfg(test)]
mod s6_io_pressure_harness_negative_tests;
#[cfg(test)]
mod s6_io_pressure_harness_tests;
mod s6_io_pressure_replay;
#[cfg(test)]
mod s6_io_pressure_replay_tests;
#[cfg(test)]
mod s6_io_pressure_shortcut_tests;
#[cfg(any(test, feature = "certification-test-support"))]
mod s6_io_pressure_test_support;
mod s6_io_pressure_vocab;
mod scenario;
mod schedule;
mod shortcut_rejection;
mod transcript;

pub use forge_store_offline_verifier::OfflineVerifierBoundarySeam;

pub use actors::{
    CheckpointActor, CompactionActor, ForegroundReadActor, ForegroundWriteActor,
    OfflineVerifierActor, PhysicalSimulationActor, PhysicalSimulationActorAdmissionDenial,
    ReclaimActor, RecoveryActor, ScrubActor,
};
pub use authoring::{
    physical_scenario, PhysicalScenarioBuilder, ScenarioBuilderActorStep,
    ScenarioBuilderExpectationStep, ScenarioBuilderFixtureStep, ScenarioBuilderScheduleStep,
};
pub use closeout::{
    FutureHarnessExtensionSlotInventory, FutureHarnessExtensionSlotReport,
    FuturePhysicalHarnessExtensionFamily, PhysicalSimulationHarnessCertificationBundle,
    PhysicalSimulationHarnessCloseoutDenial, PhysicalSimulationHarnessCloseoutReport,
    PhysicalSimulationHarnessCloseoutSuite, S45AcceptanceEvidenceLane, S45AcceptanceSuiteCoverage,
    S45AcceptanceSuiteEvidence, S45AcceptanceSuiteEvidenceSource, S45AcceptanceSuiteExecutionProof,
    S45AcceptanceSuiteMap, S45AcceptanceSuiteName, S45AcceptanceSuiteReceipt,
    S45AcceptanceSuiteReceiptSet, S45CloseoutCoverageReport, S45DogfoodSliceKind,
    S45ExecutedAcceptanceSuiteEvidence, S45ExecutedAcceptanceSuiteEvidenceSet,
    S45HarnessDogfoodEvidence, S45HarnessDogfoodReport, S4RecoveryDogfoodScenario,
    S4RecoveryDogfoodSliceEvidence, S5ReadinessShapeProbeScenario,
    S5ReadinessShapeProbeSliceEvidence, ShortcutRejectionDogfoodScenario,
    ShortcutRejectionDogfoodSliceEvidence,
};
pub use counters::{
    admit_physical_counter_evidence, reject_hostile_counter_evidence_for_readmission,
    CounterContractDenial, CounterContractKind, CounterExpectationDenial, CounterExpectationKind,
    CounterExpectationStrength, CounterMismatchEvidence, CounterStrengthJustification,
    CounterStrengthPosture, HostileCounterEvidenceRow, HostileResourceEnvelopeObservation,
    OverExactCounterDenied, PhysicalCounterContract, PhysicalCounterEvidenceReceipt,
    PhysicalCounterEvidenceRow, PhysicalCounterExecutionSources, PhysicalCounterExpectation,
    PhysicalExecutedCounterEvidence, PhysicalResourceEnvelope, PhysicalResourceEnvelopeObservation,
    RequiredCounterContractSet,
};
pub use coverage::{
    reject_edited_matrix_row, reject_manual_coverage_prose, reject_unchecked_maturity_claim,
    CoverageGapDenial, CoverageRowDimension, CoverageRowSatisfiedReceipt, CoverageSurfaceKind,
    GeneratedCoverageMatrix, HarnessMaturityEvidence, HarnessMaturityLevel, HarnessSubsystem,
    HarnessSubsystemMaturity, MutationResultCoverageRow, MutationValidationPosture,
    PhysicalCoverageMatrixRow, PhysicalMutationCoverageEvidence, RegisteredCounterCoverageRow,
    RegisteredOracleCoverageRow, RegisteredScenarioCoverageRow, RegisteredTranscriptCoverageRow,
    Roadmap2CoverageRegistry, Roadmap2HarnessReadinessReport, Roadmap2HarnessSequence,
    Roadmap2PhysicalCoverageMatrix, S5CompactionMutationCoverageRow, S5CompactionMutationKind,
    S5HarnessMaturityDependencyEvidence, S5PhysicalIsolationMutationKind, S5ReadinessDependencySet,
    S5SimulationHarnessReadiness,
};
pub use drivers::{
    private_mutation_driver_attempt, test_support_verdict_driver_attempt,
    AdmittedDriverContractSet, AdversarialStorageBoundaryDriver, CrashRuntimeIsolationDriver,
    DriverAdmissionDenial, DriverBoundaryKind, DriverCapabilityProfile, DriverEvidenceSurface,
    DriverFaultClass, IoPressureDriver, MemoryPressureDriver, OfflineVerifierDriver,
    PhysicalBoundarySeam, PhysicalBoundaryYieldpoint, PhysicalSimulationDriver,
    ProductionBoundaryDriverTrace, ProductionStorageBoundaryDriver, YieldpointDeclaration,
    YieldpointObservationReceipt, YieldpointPauseReceipt, YieldpointResumeReceipt,
    YieldpointScheduleBinding,
};
pub use evidence::{
    readmit_foundational_physical_evidence_after_boundary,
    reject_foundational_materialization_as_store_authority, reject_loose_log_evidence_attempt,
    reject_same_run_self_comparison_evidence_attempt, reject_terminal_json_evidence_attempt,
    BoundaryBridgedPhysicalCertificationEvidenceBundle, EvidenceBundleAuthority,
    EvidenceBundleReadmissionAuthority, FoundationalPhysicalCertificationEvidenceBundle,
    PhysicalCertificationEvidenceBundle, PhysicalEvidenceBundleDenial,
    PhysicalEvidenceBundlePrimary, PhysicalEvidenceReportRow,
    ReadmittedPhysicalCertificationEvidenceBundle, SimulationFailureDigest,
    TerminalProjectionOnlyEvidenceDenied,
};
pub use faults::{
    s5_stable_read_plan_fault_event, BlockedReclaimEvent, ByteCorruptionEvent, CrashEvent,
    DelayedReleaseEvent, DroppedFlushEvent, ExecutedFaultDeliveryRecipe,
    ExecutionReadyFaultDeliveryRecipe, ExecutionTimeReferenceDiscoveryEvent,
    ExpectedFaultLocalization, FaultDeliveryAttempt, FaultDeliveryBoundaryProof,
    FaultDeliveryDenial, FaultDeliveryPlan, FaultDeliveryReceipt, FaultObservedBoundaryKind,
    IoStallEvent, LoweredFaultDeliveryRecipe, NoFaultControlEvent, NoFaultProductionBoundaryParity,
    ObservedFaultBoundary, PhysicalArtifactFaultLocus, PhysicalArtifactKind, PhysicalFaultEvent,
    PhysicalFaultEventKind, PhysicalFaultFieldKind, PhysicalFaultOffset, ReorderedPersistenceEvent,
    StaleGenerationEvent, TornWriteEvent, UnboundedReadPlanFootprintEvent,
};
pub use fixtures::{
    FixtureAuthorityReceipt, FixtureCapabilityDeclaration, FixtureConstructionAuthority,
    FixtureConstructionBasis, FixtureConstructionProofBasis, FixtureMutationBoundary,
    FixtureMutationBoundarySet, FixtureNeedsBoundary, FixtureNeedsMaterialization,
    FixtureProfileNonClaim, FixtureProvenance, FixtureScaleDeclaration, LargeStoreFixtureProfile,
    PersistedStoreFixtureManifest, PhysicalArtifactFixtureCatalog, PhysicalFixtureBuilder,
    ProductionBackedFixtureMaterialization, ProductionBackedFixtureSource,
    ProductionBackedPhysicalFixture, ResolvedFixtureConstructionRecipe, StoreFixtureAuthority,
    SyntheticFixtureAuthorityDenied,
};
pub use observation::{
    CheckpointCrashReplayObservation, CheckpointInterlockObservation,
    CompactionInterlockObservation, ExecutedPhysicalSimulationObservation,
    IndependentVerifierObservation, IndependentVerifierObservationKind, ObservationDenial,
    ObservedPhysicalTrace, PhysicalObservationBuilder, PhysicalSimulationObserver,
    RecoveryOutcomeKind, RecoveryOutcomeObservation, S5CheckpointPublicationCrashLaneOutput,
    S5CheckpointPublicationLaneBinding, S5CheckpointPublicationRecoveryOutcomeLaneOutput,
    S5CheckpointPublicationScheduledLaneOutput, S5CheckpointPublicationShortcutDenialLaneOutput,
    S5CheckpointPublicationShortcutRejectionOutput, S5CompactionMutationLaneExecution,
    S5CompactionMutationObservationSet, S5CompactionMutationReplayBinding,
    S5CompactionMutationScheduledLaneOutput, ShortcutRejectionObservation,
    ShortcutRejectionObservationKind,
};
pub use oracles::{
    expected_error_text_oracle_attempt, fixture_label_oracle_attempt, log_only_oracle_attempt,
    phase7_verdict_topology, same_run_self_comparison_oracle_attempt,
    test_support_oracle_verdict_attempt, BlockedReclaimUntilReleaseOracle, CounterContractOracle,
    CrashRecoversOldOrNewNeverMixedOracle, IndependentVerifierAgreementOracle,
    NoJsonAuthorityOracle, NoMixedRootOracle, NoPrivateMutationOracle, OldReaderSeesOldRootOracle,
    OracleDenial, OracleVerdictBasis, PhysicalOracleJudgment, PhysicalOracleNonClaim,
    PhysicalOracleVerdictTopology, PhysicalOracleVerdictTopologyPosture, PhysicalProofOracle,
    PhysicalProofOracleKind, PhysicalProofOracleVerdict, PhysicalProofOracleVerdictKind,
    PostSwapReaderSeesNewRootOracle, ReusablePhysicalOracleFamily,
    S5PhysicalIsolationInterleavingOracle, S6IoPressureSimulationOracle, TranscriptReplayOracle,
};
pub use planning::{
    lower_physical_simulation_plan, reject_unresolved_simulation_plan_recipe,
    require_lowered_physical_simulation_plan, FixtureClassKind, ForbiddenShortcutKind,
    ForbiddenShortcutSet, ObserverKind, OracleFamilyKind, PhysicalDriverKind,
    PhysicalSimulationCapability, PhysicalSimulationCapabilitySet, PhysicalSimulationPlan,
    PhysicalSimulationPlanIdentity, PhysicalSimulationProfile, PhysicalSimulationProfileSet,
    RequiredActorSet, RequiredFixtureClassSet, RequiredObserverSet, RequiredOracleFamilySet,
    RequiredPhysicalDriverSet, SimulationEvidencePolicy, SimulationPlanDenial,
    SimulationPlanningContext, SupportedObserverSet, SupportedOracleFamilySet,
    SupportedPhysicalDriverSet,
};
pub use s45_entry::{
    admit_s45_simulation_harness_entry, reject_s45_copied_recovery_report,
    reject_s45_foundational_projection_authority, reject_s45_log_output,
    reject_s45_old_semantic_harness_label, reject_s45_s5_isolation_authority_attempt,
    reject_s45_same_run_self_comparison, reject_s45_terminal_projection,
    S45ExistingHarnessInventory, S45ExistingHarnessSurface, S45HarnessBoundaryDenial,
    S45HarnessNonClaim, S45HarnessSurfaceClassification, S45RegisteredHarnessSurface,
    S45RoadmapHarnessRequirement, S45RoadmapHarnessRequirementSet, S45SimulationHarnessEntry,
    S45SimulationHarnessEntryIdentity,
};
pub use s5_1_security_scope_harness::{
    S51SecurityScopeFailureKind, S51SecurityScopeHarnessCounterSnapshot,
    S51SecurityScopeHarnessEvidence, S51SecurityScopeHarnessObservation,
    S51SecurityScopeHarnessOutcomeKind, S51SecurityScopeHarnessReplayCounterSnapshot,
    S51SecurityScopeHarnessReplayTranscript, S51SecurityScopeHarnessScenario,
    S51SecurityScopeHarnessSchedule, S51SecurityScopeOracleVerdict,
    S51SecurityScopePhysicalReplayDenial, S51SecurityScopePhysicalReplayEvidence,
    S51SecurityScopePhysicalScheduleBinding, S51SecurityScopeReplayMutationKind,
};
pub use s5_executed_isolation_contract::{
    S5EvidenceProfileCounterSet, S5ExecutedIsolationFinding, S5ExecutedIsolationOutcome,
    S5ExecutedIsolationRequiredCounters, S5ExecutedIsolationSourceBasis,
};
pub use s5_executed_isolation_source::{
    S5ExecutedIsolationEvidenceSource, S5ExecutedIsolationSourceDenial,
};
pub use s5_handoff::{
    accept_store_owned_s5_harness_readiness, register_s5_physical_isolation_certification_lane,
    reject_copied_s45_readiness_rows_as_s5_lane_registration,
    reject_foundational_or_proof_projection_as_s5_harness_readiness,
    reject_future_slot_as_s5_harness_readiness, reject_generic_runner_as_s5_harness_readiness,
    reject_generic_runner_as_s5_lane_registration,
    reject_harness_projection_as_s5_lane_registration, require_store_owned_s5_harness_receipt,
    AcceptedS5SimulationHarnessReadiness, S5CounterContractReadiness,
    S5HarnessFutureExtensionReservation, S5HarnessFutureExtensionSlot, S5HarnessReadinessReceipt,
    S5InterleavingHarnessCapability, S5MaintenanceActorCapability,
    S5PhysicalIsolationCertificationLaneRegistration, S5PhysicalIsolationLaneRegistrationDenial,
    S5ProductionDriverCapability, S5RequiredYieldpoint, S5ReusableOracleReadiness,
};
pub use s5_physical_isolation_mutation::{
    s5_physical_isolation_required_mutation_rows, S5MutationReplayBasis,
    S5PhysicalIsolationMutationEvidence,
};
pub use s6_backend_qualification::{
    evaluate_row_rebind, reject_copied_backend_qualification_row,
    reject_environment_name_backend_qualification, reject_log_output_backend_qualification,
    reject_test_only_backend_label_qualification, require_profile_local_row,
    BackendQualificationMatrix, BackendQualificationMatrixDenial,
    BackendQualificationParityComparison, BackendQualificationRow, BackendQualificationRowIdentity,
    CertifiedBackendQualificationSupport, PublishedQualificationPosture,
    QualificationCapabilityProofAuthority, QualificationHarnessProof,
    QualificationHarnessProofClaim, QualificationHarnessProofStrength,
    QualificationMatrixPublisher, QualificationPublicationShortcut, QualificationRebindEvaluation,
    QualificationResidualDebt, QualificationResidualDebtReason,
};
pub use s6_io_pressure_coverage::S6ExecutedIoPressureCoverageRows;
pub use s6_io_pressure_execution::S6IoPressureExecutionCounters;
pub use s6_io_pressure_harness::{
    PhysicalFaultEvidenceClass, S6BackendSafetyQualificationDenial, S6HarnessSecureIoPosture,
    S6IoPressureFaultKind, S6IoPressureHarnessEvidence, S6IoPressureHarnessScenario,
    S6IoPressureOracleObservation, S6PressureEvidenceMaturity, S6RealBackendSafetyQualification,
};
pub use s6_io_pressure_replay::S6IoPressureHarnessEvidenceDenial;
#[cfg(feature = "certification-test-support")]
pub use s6_io_pressure_test_support::replay_bundle_for as s6_io_pressure_test_replay_bundle_for;
pub use s6_io_pressure_vocab::{all_s6_fault_evidence_classes, all_s6_io_pressure_fault_kinds};
pub use scenario::{
    reject_raw_json_scenario_authority_attempt, CertifiedPhysicalScenario,
    JsonScenarioAuthorityDenied, PhysicalScenarioActor, PhysicalScenarioActorRole,
    PhysicalScenarioActorSet, PhysicalScenarioAuthorityWitness, PhysicalScenarioCanonicalIdentity,
    PhysicalScenarioDefinitionDenial, PhysicalScenarioExpectation, PhysicalScenarioExpectationKind,
    PhysicalScenarioFault, PhysicalScenarioFaultKind, PhysicalScenarioFixtureSet,
    PhysicalScenarioIntent, PhysicalScenarioNonClaim, PhysicalScenarioSchedule,
    PhysicalSimulationScenarioDefinition, PhysicalSimulationScenarioFamily,
    TerminalProjectionScenarioDenied,
};
pub use schedule::{
    AdmittedScheduleOrderingAuthority, CounterMismatchSummary, OracleVerdictKind,
    OracleVerdictSummary, PartialOrderReductionPosture, PhysicalActorId, PhysicalActorStep,
    PhysicalActorStepSequence, PhysicalFaultLocus, PhysicalInterleavingSchedule, ReplaySeed,
    ScheduleExplorationCost, ScheduleFailureClass, ScheduleOrderingAuthorityAttempt,
    ScheduleOrderingAuthorityKind, ScheduleReplayDenial, ScheduleReplayIdentity,
    ScheduleShrinkTrace, StateSpaceBudget,
};
pub use shortcut_rejection::{
    shortcut_denial_from_evidence_bundle_denial, shortcut_denial_from_fault_delivery_denial,
    shortcut_denial_from_harness_boundary_denial, shortcut_denial_from_oracle_denial,
    shortcut_denial_from_plan_denial, shortcut_denial_from_scenario_denial,
    shortcut_denial_from_terminal_projection_denial, shortcut_denial_from_transcript_denial,
    ShortcutRejectionBoundary, SyntheticHarnessShortcutDenialReceipt,
    SyntheticHarnessShortcutRejectionDenial, SyntheticHarnessShortcutRejectionReport,
};
pub use transcript::{
    reject_copied_transcript_fields, reject_loose_log_transcript_attempt,
    reject_same_run_self_comparison_transcript_attempt, reject_terminal_json_transcript_attempt,
    DetachedSimulationReplayParts, ExecutedTranscriptParts, PhysicalSimulationTranscript,
    PhysicalSimulationTranscriptIdentity, PhysicalStoryTranscript, SimulationReplayBundle,
    SimulationRunIdentity, TranscriptReplayDenial, TranscriptReplayEvidenceIdentity,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalCertificationLane {
    PowerLoss,
    TornWrite,
    ByteFlip,
    BoundedMemory,
    RecoveryTime,
    ForegroundLatency,
    BlobScale,
    PhysicalIsolation,
}
