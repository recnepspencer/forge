use super::PartialPublicationClassification;
use crate::partial_publication::{
    PartialPublicationCounterSnapshot, RecoveredOrRejectedPartialPublication,
    UnacknowledgedPublicationOutcome,
};

pub(super) fn classification(
    outcome: UnacknowledgedPublicationOutcome,
    recovered_or_rejected: RecoveredOrRejectedPartialPublication,
    counters: PartialPublicationCounterSnapshot,
    digest: &str,
) -> PartialPublicationClassification {
    PartialPublicationClassification {
        outcome,
        recovered_or_rejected,
        counters,
        classification_digest: format!("{outcome:?}:{digest}"),
        before_wal_append_operation_digest: None,
    }
}

pub(super) fn classification_with_before_wal_operation_digest(
    outcome: UnacknowledgedPublicationOutcome,
    recovered_or_rejected: RecoveredOrRejectedPartialPublication,
    counters: PartialPublicationCounterSnapshot,
    digest: &str,
    operation_digest: String,
) -> PartialPublicationClassification {
    PartialPublicationClassification {
        outcome,
        recovered_or_rejected,
        counters,
        classification_digest: format!("{outcome:?}:{digest}"),
        before_wal_append_operation_digest: Some(operation_digest),
    }
}