use crate::{
    admit_physical_counter_evidence, BlobByteEqualityOracle, BlobChunkOrderingOracle,
    BlobConstantMemoryOracle, BlobDigestChecksumDistinctionOracle, BlobHarnessLoweredSeedPlan,
    BlobHarnessOracleObservation, BlobHarnessScenarioSeed, BlobHeavyCleanupOracle,
    BlobHeavyPatternLaneOracle, BlobHeavyQualificationEvidenceOracle, BlobNoCrossScopeDedupeOracle,
    BlobNoSidecarPathOracle, BlobReachabilityOracle, DetachedSimulationReplayParts,
    ExecutedTranscriptParts, FaultDeliveryAttempt, FixtureCapabilityDeclaration,
    FixtureMutationBoundary, GeneratedCoverageMatrix, HarnessCoverageStage,
    LargeStoreFixtureProfile, NoJsonAuthorityOracle, NoPrivateMutationOracle,
    PhysicalCounterExecutionSources, PhysicalCoverageRegistry, PhysicalExecutedCounterEvidence,
    PhysicalFixtureBuilder, PhysicalInterleavingSchedule, PhysicalMutationCoverageEvidence,
    PhysicalSimulationDriver, PhysicalSimulationObserver, ProductionBackedFixtureMaterialization,
    ReusablePhysicalOracleFamily, SchedulePerturbationSeed, ShortcutRejectionObservation,
    SimulationReplayBundle, StateSpaceBudget,
};
use worth_store_blob_chunks::certification_test_authority::{
    execute_blob_harness, materialize_blob_executed_lifecycle_evidence, BlobHarnessExecutionInput,
    ExecutedBlobLifecycleEvidenceBundle,
};

use super::lower_blob_simulation_seed_plan;
use worth_store_blob_chunks::certification_test_authority::{
    BlobHarnessExecutedWitness as BlobHarnessExecutedActorEvidence, BlobHarnessObservedYieldpoint,
};

pub fn replay_bundle_for_seed(seed: BlobHarnessScenarioSeed) -> SimulationReplayBundle {
    blob_harness_replay_artifacts_for_certification(seed).replay
}

pub fn coverage_matrix_for_seed(seed: BlobHarnessScenarioSeed) -> GeneratedCoverageMatrix {
    let lowered = lower_blob_simulation_seed_plan(seed).unwrap();
    let schedule = schedule_for_plan(&lowered);
    let mutation = PhysicalMutationCoverageEvidence::from_private_mutation_denial_plan(
        HarnessCoverageStage::SimulationAdmission,
        lowered.plan(),
        FaultDeliveryAttempt::private_mutation(),
    )
    .unwrap();

    PhysicalCoverageRegistry::for_sequence(HarnessCoverageStage::SimulationAdmission)
        .register_scenario(lowered.scenario())
        .unwrap()
        .register_plan(lowered.plan())
        .unwrap()
        .register_schedule(&schedule)
        .unwrap()
        .register_actor_set()
        .unwrap()
        .register_driver_contracts(lowered.plan().driver_contracts())
        .unwrap()
        .register_required_oracle_families_from_plan()
        .unwrap()
        .register_counter_contracts_from_plan()
        .unwrap()
        .register_transcript_surface_from_plan()
        .unwrap()
        .register_mutation_result(&mutation)
        .unwrap()
        .generate_matrix()
        .unwrap()
}

fn schedule_for_plan(lowered: &BlobHarnessLoweredSeedPlan) -> PhysicalInterleavingSchedule {
    PhysicalInterleavingSchedule::from_lowered_plan(
        lowered.plan(),
        SchedulePerturbationSeed::from_u64(22),
        StateSpaceBudget::bounded_steps(12).unwrap(),
    )
    .unwrap()
}

fn observed_trace(
    lowered: &BlobHarnessLoweredSeedPlan,
    witness: &BlobHarnessExecutedActorEvidence,
    blob_observation: BlobHarnessOracleObservation,
) -> crate::ObservedPhysicalTrace {
    let mut builder = PhysicalSimulationObserver::independent_physical_trace()
        .observe_plan(lowered.plan())
        .unwrap()
        .with_runtime_trace(production_trace(lowered, witness))
        .with_blob_harness_observation(blob_observation);
    for denial in blob_replay_shortcut_rejections() {
        builder = builder.with_shortcut_rejection_observation(denial);
    }
    builder.complete().unwrap()
}

