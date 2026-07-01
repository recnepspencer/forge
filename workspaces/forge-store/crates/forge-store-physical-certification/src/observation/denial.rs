use crate::{ObserverKind, ShortcutRejectionObservationKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObservationDenial {
    ObserverNotRequired {
        observer: ObserverKind,
    },
    MissingRuntimeTrace {
        observer: ObserverKind,
    },
    MissingExecutedProductionBoundaryTrace,
    ExecutionReceiptPlanMismatch,
    MissingIndependentVerifierObservation,
    MissingRecoveryOutcomeObservation,
    MissingShortcutRejectionObservation,
    MissingRequiredShortcutRejectionObservation {
        required: ShortcutRejectionObservationKind,
    },
    SameRunSelfComparisonDenied,
    LogOnlyObservationDenied,
    ExpectedErrorTextObservationDenied,
    FixtureLabelObservationDenied,
}
