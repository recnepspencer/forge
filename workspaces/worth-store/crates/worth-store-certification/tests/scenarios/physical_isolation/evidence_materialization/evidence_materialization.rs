#[path = "../../../support/physical_isolation/interleaving_harness_support/interleaving_harness_support.rs"]
mod support;

use worth_store_authority::{require_current_store_authority, StoreCurrentAuthorityWitness};
use worth_store_certification::{
    materialize_physical_isolation_executed_isolation_evidence, physical_isolation_lanes,
    ExecutedPhysicalIsolationEvidenceSource, ExecutedPhysicalIsolationOutcome,
    ExecutedPhysicalIsolationRequiredCounters, ExecutedPhysicalIsolationSourceDenial,
    PhysicalIsolationMutationEvidence, S5ExecutedIsolationEvidenceBundle,
};
use worth_store_physical_certification::{
    CounterContractKind, PhysicalSimulationScenarioFamily, SimulationReplayBundle,
};
use worth_store_physical_isolation::{
    PhysicalIsolationEvidenceProfile, ProjectionArtifactKind, StorePhysicalAuthoritySurface,
};

#[test]
fn executed_physical_isolation_replay_materializes_foundational_and_proof_projections() {
    let lane = physical_isolation_lanes()
        .into_iter()
        .find(|lane| {
            lane.scenario().definition().family()
                == PhysicalSimulationScenarioFamily::PhysicalIsolationCompactionInterlock
        })
        .expect("compaction S5 lane exists");
    let plan = support::lower_lane(&lane);
    let replay = support::replay_bundle(&plan, lane.expected_fault());
    let mutation = PhysicalIsolationMutationEvidence::from_replay(plan.scenario_family(), &replay);
    assert_mutation_evidence_is_bound_to_replay(&mutation, &replay);
    let replay_counter_rows = replay.counter_receipt().rows().len() as u64;
    let expected_epoch_retries = counter_count(&replay, CounterContractKind::EpochRetries);
    let source = ExecutedPhysicalIsolationEvidenceSource::from_executed_replay(
        physical_isolation_source_authority(),
        replay,
        mutation,
        PhysicalIsolationEvidenceProfile::minimal_required(),
    )
    .expect("executed S5 replay admits materialization source");

    let bundle = materialize_physical_isolation_executed_isolation_evidence(source)
        .expect("executed S5 source materializes projections");

    assert_eq!(
        bundle.source_finding().outcome(),
        ExecutedPhysicalIsolationOutcome::DeniedMutation
    );
    assert_eq!(bundle.source_finding().counters().outcome_count(), 1);
    assert_ne!(
        bundle.source_finding().counters().outcome_count(),
        replay_counter_rows
    );
    assert_eq!(
        bundle.source_finding().counters().retry_count(),
        expected_epoch_retries
    );
    assert_required_counter_values(&bundle);
    assert_eq!(bundle.diagnostics().report().rows().len(), 1);
    assert_eq!(bundle.canonical().digest().metadata().entry_count(), 9);
    assert!(bundle.proof().is_checked_from_executed_store_isolation());
}

#[test]
fn forensic_profile_keeps_required_counters_while_adding_rich_rows() {
    let lane = physical_isolation_lanes()
        .into_iter()
        .find(|lane| {
            lane.scenario().definition().family()
                == PhysicalSimulationScenarioFamily::PhysicalIsolationFutureChunkStability
        })
        .expect("future chunk S5 lane exists");
    let plan = support::lower_lane(&lane);
    let replay = support::replay_bundle(&plan, lane.expected_fault());
    let mutation = PhysicalIsolationMutationEvidence::from_replay(plan.scenario_family(), &replay);
    assert_mutation_evidence_is_bound_to_replay(&mutation, &replay);
    let source = ExecutedPhysicalIsolationEvidenceSource::from_executed_replay(
        physical_isolation_source_authority(),
        replay,
        mutation,
        PhysicalIsolationEvidenceProfile::forensic(),
    )
    .expect("future chunk replay admits materialization source");

    let bundle = materialize_physical_isolation_executed_isolation_evidence(source)
        .expect("forensic profile materializes projections");

    assert_eq!(
        bundle.source_finding().outcome(),
        ExecutedPhysicalIsolationOutcome::NonClaimStabilityOnly
    );
    assert_eq!(bundle.diagnostics().report().rows().len(), 3);
    assert_required_counter_values(&bundle);
}

