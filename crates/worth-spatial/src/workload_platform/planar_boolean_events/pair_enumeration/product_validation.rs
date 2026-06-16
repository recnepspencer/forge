use super::denial::{
    PlanarBooleanSegmentPairEnumerationDenial, PlanarBooleanSegmentPairEnumerationDenialKind,
};
use super::product::{
    PlanarBooleanCandidateIndexFallbackPosture, PlanarBooleanSegmentCandidateIndexProductInput,
};
use std::collections::HashSet;

pub(crate) fn validate_candidate_index_product_input(
    input: &PlanarBooleanSegmentCandidateIndexProductInput,
) -> Result<(), PlanarBooleanSegmentPairEnumerationDenial> {
    let counters = input.counters;
    if candidate_rows_match_counters(input)
        && culled_pairs_reconcile(input)
        && skipped_pairs_reconcile(input)
        && fallback_posture_matches_counters(input)
        && candidate_row_identities_are_unique(input)
    {
        return Ok(());
    }

    Err(PlanarBooleanSegmentPairEnumerationDenial::new(
        PlanarBooleanSegmentPairEnumerationDenialKind::EmittedPairBreadthMismatch,
        input.canonical_segment_set_identity.clone(),
        counters,
        "candidate-index product rows, counters, culls, skips, or fallback posture are inconsistent",
    ))
}

fn candidate_rows_match_counters(input: &PlanarBooleanSegmentCandidateIndexProductInput) -> bool {
    let row_count = input.rows.len();
    input.counters.emitted_pair_breadth() == row_count
        && input.counters.query_index_candidate_count() == row_count
}

fn culled_pairs_reconcile(input: &PlanarBooleanSegmentCandidateIndexProductInput) -> bool {
    input
        .rows
        .len()
        .saturating_add(input.counters.query_index_culled_pair_count())
        == input.counters.expected_pair_breadth()
}

fn skipped_pairs_reconcile(input: &PlanarBooleanSegmentCandidateIndexProductInput) -> bool {
    input
        .counters
        .emitted_pair_breadth()
        .saturating_add(input.counters.skipped_pair_count())
        == input.counters.expected_pair_breadth()
}

fn fallback_posture_matches_counters(
    input: &PlanarBooleanSegmentCandidateIndexProductInput,
) -> bool {
    match input.fallback_posture {
        PlanarBooleanCandidateIndexFallbackPosture::NotUsed => !input.counters.fallback_used(),
        PlanarBooleanCandidateIndexFallbackPosture::FullBreadthNonProduction => {
            input.counters.fallback_used()
        }
    }
}

