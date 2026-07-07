use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanEventPredicateBinding, PlanarBooleanPointEventExtraction,
    PlanarBooleanPointEventExtractionDenialKind, PlanarBooleanPointEventKind,
};

use super::point_event_support;
use super::reduced_pair_support;

use super::point_event_support::SyntheticPointRelation;

#[test]
fn proper_crossing_point_event_is_stable_under_segment_orientation_reversal() {
    reduced_pair_support::run_with_large_stack(|| {
        let forward = extraction_for_relation(
            "phase7.2 point event proper crossing reversal",
            SyntheticPointRelation::ProperCrossing,
        );
        let reversed = extraction_for_relation(
            "phase7.2 point event proper crossing reversal",
            SyntheticPointRelation::ProperCrossingReversed,
        );
        let forward_event = forward
            .point_events()
            .first()
            .expect("proper crossing must emit at least one point event");
        let reversed_event = reversed
            .point_events()
            .first()
            .expect("reversed proper crossing must emit at least one point event");

        assert_eq!(
            forward_event.kind(),
            PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing
        );
        assert_eq!(
            forward_event.event_identity(),
            reversed_event.event_identity()
        );
        assert_eq!(
            forward_event.coordinate_fact().point_2d(),
            reversed_event.coordinate_fact().point_2d()
        );
        assert_eq!(forward_event.operand_a_parameter().parameter(), 0.5);
        assert_eq!(forward_event.operand_b_parameter().parameter(), 0.5);
        assert_eq!(forward_event.predicate_receipt_identities().len(), 4);
        assert_eq!(forward_event.endpoint_projection_fact_digests().len(), 4);
        assert_every_event_has_kind(
            &forward,
            PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
        );
        assert_every_event_has_kind(
            &reversed,
            PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
        );
    });
}

#[test]
fn endpoint_interior_point_event_preserves_which_operand_contributed_endpoint() {
    reduced_pair_support::run_with_large_stack(|| {
        let operand_a_endpoint = extraction_for_relation(
            "phase7.2 point event endpoint interior a",
            SyntheticPointRelation::OperandAEndpointOnOperandBInterior,
        );
        let operand_a_event = operand_a_endpoint
            .point_events()
            .first()
            .expect("operand A endpoint on operand B interior must emit a point event");

        assert_eq!(
            operand_a_event.kind(),
            PlanarBooleanPointEventKind::OperandAEndpointOnOperandBInterior
        );
        assert!(operand_a_event
            .coordinate_fact()
            .point_2d()
            .iter()
            .all(|coordinate| coordinate.is_finite()));
        assert_eq!(operand_a_event.operand_a_parameter().parameter(), 0.0);
        assert_eq!(operand_a_event.operand_b_parameter().parameter(), 0.5);
        assert_eq!(operand_a_event.endpoint_source_identities().len(), 4);
        assert_every_event_has_kind(
            &operand_a_endpoint,
            PlanarBooleanPointEventKind::OperandAEndpointOnOperandBInterior,
        );

        let operand_b_endpoint = extraction_for_relation(
            "phase7.2 point event endpoint interior b",
            SyntheticPointRelation::OperandBEndpointOnOperandAInterior,
        );
        let operand_b_event = operand_b_endpoint
            .point_events()
            .first()
            .expect("operand B endpoint on operand A interior must emit a point event");

        assert_eq!(
            operand_b_event.kind(),
            PlanarBooleanPointEventKind::OperandBEndpointOnOperandAInterior
        );
        assert_eq!(operand_b_event.operand_a_parameter().parameter(), 0.5);
        assert_eq!(operand_b_event.operand_b_parameter().parameter(), 0.0);
        assert_every_event_has_kind(
            &operand_b_endpoint,
            PlanarBooleanPointEventKind::OperandBEndpointOnOperandAInterior,
        );
    });
}

#[test]
fn near_endpoint_miss_does_not_become_endpoint_contact_without_predicate_proof() {
    reduced_pair_support::run_with_large_stack(|| {
        let extraction = extraction_for_relation(
            "phase7.2 point event near endpoint miss",
            SyntheticPointRelation::NearEndpointMiss,
        );

        assert!(
            extraction.point_events().is_empty(),
            "near misses must remain disjoint unless the certified predicate basis proves contact"
        );
        assert_eq!(
            extraction.counters().skipped_non_point_relations(),
            extraction.counters().inspected_bound_pairs()
        );
    });
}

