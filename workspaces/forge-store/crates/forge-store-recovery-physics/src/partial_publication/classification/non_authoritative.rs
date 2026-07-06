use super::assembly::classification;
use super::PartialPublicationClassification;
use crate::partial_publication::{
    NonAuthoritativePublicationDenial, NonAuthoritativePublicationSource,
    PartialPublicationCounterSnapshot, RecoveredOrRejectedPartialPublication,
    UnacknowledgedPublicationOutcome,
};

pub(super) fn reject_non_authoritative_promotion(
    outcome: UnacknowledgedPublicationOutcome,
    source: NonAuthoritativePublicationSource,
    counters: PartialPublicationCounterSnapshot,
    digest: &str,
) -> PartialPublicationClassification {
    classification(
        outcome,
        RecoveredOrRejectedPartialPublication::RejectedNonAuthoritativePromotion {
            denial: NonAuthoritativePublicationDenial::new(source, digest),
            counters,
        },
        counters,
        digest,
    )
}