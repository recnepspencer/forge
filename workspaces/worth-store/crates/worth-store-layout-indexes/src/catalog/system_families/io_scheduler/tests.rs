use worth_store_io_scheduler::foreground_reservation::admitted_point_read_reservation_for_certification_test;
use worth_store_io_scheduler::{
    admit_background_pacing, blob_ingest_throttled_background_capacity_for_certification_test,
    BackgroundIdleCapacityLeaseRequest, BackgroundResourceBudget, QueueSlot,
};

use super::{
    project_background_pacing, project_scheduler_reservation, BackgroundPacingInterferencePosture,
    SchedulerReservationInterferencePosture,
};

#[test]
fn reservation_projection_preserves_scheduler_issued_budget_and_counters() {
    let receipt = admitted_point_read_reservation_for_certification_test();
    let report = project_scheduler_reservation(receipt);

    assert_eq!(report.family_id().label(), "scheduler_reservation_index");
    assert_eq!(
        report.interference_posture(),
        SchedulerReservationInterferencePosture::StableReadEnvelopeBound
    );
    assert_eq!(report.requested_budget().queue_slots(), 1);
    assert_eq!(report.exact_counters(), receipt.counters());
}

#[test]
fn pacing_projection_preserves_real_outcome_budget_and_exact_counters() {
    let requested = BackgroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(2).expect("requested queue slots should admit"));
    let admitted = BackgroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(1).expect("admitted queue slots should admit"));
    let capacity =
        blob_ingest_throttled_background_capacity_for_certification_test(requested, admitted);
    let outcome = admit_background_pacing(BackgroundIdleCapacityLeaseRequest::new(capacity));
    let report = project_background_pacing(&outcome);

    assert_eq!(report.family_id().label(), "background_pacing_record");
    assert_eq!(
        report.interference_posture(),
        BackgroundPacingInterferencePosture::Throttled
    );
    assert_eq!(report.requested_budget().queue_slots(), 2);
    assert_eq!(report.admitted_budget().queue_slots(), 1);
    assert_eq!(report.exact_counters().throttle_events(), 1);
}
