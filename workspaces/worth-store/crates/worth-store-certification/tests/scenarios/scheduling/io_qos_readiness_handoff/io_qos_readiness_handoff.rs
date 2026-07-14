#[path = "../../../support/physical_isolation/executed_closeout_fixture/executed_closeout_fixture.rs"]
mod executed_closeout_fixture;
use worth_store_test_support::harness::physical_isolation::epoch_scope as support;
use worth_store_test_support::harness::physical_isolation::publication as publication_support;
use worth_store_test_support::harness::physical_isolation::read_plan as plan_admission;
use worth_store_test_support::harness::physical_isolation::reclaim as reclaim_support;
use worth_store_test_support::harness::recovery::source_precedence as source_precedence_fixture;

use worth_store_io_scheduler::admit_store_published_isolation_capability;
use worth_store_physical_isolation::publish_scheduler_isolation_capability_from_executed_evidence;
use worth_store_physical_isolation::{
    reject_copied_closeout_report_as_isolation_readiness,
    reject_log_or_terminal_projection_as_isolation_readiness,
    reject_missing_latch_counters_as_isolation_readiness,
    reject_missing_protected_byte_footprint_as_isolation_readiness,
    reject_missing_reclaim_counters_as_isolation_readiness,
    reject_synthetic_wait_label_as_isolation_readiness,
    reject_unsupported_qos_claim_as_isolation_readiness, ExecutedIsolationEvidence,
    IsolationReadinessDenial, PhysicalStabilityAssumption, SchedulerIsolationCapability,
    UnsupportedQoSClaim,
};

#[test]
fn executed_physical_isolation_closeout_publishes_typed_io_qos_readiness() {
    let closeout = executed_closeout_fixture::honest_executed_physical_isolation_closeout();
    let readiness = readiness_from_executed_physical_isolation_closeout(closeout);
    let counters = readiness.counters();

    executed_closeout_fixture::assert_expected_io_qos_closeout_counters(counters);
    assert!(readiness
        .basis()
        .projection_evidence()
        .foundational_counter_receipt()
        .counter_rows()
        .iter()
        .any(|row| row.name().as_str() == "s5.closeout.reclaim-counter-rows"));
    assert_eq!(
        readiness
            .proof_handoff()
            .boundary_bridged_recipe()
            .basis()
            .weakened_basis()
            .basis()
            .value()
            .closeout_basis(),
        readiness.basis().closeout_basis()
    );
    assert_eq!(
        readiness
            .proof_handoff()
            .readmitted_recipe()
            .strong_basis()
            .value()
            .closeout_basis(),
        readiness.basis().closeout_basis()
    );
    assert_eq!(
        readiness.foreground_interference().retry_count(),
        counters.retry_count()
    );
    assert_eq!(
        readiness
            .background_maintenance()
            .blocked_maintenance_count(),
        counters.blocked_maintenance_count()
    );
    assert_eq!(
        readiness.unsupported_qos_claims(),
        &expected_unsupported_qos_claims()
    );
}

#[test]
fn identical_executed_physical_isolation_evidence_produces_equivalent_io_qos_handoff() {
    let left = readiness_from_executed_physical_isolation_closeout(
        executed_closeout_fixture::honest_executed_physical_isolation_closeout(),
    );
    let right = readiness_from_executed_physical_isolation_closeout(
        executed_closeout_fixture::honest_executed_physical_isolation_closeout(),
    );

    assert_eq!(left.counters(), right.counters());
    assert_eq!(left.assumptions(), right.assumptions());
    assert_eq!(
        left.foreground_interference(),
        right.foreground_interference()
    );
    assert_eq!(
        left.background_maintenance(),
        right.background_maintenance()
    );
    assert_eq!(
        left.unsupported_qos_claims(),
        right.unsupported_qos_claims()
    );
}