#[test]
fn foundational_and_proof_projections_cannot_mint_store_authority() {
    let bundle = executed_compaction_bundle();

    for projection in all_projection_artifact_kinds() {
        for surface in all_store_authority_surfaces() {
            let denial = bundle
                .reject_projection_as_store_authority(projection, surface)
                .expect_err("projection rejection through Store authority boundary must deny");
            assert_eq!(denial.projection(), projection);
            assert_eq!(denial.requested_surface(), surface);
        }
    }
}

#[test]
fn mutation_evidence_must_belong_to_the_replay_being_materialized() {
    let compaction =
        lane_for(PhysicalSimulationScenarioFamily::PhysicalIsolationCompactionInterlock);
    let future_chunk =
        lane_for(PhysicalSimulationScenarioFamily::PhysicalIsolationFutureChunkStability);
    let compaction_plan = support::lower_lane(&compaction);
    let future_plan = support::lower_lane(&future_chunk);
    let compaction_replay = support::replay_bundle(&compaction_plan, compaction.expected_fault());
    let future_replay = support::replay_bundle(&future_plan, future_chunk.expected_fault());
    let future_mutation = PhysicalIsolationMutationEvidence::from_replay(
        future_plan.scenario_family(),
        &future_replay,
    );

    let denial = ExecutedPhysicalIsolationEvidenceSource::from_executed_replay(
        physical_isolation_source_authority(),
        compaction_replay,
        future_mutation,
        PhysicalIsolationEvidenceProfile::minimal_required(),
    )
    .expect_err("mutation evidence from another replay cannot materialize this replay");

    assert_eq!(
        denial,
        ExecutedPhysicalIsolationSourceDenial::MutationReplayBasisMismatch
    );
}

fn executed_compaction_bundle() -> S5ExecutedIsolationEvidenceBundle {
    let lane = lane_for(PhysicalSimulationScenarioFamily::PhysicalIsolationCompactionInterlock);
    let plan = support::lower_lane(&lane);
    let replay = support::replay_bundle(&plan, lane.expected_fault());
    let mutation = PhysicalIsolationMutationEvidence::from_replay(plan.scenario_family(), &replay);
    let source = ExecutedPhysicalIsolationEvidenceSource::from_executed_replay(
        physical_isolation_source_authority(),
        replay,
        mutation,
        PhysicalIsolationEvidenceProfile::minimal_required(),
    )
    .expect("executed S5 replay admits materialization source");
    materialize_physical_isolation_executed_isolation_evidence(source)
        .expect("executed S5 source materializes projections")
}

fn assert_required_counter_values(bundle: &S5ExecutedIsolationEvidenceBundle) {
    let counters = bundle.source_finding().counters();
    let diagnostics = bundle.diagnostics().required_counter_fields();
    assert_eq!(diagnostics.outcome_count(), counters.outcome_count());
    assert_eq!(diagnostics.retry_count(), counters.retry_count());
    assert_eq!(diagnostics.latch_count(), counters.latch_count());
    assert_eq!(diagnostics.reclaim_count(), counters.reclaim_count());
    assert!(diagnostics.matches_required_counters(counters));
    assert!(bundle.performance().matches_required_counters(counters));
    assert_performance_counter_rows(bundle, counters);
    assert_eq!(bundle.canonical().basis(), bundle.source_finding().basis());
    assert_eq!(bundle.canonical().counters(), counters);
    let stable_read_projection = bundle
        .proof()
        .projection()
        .payload()
        .stable_read_projection();
    assert_eq!(
        stable_read_projection.basis(),
        bundle.source_finding().basis()
    );
    assert_eq!(stable_read_projection.counters(), counters);
}

