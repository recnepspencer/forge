use std::collections::BTreeMap;

use worth_spatial::facade::planar_predicates::PlanarPredicateFactReceipt;
use worth_spatial::facade::planar_segment_segment::CertifiedSegmentSegment2DReceipt;

pub(crate) fn unique_orientation_predicates_from_segment_receipts(
    segments: &[CertifiedSegmentSegment2DReceipt],
) -> Vec<PlanarPredicateFactReceipt> {
    let mut receipts = BTreeMap::new();
    for segment in segments {
        for receipt in segment.orientation_predicate_receipts() {
            receipts.insert(receipt.fact_digest().to_string(), receipt);
        }
    }
    receipts.into_values().cloned().collect()
}
