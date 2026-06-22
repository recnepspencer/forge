use crate::workload_platform::planar_boolean_events::endpoint_normalization::{
    normalize_endpoint_order, validate_segment_endpoint_admissibility,
};
use crate::workload_platform::planar_boolean_events::segment_carriers::{
    PlanarBooleanSegmentCarrier, PlanarBooleanSegmentCarrierSet,
};
use crate::workload_platform::planar_boolean_events::{
    enumerate_segment_pairs, PlanarBooleanSegmentPairEnumerationDenial,
    PlanarBooleanSegmentPairEnumerationReceipt,
};

use super::canonical_segment::PlanarBooleanCanonicalSegment;
use super::denial::PlanarBooleanCanonicalSegmentSetDenial;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanCanonicalSegmentSet {
    left: Vec<PlanarBooleanCanonicalSegment>,
    right: Vec<PlanarBooleanCanonicalSegment>,
}

impl PlanarBooleanCanonicalSegmentSet {
    pub(crate) fn from_carrier_set(
        carrier_set: &PlanarBooleanSegmentCarrierSet,
    ) -> Result<Self, PlanarBooleanCanonicalSegmentSetDenial> {
        Ok(Self {
            left: canonicalize_carriers(carrier_set.left())?,
            right: canonicalize_carriers(carrier_set.right())?,
        })
    }

    pub fn left(&self) -> &[PlanarBooleanCanonicalSegment] {
        &self.left
    }

    pub fn right(&self) -> &[PlanarBooleanCanonicalSegment] {
        &self.right
    }

    pub fn total_segment_count(&self) -> usize {
        self.left.len() + self.right.len()
    }

    pub fn segment_pair_enumeration_receipt(
        &self,
    ) -> Result<PlanarBooleanSegmentPairEnumerationReceipt, PlanarBooleanSegmentPairEnumerationDenial>
    {
        enumerate_segment_pairs(&self.left, &self.right)
    }

    #[cfg(test)]
    pub(crate) fn for_pair_enumeration_test(
        left: Vec<PlanarBooleanCanonicalSegment>,
        right: Vec<PlanarBooleanCanonicalSegment>,
    ) -> Self {
        Self { left, right }
    }
}

fn canonicalize_carriers(
    carriers: &[PlanarBooleanSegmentCarrier],
) -> Result<Vec<PlanarBooleanCanonicalSegment>, PlanarBooleanCanonicalSegmentSetDenial> {
    let mut canonical_segments = Vec::with_capacity(carriers.len());
    for carrier in carriers {
        validate_segment_endpoint_admissibility(carrier)?;
        canonical_segments.push(PlanarBooleanCanonicalSegment::from_carrier(
            carrier,
            normalize_endpoint_order(carrier),
        ));
    }
    Ok(canonical_segments)
}

#[cfg(test)]
#[path = "pair_enumeration_tests.rs"]
mod pair_enumeration_tests;

#[cfg(test)]
mod tests {
    use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
    use crate::workload_platform::planar_boolean_events::segment_carriers::{
        PlanarBooleanSegmentCarrier, PlanarBooleanSegmentCarrierEndpointFacts,
    };

    use super::*;
    use crate::workload_platform::planar_boolean_events::segment_identity::PlanarBooleanCanonicalSegmentSetDenialKind;

    #[test]
    fn canonical_segment_identity_is_stable_under_endpoint_order_reversal() {
        let forward = canonical_segment_from_points([0.0, 0.0], [2.0, 0.0])
            .expect("forward segment should canonicalize");
        let reversed = canonical_segment_from_points([2.0, 0.0], [0.0, 0.0])
            .expect("reversed segment should canonicalize");

        assert_ne!(forward.carrier_identity(), reversed.carrier_identity());
        assert_eq!(
            forward.canonical_segment_identity(),
            reversed.canonical_segment_identity()
        );
        assert!(!forward.orientation_was_reversed());
        assert!(reversed.orientation_was_reversed());
    }

    #[test]
    fn endpoint_normalization_rejects_zero_length_or_collapsed_segments() {
        let denial = canonical_segment_from_points([1.0, 1.0], [1.0, 1.0])
            .expect_err("collapsed projected segment must deny before canonical construction");

        assert_eq!(
            denial.kind(),
            PlanarBooleanCanonicalSegmentSetDenialKind::CollapsedProjectedSegment
        );
        assert_denial_preserves_carrier_provenance(&denial);
    }

    #[test]
    fn endpoint_normalization_rejects_non_finite_projected_endpoint() {
        let denial = canonical_segment_from_points([f64::NAN, 1.0], [1.0, 1.0])
            .expect_err("non-finite endpoints must deny before canonical construction");

        assert_eq!(
            denial.kind(),
            PlanarBooleanCanonicalSegmentSetDenialKind::NonFiniteEndpointCoordinate
        );
        assert_denial_preserves_carrier_provenance(&denial);
    }

