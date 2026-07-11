use forge_store_budgets::CounterEvidenceStrength;

use crate::foreground_reservation::admitted_point_read_reservation_for_certification_test;
use crate::layout_access::foreground_interference_family::ForegroundInterferencePosture;
use crate::layout_access::pacing_family::BackgroundPacingInterferencePosture;
use crate::layout_access::scheduler_reservation_family::SchedulerReservationInterferencePosture;
use crate::queue_execution::test_support::{admitted_plan, completion_for_plan, speculative_scope};
use crate::{
    admit_background_pacing, assess_queue_latency_envelope,
    blob_ingest_throttled_background_capacity_for_certification_test, execute_ready_queue_plan,
    BackgroundIdleCapacityLeaseRequest, BackgroundResourceBudget, InterferenceCounterName,
    InterferenceCounterRequirement, LatencyEnvelopeClaim, QueueSlot,
};

#[test]
fn scheduler_reservation_layout_preserves_budget_and_interference_basis() {
    let receipt = admitted_point_read_reservation_for_certification_test();

    let report = receipt.admit_scheduler_reservation_layout();

    assert_eq!(report.family_id().label(), "scheduler_reservation_index");
    assert_eq!(
        report.interference_posture(),
        SchedulerReservationInterferencePosture::StableReadEnvelopeBound
    );
    assert_eq!(report.requested_budget().queue_slots(), 1);
    assert_eq!(report.exact_counters(), receipt.counters());
}

#[test]
fn background_pacing_layout_reports_budget_and_exact_counters_from_real_outcome() {
    let requested = BackgroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(2).expect("test queue slots should admit"));
    let admitted = BackgroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(1).expect("test queue slots should admit"));
    let capacity =
        blob_ingest_throttled_background_capacity_for_certification_test(requested, admitted);
    let outcome = admit_background_pacing(BackgroundIdleCapacityLeaseRequest::new(capacity));

    let report = outcome.admit_background_pacing_layout();

    assert_eq!(report.family_id().label(), "background_pacing_record");
    assert_eq!(
        report.interference_posture(),
        BackgroundPacingInterferencePosture::Throttled
    );
    assert_eq!(report.requested_budget().queue_slots(), 2);
    assert_eq!(report.admitted_budget().queue_slots(), 1);
    assert_eq!(report.exact_counters().throttle_events(), 1);
}

#[test]
fn foreground_interference_layout_preserves_budget_and_filters_to_exact_counter_rows() {
    let plan = admitted_plan();
    let completion = completion_for_plan(&plan, 0, None, 0, Some(speculative_scope(&plan)))
        .observe_queue_depth(4)
        .observe_foreground_wait_events(3)
        .complete();
    let outcome = execute_ready_queue_plan(plan, completion);
    let replay_identity = outcome.replay_identity();
    let claim = LatencyEnvelopeClaim::for_queue_execution(
        replay_identity,
        "phase26-test",
        replay_identity.work_class(),
    )
    .with_max_interference_events(8)
    .require_counter(InterferenceCounterRequirement::foreground_wait())
    .require_counter(
        InterferenceCounterRequirement::queue_depth()
            .with_strength(CounterEvidenceStrength::Sampled),
    );
    let assessment = assess_queue_latency_envelope(&claim, &outcome)
        .expect("real queue execution should admit interference assessment");

    let report = assessment.admit_foreground_interference_layout();

    assert_eq!(report.family_id().label(), "foreground_interference_record");
    assert_eq!(
        report.interference_posture(),
        ForegroundInterferencePosture::Held
    );
    assert_eq!(
        report.declared_budget().requested_budget(),
        replay_identity.requested_budget()
    );
    assert_eq!(report.declared_budget().max_interference_events(), Some(8));
    assert!(report
        .exact_counter(InterferenceCounterName::QueueForegroundWaitEvents)
        .is_some());
    assert!(report
        .exact_counter(InterferenceCounterName::QueuePeakDepth)
        .is_none());
}

trait OutcomeReplayIdentity {
    fn replay_identity(&self) -> crate::QueueExecutionReplayIdentity;
}

impl OutcomeReplayIdentity for crate::QueueExecutionOutcome {
    fn replay_identity(&self) -> crate::QueueExecutionReplayIdentity {
        match self {
            crate::QueueExecutionOutcome::Executed(evidence) => evidence.plan().replay_identity(),
            crate::QueueExecutionOutcome::Backpressured(evidence) => {
                evidence.plan().replay_identity()
            }
            crate::QueueExecutionOutcome::Denied(evidence) => evidence.plan().replay_identity(),
            crate::QueueExecutionOutcome::Violation(evidence) => evidence.plan().replay_identity(),
        }
    }
}