#[test]
fn point_event_extraction_denies_ambiguous_policy_required_relation() {
    reduced_pair_support::run_with_large_stack(|| {
        let denial = extraction_denial_for_relation(
            "phase7.2 point event ambiguous policy relation",
            SyntheticPointRelation::PolicyRequiredCollinearOverlap,
        );

        assert_eq!(
            denial.kind(),
            PlanarBooleanPointEventExtractionDenialKind::AmbiguousPredicateRelation
        );
        assert_eq!(denial.counters().ambiguous_relations(), 1);
    });
}

#[test]
fn shared_endpoint_events_collapse_duplicate_loop_closure_reports_once() {
    reduced_pair_support::run_with_large_stack(|| {
        let extraction = extraction_for_relation(
            "phase7.2 shared endpoint duplicate closure",
            SyntheticPointRelation::SharedEndpoint,
        );
        let event = extraction
            .point_events()
            .first()
            .expect("shared endpoint contacts must emit one canonical point event");

        assert_eq!(extraction.point_events().len(), 1);
        assert_eq!(event.kind(), PlanarBooleanPointEventKind::SharedEndpoint);
        assert!(event.shared_endpoint_event().is_some());
        assert_eq!(
            extraction.counters().shared_endpoint_candidates(),
            extraction.counters().inspected_bound_pairs()
        );
        assert_eq!(extraction.counters().emitted_shared_endpoint_events(), 1);
        assert_eq!(
            extraction.counters().duplicate_point_reports_suppressed(),
            extraction.counters().inspected_bound_pairs() - 1
        );
    });
}

#[test]
fn shared_endpoint_event_identity_is_stable_under_operand_pair_enumeration_order() {
    reduced_pair_support::run_with_large_stack(|| {
        let forward = extraction_for_relation(
            "phase7.2 shared endpoint stable order",
            SyntheticPointRelation::SharedEndpoint,
        );
        let reversed = extraction_for_reversed_relation_receipts(
            "phase7.2 shared endpoint stable order",
            SyntheticPointRelation::SharedEndpoint,
        );
        let forward_event = forward
            .point_events()
            .first()
            .expect("forward shared endpoint event");
        let reversed_event = reversed
            .point_events()
            .first()
            .expect("reversed shared endpoint event");

        assert_eq!(
            forward_event.event_identity(),
            reversed_event.event_identity()
        );
        assert_eq!(
            forward_event.segment_pair_identities(),
            reversed_event.segment_pair_identities()
        );
        assert_eq!(
            forward_event.participating_carrier_identities(),
            reversed_event.participating_carrier_identities()
        );
    });
}

#[test]
fn shared_endpoint_identity_ignores_non_contact_endpoint_projection_noise() {
    reduced_pair_support::run_with_large_stack(|| {
        let baseline = extraction_for_relation(
            "phase7.2 shared endpoint free endpoint noise",
            SyntheticPointRelation::SharedEndpoint,
        );
        let free_endpoint_changed = extraction_for_relation(
            "phase7.2 shared endpoint free endpoint noise",
            SyntheticPointRelation::SharedEndpointWithDifferentFreeEndpoints,
        );
        let baseline_event = baseline
            .point_events()
            .first()
            .expect("baseline shared endpoint event");
        let changed_event = free_endpoint_changed
            .point_events()
            .first()
            .expect("free-endpoint-changed shared endpoint event");
        let baseline_shared = baseline_event
            .shared_endpoint_event()
            .expect("baseline shared endpoint payload");
        let changed_shared = changed_event
            .shared_endpoint_event()
            .expect("changed shared endpoint payload");

        assert_eq!(
            baseline_event.event_identity(),
            changed_event.event_identity()
        );
        assert_eq!(
            baseline_shared.shared_endpoint_event_identity(),
            changed_shared.shared_endpoint_event_identity()
        );
        assert_eq!(
            baseline_shared.source_endpoint_identities(),
            changed_shared.source_endpoint_identities()
        );
        assert_eq!(
            baseline_shared.endpoint_projection_fact_digests(),
            changed_shared.endpoint_projection_fact_digests()
        );
    });
}