fn counter_receipt(
    lowered: &BlobHarnessLoweredSeedPlan,
    schedule: &PhysicalInterleavingSchedule,
    trace: &crate::ObservedPhysicalTrace,
    witness: &BlobHarnessExecutedActorEvidence,
) -> crate::PhysicalCounterEvidenceReceipt {
    let sources = PhysicalCounterExecutionSources::admit_for_blob_harness_execution(
        lowered.plan(),
        schedule,
        trace,
        witness,
        crate::observe_real_store_residency(
            "physical-certification-blob",
            crate::CertificationResidencyWorkload::Blob,
            witness.allocation_bytes(),
        ),
        io_queue_evidence(lowered.plan()),
    )
    .unwrap();
    let evidence =
        PhysicalExecutedCounterEvidence::from_execution_sources(lowered.plan(), sources).unwrap();
    admit_physical_counter_evidence(lowered.plan(), evidence).unwrap()
}

fn io_queue_evidence(
    plan: &crate::PhysicalSimulationPlan,
) -> worth_store_io_scheduler::IoQueueExecutedEvidenceSource {
    let recorder = worth_store_io_scheduler::IoQueueExecutionRecorder::from_envelope(
        plan.resource_envelope().io_queue(),
    );
    recorder.executed_evidence().unwrap()
}

fn blob_fixture() -> crate::ProductionBackedPhysicalFixture {
    PhysicalFixtureBuilder::production_backed("s7-blob-harness")
        .materialize_with(
            ProductionBackedFixtureMaterialization::build_profile(
                LargeStoreFixtureProfile::BlobLargerThanMemoryReadiness,
                22,
            )
            .unwrap(),
        )
        .capability(FixtureCapabilityDeclaration::for_mutation_boundary(
            FixtureMutationBoundary::Chunk,
        ))
        .and_reopen_through_physical_authority()
        .unwrap()
}

fn production_trace(
    lowered: &BlobHarnessLoweredSeedPlan,
    witness: &BlobHarnessExecutedActorEvidence,
) -> crate::ProductionBoundaryDriverTrace {
    let trace = lowered
        .plan()
        .driver_contracts()
        .iter()
        .find_map(PhysicalSimulationDriver::production_boundary_trace)
        .unwrap();
    if uses_production_boundary_yieldpoint(witness.observed_yieldpoint()) {
        debug_assert!(trace
            .yieldpoints()
            .iter()
            .any(|yieldpoint| yieldpoint.name() == yielded_name(witness.observed_yieldpoint())));
    }
    trace
}

#[derive(Debug, Clone)]
pub struct ExecutedBlobHarnessReplayArtifacts {
    pub replay: SimulationReplayBundle,
    pub lifecycle_evidence: ExecutedBlobLifecycleEvidenceBundle,
}

pub fn blob_harness_replay_artifacts_for_certification(
    seed: BlobHarnessScenarioSeed,
) -> ExecutedBlobHarnessReplayArtifacts {
    let lowered = lower_blob_simulation_seed_plan(seed).unwrap();
    let input = execution_input(lowered.plan(), lowered.materialized_profile());
    let witness = execute_blob_harness(input.clone());
    let schedule = schedule_for_plan(&lowered);
    let blob_observation =
        BlobHarnessOracleObservation::from_executed_witness(lowered.plan(), &witness).unwrap();
    let trace = observed_trace(&lowered, &witness, blob_observation);
    let counter_receipt = counter_receipt(&lowered, &schedule, &trace, &witness);
    let fixture = blob_fixture();

    let mut parts =
        ExecutedTranscriptParts::new(lowered.plan(), schedule, &fixture, trace, counter_receipt)
            .unwrap();
    for verdict in blob_oracle_verdicts(&parts) {
        parts = parts.with_oracle_verdict(verdict);
    }
    parts = parts.with_transcript_replay_verdict().unwrap();

    let transcript = crate::PhysicalSimulationTranscript::from_executed_parts(parts).unwrap();
    let replay = DetachedSimulationReplayParts::from_transcript(&transcript)
        .admit_replay_bundle()
        .unwrap();
    let lifecycle_evidence = materialize_blob_executed_lifecycle_evidence(witness);
    ExecutedBlobHarnessReplayArtifacts {
        replay,
        lifecycle_evidence,
    }
}

fn execution_input(
    plan: &crate::PhysicalSimulationPlan,
    materialized_profile: &crate::BlobHarnessMaterializedProfile,
) -> BlobHarnessExecutionInput {
    let metadata = plan.blob_harness_metadata().unwrap();
    let topology = plan.blob_harness_topology().unwrap();
    BlobHarnessExecutionInput::new(
        worth_store_blob_chunks::certification_test_authority::BlobHarnessStorageShape::new(
            materialized_profile.blob_profile().envelope().profile(),
            metadata.size_class(),
            metadata.placement_class(),
            metadata.security_scope_class(),
        ),
        worth_store_blob_chunks::certification_test_authority::BlobHarnessExerciseShape::new(
            metadata.access_mode(),
            metadata.failure_point(),
            metadata.actor_mix(),
            topology,
        ),
    )
}

