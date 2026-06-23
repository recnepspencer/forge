use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanCollinearRelationReceipt, PlanarBooleanEventPredicateBinding,
    PlanarBooleanIntervalEventExtractionReceipt, PlanarBooleanPointEventExtractionReceipt,
    PlanarBooleanSegmentPairEnumerationReceipt,
};

use super::denial::{PlanarBooleanEventLedgerDenial, PlanarBooleanEventLedgerDenialKind};

pub(crate) struct EventLedgerReceiptChain<'a> {
    pub(crate) reduced_pair_identity: &'a str,
    pub(crate) segment_pair_enumeration: &'a PlanarBooleanSegmentPairEnumerationReceipt,
    pub(crate) predicate_binding: &'a PlanarBooleanEventPredicateBinding,
    pub(crate) point_events: &'a PlanarBooleanPointEventExtractionReceipt,
    pub(crate) collinear_relations: &'a PlanarBooleanCollinearRelationReceipt,
    pub(crate) interval_events: &'a PlanarBooleanIntervalEventExtractionReceipt,
}

pub(crate) fn validate_receipt_chain(
    chain: EventLedgerReceiptChain<'_>,
) -> Result<(), PlanarBooleanEventLedgerDenial> {
    if chain.predicate_binding.reduced_pair_identity() != chain.reduced_pair_identity {
        return Err(denial(
            PlanarBooleanEventLedgerDenialKind::MismatchedReducedPairForPredicateBinding,
            chain.predicate_binding.reduced_pair_identity(),
            "event ledger predicate binding must belong to the requested reduced pair",
        ));
    }
    if chain.predicate_binding.segment_pair_enumeration_identity()
        != chain
            .segment_pair_enumeration
            .segment_pair_enumeration_identity()
    {
        return Err(denial(
            PlanarBooleanEventLedgerDenialKind::MismatchedSegmentPairEnumeration,
            chain.predicate_binding.segment_pair_enumeration_identity(),
            "event ledger predicate binding must consume the supplied segment-pair enumeration",
        ));
    }
    if chain.point_events.predicate_binding_identity()
        != chain.predicate_binding.predicate_binding_identity()
    {
        return Err(denial(
            PlanarBooleanEventLedgerDenialKind::MismatchedPredicateBindingForPointEvents,
            chain.point_events.predicate_binding_identity(),
            "event ledger point events must come from the supplied predicate binding",
        ));
    }
    if chain.collinear_relations.predicate_binding_identity()
        != chain.predicate_binding.predicate_binding_identity()
    {
        return Err(denial(
            PlanarBooleanEventLedgerDenialKind::MismatchedPredicateBindingForCollinearRelations,
            chain.collinear_relations.predicate_binding_identity(),
            "event ledger collinear relations must come from the supplied predicate binding",
        ));
    }
    if chain.interval_events.collinear_relation_receipt_identity()
        != chain.collinear_relations.receipt_identity()
    {
        return Err(denial(
            PlanarBooleanEventLedgerDenialKind::MismatchedCollinearRelationReceiptForIntervals,
            chain.interval_events.collinear_relation_receipt_identity(),
            "event ledger interval events must come from the supplied collinear relation receipt",
        ));
    }
    Ok(())
}

fn denial(
    kind: PlanarBooleanEventLedgerDenialKind,
    evidence_identity: impl Into<String>,
    human_reason: impl Into<String>,
) -> PlanarBooleanEventLedgerDenial {
    PlanarBooleanEventLedgerDenial::new(kind, evidence_identity, human_reason)
}