#[test]
fn high_valence_shared_endpoint_group_preserves_all_participating_carriers() {
    reduced_pair_support::run_with_large_stack(|| {
        let extraction = extraction_for_relation(
            "phase7.2 shared endpoint high valence",
            SyntheticPointRelation::SharedEndpoint,
        );
        let event = extraction
            .point_events()
            .first()
            .expect("shared endpoint high-valence event");
        let shared_endpoint = event
            .shared_endpoint_event()
            .expect("shared endpoint payload must be preserved");

        assert!(event.participating_carrier_identities().len() > 2);
        assert_eq!(
            event.participating_carrier_identities(),
            shared_endpoint.carrier_identities()
        );
        assert!(shared_endpoint.source_endpoint_identities().len() > 2);
        assert!(
            shared_endpoint.source_endpoint_identities().len()
                < event.endpoint_source_identities().len()
        );
        assert!(
            shared_endpoint.endpoint_projection_fact_digests().len()
                < event.endpoint_projection_fact_digests().len()
        );
        assert!(shared_endpoint
            .source_endpoint_identities()
            .iter()
            .all(|identity| event.endpoint_source_identities().contains(identity)));
        assert!(shared_endpoint
            .endpoint_projection_fact_digests()
            .iter()
            .all(|digest| event.endpoint_projection_fact_digests().contains(digest)));
        assert_eq!(
            extraction.counters().high_valence_point_groups_detected(),
            1
        );
    });
}

fn extraction_for_relation(
    readiness_scope: &'static str,
    relation: SyntheticPointRelation,
) -> worth_spatial::facade::planar_boolean_events::PlanarBooleanPointEventExtractionReceipt {
    let subject = point_event_support::binding_subject_with_relation(readiness_scope, relation);
    let binding = PlanarBooleanEventPredicateBinding::plan(&subject.pair_worklist)
        .for_reduced_pair(subject.reduced_pair_identity)
        .with_segment_segment_receipts(subject.segment_receipts)
        .with_predicate_consumption_receipt(subject.predicate_consumption)
        .compile()
        .expect("predicate binding plan should compile for point-event extraction")
        .certify()
        .expect("predicate binding should certify for point-event extraction");

    PlanarBooleanPointEventExtraction::from_predicate_binding(&binding)
        .compile()
        .expect("point-event extraction plan should compile")
        .certify()
        .expect("point-event extraction should certify")
}

fn extraction_for_reversed_relation_receipts(
    readiness_scope: &'static str,
    relation: SyntheticPointRelation,
) -> worth_spatial::facade::planar_boolean_events::PlanarBooleanPointEventExtractionReceipt {
    let subject = point_event_support::binding_subject_with_reversed_relation_receipts(
        readiness_scope,
        relation,
    );
    let binding = PlanarBooleanEventPredicateBinding::plan(&subject.pair_worklist)
        .for_reduced_pair(subject.reduced_pair_identity)
        .with_segment_segment_receipts(subject.segment_receipts)
        .with_predicate_consumption_receipt(subject.predicate_consumption)
        .compile()
        .expect("predicate binding plan should compile for reversed point-event extraction")
        .certify()
        .expect("predicate binding should certify for reversed point-event extraction");

    PlanarBooleanPointEventExtraction::from_predicate_binding(&binding)
        .compile()
        .expect("reversed point-event extraction plan should compile")
        .certify()
        .expect("reversed point-event extraction should certify")
}

fn assert_every_event_has_kind(
    extraction: &worth_spatial::facade::planar_boolean_events::PlanarBooleanPointEventExtractionReceipt,
    expected_kind: PlanarBooleanPointEventKind,
) {
    assert_eq!(
        extraction.point_events().len(),
        extraction.counters().inspected_bound_pairs()
    );
    assert_eq!(
        extraction.counters().emitted_point_events(),
        extraction.counters().inspected_bound_pairs()
    );
    assert!(extraction
        .point_events()
        .iter()
        .all(|event| event.kind() == expected_kind));
}

fn extraction_denial_for_relation(
    readiness_scope: &'static str,
    relation: SyntheticPointRelation,
) -> worth_spatial::facade::planar_boolean_events::PlanarBooleanPointEventExtractionDenial {
    let subject = point_event_support::binding_subject_with_relation(readiness_scope, relation);
    let binding = PlanarBooleanEventPredicateBinding::plan(&subject.pair_worklist)
        .for_reduced_pair(subject.reduced_pair_identity)
        .with_segment_segment_receipts(subject.segment_receipts)
        .with_predicate_consumption_receipt(subject.predicate_consumption)
        .compile()
        .expect("predicate binding plan should compile for point-event denial")
        .certify()
        .expect("predicate binding should certify for point-event denial");

    PlanarBooleanPointEventExtraction::from_predicate_binding(&binding)
        .compile()
        .expect("point-event extraction plan should compile before denial")
        .certify()
        .expect_err("ambiguous point-event relation must deny")
}
