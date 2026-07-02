#[path = "s4_closeout/fixture.rs"]
mod closeout_fixture;
#[path = "s5_executed_closeout_fixture.rs"]
mod executed_closeout_fixture;
#[path = "s5_interleaving_harness_support.rs"]
mod harness_support;
#[path = "s5_stable_read_execution/plan_admission.rs"]
mod plan_admission;
#[path = "s5_copy_on_write_publication/support.rs"]
mod publication_support;
#[path = "s5_reclaim_reachability_hazard_barriers/support.rs"]
mod reclaim_support;
#[path = "s4_recovery_source_precedence/source_precedence_fixture.rs"]
mod source_precedence_fixture;
#[path = "s5_epoch_scope_and_root_kind/support.rs"]
mod support;

use forge_store_authority::{require_current_store_authority, StoreCurrentAuthorityWitness};
use forge_store_certification::{
    materialize_s5_executed_isolation_evidence, s5_physical_isolation_coverage_matrix,
    s5_physical_isolation_lanes, PhysicalIsolationCloseoutLaneEvidence,
    PhysicalIsolationCloseoutSuite, S5CloseoutReservedScope, S5ExecutedIsolationEvidenceSource,
    S5PhysicalIsolationMutationEvidence,
};
use forge_store_physical_certification::{
    CoverageSurfaceKind, PhysicalCertificationEvidenceBundle, PhysicalSimulationProfile,
    PhysicalSimulationScenarioFamily, Roadmap2HarnessSequence,
};
use forge_store_physical_isolation::{
    PhysicalStabilityAssumption, S5IsolationEvidenceProfile, UnsupportedQoSClaim,
};

#[test]
fn phase15_closeout_aggregates_all_s5_hostile_lanes_with_machine_evidence() {
    let suite = closeout_suite();

    assert_eq!(suite.lanes().len(), 6);
    assert!(suite.s45_readiness().shortcut_denial_count() > 0);
    assert!(suite
        .reservations()
        .contains(S5CloseoutReservedScope::S6IoQosIsolation));
    assert!(suite
        .reservations()
        .contains(S5CloseoutReservedScope::S7BlobLifecycle));
    assert!(suite
        .reservations()
        .contains(S5CloseoutReservedScope::S8Layout));
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
        assert_eq!(lane.coverage().sequence(), Roadmap2HarnessSequence::S45);
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
            forge_store_certification::s5_physical_isolation_required_mutation_rows(family)
        );
        assert!(lane
            .executed()
            .proof()
            .is_checked_from_executed_store_isolation());
    }
}

#[test]
fn phase15_closeout_denies_smoke_profile_lane_evidence() {
    let mut lanes = s5_physical_isolation_lanes();
    let lane = lanes.remove(0);
    let plan = forge_store_physical_certification::lower_physical_simulation_plan(
        lane.scenario().clone(),
        harness_support::developer_smoke_context(),
    )
    .unwrap();
    let replay = harness_support::replay_bundle(&plan, lane.expected_fault());
    let mutation =
        S5PhysicalIsolationMutationEvidence::from_replay(plan.scenario_family(), &replay);
    let coverage =
        s5_physical_isolation_coverage_matrix(lane.scenario(), &plan, &replay, &mutation);
    let certification =
        PhysicalCertificationEvidenceBundle::from_replay_bundle(replay.clone()).unwrap();
    let source = S5ExecutedIsolationEvidenceSource::from_executed_replay(
        s5_source_authority(),
        replay,
        mutation.clone(),
        S5IsolationEvidenceProfile::minimal_required(),
    )
    .unwrap();
    let executed = materialize_s5_executed_isolation_evidence(source).unwrap();
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
        forge_store_certification::PhysicalIsolationCloseoutDenial::NonCiCertificationProfile
    );
}