#[test]
fn shortcuts_cannot_satisfy_io_qos_readiness() {
    assert_eq!(
        reject_copied_closeout_report_as_isolation_readiness().unwrap_err(),
        IsolationReadinessDenial::CopiedCloseoutReport
    );
    assert_eq!(
        reject_log_or_terminal_projection_as_isolation_readiness().unwrap_err(),
        IsolationReadinessDenial::LogOrTerminalProjection
    );
    assert_eq!(
        reject_synthetic_wait_label_as_isolation_readiness().unwrap_err(),
        IsolationReadinessDenial::SyntheticWaitLabel
    );
}

#[test]
fn missing_latch_reclaim_or_footprint_counters_deny_io_qos_readiness() {
    assert_eq!(
        reject_missing_latch_counters_as_isolation_readiness().unwrap_err(),
        IsolationReadinessDenial::MissingLatchCounters
    );
    assert_eq!(
        reject_missing_reclaim_counters_as_isolation_readiness().unwrap_err(),
        IsolationReadinessDenial::MissingReclaimCounters
    );
    assert_eq!(
        reject_missing_protected_byte_footprint_as_isolation_readiness().unwrap_err(),
        IsolationReadinessDenial::MissingProtectedByteFootprint
    );
}

#[test]
fn physical_isolation_names_every_qos_claim_it_does_not_make() {
    for claim in expected_unsupported_qos_claims() {
        assert_eq!(
            reject_unsupported_qos_claim_as_isolation_readiness(claim).unwrap_err(),
            IsolationReadinessDenial::UnsupportedQoSClaimRequested(claim)
        );
    }
}

#[test]
fn io_qos_readiness_exposes_scheduler_required_surfaces_without_qos_authority() {
    let readiness = readiness_from_executed_physical_isolation_closeout(
        executed_closeout_fixture::honest_executed_physical_isolation_closeout(),
    );
    let counters = readiness.counters();
    let scheduler_admission = admit_store_published_isolation_capability(&readiness)
        .expect("scheduler receives Store-published scheduler readiness");

    assert_eq!(
        readiness.assumptions(),
        &PhysicalStabilityAssumption::required()
    );
    assert_eq!(
        readiness.foreground_interference().wait_count(),
        counters.wait_count()
    );
    assert_eq!(
        readiness.foreground_interference().retry_count(),
        counters.retry_count()
    );
    assert_eq!(
        readiness
            .foreground_interference()
            .protected_byte_footprint(),
        counters.protected_byte_footprint()
    );
    assert_eq!(
        readiness
            .background_maintenance()
            .blocked_maintenance_count(),
        counters.blocked_maintenance_count()
    );
    assert_eq!(
        scheduler_admission.foreground_interference().wait_count(),
        readiness.foreground_interference().wait_count()
    );
    assert_eq!(
        scheduler_admission.foreground_interference().retry_count(),
        readiness.foreground_interference().retry_count()
    );
    assert_eq!(
        scheduler_admission
            .foreground_interference()
            .protected_byte_footprint(),
        readiness
            .foreground_interference()
            .protected_byte_footprint()
    );
    assert_eq!(
        scheduler_admission
            .background_maintenance()
            .blocked_maintenance_count(),
        readiness
            .background_maintenance()
            .blocked_maintenance_count()
    );
}

const fn expected_unsupported_qos_claims() -> [UnsupportedQoSClaim; 5] {
    [
        UnsupportedQoSClaim::P99Latency,
        UnsupportedQoSClaim::P999Latency,
        UnsupportedQoSClaim::HardwareQueueDepth,
        UnsupportedQoSClaim::MediaQoS,
        UnsupportedQoSClaim::BackgroundWorkPacing,
    ]
}

fn readiness_from_executed_physical_isolation_closeout(
    closeout: ExecutedIsolationEvidence,
) -> SchedulerIsolationCapability {
    publish_scheduler_isolation_capability_from_executed_evidence(closeout)
        .expect("executed S5 physical closeout publishes S6 readiness")
}
