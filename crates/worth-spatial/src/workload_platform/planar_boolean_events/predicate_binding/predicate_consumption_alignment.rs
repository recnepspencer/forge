use std::collections::BTreeSet;

use crate::planar_contracts::predicate_consumption::{
    PredicateCertificateConsumerKind, PredicateCertificateConsumptionReceipt,
};
use crate::planar_contracts::segment_segment_2d::CertifiedSegmentSegment2DReceipt;
use crate::workload_platform::planar_boolean_events::PlanarBooleanSegmentPairEnumerationReceipt;

use super::counters::PlanarBooleanEventPredicateBindingCounters;
use super::denial::{
    PlanarBooleanEventPredicateBindingDenial, PlanarBooleanEventPredicateBindingDenialKind,
};

pub(crate) fn validate_predicate_consumption_alignment(
    reduced_pair_identity: &str,
    pair_enumeration: &PlanarBooleanSegmentPairEnumerationReceipt,
    segment_receipts: &[CertifiedSegmentSegment2DReceipt],
    predicate_consumption: &PredicateCertificateConsumptionReceipt,
) -> Result<(), PlanarBooleanEventPredicateBindingDenial> {
    let counters = counters(pair_enumeration, segment_receipts, predicate_consumption);
    if !predicate_consumption.proves_no_second_predicate_engine() {
        return Err(denial(
            PlanarBooleanEventPredicateBindingDenialKind::PredicateConsumptionMissingNoSecondEngineProof,
            reduced_pair_identity,
            "",
            counters,
            "predicate binding requires consumption proof that no second predicate engine was used",
        ));
    }
    validate_segment_receipt_set(
        reduced_pair_identity,
        pair_enumeration,
        segment_receipts,
        predicate_consumption,
    )?;
    validate_consumption_rows(
        reduced_pair_identity,
        pair_enumeration,
        segment_receipts,
        predicate_consumption,
    )
}

fn validate_segment_receipt_set(
    reduced_pair_identity: &str,
    pair_enumeration: &PlanarBooleanSegmentPairEnumerationReceipt,
    segment_receipts: &[CertifiedSegmentSegment2DReceipt],
    predicate_consumption: &PredicateCertificateConsumptionReceipt,
) -> Result<(), PlanarBooleanEventPredicateBindingDenial> {
    let supplied = segment_receipts
        .iter()
        .map(|receipt| receipt.fact_digest())
        .collect::<BTreeSet<_>>();
    let consumed = predicate_consumption
        .basis()
        .segment_receipts()
        .iter()
        .map(|receipt| receipt.fact_digest())
        .collect::<BTreeSet<_>>();
    if supplied == consumed {
        Ok(())
    } else {
        Err(denial(
            PlanarBooleanEventPredicateBindingDenialKind::PredicateConsumptionSegmentSetMismatch,
            reduced_pair_identity,
            "",
            counters(pair_enumeration, segment_receipts, predicate_consumption),
            "predicate-consumption receipt must consume exactly the segment receipts being bound",
        ))
    }
}

fn validate_consumption_rows(
    reduced_pair_identity: &str,
    pair_enumeration: &PlanarBooleanSegmentPairEnumerationReceipt,
    segment_receipts: &[CertifiedSegmentSegment2DReceipt],
    predicate_consumption: &PredicateCertificateConsumptionReceipt,
) -> Result<(), PlanarBooleanEventPredicateBindingDenial> {
    let rows = predicate_consumption.basis().consumption_rows();
    let required_rows = segment_receipts.len().saturating_mul(4);
    let segment_contact_rows = rows
        .iter()
        .filter(|row| row.consumer_kind() == PredicateCertificateConsumerKind::SegmentContact)
        .collect::<Vec<_>>();
    if segment_contact_rows.len() != required_rows
        || predicate_consumption.certified_predicate_rows() != required_rows
    {
        return Err(denial(
            PlanarBooleanEventPredicateBindingDenialKind::PredicateConsumptionRowCountMismatch,
            reduced_pair_identity,
            "",
            counters(pair_enumeration, segment_receipts, predicate_consumption),
            "predicate-consumption receipt must certify four predicate rows per segment-pair receipt",
        ));
    }

    for segment_receipt in segment_receipts {
        validate_rows_for_segment(
            reduced_pair_identity,
            pair_enumeration,
            segment_receipts,
            predicate_consumption,
            segment_receipt,
        )?;
    }
    Ok(())
}

fn validate_rows_for_segment(
    reduced_pair_identity: &str,
    pair_enumeration: &PlanarBooleanSegmentPairEnumerationReceipt,
    segment_receipts: &[CertifiedSegmentSegment2DReceipt],
    predicate_consumption: &PredicateCertificateConsumptionReceipt,
    segment_receipt: &CertifiedSegmentSegment2DReceipt,
) -> Result<(), PlanarBooleanEventPredicateBindingDenial> {
    let matching_rows = predicate_consumption
        .basis()
        .consumption_rows()
        .iter()
        .filter(|row| row.consumer_fact_digest() == segment_receipt.fact_digest())
        .collect::<Vec<_>>();
    if matching_rows.len() != 4 {
        return Err(denial(
            PlanarBooleanEventPredicateBindingDenialKind::PredicateConsumptionRowCountMismatch,
            reduced_pair_identity,
            "",
            counters(pair_enumeration, segment_receipts, predicate_consumption),
            "each segment-segment receipt must contribute exactly four predicate rows",
        ));
    }
    for row in matching_rows {
        if row.local_frame_identity() != segment_receipt.basis().frame_identity() {
            return Err(denial(
                PlanarBooleanEventPredicateBindingDenialKind::PredicateConsumptionLocalFrameMismatch,
                reduced_pair_identity,
                "",
                counters(pair_enumeration, segment_receipts, predicate_consumption),
                "predicate-consumption row local frame must match its segment-segment receipt",
            ));
        }
        if row.tolerance_policy_identity() != segment_receipt.basis().tolerance_policy_identity() {
            return Err(denial(
                PlanarBooleanEventPredicateBindingDenialKind::PredicateConsumptionPrecisionBasisMismatch,
                reduced_pair_identity,
                "",
                counters(pair_enumeration, segment_receipts, predicate_consumption),
                "predicate-consumption row precision basis must match its segment-segment receipt",
            ));
        }
    }
    Ok(())
}

fn counters(
    pair_enumeration: &PlanarBooleanSegmentPairEnumerationReceipt,
    segment_receipts: &[CertifiedSegmentSegment2DReceipt],
    predicate_consumption: &PredicateCertificateConsumptionReceipt,
) -> PlanarBooleanEventPredicateBindingCounters {
    PlanarBooleanEventPredicateBindingCounters::new(
        pair_enumeration.candidate_rows().len(),
        segment_receipts.len(),
        0,
        predicate_consumption.certified_predicate_rows(),
    )
}

fn denial(
    kind: PlanarBooleanEventPredicateBindingDenialKind,
    reduced_pair_identity: impl Into<String>,
    segment_pair_identity: impl Into<String>,
    counters: PlanarBooleanEventPredicateBindingCounters,
    human_reason: impl Into<String>,
) -> PlanarBooleanEventPredicateBindingDenial {
    PlanarBooleanEventPredicateBindingDenial::new(
        kind,
        reduced_pair_identity,
        segment_pair_identity,
        counters,
        human_reason,
    )
}
