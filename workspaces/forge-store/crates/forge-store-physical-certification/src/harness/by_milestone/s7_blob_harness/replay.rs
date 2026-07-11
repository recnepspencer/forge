use forge_store_blob_chunks::certification_test_authority::{
    execute_s7_blob_harness, materialize_s7_executed_lifecycle_evidence, BlobHarnessExecutionInput,
    S7ExecutedLifecycleEvidenceBundle,
};
use forge_store_buffer_pool::{
    streaming_window_allocation_receipt, AllocationAdmission, BufferPoolExecutedEvidenceSource,
};

use crate::{
    admit_physical_counter_evidence, BlobByteEqualityOracle, BlobChunkOrderingOracle,
    BlobConstantMemoryOracle, BlobDigestChecksumDistinctionOracle, BlobHarnessLoweredSeedPlan,
    BlobHarnessScenarioSeed, BlobHeavyCleanupOracle, BlobHeavyPatternLaneOracle,
    BlobHeavyQualificationEvidenceOracle, BlobNoCrossScopeDedupeOracle, BlobNoSidecarPathOracle,
    BlobReachabilityOracle, DetachedSimulationReplayParts, ExecutedTranscriptParts,
    FaultDeliveryAttempt, FixtureCapabilityDeclaration, FixtureMutationBoundary,
    GeneratedCoverageMatrix, LargeStoreFixtureProfile, NoJsonAuthorityOracle,
    NoPrivateMutationOracle, PhysicalCounterExecutionSources, PhysicalExecutedCounterEvidence,
    PhysicalFixtureBuilder, PhysicalInterleavingSchedule, PhysicalMutationCoverageEvidence,
    PhysicalSimulationDriver, PhysicalSimulationObserver, ProductionBackedFixtureMaterialization,
    ReplaySeed, ReusablePhysicalOracleFamily, Roadmap2CoverageRegistry, Roadmap2HarnessSequence,
    S7BlobHarnessOracleObservation, ShortcutRejectionObservation, SimulationReplayBundle,
    StateSpaceBudget,
};

use super::lower_blob_simulation_seed_plan;
use forge_store_blob_chunks::certification_test_authority::{
    BlobHarnessExecutedWitness as S7BlobHarnessExecutedActorEvidence,
    BlobHarnessObservedYieldpoint as S7BlobHarnessObservedYieldpoint,
};

pub fn replay_bundle_for_seed(seed: BlobHarnessScenarioSeed) -> SimulationReplayBundle {
    execute_replay_artifacts_for_seed(seed).replay
}

pub fn coverage_matrix_for_seed(seed: BlobHarnessScenarioSeed) -> GeneratedCoverageMatrix {
    let lowered = lower_blob_simulation_seed_plan(seed).unwrap();
    let schedule = schedule_for_plan(&lowered);
    let mutation = PhysicalMutationCoverageEvidence::from_private_mutation_denial_plan(
        Roadmap2HarnessSequence::S45,
        lowered.plan(),
        FaultDeliveryAttempt::private_mutation(),
    )
    .unwrap();

    Roadmap2CoverageRegistry::for_sequence(Roadmap2HarnessSequence::S45)
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
        ReplaySeed::from_u64(22),
        StateSpaceBudget::bounded_steps(12).unwrap(),
    )
    .unwrap()
}

fn observed_trace(
    lowered: &BlobHarnessLoweredSeedPlan,
    witness: &S7BlobHarnessExecutedActorEvidence,
    blob_observation: S7BlobHarnessOracleObservation,
) -> crate::ObservedPhysicalTrace {
    let mut builder = PhysicalSimulationObserver::independent_physical_trace()
        .observe_plan(lowered.plan())
        .unwrap()
        .with_runtime_trace(production_trace(lowered, witness))
        .with_s7_blob_harness_observation(blob_observation);
    for denial in phase22_shortcut_rejections() {
        builder = builder.with_shortcut_rejection_observation(denial);
    }
    builder.complete().unwrap()
}

fn counter_receipt(
    lowered: &BlobHarnessLoweredSeedPlan,
    schedule: &PhysicalInterleavingSchedule,
    trace: &crate::ObservedPhysicalTrace,
    witness: &S7BlobHarnessExecutedActorEvidence,
) -> crate::PhysicalCounterEvidenceReceipt {
    let sources = PhysicalCounterExecutionSources::admit_for_blob_harness_execution(
        lowered.plan(),
        schedule,
        trace,
        witness,
        buffer_pool_evidence(lowered.plan(), witness),
        io_queue_evidence(lowered.plan()),
    )
    .unwrap();
    let evidence =
        PhysicalExecutedCounterEvidence::from_execution_sources(lowered.plan(), sources).unwrap();
    admit_physical_counter_evidence(lowered.plan(), evidence).unwrap()
}

fn buffer_pool_evidence(
    plan: &crate::PhysicalSimulationPlan,
    witness: &S7BlobHarnessExecutedActorEvidence,
) -> BufferPoolExecutedEvidenceSource {
    let mut allocation =
        AllocationAdmission::from_declaration(plan.resource_envelope().allocation());
    streaming_window_allocation_receipt(&mut allocation, witness.allocation_bytes()).unwrap();
    BufferPoolExecutedEvidenceSource::from_allocation_execution(&allocation).unwrap()
}

