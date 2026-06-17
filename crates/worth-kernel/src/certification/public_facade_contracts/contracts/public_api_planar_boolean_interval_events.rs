use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanCollinearRelationExtraction, PlanarBooleanEventPredicateBinding,
    PlanarBooleanIntervalEventExtraction, PlanarBooleanIntervalEventExtractionCounters,
    PlanarBooleanIntervalEventExtractionReceipt, PlanarBooleanIntervalEventKind,
    PlanarBooleanSourceIntervalSense,
};

#[path = "public_api_planar_boolean_collinear_relations_support/mod.rs"]
#[allow(dead_code)]
mod collinear_relation_support;
#[path = "public_api_planar_boolean_event_predicate_binding_support.rs"]
#[allow(dead_code)]
mod predicate_binding_support;
#[path = "public_api_planar_boolean_common_plane_reduced_operand_pair_support.rs"]
mod reduced_pair_support;

use collinear_relation_support::SyntheticCollinearRelation;

#[test]
fn partial_overlap_interval_event_preserves_normalized_and_source_sense_bounds() {
    reduced_pair_support::run_with_large_stack(|| {
        let receipt = interval_extraction_for_relation(
            "phase7.2 interval event partial reversed",
            SyntheticCollinearRelation::DiagonalPartialOverlapWithSecondReversed,
        );

        assert_every_event_has_kind(
            &receipt,
            PlanarBooleanIntervalEventKind::PartialOverlap,
            |counters| counters.emitted_partial_overlap_events(),
        );
        assert!(receipt.interval_events().iter().all(|event| {
            event.normalized_interval().parameter_range() == [0.5, 1.0]
                && event.left_source_interval().source_parameter_range() == [0.5, 1.0]
                && event.right_source_interval().source_parameter_range() == [1.0, 0.5]
                && event.right_source_interval().sense()
                    == PlanarBooleanSourceIntervalSense::Reversed
        }));
    });
}

#[test]
fn containment_overlap_interval_event_preserves_contained_segment_identity() {
    reduced_pair_support::run_with_large_stack(|| {
        let receipt = interval_extraction_for_relation(
            "phase7.2 interval event containment",
            SyntheticCollinearRelation::ContainmentOverlap,
        );

        assert_every_event_has_kind(
            &receipt,
            PlanarBooleanIntervalEventKind::ContainmentOverlap,
            |counters| counters.emitted_containment_overlap_events(),
        );
        for event in receipt.interval_events() {
            assert!(
                range_matches(
                    event.normalized_interval().parameter_range(),
                    [1.0 / 3.0, 2.0 / 3.0],
                ) && event.right_source_interval().source_parameter_range() == [1.0, 0.0]
                    && event.right_source_interval().source_parameter_range()[1].to_bits()
                        == 0.0f64.to_bits()
                    && event.right_source_interval().sense()
                        == PlanarBooleanSourceIntervalSense::Reversed
                    && event.right_source_interval().segment_identity()
                        == event.right_segment_identity()
                    && event.right_source_interval().carrier_identity()
                        == event.right_carrier_identity(),
                "unexpected containment interval: normalized={:?}, right_source={:?}, right_sense={:?}, right_segment={}, event_right_segment={}",
                event.normalized_interval().parameter_range(),
                event.right_source_interval().source_parameter_range(),
                event.right_source_interval().sense(),
                event.right_source_interval().segment_identity(),
                event.right_segment_identity()
            );
        }
    });
}

#[test]
fn identical_same_direction_interval_event_preserves_matching_source_sense() {
    reduced_pair_support::run_with_large_stack(|| {
        let receipt = interval_extraction_for_relation(
            "phase7.2 interval event same direction identical",
            SyntheticCollinearRelation::IdenticalSameDirection,
        );

        assert_every_event_has_kind(
            &receipt,
            PlanarBooleanIntervalEventKind::IdenticalSameDirection,
            |counters| counters.emitted_identical_same_direction_events(),
        );
        assert!(receipt.interval_events().iter().all(|event| {
            event.normalized_interval().parameter_range() == [0.0, 1.0]
                && event.left_source_interval().source_parameter_range() == [0.0, 1.0]
                && event.left_source_interval().sense() == PlanarBooleanSourceIntervalSense::Forward
                && event.right_source_interval().source_parameter_range() == [0.0, 1.0]
                && event.right_source_interval().sense()
                    == PlanarBooleanSourceIntervalSense::Forward
        }));
    });
}

