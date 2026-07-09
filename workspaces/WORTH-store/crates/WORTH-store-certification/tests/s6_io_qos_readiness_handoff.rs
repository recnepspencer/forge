#[path = "s4_closeout/fixture.rs"]
mod closeout_fixture;
#[path = "s5_executed_closeout_fixture.rs"]
mod executed_closeout_fixture;
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

use worth_store_io_scheduler::{
    admit_store_published_s6_io_qos_isolation_readiness,
    reject_hardware_queue_depth_claim_as_s6_readiness,
    reject_log_or_metric_projection_as_s6_readiness, reject_media_qos_claim_as_s6_readiness,
    IoSchedulerS6ReadinessDenial,
};
use worth_store_physical_isolation::publish_s6_io_qos_isolation_readiness_from_s5_closeout;
use worth_store_physical_isolation::{
    reject_copied_closeout_report_as_s6_readiness,
    reject_log_or_terminal_projection_as_s6_readiness,
    reject_missing_latch_counters_as_s6_readiness,
    reject_missing_protected_byte_footprint_as_s6_readiness,
    reject_missing_reclaim_counters_as_s6_readiness, reject_qos_claim_as_s5_readiness,
    reject_synthetic_wait_label_as_s6_readiness, ExecutedS5IsolationCloseout,
    PhysicalStabilityAssumption, S6IoQosIsolationReadiness, S6IoQosIsolationReadinessDenial,
    S6ReadinessAuthorityPosture, UnsupportedQoSClaim,
};

#[test]
fn executed_s5_closeout_publishes_typed_s6_io_qos_readiness() {
    let closeout = executed_closeout_fixture::honest_executed_s5_closeout();
    let readiness = readiness_from_executed_s5_closeout(closeout);
    let counters = readiness.counters();

    executed_closeout_fixture::assert_expected_s6_closeout_counters(counters);
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
        &UnsupportedQoSClaim::canonical_s5_non_claims()
    );
    assert_eq!(
        readiness.basis().projection_evidence().authority_posture(),
        S6ReadinessAuthorityPosture::StoreExecutedIsolationMaterialized
    );
}

#[test]
fn identical_executed_s5_evidence_produces_equivalent_s6_handoff() {
    let left = readiness_from_executed_s5_closeout(
        executed_closeout_fixture::honest_executed_s5_closeout(),
    );
    let right = readiness_from_executed_s5_closeout(
        executed_closeout_fixture::honest_executed_s5_closeout(),
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
fn shortcuts_cannot_satisfy_s6_readiness() {
    assert_eq!(
        reject_copied_closeout_report_as_s6_readiness().unwrap_err(),
        S6IoQosIsolationReadinessDenial::CopiedCloseoutReport
    );
    assert_eq!(
        reject_log_or_terminal_projection_as_s6_readiness().unwrap_err(),
        S6IoQosIsolationReadinessDenial::LogOrTerminalProjection
    );
    assert_eq!(
        reject_synthetic_wait_label_as_s6_readiness().unwrap_err(),
        S6IoQosIsolationReadinessDenial::SyntheticWaitLabel
    );
}

#[test]
fn missing_latch_reclaim_or_footprint_counters_deny_s6_readiness() {
    assert_eq!(
        reject_missing_latch_counters_as_s6_readiness().unwrap_err(),
        S6IoQosIsolationReadinessDenial::MissingLatchCounters
    );
    assert_eq!(
        reject_missing_reclaim_counters_as_s6_readiness().unwrap_err(),
        S6IoQosIsolationReadinessDenial::MissingReclaimCounters
    );
    assert_eq!(
        reject_missing_protected_byte_footprint_as_s6_readiness().unwrap_err(),
        S6IoQosIsolationReadinessDenial::MissingProtectedByteFootprint
    );
}

#[test]
fn s5_names_every_qos_claim_it_does_not_make() {
    for claim in UnsupportedQoSClaim::canonical_s5_non_claims() {
        assert_eq!(
            reject_qos_claim_as_s5_readiness(claim).unwrap_err(),
            S6IoQosIsolationReadinessDenial::UnsupportedQoSClaimRequested(claim)
        );
    }
}

#[test]
fn s6_readiness_exposes_scheduler_required_surfaces_without_qos_authority() {
    let readiness = readiness_from_executed_s5_closeout(
        executed_closeout_fixture::honest_executed_s5_closeout(),
    );
    let counters = readiness.counters();
    let scheduler_admission = admit_store_published_s6_io_qos_isolation_readiness(&readiness)
        .expect("scheduler receives Store-published S6 readiness");

    assert_eq!(
        readiness.assumptions(),
        &PhysicalStabilityAssumption::s6_handoff_assumptions()
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

#[test]
fn scheduler_denies_projection_and_claim_shortcuts() {
    assert_eq!(
        reject_log_or_metric_projection_as_s6_readiness().unwrap_err(),
        IoSchedulerS6ReadinessDenial::LogOrMetricProjection
    );
    assert_eq!(
        reject_hardware_queue_depth_claim_as_s6_readiness().unwrap_err(),
        IoSchedulerS6ReadinessDenial::HardwareQueueDepthClaim
    );
    assert_eq!(
        reject_media_qos_claim_as_s6_readiness().unwrap_err(),
        IoSchedulerS6ReadinessDenial::MediaQosClaim
    );
}

fn readiness_from_executed_s5_closeout(
    closeout: ExecutedS5IsolationCloseout,
) -> S6IoQosIsolationReadiness {
    publish_s6_io_qos_isolation_readiness_from_s5_closeout(closeout)
        .expect("executed S5 physical closeout publishes S6 readiness")
}