fn io_queue_evidence(
    plan: &crate::PhysicalSimulationPlan,
) -> forge_store_io_scheduler::IoQueueExecutedEvidenceSource {
    let recorder = forge_store_io_scheduler::IoQueueExecutionRecorder::from_envelope(
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
    witness: &S7BlobHarnessExecutedActorEvidence,
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
pub(crate) struct ExecutedBlobHarnessReplayArtifacts {
    pub(crate) replay: SimulationReplayBundle,
    pub(crate) lifecycle_evidence: S7ExecutedLifecycleEvidenceBundle,
}

pub(crate) fn execute_replay_artifacts_for_seed(
    seed: BlobHarnessScenarioSeed,
) -> ExecutedBlobHarnessReplayArtifacts {
    let lowered = lower_blob_simulation_seed_plan(seed).unwrap();
    let input = execution_input(lowered.plan(), lowered.materialized_profile());
    let witness = execute_s7_blob_harness(input.clone());
    let schedule = schedule_for_plan(&lowered);
    let blob_observation =
        S7BlobHarnessOracleObservation::from_executed_witness(lowered.plan(), &witness).unwrap();
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
    let lifecycle_evidence = materialize_s7_executed_lifecycle_evidence(witness);
    ExecutedBlobHarnessReplayArtifacts {
        replay,
        lifecycle_evidence,
    }
}

fn execution_input(
    plan: &crate::PhysicalSimulationPlan,
    materialized_profile: &crate::BlobHarnessMaterializedProfile,
) -> BlobHarnessExecutionInput {
    let metadata = plan.s7_blob_harness_metadata().unwrap();
    let topology = plan.s7_blob_harness_topology().unwrap();
    BlobHarnessExecutionInput::new(
        materialized_profile.blob_profile().envelope().profile(),
        metadata.size_class(),
        metadata.placement_class(),
        metadata.security_scope_class(),
        metadata.access_mode(),
        metadata.failure_point(),
        metadata.actor_mix(),
        topology,
    )
}

const fn yielded_name(yieldpoint: S7BlobHarnessObservedYieldpoint) -> &'static str {
    match yieldpoint {
        S7BlobHarnessObservedYieldpoint::WalAppendBeforeFlush => "wal-append-before-flush",
        S7BlobHarnessObservedYieldpoint::FreshRuntimeReplayOpen => "fresh-runtime-replay-open",
        S7BlobHarnessObservedYieldpoint::RootPublicationBeforeObserve => {
            "root-publication-before-observe"
        }
        S7BlobHarnessObservedYieldpoint::MemoryPressureBoundary => "memory-pressure-boundary",
        S7BlobHarnessObservedYieldpoint::IoPressureBoundary => "io-pressure-boundary",
        S7BlobHarnessObservedYieldpoint::OfflineVerifierLayoutWalkBeforeRuntimeRecovery => {
            "offline-verifier-layout-walk-before-runtime-recovery"
        }
        S7BlobHarnessObservedYieldpoint::ShortcutRejectionBoundary => "shortcut-rejection-boundary",
    }
}

const fn uses_production_boundary_yieldpoint(yieldpoint: S7BlobHarnessObservedYieldpoint) -> bool {
    matches!(
        yieldpoint,
        S7BlobHarnessObservedYieldpoint::WalAppendBeforeFlush
            | S7BlobHarnessObservedYieldpoint::RootPublicationBeforeObserve
    )
}

fn phase22_shortcut_rejections() -> [ShortcutRejectionObservation; 6] {
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
        ReusablePhysicalOracleFamily::s7_blob_harness_evidence()
            .oracle(BlobByteEqualityOracle)
            .judge(parts.plan(), parts.trace())
            .unwrap(),
        ReusablePhysicalOracleFamily::s7_blob_harness_evidence()
            .oracle(BlobChunkOrderingOracle)
            .judge(parts.plan(), parts.trace())
            .unwrap(),
        ReusablePhysicalOracleFamily::s7_blob_harness_evidence()
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
        ReusablePhysicalOracleFamily::s7_blob_harness_evidence()
            .oracle(BlobNoSidecarPathOracle)
            .judge(parts.plan(), parts.trace())
            .unwrap(),
        ReusablePhysicalOracleFamily::s7_blob_harness_evidence()
            .oracle(BlobNoCrossScopeDedupeOracle)
            .judge(parts.plan(), parts.trace())
            .unwrap(),
        ReusablePhysicalOracleFamily::s7_blob_harness_evidence()
            .oracle(BlobReachabilityOracle)
            .judge(parts.plan(), parts.trace())
            .unwrap(),
        ReusablePhysicalOracleFamily::s7_blob_harness_evidence()
            .oracle(BlobConstantMemoryOracle)
            .judge(parts.plan(), parts.trace())
            .unwrap(),
        ReusablePhysicalOracleFamily::s7_blob_heavy_qualification()
            .oracle(BlobHeavyQualificationEvidenceOracle)
            .judge(parts.plan(), parts.trace())
            .unwrap(),
        ReusablePhysicalOracleFamily::s7_blob_heavy_qualification()
            .oracle(BlobHeavyCleanupOracle)
            .judge(parts.plan(), parts.trace())
            .unwrap(),
        ReusablePhysicalOracleFamily::s7_blob_heavy_qualification()
            .oracle(BlobHeavyPatternLaneOracle)
            .judge(parts.plan(), parts.trace())
            .unwrap(),
    ]
}