#[test]
fn identical_anti_parallel_interval_event_preserves_opposite_source_sense() {
    reduced_pair_support::run_with_large_stack(|| {
        let receipt = interval_extraction_for_relation(
            "phase7.2 interval event anti parallel identical",
            SyntheticCollinearRelation::IdenticalAntiParallel,
        );

        assert_every_event_has_kind(
            &receipt,
            PlanarBooleanIntervalEventKind::IdenticalAntiParallel,
            |counters| counters.emitted_identical_anti_parallel_events(),
        );
        assert!(receipt.interval_events().iter().all(|event| {
            event.normalized_interval().parameter_range() == [0.0, 1.0]
                && event.left_source_interval().source_parameter_range() == [0.0, 1.0]
                && event.left_source_interval().sense() == PlanarBooleanSourceIntervalSense::Forward
                && event.right_source_interval().source_parameter_range() == [1.0, 0.0]
                && event.right_source_interval().sense()
                    == PlanarBooleanSourceIntervalSense::Reversed
        }));
    });
}

#[test]
fn interval_event_extraction_skips_non_interval_collinear_relations_with_counters() {
    reduced_pair_support::run_with_large_stack(|| {
        let disjoint = interval_extraction_for_relation(
            "phase7.2 interval event disjoint skip",
            SyntheticCollinearRelation::Disjoint,
        );
        let endpoint_touch = interval_extraction_for_relation(
            "phase7.2 interval event endpoint skip",
            SyntheticCollinearRelation::EndpointTouch,
        );

        assert!(disjoint.interval_events().is_empty());
        assert_eq!(
            disjoint.counters().skipped_disjoint_relations(),
            disjoint.counters().inspected_collinear_relations()
        );
        assert!(endpoint_touch.interval_events().is_empty());
        assert_eq!(
            endpoint_touch.counters().skipped_endpoint_touch_relations(),
            endpoint_touch.counters().inspected_collinear_relations()
        );
    });
}

fn interval_extraction_for_relation(
    readiness_scope: &'static str,
    relation: SyntheticCollinearRelation,
) -> PlanarBooleanIntervalEventExtractionReceipt {
    let subject =
        collinear_relation_support::binding_subject_with_relation(readiness_scope, relation);
    let binding = PlanarBooleanEventPredicateBinding::plan(&subject.pair_worklist)
        .for_reduced_pair(subject.reduced_pair_identity)
        .with_segment_segment_receipts(subject.segment_receipts)
        .with_predicate_consumption_receipt(subject.predicate_consumption)
        .compile()
        .expect("predicate binding plan should compile for interval-event extraction")
        .certify()
        .expect("predicate binding should certify for interval-event extraction");
    let collinear_relations =
        PlanarBooleanCollinearRelationExtraction::from_predicate_binding(&binding)
            .compile()
            .expect("collinear relation plan should compile for interval-event extraction")
            .certify()
            .expect("collinear relation receipt should certify for interval-event extraction");

    PlanarBooleanIntervalEventExtraction::from_collinear_relations(&collinear_relations)
        .compile()
        .expect("interval-event extraction plan should compile")
        .certify()
        .expect("interval-event extraction should certify")
}

fn assert_every_event_has_kind(
    receipt: &PlanarBooleanIntervalEventExtractionReceipt,
    expected_kind: PlanarBooleanIntervalEventKind,
    emitted_kind_counter: impl FnOnce(PlanarBooleanIntervalEventExtractionCounters) -> usize,
) {
    assert_eq!(
        receipt.interval_events().len(),
        receipt.counters().inspected_collinear_relations()
    );
    assert_eq!(
        emitted_kind_counter(receipt.counters()),
        receipt.interval_events().len()
    );
    assert!(receipt
        .interval_events()
        .iter()
        .all(|event| event.kind() == expected_kind));
    assert!(!receipt.extraction_identity().is_empty());
    assert!(!receipt.collinear_relation_receipt_identity().is_empty());
}

fn range_matches(actual: [f64; 2], expected: [f64; 2]) -> bool {
    const EPSILON: f64 = 1.0e-12;
    (actual[0] - expected[0]).abs() <= EPSILON && (actual[1] - expected[1]).abs() <= EPSILON
}