fn candidate_row_identities_are_unique(
    input: &PlanarBooleanSegmentCandidateIndexProductInput,
) -> bool {
    let mut identities = HashSet::with_capacity(input.rows.len());
    input
        .rows
        .iter()
        .all(|row| identities.insert(row.candidate_identity()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
    use crate::workload_platform::planar_boolean_events::endpoint_normalization::{
        normalize_endpoint_order, validate_segment_endpoint_admissibility,
    };
    use crate::workload_platform::planar_boolean_events::pair_enumeration::{
        PlanarBooleanCandidateBroadPhaseReason, PlanarBooleanCandidateEnvelopeBasis,
        PlanarBooleanCandidateIndexLifecycleOutcome, PlanarBooleanCandidateIndexStrategy,
        PlanarBooleanSegmentCandidateIndexProduct, PlanarBooleanSegmentPairEnumerationCounters,
    };
    use crate::workload_platform::planar_boolean_events::segment_carriers::{
        PlanarBooleanSegmentCarrier, PlanarBooleanSegmentCarrierEndpointFacts,
    };
    use crate::workload_platform::planar_boolean_events::segment_identity::PlanarBooleanCanonicalSegment;

    #[test]
    fn candidate_index_fallback_posture_cannot_certify_production_discovery() {
        let product = PlanarBooleanSegmentCandidateIndexProduct::new(
            PlanarBooleanSegmentCandidateIndexProductInput {
                canonical_segment_set_identity: "canonical-segment-set".to_string(),
                declaration_digest: "declaration".to_string(),
                plan_digest: "plan".to_string(),
                envelope_digest: "envelope".to_string(),
                strategy: PlanarBooleanCandidateIndexStrategy::AabbSweep,
                fallback_posture:
                    PlanarBooleanCandidateIndexFallbackPosture::FullBreadthNonProduction,
                lifecycle_outcome: PlanarBooleanCandidateIndexLifecycleOutcome::Bound,
                counters: PlanarBooleanSegmentPairEnumerationCounters::new(2, 2, 0, 4)
                    .with_strategy_counts(0, 4, 0, true),
                rows: Vec::new(),
            },
        )
        .expect("fallback products may exist only with internally coherent counters");

        assert!(!product.certifies_production_candidate_discovery());
    }

    #[test]
    fn candidate_index_product_rejects_row_counter_mismatch() {
        let denial = PlanarBooleanSegmentCandidateIndexProduct::new(
            PlanarBooleanSegmentCandidateIndexProductInput {
                canonical_segment_set_identity: "canonical-segment-set".to_string(),
                declaration_digest: "declaration".to_string(),
                plan_digest: "plan".to_string(),
                envelope_digest: "envelope".to_string(),
                strategy: PlanarBooleanCandidateIndexStrategy::AabbSweep,
                fallback_posture: PlanarBooleanCandidateIndexFallbackPosture::NotUsed,
                lifecycle_outcome: PlanarBooleanCandidateIndexLifecycleOutcome::Bound,
                counters: PlanarBooleanSegmentPairEnumerationCounters::new(2, 2, 4, 0)
                    .with_strategy_counts(4, 4, 0, false),
                rows: Vec::new(),
            },
        )
        .expect_err("candidate index products must not certify counters without rows");

        assert_eq!(
            denial.kind(),
            PlanarBooleanSegmentPairEnumerationDenialKind::EmittedPairBreadthMismatch
        );
    }

    #[test]
    fn candidate_index_product_rejects_duplicate_candidate_row_identities() {
        let left = canonical_segment_from_side_edge_and_points(
            PlanarBooleanCommonPlaneOperandSide::Left,
            "left-edge",
            [0.0, 0.0],
            [1.0, 0.0],
        );
        let right = canonical_segment_from_side_edge_and_points(
            PlanarBooleanCommonPlaneOperandSide::Right,
            "right-edge",
            [0.5, -1.0],
            [0.5, 1.0],
        );
        let envelope = PlanarBooleanCandidateEnvelopeBasis::from_segments(&left, &right)
            .expect("finite segments have envelopes");
        let row = super::super::product::PlanarBooleanSegmentCandidateRowReceipt::new(
            left,
            right,
            PlanarBooleanCandidateBroadPhaseReason::AabbEnvelopeOverlap,
            envelope,
        )
        .expect("left/right row should bind");

        let denial = PlanarBooleanSegmentCandidateIndexProduct::new(
            PlanarBooleanSegmentCandidateIndexProductInput {
                canonical_segment_set_identity: "canonical-segment-set".to_string(),
                declaration_digest: "declaration".to_string(),
                plan_digest: "plan".to_string(),
                envelope_digest: "envelope".to_string(),
                strategy: PlanarBooleanCandidateIndexStrategy::AabbSweep,
                fallback_posture: PlanarBooleanCandidateIndexFallbackPosture::NotUsed,
                lifecycle_outcome: PlanarBooleanCandidateIndexLifecycleOutcome::Bound,
                counters: PlanarBooleanSegmentPairEnumerationCounters::new(1, 2, 2, 0)
                    .with_strategy_counts(2, 2, 0, false),
                rows: vec![row.clone(), row],
            },
        )
        .expect_err("candidate index products must reject duplicated canonical rows");

        assert_eq!(
            denial.kind(),
            PlanarBooleanSegmentPairEnumerationDenialKind::EmittedPairBreadthMismatch
        );
    }

    fn canonical_segment_from_side_edge_and_points(
        side: PlanarBooleanCommonPlaneOperandSide,
        source_edge_identity: impl Into<String>,
        start: [f64; 2],
        end: [f64; 2],
    ) -> PlanarBooleanCanonicalSegment {
        let carrier =
            PlanarBooleanSegmentCarrier::for_canonical_segment_test_on_side_with_source_edge(
                side,
                source_edge_identity,
                test_endpoint_for_point(start),
                test_endpoint_for_point(end),
            );
        validate_segment_endpoint_admissibility(&carrier).expect("test carrier is admissible");
        PlanarBooleanCanonicalSegment::from_carrier(&carrier, normalize_endpoint_order(&carrier))
    }

    fn test_endpoint_for_point(point: [f64; 2]) -> PlanarBooleanSegmentCarrierEndpointFacts {
        PlanarBooleanSegmentCarrierEndpointFacts::from_projected_loop_boundary(
            point,
            format!("source endpoint {point:?}"),
            "test projected loop",
            "test projection stage",
            "test projection local basis",
        )
    }
}
