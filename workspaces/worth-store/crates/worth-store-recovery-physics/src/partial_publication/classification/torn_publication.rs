use super::assembly::classification;
use super::PartialPublicationClassification;
use crate::partial_publication::{
    PartialPublicationCounterSnapshot, RecoveredOrRejectedPartialPublication,
    TornPublicationDenial, UnacknowledgedPublicationOutcome,
};

pub(super) fn reject_torn_publication(
    denial: TornPublicationDenial,
    digest: &str,
) -> PartialPublicationClassification {
    let counters = PartialPublicationCounterSnapshot::default().with_torn_publication_denial();
    classification(
        UnacknowledgedPublicationOutcome::TornPublicationRejected,
        RecoveredOrRejectedPartialPublication::RejectedTornPublication { denial, counters },
        counters,
        digest,
    )
}
