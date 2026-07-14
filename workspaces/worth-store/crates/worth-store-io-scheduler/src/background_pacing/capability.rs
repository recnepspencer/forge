use super::{
    BackgroundIoPressureClass, BackgroundPacingCounterSnapshot, BackgroundPacingDenial,
    BackgroundPacingOutcome,
};

/// Sealed background-pacing capability issued only from verified background pacing outcomes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackgroundPacingCapability {
    class: BackgroundIoPressureClass,
    counters: BackgroundPacingCounterSnapshot,
    outcome: BackgroundPacingOutcome,
}

impl BackgroundPacingCapability {
    pub fn from_admitted_outcome(
        outcome: BackgroundPacingOutcome,
    ) -> Result<Self, BackgroundPacingDenial> {
        let counters = match outcome {
            BackgroundPacingOutcome::AdmittedWithDebt(outcome) => outcome.counters(),
            BackgroundPacingOutcome::Throttled(outcome) => outcome.counters(),
            BackgroundPacingOutcome::Yield(outcome) => outcome.counters(),
            BackgroundPacingOutcome::Deferred(outcome) => outcome.counters(),
            BackgroundPacingOutcome::Denied(outcome) => {
                return Err(outcome.denial());
            }
            BackgroundPacingOutcome::StaleRebindRequired(outcome) => {
                return Err(BackgroundPacingDenial::PacingProgressionDenied(
                    outcome.class(),
                ));
            }
            BackgroundPacingOutcome::Violation(outcome) => {
                return Err(BackgroundPacingDenial::PacingProgressionFailed(
                    outcome.class(),
                ));
            }
        };
        Ok(Self {
            class: outcome.class(),
            counters,
            outcome,
        })
    }

    pub const fn class(self) -> BackgroundIoPressureClass {
        self.class
    }

    pub const fn counters(self) -> BackgroundPacingCounterSnapshot {
        self.counters
    }

    pub const fn outcome(self) -> BackgroundPacingOutcome {
        self.outcome
    }
}
