use super::assembly::classification;
use super::PartialPublicationClassification;
use crate::partial_publication::{
    NoUndoPartialPublicationClassification, PartialPublicationCounterSnapshot,
    RecoveredOrRejectedPartialPublication, RollbackImageRequiredPosture,
    UnacknowledgedPublicationOutcome,
};

pub(super) fn reject_no_undo_hazard(
    no_undo: NoUndoPartialPublicationClassification,
    digest: &str,
) -> PartialPublicationClassification {
    match no_undo.posture() {
        RollbackImageRequiredPosture::RequiredButMissing => reject_missing_rollback_image(
            no_undo,
            PartialPublicationCounterSnapshot::default().with_no_undo_denial(),
            digest,
        ),
        RollbackImageRequiredPosture::DeferredToUndoCapableRecovery => {
            defer_to_undo_capable_recovery(no_undo, digest)
        }
        RollbackImageRequiredPosture::NotRequiredForAdmittedRedoOnlyMutation
        | RollbackImageRequiredPosture::ProtectedByRollbackImage => {
            accept_no_undo_posture(no_undo, digest)
        }
    }
}

fn reject_missing_rollback_image(
    no_undo: NoUndoPartialPublicationClassification,
    counters: PartialPublicationCounterSnapshot,
    digest: &str,
) -> PartialPublicationClassification {
    classification(
        UnacknowledgedPublicationOutcome::RejectedNoUndoHazard,
        RecoveredOrRejectedPartialPublication::RejectedNoUndoHazard {
            classification: no_undo,
            counters,
        },
        counters,
        digest,
    )
}

fn defer_to_undo_capable_recovery(
    no_undo: NoUndoPartialPublicationClassification,
    digest: &str,
) -> PartialPublicationClassification {
    let counters = PartialPublicationCounterSnapshot::default().with_no_undo_posture();
    classification(
        UnacknowledgedPublicationOutcome::UndoCapableRecoveryDeferred,
        RecoveredOrRejectedPartialPublication::UndoCapableRecoveryDeferred {
            classification: no_undo,
            counters,
        },
        counters,
        digest,
    )
}

fn accept_no_undo_posture(
    no_undo: NoUndoPartialPublicationClassification,
    digest: &str,
) -> PartialPublicationClassification {
    let counters = PartialPublicationCounterSnapshot::default().with_no_undo_posture();
    let outcome = match no_undo.posture() {
        RollbackImageRequiredPosture::NotRequiredForAdmittedRedoOnlyMutation => {
            UnacknowledgedPublicationOutcome::NoUndoPostureSatisfied
        }
        RollbackImageRequiredPosture::ProtectedByRollbackImage => {
            UnacknowledgedPublicationOutcome::RollbackImageProtected
        }
        RollbackImageRequiredPosture::RequiredButMissing
        | RollbackImageRequiredPosture::DeferredToUndoCapableRecovery => {
            unreachable!("callers route rejected and deferred no-undo postures first")
        }
    };
    classification(
        outcome,
        RecoveredOrRejectedPartialPublication::NoUndoPostureAccepted {
            classification: no_undo,
            counters,
        },
        counters,
        digest,
    )
}