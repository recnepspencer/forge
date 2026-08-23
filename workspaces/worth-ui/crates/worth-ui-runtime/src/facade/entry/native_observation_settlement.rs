use crate::facade::interaction::UiHostInteractionIngressOutcome;
use worth_ui_host_contract::UiHostObservationDrainDenial;

pub(crate) enum UiNativeObservationIngressSettlement {
    Drained(UiNativeObservationDrainReport),
    DrainDenied(UiHostObservationDrainDenial),
}

pub(crate) struct UiNativeObservationDrainReport {
    outcomes: Box<[UiHostInteractionIngressOutcome]>,
    applied_batches: usize,
    duplicate_batches: usize,
    quarantined_batches: usize,
    denied_batches: usize,
}

impl UiNativeObservationIngressSettlement {
    pub(crate) fn from_outcomes(outcomes: Box<[UiHostInteractionIngressOutcome]>) -> Self {
        let mut report = UiNativeObservationDrainReport {
            outcomes,
            applied_batches: 0,
            duplicate_batches: 0,
            quarantined_batches: 0,
            denied_batches: 0,
        };
        for outcome in &report.outcomes {
            match outcome {
                UiHostInteractionIngressOutcome::Applied(_) => report.applied_batches += 1,
                UiHostInteractionIngressOutcome::Duplicate(_) => report.duplicate_batches += 1,
                UiHostInteractionIngressOutcome::Quarantined(_) => report.quarantined_batches += 1,
                UiHostInteractionIngressOutcome::Denied(_) => report.denied_batches += 1,
            }
        }
        Self::Drained(report)
    }

    pub(crate) fn outcomes(&self) -> &[UiHostInteractionIngressOutcome] {
        match self {
            Self::Drained(report) => &report.outcomes,
            Self::DrainDenied(_) => &[],
        }
    }

    pub(crate) const fn drain_denial(&self) -> Option<UiHostObservationDrainDenial> {
        match self {
            Self::Drained(_) => None,
            Self::DrainDenied(denial) => Some(*denial),
        }
    }

    pub(crate) const fn counts(&self) -> (usize, usize, usize, usize) {
        match self {
            Self::Drained(report) => (
                report.applied_batches,
                report.duplicate_batches,
                report.quarantined_batches,
                report.denied_batches,
            ),
            Self::DrainDenied(_) => (0, 0, 0, 0),
        }
    }
}
