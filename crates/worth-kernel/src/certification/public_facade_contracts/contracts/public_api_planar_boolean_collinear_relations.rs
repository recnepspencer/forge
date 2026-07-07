use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanCollinearRelationExtraction, PlanarBooleanCollinearRelationKind,
    PlanarBooleanCollinearRelationReceipt, PlanarBooleanEventPredicateBinding,
};

use super::collinear_relation_support;
use super::reduced_pair_support;

use super::collinear_relation_support::SyntheticCollinearRelation;

#[test]
fn collinear_disjoint_pairs_emit_typed_no_event_relation() {
    reduced_pair_support::run_with_large_stack(|| {
        let receipt = extraction_for_relation(
            "phase7.2 collinear relation disjoint",
            SyntheticCollinearRelation::Disjoint,
        );

        assert_every_relation_has_kind(&receipt, PlanarBooleanCollinearRelationKind::Disjoint);
        assert_eq!(
            receipt.counters().emitted_disjoint_relations(),
            receipt.counters().inspected_bound_pairs()
        );
        assert_eq!(receipt.counters().skipped_non_collinear_pairs(), 0);
        assert!(receipt.relations().iter().all(|relation| {
            relation.interval_basis().is_none() && relation.touch_point().is_none()
        }));
    });
}

#[test]
fn collinear_touching_endpoint_does_not_emit_overlap_interval() {
    reduced_pair_support::run_with_large_stack(|| {
        let receipt = extraction_for_relation(
            "phase7.2 collinear relation endpoint touch",
            SyntheticCollinearRelation::EndpointTouch,
        );

        assert_every_relation_has_kind(&receipt, PlanarBooleanCollinearRelationKind::EndpointTouch);
        assert_eq!(
            receipt.counters().emitted_endpoint_touch_relations(),
            receipt.counters().inspected_bound_pairs()
        );
        assert!(receipt.relations().iter().all(|relation| {
            relation.interval_basis().is_none() && relation.touch_point().is_some()
        }));
    });
}

#[test]
fn collinear_touching_endpoint_preserves_reversed_segment_source_parameters() {
    reduced_pair_support::run_with_large_stack(|| {
        let first_reversed = extraction_for_relation(
            "phase7.2 collinear relation endpoint first reversed",
            SyntheticCollinearRelation::EndpointTouchWithFirstReversed,
        );
        let second_reversed = extraction_for_relation(
            "phase7.2 collinear relation endpoint second reversed",
            SyntheticCollinearRelation::EndpointTouchWithSecondReversed,
        );

        assert_every_touch_has_parameters(&first_reversed, 0.0, 0.0);
        assert_every_touch_has_parameters(&second_reversed, 1.0, 1.0);
        assert_ne!(
            first_reversed.relations()[0].relation_identity(),
            second_reversed.relations()[0].relation_identity()
        );
    });
}

#[test]
fn anti_parallel_identical_segments_classify_distinct_from_same_direction() {
    reduced_pair_support::run_with_large_stack(|| {
        let same_direction = extraction_for_relation(
            "phase7.2 collinear relation identical same",
            SyntheticCollinearRelation::IdenticalSameDirection,
        );
        let anti_parallel = extraction_for_relation(
            "phase7.2 collinear relation identical anti",
            SyntheticCollinearRelation::IdenticalAntiParallel,
        );

        assert_every_relation_has_kind(
            &same_direction,
            PlanarBooleanCollinearRelationKind::IdenticalSameDirection,
        );
        assert_every_relation_has_kind(
            &anti_parallel,
            PlanarBooleanCollinearRelationKind::IdenticalAntiParallel,
        );
        assert_ne!(
            same_direction.relations()[0].relation_identity(),
            anti_parallel.relations()[0].relation_identity()
        );
    });
}

