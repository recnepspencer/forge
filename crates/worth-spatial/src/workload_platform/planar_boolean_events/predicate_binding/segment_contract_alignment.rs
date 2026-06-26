use std::collections::BTreeMap;

use crate::planar_contracts::segment_segment_2d::CertifiedSegmentSegment2DReceipt;
use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanSegmentCandidateRowReceipt, PlanarBooleanSegmentPairEnumerationReceipt,
};

use super::bound_pair::PlanarBooleanPredicateBoundPair;
use super::counters::PlanarBooleanEventPredicateBindingCounters;
use super::denial::{
    PlanarBooleanEventPredicateBindingDenial, PlanarBooleanEventPredicateBindingDenialKind,
};

pub(crate) fn aligned_segment_contracts(
    reduced_pair_identity: &str,
    pair_enumeration: &PlanarBooleanSegmentPairEnumerationReceipt,
    segment_receipts: &[CertifiedSegmentSegment2DReceipt],
    predicate_consumption_fact_digest: &str,
) -> Result<Vec<PlanarBooleanPredicateBoundPair>, PlanarBooleanEventPredicateBindingDenial> {
    let counters = counters(pair_enumeration, segment_receipts, 0, 0);
    if pair_enumeration.candidate_rows().is_empty() {
        return Err(denial(
            PlanarBooleanEventPredicateBindingDenialKind::EmptyPairWorklist,
            reduced_pair_identity,
            "",
            counters,
            "predicate binding requires a non-empty segment-pair worklist",
        ));
    }
    if pair_enumeration.candidate_rows().len() != segment_receipts.len() {
        return Err(denial(
            PlanarBooleanEventPredicateBindingDenialKind::SegmentContractCountMismatch,
            reduced_pair_identity,
            "",
            counters,
            "every segment-pair work item requires exactly one segment-segment receipt",
        ));
    }

    let receipt_map =
        segment_receipt_map(reduced_pair_identity, pair_enumeration, segment_receipts)?;
    let mut bound_pairs = Vec::with_capacity(pair_enumeration.candidate_rows().len());
    for candidate_row in pair_enumeration.candidate_rows() {
        let Some(segment_receipt) = receipt_map.get(candidate_row.candidate_identity()) else {
            return Err(denial(
                PlanarBooleanEventPredicateBindingDenialKind::MissingSegmentContractForPair,
                reduced_pair_identity,
                candidate_row.candidate_identity(),
                counters,
                "candidate row is missing its certified segment-segment receipt",
            ));
        };
        validate_segment_receipt_scope(
            reduced_pair_identity,
            pair_enumeration,
            segment_receipts,
            candidate_row,
            segment_receipt,
        )?;
        bound_pairs.push(PlanarBooleanPredicateBoundPair::new(
            reduced_pair_identity,
            candidate_row,
            segment_receipt,
            predicate_consumption_fact_digest,
        ));
    }
    Ok(bound_pairs)
}

fn segment_receipt_map<'a>(
    reduced_pair_identity: &str,
    pair_enumeration: &PlanarBooleanSegmentPairEnumerationReceipt,
    segment_receipts: &'a [CertifiedSegmentSegment2DReceipt],
) -> Result<
    BTreeMap<String, &'a CertifiedSegmentSegment2DReceipt>,
    PlanarBooleanEventPredicateBindingDenial,
> {
    let candidate_identity_by_segments = candidate_identity_by_segment_pair(pair_enumeration);
    let mut map = BTreeMap::new();
    for receipt in segment_receipts {
        let key =
            matching_pair_identity(&candidate_identity_by_segments, receipt).ok_or_else(|| {
                denial(
                    PlanarBooleanEventPredicateBindingDenialKind::SegmentContractIdentityMismatch,
                    reduced_pair_identity,
                    "",
                    counters(pair_enumeration, segment_receipts, 0, 0),
                    "segment-segment receipt identities do not match any enumerated segment pair",
                )
            })?;
        if map.insert(key.clone(), receipt).is_some() {
            return Err(denial(
                PlanarBooleanEventPredicateBindingDenialKind::DuplicateSegmentContractForPair,
                reduced_pair_identity,
                key,
                counters(pair_enumeration, segment_receipts, 0, 0),
                "segment-pair work item cannot be bound to duplicate segment-segment receipts",
            ));
        }
    }
    Ok(map)
}

