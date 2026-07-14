use core::num::NonZeroU64;

use super::test_support::{
    background_budget_with_bandwidth, background_budget_with_flush_permits,
    background_budget_with_queue_slots, background_budget_with_worker_permits,
    background_budget_with_write_back_pages, World,
};

use crate::{
    admit_background_pacing, BackgroundIoPressureShape, BackgroundPacingOutcome,
    BackgroundResourceBudget, QueueSlot,
};

#[test]
fn revocation_records_each_required_idle_capacity_unit() {
    let cases = [
        (
            World::new(),
            BackgroundIoPressureShape::compaction_rewrite(),
            background_budget_with_queue_slots(QueueSlot::new(1).unwrap()),
            UnitExpectation::QueueSlot,
        ),
        (
            World::new(),
            BackgroundIoPressureShape::compaction_rewrite(),
            background_budget_with_bandwidth(4096),
            UnitExpectation::Bandwidth,
        ),
        (
            World::new(),
            BackgroundIoPressureShape::compaction_rewrite(),
            background_budget_with_worker_permits(1),
            UnitExpectation::WorkerPermit,
        ),
        (
            World::commit_wal(),
            BackgroundIoPressureShape::checkpoint_flush(),
            background_budget_with_flush_permits(1),
            UnitExpectation::FlushPermit,
        ),
        (
            World::new(),
            BackgroundIoPressureShape::compaction_rewrite(),
            background_budget_with_write_back_pages(1),
            UnitExpectation::WriteBackWindow,
        ),
    ];

    for (world, shape, admitted, expectation) in cases {
        let outcome = admit_background_pacing(world.request_with_current(
            shape.requesting(admitted),
            admitted,
            admitted,
            BackgroundResourceBudget::new(),
        ));
        let BackgroundPacingOutcome::AdmittedWithDebt(admitted_with_debt) = outcome else {
            panic!("expected admitted lease for {expectation:?}, got {outcome:?}");
        };
        let revocation = admitted_with_debt
            .lease()
            .revoke_for_foreground_pressure(NonZeroU64::new(1).unwrap());
        assert_eq!(revocation.revoked_budget(), admitted);
        assert_eq!(revocation.counters().revoked_budget(), admitted);
        assert_eq!(revocation.counters().revoke_events(), 1);
        assert_eq!(revocation.counters().foreground_pressure_events(), 1);
        expectation.assert_visible(revocation.revoked_budget());
    }
}

#[derive(Clone, Copy, Debug)]
enum UnitExpectation {
    QueueSlot,
    Bandwidth,
    WorkerPermit,
    FlushPermit,
    WriteBackWindow,
}

impl UnitExpectation {
    fn assert_visible(self, budget: BackgroundResourceBudget) {
        match self {
            Self::QueueSlot => assert_eq!(budget.queue_slots(), 1),
            Self::Bandwidth => assert_eq!(budget.bandwidth_tokens(), 4096),
            Self::WorkerPermit => assert_eq!(budget.worker_permits(), 1),
            Self::FlushPermit => assert_eq!(budget.flush_permits(), 1),
            Self::WriteBackWindow => assert_eq!(budget.write_back_window(), 1),
        }
    }
}