const fn yielded_name(yieldpoint: BlobHarnessObservedYieldpoint) -> &'static str {
    match yieldpoint {
        BlobHarnessObservedYieldpoint::WalAppendBeforeFlush => "wal-append-before-flush",
        BlobHarnessObservedYieldpoint::FreshRuntimeReplayOpen => "fresh-runtime-replay-open",
        BlobHarnessObservedYieldpoint::RootPublicationBeforeObserve => {
            "root-publication-before-observe"
        }
        BlobHarnessObservedYieldpoint::MemoryPressureBoundary => "memory-pressure-boundary",
        BlobHarnessObservedYieldpoint::IoPressureBoundary => "io-pressure-boundary",
        BlobHarnessObservedYieldpoint::OfflineVerifierLayoutWalkBeforeRuntimeRecovery => {
            "offline-verifier-layout-walk-before-runtime-recovery"
        }
        BlobHarnessObservedYieldpoint::ShortcutRejectionBoundary => "shortcut-rejection-boundary",
    }
}

const fn uses_production_boundary_yieldpoint(yieldpoint: BlobHarnessObservedYieldpoint) -> bool {
    matches!(
        yieldpoint,
        BlobHarnessObservedYieldpoint::WalAppendBeforeFlush
            | BlobHarnessObservedYieldpoint::RootPublicationBeforeObserve
    )
}

fn blob_replay_shortcut_rejections() -> [ShortcutRejectionObservation; 6] {
    [
        ShortcutRejectionObservation::whole_object_helper_denied(),
        ShortcutRejectionObservation::missing_chunk_counters_denied(),
        ShortcutRejectionObservation::log_only_evidence_denied(),
        ShortcutRejectionObservation::synthetic_success_row_denied(),
        ShortcutRejectionObservation::private_mutation_denied(),
        ShortcutRejectionObservation::json_authority_denied(),
    ]
}

fn blob_oracle_verdicts(parts: &ExecutedTranscriptParts) -> Vec<crate::PhysicalProofOracleVerdict> {
    vec![
        ReusablePhysicalOracleFamily::blob_harness_evidence()
            .oracle(BlobByteEqualityOracle)
            .judge(parts.plan(), parts.trace())
            .unwrap(),
        ReusablePhysicalOracleFamily::blob_harness_evidence()
            .oracle(BlobChunkOrderingOracle)
            .judge(parts.plan(), parts.trace())
            .unwrap(),
        ReusablePhysicalOracleFamily::blob_harness_evidence()
            .oracle(BlobDigestChecksumDistinctionOracle)
            .judge(parts.plan(), parts.trace())
            .unwrap(),
        ReusablePhysicalOracleFamily::forbidden_shortcut_rejection()
            .oracle(NoPrivateMutationOracle)
            .judge(parts.plan(), parts.trace())
            .unwrap(),
        ReusablePhysicalOracleFamily::forbidden_shortcut_rejection()
            .oracle(NoJsonAuthorityOracle)
            .judge(parts.plan(), parts.trace())
            .unwrap(),
        ReusablePhysicalOracleFamily::blob_harness_evidence()
            .oracle(BlobNoSidecarPathOracle)
            .judge(parts.plan(), parts.trace())
            .unwrap(),
        ReusablePhysicalOracleFamily::blob_harness_evidence()
            .oracle(BlobNoCrossScopeDedupeOracle)
            .judge(parts.plan(), parts.trace())
            .unwrap(),
        ReusablePhysicalOracleFamily::blob_harness_evidence()
            .oracle(BlobReachabilityOracle)
            .judge(parts.plan(), parts.trace())
            .unwrap(),
        ReusablePhysicalOracleFamily::blob_harness_evidence()
            .oracle(BlobConstantMemoryOracle)
            .judge(parts.plan(), parts.trace())
            .unwrap(),
        ReusablePhysicalOracleFamily::blob_heavy_qualification()
            .oracle(BlobHeavyQualificationEvidenceOracle)
            .judge(parts.plan(), parts.trace())
            .unwrap(),
        ReusablePhysicalOracleFamily::blob_heavy_qualification()
            .oracle(BlobHeavyCleanupOracle)
            .judge(parts.plan(), parts.trace())
            .unwrap(),
        ReusablePhysicalOracleFamily::blob_heavy_qualification()
            .oracle(BlobHeavyPatternLaneOracle)
            .judge(parts.plan(), parts.trace())
            .unwrap(),
    ]
}