#[test]
fn partial_overlap_and_containment_do_not_collapse_to_same_relation_kind() {
    reduced_pair_support::run_with_large_stack(|| {
        let partial = extraction_for_relation(
            "phase7.2 collinear relation partial overlap",
            SyntheticCollinearRelation::PartialOverlap,
        );
        let containment = extraction_for_relation(
            "phase7.2 collinear relation containment overlap",
            SyntheticCollinearRelation::ContainmentOverlap,
        );

        assert_every_relation_has_kind(
            &partial,
            PlanarBooleanCollinearRelationKind::PartialOverlap,
        );
        assert_every_relation_has_kind(
            &containment,
            PlanarBooleanCollinearRelationKind::ContainmentOverlap,
        );
        assert!(partial
            .relations()
            .iter()
            .all(|relation| relation.interval_basis().is_some()));
        assert!(containment.relations().iter().all(|relation| {
            let basis = relation.interval_basis().expect("containment interval");
            basis.left_parameter_range() == [0.0, 1.0]
                || basis.right_parameter_range() == [0.0, 1.0]
        }));
    });
}

#[test]
fn diagonal_reversed_partial_overlap_preserves_both_parameter_ranges() {
    reduced_pair_support::run_with_large_stack(|| {
        let receipt = extraction_for_relation(
            "phase7.2 collinear relation diagonal reversed overlap",
            SyntheticCollinearRelation::DiagonalPartialOverlapWithSecondReversed,
        );

        assert_every_relation_has_kind(
            &receipt,
            PlanarBooleanCollinearRelationKind::PartialOverlap,
        );
        assert!(receipt.relations().iter().all(|relation| {
            let basis = relation.interval_basis().expect("partial overlap interval");
            basis.left_parameter_range() == [0.5, 1.0]
                && basis.right_parameter_range() == [0.5, 1.0]
                && relation.touch_point().is_none()
        }));
        assert_eq!(
            receipt.counters().emitted_partial_overlap_relations(),
            receipt.counters().inspected_bound_pairs()
        );
    });
}

fn extraction_for_relation(
    readiness_scope: &'static str,
    relation: SyntheticCollinearRelation,
) -> PlanarBooleanCollinearRelationReceipt {
    let subject =
        collinear_relation_support::binding_subject_with_relation(readiness_scope, relation);
    let binding = PlanarBooleanEventPredicateBinding::plan(&subject.pair_worklist)
        .for_reduced_pair(subject.reduced_pair_identity)
        .with_segment_segment_receipts(subject.segment_receipts)
        .with_predicate_consumption_receipt(subject.predicate_consumption)
        .compile()
        .expect("predicate binding plan should compile for collinear relation extraction")
        .certify()
        .expect("predicate binding should certify for collinear relation extraction");

    PlanarBooleanCollinearRelationExtraction::from_predicate_binding(&binding)
        .compile()
        .expect("collinear relation extraction plan should compile")
        .certify()
        .expect("collinear relation extraction should certify")
}

fn assert_every_relation_has_kind(
    receipt: &PlanarBooleanCollinearRelationReceipt,
    expected_kind: PlanarBooleanCollinearRelationKind,
) {
    assert_eq!(
        receipt.relations().len(),
        receipt.counters().inspected_bound_pairs()
    );
    assert!(receipt
        .relations()
        .iter()
        .all(|relation| relation.kind() == expected_kind));
    assert!(!receipt.receipt_identity().is_empty());
    assert!(!receipt.predicate_binding_identity().is_empty());
}

fn assert_every_touch_has_parameters(
    receipt: &PlanarBooleanCollinearRelationReceipt,
    expected_left_parameter: f64,
    expected_right_parameter: f64,
) {
    assert_every_relation_has_kind(receipt, PlanarBooleanCollinearRelationKind::EndpointTouch);
    assert!(receipt.relations().iter().all(|relation| {
        let touch = relation.touch_point().expect("endpoint touch payload");
        relation.interval_basis().is_none()
            && touch.left_parameter() == expected_left_parameter
            && touch.right_parameter() == expected_right_parameter
    }));
}