fn candidate_identity_by_segment_pair(
    pair_enumeration: &PlanarBooleanSegmentPairEnumerationReceipt,
) -> BTreeMap<(String, String), String> {
    pair_enumeration
        .candidate_rows()
        .iter()
        .map(|candidate_row| {
            (
                (
                    candidate_row
                        .left()
                        .canonical_segment_identity()
                        .to_string(),
                    candidate_row
                        .right()
                        .canonical_segment_identity()
                        .to_string(),
                ),
                candidate_row.candidate_identity().to_string(),
            )
        })
        .collect()
}

fn matching_pair_identity(
    candidate_identity_by_segments: &BTreeMap<(String, String), String>,
    receipt: &CertifiedSegmentSegment2DReceipt,
) -> Option<String> {
    candidate_identity_by_segments
        .get(&(
            receipt.basis().first_segment_identity().to_string(),
            receipt.basis().second_segment_identity().to_string(),
        ))
        .cloned()
}

fn validate_segment_receipt_scope(
    reduced_pair_identity: &str,
    pair_enumeration: &PlanarBooleanSegmentPairEnumerationReceipt,
    segment_receipts: &[CertifiedSegmentSegment2DReceipt],
    candidate_row: &PlanarBooleanSegmentCandidateRowReceipt,
    receipt: &CertifiedSegmentSegment2DReceipt,
) -> Result<(), PlanarBooleanEventPredicateBindingDenial> {
    let segment_pair_identity = candidate_row.candidate_identity();
    let expected_local_frame = candidate_row.local_frame_identity();
    let expected_precision_basis = candidate_row.precision_basis_identity();

    if candidate_row.right().local_frame_identity() != expected_local_frame {
        return Err(denial(
            PlanarBooleanEventPredicateBindingDenialKind::SegmentContractLocalFrameMismatch,
            reduced_pair_identity,
            segment_pair_identity,
            counters(pair_enumeration, segment_receipts, 0, 0),
            "candidate row spans mismatched local-frame identities",
        ));
    }
    if candidate_row.right().precision_basis_identity() != expected_precision_basis {
        return Err(denial(
            PlanarBooleanEventPredicateBindingDenialKind::SegmentContractPrecisionBasisMismatch,
            reduced_pair_identity,
            segment_pair_identity,
            counters(pair_enumeration, segment_receipts, 0, 0),
            "candidate row spans mismatched precision-basis identities",
        ));
    }
    if receipt.basis().frame_identity() != expected_local_frame {
        return Err(denial(
            PlanarBooleanEventPredicateBindingDenialKind::SegmentContractLocalFrameMismatch,
            reduced_pair_identity,
            segment_pair_identity,
            counters(pair_enumeration, segment_receipts, 0, 0),
            "segment-segment receipt local-frame identity must match the candidate row",
        ));
    }
    if receipt.basis().tolerance_policy_identity() != expected_precision_basis {
        return Err(denial(
            PlanarBooleanEventPredicateBindingDenialKind::SegmentContractPrecisionBasisMismatch,
            reduced_pair_identity,
            segment_pair_identity,
            counters(pair_enumeration, segment_receipts, 0, 0),
            "segment-segment receipt precision basis must match the candidate row",
        ));
    }
    Ok(())
}

fn counters(
    pair_enumeration: &PlanarBooleanSegmentPairEnumerationReceipt,
    segment_receipts: &[CertifiedSegmentSegment2DReceipt],
    bound_segment_pairs: usize,
    certified_predicate_rows: usize,
) -> PlanarBooleanEventPredicateBindingCounters {
    PlanarBooleanEventPredicateBindingCounters::new(
        pair_enumeration.candidate_rows().len(),
        segment_receipts.len(),
        bound_segment_pairs,
        certified_predicate_rows,
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