#[test]
fn phase15_closeout_denies_missing_or_duplicate_hostile_lanes() {
    let mut rows = closeout_rows();
    rows.pop();
    let denial = PhysicalIsolationCloseoutSuite::from_s45_readiness(s45_readiness(), rows)
        .expect_err("missing required hostile lane cannot close S5");
    assert!(matches!(
        denial,
        forge_store_certification::PhysicalIsolationCloseoutDenial::MissingLane(_)
    ));

    let mut rows = closeout_rows();
    rows.push(rows[0].clone());
    let denial = PhysicalIsolationCloseoutSuite::from_s45_readiness(s45_readiness(), rows)
        .expect_err("duplicate hostile lane cannot close S5");
    assert!(matches!(
        denial,
        forge_store_certification::PhysicalIsolationCloseoutDenial::DuplicateLane(_)
    ));
}

#[test]
fn phase15_closeout_denies_mismatched_lane_evidence_fragments() {
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
        forge_store_certification::PhysicalIsolationCloseoutDenial::CoverageIdentityMismatch(_)
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
        forge_store_certification::PhysicalIsolationCloseoutDenial::MutationReplayBasisMismatch
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
        forge_store_certification::PhysicalIsolationCloseoutDenial::ExecutedEvidenceReplayBasisMismatch
    );
}

#[test]
fn phase15_closeout_publishes_s6_readiness_only_from_executed_store_closeout() {
    let published = closeout_suite()
        .publish_s6_readiness(executed_closeout_fixture::honest_executed_s5_closeout())
        .expect("aggregate closeout publishes S6 from executed Store closeout");

    assert_eq!(published.suite().lanes().len(), 6);
    assert_eq!(
        published.s6_readiness().assumptions(),
        &PhysicalStabilityAssumption::s6_handoff_assumptions()
    );
    assert_eq!(
        published.s6_readiness().unsupported_qos_claims(),
        &UnsupportedQoSClaim::canonical_s5_non_claims()
    );
    executed_closeout_fixture::assert_expected_s6_closeout_counters(
        published.s6_readiness().counters(),
    );
}

fn closeout_suite() -> PhysicalIsolationCloseoutSuite {
    PhysicalIsolationCloseoutSuite::from_s45_readiness(s45_readiness(), closeout_rows())
        .expect("complete S5 physical isolation closeout suite admits")
}

fn s45_readiness() -> forge_store_physical_certification::S5HarnessReadinessReceipt {
    harness_support::s45_harness_readiness_receipt()
}

fn closeout_rows() -> Vec<PhysicalIsolationCloseoutLaneEvidence> {
    s5_physical_isolation_lanes()
        .into_iter()
        .map(|lane| {
            let plan = harness_support::lower_lane(&lane);
            let replay = harness_support::replay_bundle(&plan, lane.expected_fault());
            let mutation =
                S5PhysicalIsolationMutationEvidence::from_replay(plan.scenario_family(), &replay);
            let coverage =
                s5_physical_isolation_coverage_matrix(lane.scenario(), &plan, &replay, &mutation);
            let certification =
                PhysicalCertificationEvidenceBundle::from_replay_bundle(replay.clone()).unwrap();
            let source = S5ExecutedIsolationEvidenceSource::from_executed_replay(
                s5_source_authority(),
                replay,
                mutation.clone(),
                S5IsolationEvidenceProfile::minimal_required(),
            )
            .unwrap();
            let executed = materialize_s5_executed_isolation_evidence(source).unwrap();
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
        PhysicalSimulationScenarioFamily::S5CompactionInterlock,
        PhysicalSimulationScenarioFamily::S5CheckpointPublicationInterlock,
        PhysicalSimulationScenarioFamily::S5ReclaimReachability,
        PhysicalSimulationScenarioFamily::S5TierMovementStability,
        PhysicalSimulationScenarioFamily::S5FutureChunkStability,
        PhysicalSimulationScenarioFamily::S5RestartDuringCutover,
    ]
}

fn s5_source_authority() -> StoreCurrentAuthorityWitness {
    require_current_store_authority(forge_store_test_support::s5_boundary_fact(
        "s5.phase15.closeout",
        15,
    ))
}
