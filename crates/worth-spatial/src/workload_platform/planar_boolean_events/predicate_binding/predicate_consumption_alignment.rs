use std::collections::{BTreeMap, BTreeSet};

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
    let required_rows = segment_receipts.len().saturating_mul(4);
    let row_summaries = predicate_row_summaries_by_segment_receipt(predicate_consumption);
    let segment_contact_rows = row_summaries
        .values()
        .map(|summary| summary.row_count)
        .sum::<usize>();
    if segment_contact_rows != required_rows
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
            &row_summaries,
            segment_receipt,
        )?;
    }
    Ok(())
}

fn predicate_row_summaries_by_segment_receipt(
    predicate_consumption: &PredicateCertificateConsumptionReceipt,
) -> BTreeMap<String, SegmentPredicateRowSummary> {
    let mut summaries = BTreeMap::<String, SegmentPredicateRowSummary>::new();
    for row in predicate_consumption
        .basis()
        .consumption_rows()
        .iter()
        .filter(|row| row.consumer_kind() == PredicateCertificateConsumerKind::SegmentContact)
    {
        summaries
            .entry(row.consumer_fact_digest().to_string())
            .or_default()
            .record(row.local_frame_identity(), row.tolerance_policy_identity());
    }
    summaries
}

#[derive(Debug, Default, Eq, PartialEq)]
struct SegmentPredicateRowSummary {
    row_count: usize,
    local_frame_identities: BTreeSet<String>,
    tolerance_policy_identities: BTreeSet<String>,
}

impl SegmentPredicateRowSummary {
    fn record(&mut self, local_frame_identity: &str, tolerance_policy_identity: &str) {
        self.row_count += 1;
        self.local_frame_identities
            .insert(local_frame_identity.to_string());
        self.tolerance_policy_identities
            .insert(tolerance_policy_identity.to_string());
    }

    fn matches_local_frame(&self, local_frame_identity: &str) -> bool {
        self.local_frame_identities.len() == 1
            && self.local_frame_identities.contains(local_frame_identity)
    }

    fn matches_tolerance_policy(&self, tolerance_policy_identity: &str) -> bool {
        self.tolerance_policy_identities.len() == 1
            && self
                .tolerance_policy_identities
                .contains(tolerance_policy_identity)
    }
}

fn validate_rows_for_segment(
    reduced_pair_identity: &str,
    pair_enumeration: &PlanarBooleanSegmentPairEnumerationReceipt,
    segment_receipts: &[CertifiedSegmentSegment2DReceipt],
    predicate_consumption: &PredicateCertificateConsumptionReceipt,
    row_summaries: &BTreeMap<String, SegmentPredicateRowSummary>,
    segment_receipt: &CertifiedSegmentSegment2DReceipt,
) -> Result<(), PlanarBooleanEventPredicateBindingDenial> {
    let Some(summary) = row_summaries.get(segment_receipt.fact_digest()) else {
        return Err(denial(
            PlanarBooleanEventPredicateBindingDenialKind::PredicateConsumptionRowCountMismatch,
            reduced_pair_identity,
            "",
            counters(pair_enumeration, segment_receipts, predicate_consumption),
            "each segment-segment receipt must contribute exactly four predicate rows",
        ));
    };
    if summary.row_count != 4 {
        return Err(denial(
            PlanarBooleanEventPredicateBindingDenialKind::PredicateConsumptionRowCountMismatch,
            reduced_pair_identity,
            "",
            counters(pair_enumeration, segment_receipts, predicate_consumption),
            "each segment-segment receipt must contribute exactly four predicate rows",
        ));
    }
    if !summary.matches_local_frame(segment_receipt.basis().frame_identity()) {
        return Err(denial(
            PlanarBooleanEventPredicateBindingDenialKind::PredicateConsumptionLocalFrameMismatch,
            reduced_pair_identity,
            "",
            counters(pair_enumeration, segment_receipts, predicate_consumption),
            "predicate-consumption row local frame must match its segment-segment receipt",
        ));
    }
    if !summary.matches_tolerance_policy(segment_receipt.basis().tolerance_policy_identity()) {
        return Err(denial(
            PlanarBooleanEventPredicateBindingDenialKind::PredicateConsumptionPrecisionBasisMismatch,
            reduced_pair_identity,
            "",
            counters(pair_enumeration, segment_receipts, predicate_consumption),
            "predicate-consumption row precision basis must match its segment-segment receipt",
        ));
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