    #[test]
    fn canonical_segment_identity_uses_endpoint_facts_not_coordinate_rendering() {
        let baseline = canonical_segment_from_distinct_points_with_shared_endpoint_fact_identity(
            [0.0, 0.0],
            [2.0, 0.0],
        )
        .expect("baseline segment should canonicalize");
        let coordinate_variant =
            canonical_segment_from_distinct_points_with_shared_endpoint_fact_identity(
                [10.25, -3.5],
                [10.25, 9.75],
            )
            .expect("coordinate variant should canonicalize");

        assert_ne!(
            baseline.normalized_endpoints().low().point(),
            coordinate_variant.normalized_endpoints().low().point()
        );
        assert_ne!(
            baseline.normalized_endpoints().high().point(),
            coordinate_variant.normalized_endpoints().high().point()
        );
        assert_eq!(
            baseline.canonical_segment_identity(),
            coordinate_variant.canonical_segment_identity()
        );
    }

    #[test]
    fn segment_pair_enumeration_is_deterministic_under_input_order_variation() {
        let left_low = canonical_segment_from_side_and_points(
            PlanarBooleanCommonPlaneOperandSide::Left,
            [0.0, 0.0],
            [1.0, 0.0],
        )
        .expect("left low should canonicalize");
        let left_high = canonical_segment_from_side_and_points(
            PlanarBooleanCommonPlaneOperandSide::Left,
            [2.0, 0.0],
            [3.0, 0.0],
        )
        .expect("left high should canonicalize");
        let right_low = canonical_segment_from_side_and_points(
            PlanarBooleanCommonPlaneOperandSide::Right,
            [0.0, 1.0],
            [1.0, 1.0],
        )
        .expect("right low should canonicalize");
        let right_high = canonical_segment_from_side_and_points(
            PlanarBooleanCommonPlaneOperandSide::Right,
            [2.0, 1.0],
            [3.0, 1.0],
        )
        .expect("right high should canonicalize");

        let ordinary = PlanarBooleanCanonicalSegmentSet::for_pair_enumeration_test(
            vec![left_low.clone(), left_high.clone()],
            vec![right_low.clone(), right_high.clone()],
        )
        .segment_pair_enumeration_receipt()
        .expect("ordinary pair enumeration should certify");
        let reordered = PlanarBooleanCanonicalSegmentSet::for_pair_enumeration_test(
            vec![left_high, left_low],
            vec![right_high, right_low],
        )
        .segment_pair_enumeration_receipt()
        .expect("reordered pair enumeration should certify");

        assert_eq!(
            pair_identities(&ordinary),
            pair_identities(&reordered),
            "pair worklist ordering must be canonical, not input-vector order"
        );
        assert_eq!(
            ordinary.segment_pair_enumeration_identity(),
            reordered.segment_pair_enumeration_identity()
        );
    }

    #[test]
    fn segment_pair_enumeration_is_deterministic_when_canonical_segment_identities_tie() {
        let left_forward = canonical_segment_from_side_and_points(
            PlanarBooleanCommonPlaneOperandSide::Left,
            [0.0, 0.0],
            [1.0, 0.0],
        )
        .expect("left forward should canonicalize");
        let left_reversed = canonical_segment_from_side_and_points(
            PlanarBooleanCommonPlaneOperandSide::Left,
            [1.0, 0.0],
            [0.0, 0.0],
        )
        .expect("left reversed should canonicalize");
        let right = canonical_segment_from_side_and_points(
            PlanarBooleanCommonPlaneOperandSide::Right,
            [0.0, 0.0],
            [1.0, 0.0],
        )
        .expect("right should canonicalize");
        let right_reversed = canonical_segment_from_side_and_points(
            PlanarBooleanCommonPlaneOperandSide::Right,
            [1.0, 0.0],
            [0.0, 0.0],
        )
        .expect("reversed right should canonicalize");

        assert_eq!(
            left_forward.canonical_segment_identity(),
            left_reversed.canonical_segment_identity(),
            "regression setup requires two carriers with the same canonical segment identity"
        );
        assert_ne!(
            left_forward.carrier_identity(),
            left_reversed.carrier_identity(),
            "carrier provenance remains distinct and must participate in ordering ties"
        );
        assert_eq!(
            right.canonical_segment_identity(),
            right_reversed.canonical_segment_identity(),
            "right-side duplicate canonical identities must also be tie-broken deterministically"
        );
        assert_ne!(
            right.carrier_identity(),
            right_reversed.carrier_identity(),
            "right-side carrier provenance remains distinct across reversed orientation"
        );

        let ordinary = PlanarBooleanCanonicalSegmentSet::for_pair_enumeration_test(
            vec![left_forward.clone(), left_reversed.clone()],
            vec![right.clone(), right_reversed.clone()],
        )
        .segment_pair_enumeration_receipt()
        .expect("ordinary tie pair enumeration should certify");
        let reordered = PlanarBooleanCanonicalSegmentSet::for_pair_enumeration_test(
            vec![left_reversed, left_forward],
            vec![right_reversed, right],
        )
        .segment_pair_enumeration_receipt()
        .expect("reordered tie pair enumeration should certify");

        assert_eq!(pair_identities(&ordinary), pair_identities(&reordered));
        assert_eq!(
            ordinary.segment_pair_enumeration_identity(),
            reordered.segment_pair_enumeration_identity()
        );
    }

