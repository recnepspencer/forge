use super::assembly::classification;
use super::PartialPublicationClassification;
use crate::partial_publication::{
    AmbiguousPublicationReport, RecoveredOrRejectedPartialPublication,
    UnacknowledgedPublicationOutcome,
};

pub(super) fn classify_ambiguity(ambiguity_digest: &str) -> PartialPublicationClassification {
    let report = AmbiguousPublicationReport::insufficient_persisted_evidence(ambiguity_digest);
    let counters = report.counters();
    classification(
        UnacknowledgedPublicationOutcome::Ambiguous,
        RecoveredOrRejectedPartialPublication::Ambiguous { report, counters },
        counters,
        ambiguity_digest,
    )
}
