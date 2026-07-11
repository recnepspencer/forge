use forge_store_io_scheduler::{
    BackgroundIoDebt, BackgroundPacingCounterSnapshot, BackgroundPacingOutcome,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum S6BackgroundPacingCertificationDenial {
    OutcomeMismatch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum S6BackgroundPacingOutcomeKind {
    Yield,
    Deferred,
    Denied,
    StaleRebindRequired,
    Throttled,
    AdmittedWithDebt,
    Violation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct S6BackgroundPacingCertificationEvidence {
    outcome: S6BackgroundPacingOutcomeKind,
    counters: BackgroundPacingCounterSnapshot,
    debt: Option<BackgroundIoDebt>,
}

pub fn certify_io_qos_background_pacing(
    outcome: BackgroundPacingOutcome,
    expected: BackgroundPacingOutcome,
) -> Result<S6BackgroundPacingCertificationEvidence, S6BackgroundPacingCertificationDenial> {
    if outcome != expected {
        return Err(S6BackgroundPacingCertificationDenial::OutcomeMismatch);
    }
    Ok(S6BackgroundPacingCertificationEvidence::from_outcome(
        outcome,
    ))
}

impl S6BackgroundPacingCertificationEvidence {
    pub const fn outcome(self) -> S6BackgroundPacingOutcomeKind {
        self.outcome
    }

    pub const fn counters(self) -> BackgroundPacingCounterSnapshot {
        self.counters
    }

    pub const fn debt(self) -> Option<BackgroundIoDebt> {
        self.debt
    }

    const fn from_outcome(outcome: BackgroundPacingOutcome) -> Self {
        match outcome {
            BackgroundPacingOutcome::Yield(receipt) => Self {
                outcome: S6BackgroundPacingOutcomeKind::Yield,
                counters: receipt.counters(),
                debt: None,
            },
            BackgroundPacingOutcome::Deferred(receipt) => Self {
                outcome: S6BackgroundPacingOutcomeKind::Deferred,
                counters: receipt.counters(),
                debt: None,
            },
            BackgroundPacingOutcome::Denied(receipt) => Self {
                outcome: S6BackgroundPacingOutcomeKind::Denied,
                counters: receipt.counters(),
                debt: None,
            },
            BackgroundPacingOutcome::StaleRebindRequired(receipt) => Self {
                outcome: S6BackgroundPacingOutcomeKind::StaleRebindRequired,
                counters: receipt.counters(),
                debt: None,
            },
            BackgroundPacingOutcome::Throttled(receipt) => Self {
                outcome: S6BackgroundPacingOutcomeKind::Throttled,
                counters: receipt.counters(),
                debt: None,
            },
            BackgroundPacingOutcome::AdmittedWithDebt(receipt) => Self {
                outcome: S6BackgroundPacingOutcomeKind::AdmittedWithDebt,
                counters: receipt.counters(),
                debt: Some(receipt.debt()),
            },
            BackgroundPacingOutcome::Violation(receipt) => Self {
                outcome: S6BackgroundPacingOutcomeKind::Violation,
                counters: receipt.counters(),
                debt: Some(receipt.causal_debt()),
            },
        }
    }
}
