pub(in crate::intent) mod consequence;
mod ia_08;
pub(in crate::intent) mod lifecycle;
mod phase4;
mod portal_service;
pub(in crate::intent) mod registration;
pub(in crate::intent) mod reservation;

use worth_ui::facade::intent::{UiIntentExecutionClockReading, UiIntentExecutionDeadlineBasis};

pub(in crate::intent) const fn execution_reading(tick: u64) -> UiIntentExecutionClockReading {
    UiIntentExecutionClockReading::at_tick(tick)
}

pub(in crate::intent) fn execution_deadline(tick: u64) -> UiIntentExecutionDeadlineBasis {
    UiIntentExecutionClockReading::at_tick(0)
        .deadline_after_ticks(tick)
        .expect("certification deadline must fit the execution clock")
}