    #[test]
    fn segment_pair_enumeration_rejects_operand_side_mismatch_before_receipt() {
        let left = canonical_segment_from_side_and_points(
            PlanarBooleanCommonPlaneOperandSide::Left,
            [0.0, 0.0],
            [1.0, 0.0],
        )
        .expect("left should canonicalize");
        let wrong_right = canonical_segment_from_side_and_points(
            PlanarBooleanCommonPlaneOperandSide::Left,
            [0.0, 1.0],
            [1.0, 1.0],
        )
        .expect("wrong-side right slot should canonicalize");

        let denial = PlanarBooleanCanonicalSegmentSet::for_pair_enumeration_test(
            vec![left],
            vec![wrong_right],
        )
        .segment_pair_enumeration_receipt()
        .expect_err("side mismatch must deny before receipt construction");

        assert_eq!(
            denial.kind(),
            crate::workload_platform::planar_boolean_events::PlanarBooleanSegmentPairEnumerationDenialKind::OperandSideMismatch
        );
        assert_eq!(denial.counters().emitted_pair_breadth(), 0);
    }

    fn assert_denial_preserves_carrier_provenance(denial: &PlanarBooleanCanonicalSegmentSetDenial) {
        assert!(!denial.carrier_identity().is_empty());
        assert_eq!(denial.local_frame_identity(), "test local frame");
        assert_eq!(denial.projection_stage_identity(), "test projection stage");
        assert_eq!(denial.precision_basis_identity(), "test precision basis");
    }

    fn canonical_segment_from_points(
        start: [f64; 2],
        end: [f64; 2],
    ) -> Result<PlanarBooleanCanonicalSegment, PlanarBooleanCanonicalSegmentSetDenial> {
        let carrier = test_carrier(start, end);
        validate_segment_endpoint_admissibility(&carrier)?;
        Ok(PlanarBooleanCanonicalSegment::from_carrier(
            &carrier,
            normalize_endpoint_order(&carrier),
        ))
    }

    fn canonical_segment_from_side_and_points(
        side: PlanarBooleanCommonPlaneOperandSide,
        start: [f64; 2],
        end: [f64; 2],
    ) -> Result<PlanarBooleanCanonicalSegment, PlanarBooleanCanonicalSegmentSetDenial> {
        let carrier = PlanarBooleanSegmentCarrier::for_canonical_segment_test_on_side(
            side,
            test_endpoint_for_point(start),
            test_endpoint_for_point(end),
        );
        validate_segment_endpoint_admissibility(&carrier)?;
        Ok(PlanarBooleanCanonicalSegment::from_carrier(
            &carrier,
            normalize_endpoint_order(&carrier),
        ))
    }

    fn pair_identities(receipt: &PlanarBooleanSegmentPairEnumerationReceipt) -> Vec<String> {
        receipt
            .work_items()
            .iter()
            .map(|work_item| work_item.segment_pair_identity().to_string())
            .collect()
    }

    fn canonical_segment_from_distinct_points_with_shared_endpoint_fact_identity(
        start: [f64; 2],
        end: [f64; 2],
    ) -> Result<PlanarBooleanCanonicalSegment, PlanarBooleanCanonicalSegmentSetDenial> {
        let carrier = PlanarBooleanSegmentCarrier::for_canonical_segment_test(
            PlanarBooleanSegmentCarrierEndpointFacts::for_canonical_segment_test(
                start,
                "source low",
                "stable projected low",
            ),
            PlanarBooleanSegmentCarrierEndpointFacts::for_canonical_segment_test(
                end,
                "source high",
                "stable projected high",
            ),
        );
        validate_segment_endpoint_admissibility(&carrier)?;
        Ok(PlanarBooleanCanonicalSegment::from_carrier(
            &carrier,
            normalize_endpoint_order(&carrier),
        ))
    }

    fn test_carrier(start: [f64; 2], end: [f64; 2]) -> PlanarBooleanSegmentCarrier {
        PlanarBooleanSegmentCarrier::for_canonical_segment_test(
            test_endpoint_for_point(start),
            test_endpoint_for_point(end),
        )
    }

    fn test_endpoint_for_point(point: [f64; 2]) -> PlanarBooleanSegmentCarrierEndpointFacts {
        let (source_identity, projected_identity) = if point == [0.0, 0.0] {
            ("source low", "projected low")
        } else {
            ("source high", "projected high")
        };
        PlanarBooleanSegmentCarrierEndpointFacts::for_canonical_segment_test(
            point,
            source_identity,
            projected_identity,
        )
    }
}
