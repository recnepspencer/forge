use super::policy_receipts::background_policy_receipt;
use super::test_support::{
    background_budget_with_bandwidth, background_budget_with_flush_permits,
    background_budget_with_queue_slots, background_budget_with_write_back_pages, World,
};

use crate::{
    admit_background_capacity, admit_background_pacing, BackgroundCapacityAdmissionRequest,
    BackgroundIoPressureShape, BackgroundPacingDenial, BackgroundPacingOutcome,
    BackgroundResourceBudget, QueueSlot,
};

#[test]
fn mismatched_policy_receipt_denies_background_capacity() {
    let world = World::new();
    let requested = background_budget_with_bandwidth(4096);
    let shape = BackgroundIoPressureShape::compaction_rewrite().requesting(requested);
    let request = BackgroundCapacityAdmissionRequest::new(
        shape,
        world.foreground(),
        world.backend(),
        background_policy_receipt(requested, BackgroundResourceBudget::new()),
    )
    .with_policy_admitted(requested);

    assert!(matches!(
        admit_background_capacity(request),
        Err(BackgroundPacingDenial::PolicyReceiptBudgetMismatch { .. })
    ));
}

#[test]
fn per_resource_starvation_pressure_throttles_before_lease() {
    let cases = [
        (
            World::new(),
            BackgroundIoPressureShape::compaction_rewrite(),
            background_budget_with_queue_slots(QueueSlot::new(1).unwrap()),
        ),
        (
            World::new(),
            BackgroundIoPressureShape::compaction_rewrite(),
            background_budget_with_bandwidth(4096),
        ),
        (
            World::commit_wal(),
            BackgroundIoPressureShape::checkpoint_flush(),
            background_budget_with_flush_permits(1),
        ),
        (
            World::new(),
            BackgroundIoPressureShape::compaction_rewrite(),
            background_budget_with_write_back_pages(1),
        ),
    ];

    for (world, shape, requested) in cases {
        let outcome = admit_background_pacing(world.request_with_current(
            shape.requesting(requested),
            BackgroundResourceBudget::new(),
            requested,
            BackgroundResourceBudget::new(),
        ));
        let BackgroundPacingOutcome::Throttled(throttled) = outcome else {
            panic!("expected throttle before lease, got {outcome:?}");
        };
        assert_eq!(throttled.admitted_budget(), BackgroundResourceBudget::new());
        assert_eq!(throttled.throttled_budget(), requested);
        assert_eq!(throttled.counters().throttle_events(), 1);
    }
}