fn assert_performance_counter_rows(
    bundle: &S5ExecutedIsolationEvidenceBundle,
    counters: ExecutedPhysicalIsolationRequiredCounters,
) {
    let rows = bundle
        .performance()
        .required_counter_receipt()
        .counter_rows();
    assert_eq!(
        counter_row_value(rows, "store.s5.isolation.outcome"),
        counters.outcome_count()
    );
    assert_eq!(
        counter_row_value(rows, "store.s5.isolation.retry"),
        counters.retry_count()
    );
    assert_eq!(
        counter_row_value(rows, "store.s5.isolation.latch"),
        counters.latch_count()
    );
    assert_eq!(
        counter_row_value(rows, "store.s5.isolation.reclaim"),
        counters.reclaim_count()
    );
}

fn counter_row_value(
    rows: &[worth_foundational::FoundationalPerformanceCounterRow],
    name: &str,
) -> u64 {
    rows.iter()
        .find(|row| row.name().as_str() == name)
        .map(|row| row.observed_count())
        .expect("required S5 performance counter row exists")
}

fn all_projection_artifact_kinds() -> [ProjectionArtifactKind; 11] {
    [
        ProjectionArtifactKind::FoundationalAuthoritativeCurrent,
        ProjectionArtifactKind::FoundationalDerivedProjection,
        ProjectionArtifactKind::FoundationalSupportOnly,
        ProjectionArtifactKind::FoundationalPlannedWork,
        ProjectionArtifactKind::FoundationalReceiptEvidence,
        ProjectionArtifactKind::FoundationalDiagnostic,
        ProjectionArtifactKind::FoundationalPerformanceReceipt,
        ProjectionArtifactKind::FoundationalCanonicalBasis,
        ProjectionArtifactKind::ProofProgressionTrace,
        ProjectionArtifactKind::LogOrJsonProjection,
        ProjectionArtifactKind::PlannedOrSupportArtifact,
    ]
}

fn all_store_authority_surfaces() -> [StorePhysicalAuthoritySurface; 4] {
    [
        StorePhysicalAuthoritySurface::StablePhysicalReadPlan,
        StorePhysicalAuthoritySurface::LatchOrderProof,
        StorePhysicalAuthoritySurface::PhysicalEpochBasis,
        StorePhysicalAuthoritySurface::ReclaimEligibilityProof,
    ]
}

fn assert_mutation_evidence_is_bound_to_replay(
    mutation: &PhysicalIsolationMutationEvidence,
    replay: &SimulationReplayBundle,
) {
    assert_eq!(
        mutation.plan_identity(),
        replay.plan().identity().digest_bytes()
    );
    assert_eq!(
        mutation.schedule_identity(),
        replay.schedule().identity().digest_bytes()
    );
    assert_eq!(
        mutation.transcript_identity(),
        replay.transcript_identity().digest_bytes()
    );
    assert_eq!(
        mutation.replay_basis_identity(),
        replay.replay_basis_identity().digest_bytes()
    );
}

fn counter_count(replay: &SimulationReplayBundle, kind: CounterContractKind) -> u64 {
    replay
        .counter_receipt()
        .rows()
        .iter()
        .find(|row| row.kind() == kind)
        .map(|row| row.observed_count())
        .unwrap_or(0)
}

fn lane_for(
    family: PhysicalSimulationScenarioFamily,
) -> worth_store_certification::PhysicalIsolationHarnessLane {
    physical_isolation_lanes()
        .into_iter()
        .find(|lane| lane.scenario().definition().family() == family)
        .expect("requested S5 lane exists")
}

fn physical_isolation_source_authority() -> StoreCurrentAuthorityWitness {
    require_current_store_authority(worth_store_test_support::physical_isolation_boundary_fact(
        "s5.executed.isolation",
        11,
    ))
}
