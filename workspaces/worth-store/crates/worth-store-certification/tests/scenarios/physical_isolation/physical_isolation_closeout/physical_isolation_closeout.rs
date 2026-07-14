#[path = "../../../support/physical_isolation/executed_closeout_fixture/executed_closeout_fixture.rs"]
mod executed_closeout_fixture;
#[path = "../../../support/physical_isolation/interleaving_harness_support/interleaving_harness_support.rs"]
mod harness_support;
use worth_store_test_support::harness::physical_isolation::epoch_scope as support;
use worth_store_test_support::harness::physical_isolation::publication as publication_support;
use worth_store_test_support::harness::physical_isolation::read_plan as plan_admission;
use worth_store_test_support::harness::physical_isolation::reclaim as reclaim_support;
use worth_store_test_support::harness::recovery::source_precedence as source_precedence_fixture;

use worth_store_authority::{require_current_store_authority, StoreCurrentAuthorityWitness};
use worth_store_certification::{
    materialize_physical_isolation_executed_isolation_evidence, physical_isolation_coverage_matrix,
    physical_isolation_lanes, ExecutedPhysicalIsolationEvidenceSource,
    PhysicalIsolationCloseoutLaneEvidence, PhysicalIsolationCloseoutSuite,
    PhysicalIsolationMutationEvidence, S5CloseoutReservedScope,
};
use worth_store_physical_certification::{
    CoverageSurfaceKind, HarnessCoverageStage, PhysicalCertificationEvidenceBundle,
    PhysicalSimulationProfile, PhysicalSimulationScenarioFamily,
};
use worth_store_physical_isolation::{
    publish_scheduler_isolation_capability_from_executed_evidence,
    PhysicalIsolationEvidenceProfile, PhysicalStabilityAssumption, UnsupportedQoSClaim,
};

#[test]
fn closeout_aggregates_all_physical_isolation_hostile_lanes_with_machine_evidence() {
    let suite = closeout_suite();

    assert_eq!(suite.lanes().len(), 6);
    assert!(suite.simulation_harness_readiness().shortcut_denial_count() > 0);
    assert!(suite
        .reservations()
        .contains(S5CloseoutReservedScope::S6IoQosIsolation));
    assert!(suite
        .reservations()
        .contains(S5CloseoutReservedScope::BlobLifecycle));
    assert!(suite
        .reservations()
        .contains(S5CloseoutReservedScope::LayoutIndexes));
    assert!(suite
        .reservations()
        .contains(S5CloseoutReservedScope::S10Repair));
    assert!(suite
        .reservations()
        .contains(S5CloseoutReservedScope::S11Security));
    assert!(suite
        .reservations()
        .contains(S5CloseoutReservedScope::S12Certification));

    for family in required_families() {
        let lane = suite
            .lanes()
            .iter()
            .find(|lane| lane.family() == family)
            .expect("required S5 family is closed");
        assert_eq!(
            lane.coverage().sequence(),
            HarnessCoverageStage::SimulationAdmission
        );
        assert_eq!(
            lane.plan().profile(),
            PhysicalSimulationProfile::CiCertification
        );
        assert_eq!(
            lane.certification().replay().schedule().profile(),
            PhysicalSimulationProfile::CiCertification
        );
        assert_required_surfaces(lane);
        assert_eq!(
            lane.mutation().required_rows(),
            worth_store_certification::physical_isolation_required_mutation_rows(family)
        );
        assert!(lane
            .executed()
            .proof()
            .is_checked_from_executed_store_isolation());
    }
}

