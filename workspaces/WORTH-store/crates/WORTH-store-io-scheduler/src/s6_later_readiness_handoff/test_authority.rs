use crate::{
    BackgroundIoDebt, BackgroundIoPressureClass, BackgroundPacingCounterSnapshot,
    BackgroundPacingOutcome, BackgroundPacingViolation, BackgroundResourceBudget, QueueSlot,
};

pub fn background_pacing_outcome_for_later_readiness_certification_test(
    class: BackgroundIoPressureClass,
) -> BackgroundPacingOutcome {
    let requested = BackgroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(1).expect("one queue slot is nonzero"));
    BackgroundPacingOutcome::Violation(BackgroundPacingViolation::new(
        BackgroundIoDebt::new(class, requested),
        BackgroundPacingCounterSnapshot::violation(
            requested,
            BackgroundResourceBudget::new(),
            BackgroundResourceBudget::new(),
            requested,
            class.debt_kind(),
            1,
        ),
    ))
}
