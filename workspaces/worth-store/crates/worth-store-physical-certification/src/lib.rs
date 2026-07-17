#![forbid(unsafe_code)]

pub mod harness;

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
mod operational_recovery_audit_driver;
#[cfg(test)]
mod operational_recovery_authorization_fixture;
mod operational_recovery_control_driver;
mod operational_recovery_control_transition;
mod operational_recovery_driver;
#[cfg(test)]
mod operational_recovery_driver_tests;
mod operational_recovery_rejoin_driver;
#[cfg(test)]
mod operational_recovery_replica_driver_fixture;
#[cfg(test)]
mod operational_recovery_replica_driver_tests;
#[cfg(test)]
mod operational_recovery_replica_promotion_driver_tests;
mod operational_recovery_trace;
mod operational_recovery_yieldpoint;
mod oracles;
mod physical_isolation_handoff;
mod planning;
mod pressure_harness;
mod qualification;
mod scenario;
mod scenarios;
mod schedule;
mod security_scope_harness;
mod shortcut_rejection;
mod simulation_admission;
mod transcript;

pub use scenarios::*;
pub use worth_store_offline_verifier::OfflineVerifierBoundarySeam;

pub use actors::{
    BlobDedupeActor, BlobExportActor, BlobImportActor, BlobIngestActor,
    BlobPartialReplicationActor, BlobPlacementMoveActor, BlobReadActor, BlobReclaimActor,
    BlobResumeActor, BlobVerifyActor, CheckpointActor, CompactionActor, ForegroundReadActor,
    ForegroundWriteActor, OfflineVerifierActor, PhysicalSimulationActor,
    PhysicalSimulationActorAdmissionDenial, ReclaimActor, RecoveryActor, ScrubActor,
};
pub use authoring::{
    physical_scenario, PhysicalScenarioBuilder, ScenarioBuilderActorStep,
    ScenarioBuilderExpectationStep, ScenarioBuilderFixtureStep, ScenarioBuilderScheduleStep,
};
pub use closeout::{
    ExecutedSimulationHarnessAcceptanceSuiteEvidence,
    ExecutedSimulationHarnessAcceptanceSuiteEvidenceSet, FutureHarnessExtensionSlotInventory,
    FutureHarnessExtensionSlotReport, FuturePhysicalHarnessExtensionFamily,
    PhysicalIsolationReadinessShapeProbeScenario,
    PhysicalIsolationReadinessShapeProbeSliceEvidence,
    PhysicalSimulationHarnessCertificationBundle, PhysicalSimulationHarnessCloseoutDenial,
    PhysicalSimulationHarnessCloseoutReport, PhysicalSimulationHarnessCloseoutSuite,
    RecoveryDogfoodScenario, RecoveryDogfoodSliceEvidence, ShortcutRejectionDogfoodScenario,
    ShortcutRejectionDogfoodSliceEvidence, SimulationHarnessAcceptanceEvidenceLane,
    SimulationHarnessAcceptanceSuiteCoverage, SimulationHarnessAcceptanceSuiteEvidence,
    SimulationHarnessAcceptanceSuiteEvidenceSource, SimulationHarnessAcceptanceSuiteExecutionProof,
    SimulationHarnessAcceptanceSuiteMap, SimulationHarnessAcceptanceSuiteName,
    SimulationHarnessAcceptanceSuiteReceipt, SimulationHarnessAcceptanceSuiteReceiptSet,
    SimulationHarnessCloseoutCoverageReport, SimulationHarnessDogfoodEvidence,
    SimulationHarnessDogfoodReport, SimulationHarnessDogfoodSliceKind,
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
    reject_copied_physical_isolation_simulation_harness_readiness_fields, reject_edited_matrix_row,
    reject_manual_coverage_prose, reject_missing_physical_isolation_correctness_non_claim,
    reject_unchecked_maturity_claim, CoverageGapDenial, CoverageRowDimension,
    CoverageRowSatisfiedReceipt, CoverageSurfaceKind, GeneratedCoverageMatrix,
    HarnessCoverageStage, HarnessMaturityEvidence, HarnessMaturityLevel, HarnessSubsystem,
    HarnessSubsystemMaturity, MutationResultCoverageRow, MutationValidationPosture,
    PhysicalCoverageMatrix, PhysicalCoverageMatrixRow, PhysicalCoverageRegistry,
    PhysicalHarnessReadinessReport, PhysicalIsolationCompactionMutationCoverageRow,
    PhysicalIsolationCompactionMutationKind, PhysicalIsolationCorrectnessNonClaimEvidence,
    PhysicalIsolationHarnessMaturityDependency, PhysicalIsolationHarnessMaturityDependencyEvidence,
    PhysicalIsolationHarnessReadiness, PhysicalIsolationHarnessReadinessDenial,
    PhysicalIsolationMutationKind, PhysicalIsolationReadinessDependencySet,
    PhysicalMutationCoverageEvidence, RegisteredCounterCoverageRow, RegisteredOracleCoverageRow,
    RegisteredScenarioCoverageRow, RegisteredTranscriptCoverageRow,
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
    physical_isolation_stable_read_plan_fault_event, BlockedReclaimEvent,
    BoundaryObservedFaultDeliveryRecipe, ByteCorruptionEvent, CrashEvent, DelayedReleaseEvent,
    DroppedFlushEvent, ExecutionReadyFaultDeliveryRecipe, ExecutionTimeReferenceDiscoveryEvent,
    ExpectedFaultLocalization, FaultDeliveryAttempt, FaultDeliveryBoundaryProof,
    FaultDeliveryDenial, FaultDeliveryPlan, FaultDeliveryReceipt, FaultObservedBoundaryKind,
    IoStallEvent, LoweredFaultDeliveryRecipe, NoFaultControlEvent, NoFaultProductionBoundaryParity,
    ObservedFaultBoundary, PhysicalArtifactFaultLocus, PhysicalArtifactKind, PhysicalFaultEvent,
    PhysicalFaultEventKind, PhysicalFaultFieldKind, PhysicalFaultOffset,
    PhysicalStorageFaultExecution, PhysicalStorageFaultInjection, ReorderedPersistenceEvent,
    StaleGenerationEvent, TornWriteEvent, UnboundedReadPlanFootprintEvent,
};
pub use fixtures::{
    FixtureActivityScale, FixtureAuthorityReceipt, FixtureCapabilityDeclaration,
    FixtureConstructionAuthority, FixtureConstructionBasis, FixtureConstructionProofBasis,
    FixtureMutationBoundary, FixtureMutationBoundarySet, FixtureNeedsBoundary,
    FixtureNeedsMaterialization, FixtureProfileNonClaim, FixtureProvenance,
    FixtureScaleDeclaration, FixtureStorageScale, LargeStoreFixtureProfile,
    PersistedStoreFixtureManifest, PhysicalArtifactFixtureCatalog, PhysicalFixtureBuilder,
    ProductionBackedFixtureMaterialization, ProductionBackedFixtureSource,
    ProductionBackedPhysicalFixture, ResolvedFixtureConstructionRecipe, StoreFixtureAuthority,
    SyntheticFixtureAuthorityDenied,
};
#[cfg(any(test, feature = "certification-test-support"))]
pub use harness::blob::{
    blob_harness_replay_artifacts_for_certification,
    synthetic_blob_harness_coverage_matrix_for_test_support,
    synthetic_blob_harness_replay_bundle_for_test_support,
};
pub use harness::blob::{
    lower_blob_simulation_seed_plan, BlobHarnessLoweredSeedPlan, BlobHarnessLoweringDenial,
    BlobHarnessMaterializedProfile, BlobHarnessOracleObservation, BlobHarnessProfile,
    BlobHarnessProfileSet, BlobHarnessScenarioSeed, BlobHarnessScenarioSeedBuilder,
    BlobHarnessShortcutAttempt, BlobHarnessShortcutDenial, BlobResumeCrashPoint,
    BlobResumeExpectedOutcome, BlobResumeRecoveryScenario,
};
pub use observation::{
    CheckpointCrashReplayObservation, CheckpointInterlockObservation,
    CheckpointPublicationRecoveryExecution, CompactionInterlockObservation,
    IndependentVerifierObservation, IndependentVerifierObservationKind, ObservationDenial,
    ObservedPhysicalTrace, PhysicalIsolationCheckpointPublicationCrashLaneOutput,
    PhysicalIsolationCheckpointPublicationLaneBinding,
    PhysicalIsolationCheckpointPublicationRecoveryOutcomeLaneOutput,
    PhysicalIsolationCheckpointPublicationScheduledLaneOutput,
    PhysicalIsolationCheckpointPublicationShortcutDenialLaneOutput,
    PhysicalIsolationCheckpointPublicationShortcutRejectionOutput,
    PhysicalIsolationCompactionMutationLaneExecution,
    PhysicalIsolationCompactionMutationObservationSet,
    PhysicalIsolationCompactionMutationReplayBinding,
    PhysicalIsolationCompactionMutationScheduledLaneOutput, PhysicalObservationBuilder,
    PhysicalSimulationBoundaryObservation, PhysicalSimulationObservationBasis,
    PhysicalSimulationObserver, RecoveryOutcomeKind, RecoveryOutcomeObservation,
    ShortcutRejectionObservation, ShortcutRejectionObservationKind,
};
pub use operational_recovery_control_driver::DrivenOperationalControlStore;
pub use operational_recovery_control_transition::OperationalRecoveryControlTransitionKind;
pub use operational_recovery_driver::{
    DrivenOperationalTransition, OperationalRecoveryProductionDriver, OperationalRecoveryYieldpoint,
};
pub use operational_recovery_trace::{
    OperationalRecoveryDriverTrace, OperationalRecoveryTraceJoinDenial,
};
pub use oracles::{
    expected_error_text_oracle_attempt, fixture_label_oracle_attempt, log_only_oracle_attempt,
    oracle_verdict_topology, same_run_self_comparison_oracle_attempt,
    test_support_oracle_verdict_attempt, BlobByteEqualityOracle, BlobChunkOrderingOracle,
    BlobConstantMemoryOracle, BlobDigestChecksumDistinctionOracle, BlobHeavyCleanupOracle,
    BlobHeavyPatternLaneOracle, BlobHeavyQualificationEvidenceOracle, BlobNoCrossScopeDedupeOracle,
    BlobNoSidecarPathOracle, BlobReachabilityOracle, BlockedReclaimUntilReleaseOracle,
    CounterContractOracle, CrashRecoversOldOrNewNeverMixedOracle,
    IndependentVerifierAgreementOracle, IoPressureSimulationOracle, NoJsonAuthorityOracle,
    NoMixedRootOracle, NoPrivateMutationOracle, OldReaderSeesOldRootOracle, OracleDenial,
    OracleVerdictBasis, PhysicalIsolationInterleavingOracle, PhysicalOracleJudgment,
    PhysicalOracleNonClaim, PhysicalOracleVerdictTopology, PhysicalOracleVerdictTopologyPosture,
    PhysicalProofOracle, PhysicalProofOracleKind, PhysicalProofOracleVerdict,
    PhysicalProofOracleVerdictKind, PostSwapReaderSeesNewRootOracle, ReusablePhysicalOracleFamily,
    TranscriptReplayOracle,
};
pub use physical_isolation_handoff::{
    accept_store_owned_physical_isolation_harness_readiness,
    physical_isolation_required_mutation_rows, register_physical_isolation_certification_lane,
    reject_copied_simulation_harness_readiness_rows_as_physical_isolation_lane_registration,
    reject_foundational_or_proof_projection_as_physical_isolation_harness_readiness,
    reject_future_slot_as_physical_isolation_harness_readiness,
    reject_generic_runner_as_physical_isolation_harness_readiness,
    reject_generic_runner_as_physical_isolation_lane_registration,
    reject_harness_projection_as_physical_isolation_lane_registration,
    require_store_owned_physical_isolation_harness_receipt,
    AcceptedPhysicalIsolationHarnessReadiness, ExecutedPhysicalIsolationEvidenceSource,
    ExecutedPhysicalIsolationFinding, ExecutedPhysicalIsolationOutcome,
    ExecutedPhysicalIsolationRequiredCounters, ExecutedPhysicalIsolationSourceBasis,
    ExecutedPhysicalIsolationSourceDenial, PhysicalIsolationCertificationLaneRegistration,
    PhysicalIsolationCounterContractReadiness, PhysicalIsolationEvidenceProfileCounterSet,
    PhysicalIsolationHarnessFutureExtensionReservation,
    PhysicalIsolationHarnessFutureExtensionSlot, PhysicalIsolationHarnessReadinessReceipt,
    PhysicalIsolationInterleavingHarnessCapability, PhysicalIsolationLaneRegistrationDenial,
    PhysicalIsolationMaintenanceActorCapability, PhysicalIsolationMutationEvidence,
    PhysicalIsolationMutationReplayBasis, PhysicalIsolationProductionDriverCapability,
    PhysicalIsolationRequiredYieldpoint, PhysicalIsolationReusableOracleReadiness,
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
#[cfg(feature = "certification-test-support")]
pub use pressure_harness::test_replay_bundle_for as io_pressure_test_replay_bundle_for;
pub use pressure_harness::{
    all_io_pressure_fault_evidence_classes, all_io_pressure_fault_kinds,
    ExecutedIoPressureCoverageRows, IoPressureBackendSafetyQualificationDenial,
    IoPressureEvidenceMaturity, IoPressureExecutionCounters, IoPressureFaultKind,
    IoPressureHarnessEvidence, IoPressureHarnessEvidenceDenial, IoPressureHarnessScenario,
    IoPressureHarnessSecureIoPosture, IoPressureOracleObservation, PhysicalFaultEvidenceClass,
    RealBackendSafetyQualification,
};
pub use qualification::{
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
pub use scenario::{
    reject_raw_json_scenario_authority_attempt, BlobHarnessScenarioMetadata,
    CertifiedPhysicalScenario, FreshRuntimeCrashRecoveryEvidence,
    FreshRuntimeCrashRecoveryEvidenceDenial, JsonScenarioAuthorityDenied, PhysicalScenarioActor,
    PhysicalScenarioActorRole, PhysicalScenarioActorSet, PhysicalScenarioAuthorityWitness,
    PhysicalScenarioCanonicalIdentity, PhysicalScenarioDefinitionDenial,
    PhysicalScenarioExpectation, PhysicalScenarioExpectationKind, PhysicalScenarioFault,
    PhysicalScenarioFaultKind, PhysicalScenarioFixtureSet, PhysicalScenarioIntent,
    PhysicalScenarioNonClaim, PhysicalScenarioSchedule, PhysicalSimulationScenarioDefinition,
    PhysicalSimulationScenarioFamily, RecoveryCrashSeam, TerminalProjectionScenarioDenied,
};
pub use schedule::{
    execute_physical_schedule, explore_physical_interleavings, AdmittedScheduleOrderingAuthority,
    CounterMismatchSummary, OracleVerdictKind, OracleVerdictSummary, PartialOrderReductionPosture,
    PhysicalActorId, PhysicalActorStep, PhysicalActorStepSequence, PhysicalActorStorageExecution,
    PhysicalFaultLocus, PhysicalInterleavingSchedule, PhysicalScheduleExecution,
    PhysicalScheduleExecutionError, PhysicalScheduleExploration,
    PhysicalScheduleOwnerEvidenceDenial, PhysicalScheduleOwnerExecution, PhysicalScheduleOwnerKind,
    ReplaySeed, ScheduleExplorationCompletion, ScheduleExplorationCost, ScheduleFailureClass,
    ScheduleFailureSignature, ScheduleOrderingAuthorityAttempt, ScheduleOrderingAuthorityKind,
    ScheduleReplayDenial, ScheduleReplayIdentity, ScheduleShrinkTrace, StateSpaceBudget,
};
pub use security_scope_harness::{
    SecurityScopeFailureKind, SecurityScopeHarnessCounterSnapshot, SecurityScopeHarnessEvidence,
    SecurityScopeHarnessObservation, SecurityScopeHarnessOutcomeKind,
    SecurityScopeHarnessReplayCounterSnapshot, SecurityScopeHarnessReplayTranscript,
    SecurityScopeHarnessScenario, SecurityScopeHarnessSchedule, SecurityScopeOracleVerdict,
    SecurityScopePhysicalReplayDenial, SecurityScopePhysicalReplayEvidence,
    SecurityScopePhysicalScheduleBinding, SecurityScopeReplayMutationKind,
};
pub use shortcut_rejection::{
    shortcut_denial_from_evidence_bundle_denial, shortcut_denial_from_fault_delivery_denial,
    shortcut_denial_from_harness_boundary_denial, shortcut_denial_from_oracle_denial,
    shortcut_denial_from_plan_denial, shortcut_denial_from_scenario_denial,
    shortcut_denial_from_terminal_projection_denial, shortcut_denial_from_transcript_denial,
    ShortcutRejectionBoundary, SyntheticHarnessShortcutDenialReceipt,
    SyntheticHarnessShortcutRejectionDenial, SyntheticHarnessShortcutRejectionReport,
};
pub use simulation_admission::{
    admit_simulation_harness_entry, reject_simulation_harness_copied_recovery_report,
    reject_simulation_harness_foundational_projection_authority,
    reject_simulation_harness_log_output, reject_simulation_harness_old_semantic_harness_label,
    reject_simulation_harness_physical_isolation_authority_attempt,
    reject_simulation_harness_same_run_self_comparison,
    reject_simulation_harness_terminal_projection, ExistingSimulationHarnessInventory,
    ExistingSimulationHarnessSurface, RegisteredSimulationHarnessSurface,
    SimulationHarnessBoundaryDenial, SimulationHarnessEntry, SimulationHarnessEntryIdentity,
    SimulationHarnessNonClaim, SimulationHarnessRoadmapRequirement,
    SimulationHarnessRoadmapRequirementSet, SimulationHarnessSurfaceClassification,
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