#[test]
fn closeout_denies_smoke_profile_lane_evidence() {
    let mut lanes = physical_isolation_lanes();
    let lane = lanes.remove(0);
    let plan = worth_store_physical_certification::lower_physical_simulation_plan(
        lane.scenario().clone(),
        harness_support::developer_smoke_context(),
    )
    .unwrap();
    let replay = harness_support::replay_bundle(&plan, lane.expected_fault());
    let mutation = PhysicalIsolationMutationEvidence::from_replay(plan.scenario_family(), &replay);
    let coverage = physical_isolation_coverage_matrix(lane.scenario(), &plan, &replay, &mutation);
    let certification =
        PhysicalCertificationEvidenceBundle::from_replay_bundle(replay.clone()).unwrap();
    let source = ExecutedPhysicalIsolationEvidenceSource::from_executed_replay(
        physical_isolation_source_authority(),
        replay,
        mutation.clone(),
        PhysicalIsolationEvidenceProfile::minimal_required(),
    )
    .unwrap();
    let executed = materialize_physical_isolation_executed_isolation_evidence(source).unwrap();
    let denial = PhysicalIsolationCloseoutLaneEvidence::from_executed_lane(
        lane.scenario().clone(),
        plan,
        coverage,
        certification,
        mutation,
        executed,
    )
    .expect_err("DeveloperSmoke evidence cannot satisfy phase 15 closeout");

    assert_eq!(
        denial,
        worth_store_certification::PhysicalIsolationCloseoutDenial::NonCiCertificationProfile
    );
}

#[test]
fn closeout_denies_missing_or_duplicate_hostile_lanes() {
    let mut rows = closeout_rows();
    rows.pop();
    let denial = PhysicalIsolationCloseoutSuite::from_simulation_harness_readiness(
        simulation_harness_readiness(),
        rows,
    )
    .expect_err("missing required hostile lane cannot close S5");
    assert!(matches!(
        denial,
        worth_store_certification::PhysicalIsolationCloseoutDenial::MissingLane(_)
    ));

    let mut rows = closeout_rows();
    rows.push(rows[0].clone());
    let denial = PhysicalIsolationCloseoutSuite::from_simulation_harness_readiness(
        simulation_harness_readiness(),
        rows,
    )
    .expect_err("duplicate hostile lane cannot close S5");
    assert!(matches!(
        denial,
        worth_store_certification::PhysicalIsolationCloseoutDenial::DuplicateLane(_)
    ));
}

#[test]
fn closeout_denies_mismatched_lane_evidence_fragments() {
    let rows = closeout_rows();
    let coverage_denial = PhysicalIsolationCloseoutLaneEvidence::from_executed_lane(
        rows[0].scenario().clone(),
        rows[0].plan().clone(),
        rows[1].coverage().clone(),
        rows[0].certification().clone(),
        rows[0].mutation().clone(),
        rows[0].executed().clone(),
    )
    .expect_err("coverage from another replay cannot close a lane");
    assert!(matches!(
        coverage_denial,
        worth_store_certification::PhysicalIsolationCloseoutDenial::CoverageIdentityMismatch(_)
    ));

    let mutation_denial = PhysicalIsolationCloseoutLaneEvidence::from_executed_lane(
        rows[0].scenario().clone(),
        rows[0].plan().clone(),
        rows[0].coverage().clone(),
        rows[0].certification().clone(),
        rows[1].mutation().clone(),
        rows[0].executed().clone(),
    )
    .expect_err("mutation evidence from another replay cannot close a lane");
    assert_eq!(
        mutation_denial,
        worth_store_certification::PhysicalIsolationCloseoutDenial::MutationReplayBasisMismatch
    );

    let executed_denial = PhysicalIsolationCloseoutLaneEvidence::from_executed_lane(
        rows[0].scenario().clone(),
        rows[0].plan().clone(),
        rows[0].coverage().clone(),
        rows[0].certification().clone(),
        rows[0].mutation().clone(),
        rows[1].executed().clone(),
    )
    .expect_err("executed evidence from another replay cannot close a lane");
    assert_eq!(
        executed_denial,
        worth_store_certification::PhysicalIsolationCloseoutDenial::ExecutedEvidenceReplayBasisMismatch
    );
}

