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
    ScheduleExecutionMismatch,
    StorageExecutionDidNotReachScheduledSeam,
    ExecutionReceiptPlanMismatch,
    MissingIndependentVerifierObservation,
    MissingRecoveryOutcomeObservation,
    MissingCheckpointPublicationLane,
    MissingShortcutRejectionObservation,
    MissingRequiredShortcutRejectionObservation {
        required: ShortcutRejectionObservationKind,
    },
    SameRunSelfComparisonDenied,
    CheckpointPublicationLanePlanMismatch,
    CheckpointPublicationLaneScheduleMismatch,
    CheckpointPublicationCrashLaneScheduleMismatch,
    CheckpointPublicationCrashRecoveryTraceMismatch,
    CheckpointPublicationCrashOutcomeMixedRoot,
    CheckpointPublicationShortcutLaneScheduleMismatch,
    CheckpointPublicationShortcutBoundaryMismatch,
    CheckpointPublicationEvidenceOriginMismatch,
    CopiedCheckpointReportObservationDenied,
    LogOnlyObservationDenied,
    ExpectedErrorTextObservationDenied,
    FixtureLabelObservationDenied,
}