#[test]
fn closeout_seals_handoff_evidence_without_minting_production_readiness() {
    let closeout = executed_closeout_fixture::honest_executed_physical_isolation_closeout();
    let handoff = closeout_suite()
        .seal_executed_closeout_handoff(closeout.clone())
        .expect("aggregate closeout seals executed handoff evidence");

    assert_eq!(handoff.suite().lanes().len(), 6);
    assert_eq!(handoff.executed_closeout(), &closeout);

    let readiness = publish_scheduler_isolation_capability_from_executed_evidence(closeout)
        .expect("production readiness minting belongs to physical-isolation");
    assert_eq!(
        readiness.assumptions(),
        &PhysicalStabilityAssumption::required()
    );
    assert_eq!(
        readiness.unsupported_qos_claims(),
        &[
            UnsupportedQoSClaim::P99Latency,
            UnsupportedQoSClaim::P999Latency,
            UnsupportedQoSClaim::HardwareQueueDepth,
            UnsupportedQoSClaim::MediaQoS,
            UnsupportedQoSClaim::BackgroundWorkPacing,
        ]
    );
    executed_closeout_fixture::assert_expected_io_qos_closeout_counters(readiness.counters());
}

fn closeout_suite() -> PhysicalIsolationCloseoutSuite {
    PhysicalIsolationCloseoutSuite::from_simulation_harness_readiness(
        simulation_harness_readiness(),
        closeout_rows(),
    )
    .expect("complete S5 physical isolation closeout suite admits")
}

fn simulation_harness_readiness(
) -> worth_store_physical_certification::PhysicalIsolationHarnessReadinessReceipt {
    harness_support::simulation_harness_readiness_receipt()
}

fn closeout_rows() -> Vec<PhysicalIsolationCloseoutLaneEvidence> {
    physical_isolation_lanes()
        .into_iter()
        .map(|lane| {
            let plan = harness_support::lower_lane(&lane);
            let replay = harness_support::replay_bundle(&plan, lane.expected_fault());
            let mutation =
                PhysicalIsolationMutationEvidence::from_replay(plan.scenario_family(), &replay);
            let coverage =
                physical_isolation_coverage_matrix(lane.scenario(), &plan, &replay, &mutation);
            let certification =
                PhysicalCertificationEvidenceBundle::from_replay_bundle(replay.clone()).unwrap();
            let source = ExecutedPhysicalIsolationEvidenceSource::from_executed_replay(
                physical_isolation_source_authority(),
                replay,
                mutation.clone(),
                PhysicalIsolationEvidenceProfile::minimal_required(),
            )
            .unwrap();
            let executed =
                materialize_physical_isolation_executed_isolation_evidence(source).unwrap();
            PhysicalIsolationCloseoutLaneEvidence::from_executed_lane(
                lane.scenario().clone(),
                plan,
                coverage,
                certification,
                mutation,
                executed,
            )
            .unwrap()
        })
        .collect()
}

fn assert_required_surfaces(row: &PhysicalIsolationCloseoutLaneEvidence) {
    for surface in required_surfaces() {
        assert!(
            row.coverage()
                .rows()
                .iter()
                .any(|row| row.surface() == surface),
            "missing closeout coverage surface {surface:?}"
        );
    }
}

fn required_surfaces() -> [CoverageSurfaceKind; 9] {
    [
        CoverageSurfaceKind::Scenario,
        CoverageSurfaceKind::Plan,
        CoverageSurfaceKind::YieldpointSchedule,
        CoverageSurfaceKind::Actor,
        CoverageSurfaceKind::Driver,
        CoverageSurfaceKind::Oracle,
        CoverageSurfaceKind::Counter,
        CoverageSurfaceKind::Transcript,
        CoverageSurfaceKind::MutationResult,
    ]
}

fn required_families() -> [PhysicalSimulationScenarioFamily; 6] {
    [
        PhysicalSimulationScenarioFamily::PhysicalIsolationCompactionInterlock,
        PhysicalSimulationScenarioFamily::PhysicalIsolationCheckpointPublicationInterlock,
        PhysicalSimulationScenarioFamily::PhysicalIsolationReclaimReachability,
        PhysicalSimulationScenarioFamily::PhysicalIsolationTierMovementStability,
        PhysicalSimulationScenarioFamily::PhysicalIsolationFutureChunkStability,
        PhysicalSimulationScenarioFamily::PhysicalIsolationRestartDuringCutover,
    ]
}

fn physical_isolation_source_authority() -> StoreCurrentAuthorityWitness {
    require_current_store_authority(worth_store_test_support::physical_isolation_boundary_fact(
        "s5.phase15.closeout",
        15,
    ))
}
