use std::collections::HashSet;

use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanCollinearRelationKind, PlanarBooleanEventLedgerReceipt,
    PlanarBooleanIntervalEventKind, PlanarBooleanPointEventKind,
};

use super::subject::MetabossEventExtractionSubject;

pub(crate) fn assert_event_ledger_shape(subject: &MetabossEventExtractionSubject) {
    let ledger = subject.ledger();
    let inputs = subject.inputs();
    assert_eq!(
        subject.pair().recipe().query_key(),
        "worth.catalog.boolean_event_extraction_metaboss_pair"
    );
    assert_eq!(
        ledger.reduced_pair_identity(),
        inputs.event_request.reduced_operand_pair_identity()
    );
    assert_eq!(
        ledger.segment_pair_enumeration_identity(),
        inputs.pair_worklist.segment_pair_enumeration_identity()
    );
    assert_eq!(
        inputs.pair_worklist.work_items().len(),
        subject.expected().expected_segment_pair_breadth()
    );
    assert_query_index_shape(subject);
    assert_eq!(
        inputs.carriers.segment_carrier_set_identity(),
        ledger.segment_carrier_set_identity()
    );
    assert_point_event_kind_multiplicity(subject);
    assert_collinear_relation_families(ledger);
    assert_interval_event_kind_multiplicity(subject);
    assert_counter_shape(subject);
    assert_ordered_identity_shape(ledger);
    assert_provenance_shape(subject);
    assert_eq!(subject.policy_stop().counters().policy_exits(), 1);
}

fn assert_query_index_shape(subject: &MetabossEventExtractionSubject) {
    let pair_worklist = &subject.inputs().pair_worklist;
    let counters = pair_worklist.counters();
    assert_eq!(
        counters.expected_pair_breadth(),
        subject.expected().expected_possible_segment_pair_breadth()
    );
    assert_eq!(
        counters.emitted_pair_breadth(),
        subject.expected().expected_segment_pair_breadth()
    );
    assert_eq!(
        counters.query_index_candidate_count(),
        subject.expected().expected_segment_pair_breadth()
    );
    assert_eq!(
        counters.skipped_pair_count(),
        subject.expected().expected_query_index_culled_pair_count()
    );
    assert_eq!(
        counters.query_index_culled_pair_count(),
        subject.expected().expected_query_index_culled_pair_count()
    );
    assert!(!pair_worklist.query_index_identity().is_empty());
    assert!(!pair_worklist.query_index_declaration_digest().is_empty());
    assert!(!pair_worklist.query_index_envelope_digest().is_empty());
}

fn assert_point_event_kind_multiplicity(subject: &MetabossEventExtractionSubject) {
    let ledger = subject.ledger();
    let expected = subject.expected();
    assert_eq!(
        count_point_kind(
            ledger,
            PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing
        ),
        expected.expected_proper_crossing_point_count()
    );
    assert_eq!(
        count_point_kind(
            ledger,
            PlanarBooleanPointEventKind::OperandAEndpointOnOperandBInterior
        ),
        expected.expected_operand_a_endpoint_on_b_interior_point_count()
    );
    assert_eq!(
        count_point_kind(
            ledger,
            PlanarBooleanPointEventKind::OperandBEndpointOnOperandAInterior
        ),
        expected.expected_operand_b_endpoint_on_a_interior_point_count()
    );
    assert_eq!(
        count_point_kind(ledger, PlanarBooleanPointEventKind::SharedEndpoint),
        expected.expected_shared_endpoint_point_count()
    );
}

fn count_point_kind(
    ledger: &PlanarBooleanEventLedgerReceipt,
    kind: PlanarBooleanPointEventKind,
) -> usize {
    ledger
        .point_events()
        .iter()
        .filter(|event| event.kind() == kind)
        .count()
}

fn assert_collinear_relation_families(ledger: &PlanarBooleanEventLedgerReceipt) {
    let kinds = ledger
        .relation_diagnostics()
        .iter()
        .map(|relation| relation.kind())
        .collect::<Vec<_>>();
    assert!(
        !kinds.contains(&PlanarBooleanCollinearRelationKind::Disjoint),
        "spatial candidate indexing should cull disjoint segment pairs before relation diagnostics"
    );
    assert!(kinds.contains(&PlanarBooleanCollinearRelationKind::EndpointTouch));
}

fn assert_interval_event_kind_multiplicity(subject: &MetabossEventExtractionSubject) {
    let ledger = subject.ledger();
    let expected = subject.expected();
    assert_eq!(
        count_interval_kind(ledger, PlanarBooleanIntervalEventKind::PartialOverlap),
        expected.expected_partial_overlap_interval_count()
    );
    assert_eq!(
        count_interval_kind(ledger, PlanarBooleanIntervalEventKind::ContainmentOverlap),
        expected.expected_containment_overlap_interval_count()
    );
    assert_eq!(
        count_interval_kind(
            ledger,
            PlanarBooleanIntervalEventKind::IdenticalSameDirection
        ),
        expected.expected_identical_same_direction_interval_count()
    );
    assert_eq!(
        count_interval_kind(
            ledger,
            PlanarBooleanIntervalEventKind::IdenticalAntiParallel
        ),
        expected.expected_identical_anti_parallel_interval_count()
    );
}

fn count_interval_kind(
    ledger: &PlanarBooleanEventLedgerReceipt,
    kind: PlanarBooleanIntervalEventKind,
) -> usize {
    ledger
        .interval_events()
        .iter()
        .filter(|event| event.kind() == kind)
        .count()
}

fn assert_counter_shape(subject: &MetabossEventExtractionSubject) {
    let ledger = subject.ledger();
    let expected = subject.expected();
    let counters = ledger.counters();
    assert_eq!(
        ledger.point_events().len(),
        expected.expected_point_event_count()
    );
    assert_eq!(
        ledger.interval_events().len(),
        expected.expected_interval_event_count()
    );
    assert_eq!(
        ledger.relation_diagnostics().len(),
        expected.expected_relation_diagnostic_count()
    );
    assert_eq!(
        ledger.event_groups().len(),
        expected.expected_grouped_event_count()
    );
    assert_eq!(
        counters.point_events_consumed(),
        expected.expected_point_event_count()
    );
    assert_eq!(
        counters.interval_events_consumed(),
        expected.expected_interval_event_count()
    );
    assert_eq!(
        counters.collinear_relations_consumed(),
        expected.expected_collinear_relation_count()
    );
    assert_eq!(
        counters.relation_diagnostics_retained(),
        expected.expected_relation_diagnostic_count()
    );
    assert_eq!(
        counters.point_groups_emitted(),
        expected.expected_point_group_count()
    );
    assert_eq!(
        counters.interval_groups_emitted(),
        expected.expected_interval_group_count()
    );
    assert_eq!(
        counters.total_grouped_event_count(),
        expected.expected_grouped_event_count()
    );
    assert_eq!(
        counters.duplicate_point_reports_suppressed(),
        expected.expected_duplicate_point_reports_suppressed()
    );
    assert_eq!(
        counters.duplicate_point_group_reports_merged(),
        expected.expected_duplicate_point_groups_merged()
    );
    assert_eq!(
        counters.duplicate_interval_group_reports_merged(),
        expected.expected_duplicate_interval_groups_merged()
    );
    assert_eq!(counters.downstream_consumable_artifact_count(), 1);
}

fn assert_ordered_identity_shape(ledger: &PlanarBooleanEventLedgerReceipt) {
    let ordered = ledger.ordered_events();
    assert_same_identity_set(
        ordered.point_event_identities(),
        ledger
            .point_events()
            .iter()
            .map(|event| event.event_identity()),
    );
    assert_same_identity_set(
        ordered.interval_event_identities(),
        ledger
            .interval_events()
            .iter()
            .map(|event| event.event_identity()),
    );
    assert_same_identity_set(
        ordered.relation_diagnostic_identities(),
        ledger
            .relation_diagnostics()
            .iter()
            .map(|relation| relation.relation_identity()),
    );
    assert_same_identity_set(
        ordered.event_group_identities(),
        ledger
            .event_groups()
            .iter()
            .map(|group| group.group_identity()),
    );
}

fn assert_provenance_shape(subject: &MetabossEventExtractionSubject) {
    let ledger = subject.ledger();
    let (left_carriers, right_carriers) = carrier_partition_identity_sets(subject);
    let carriers = left_carriers
        .iter()
        .chain(right_carriers.iter())
        .copied()
        .collect::<HashSet<_>>();
    assert!(!ledger.event_ledger_identity().is_empty());
    assert!(!ledger.downstream_consumption_identity().is_empty());
    assert!(ledger.point_events().iter().all(|event| {
        !event.segment_pair_identity().is_empty()
            && !event.left_carrier_identity().is_empty()
            && !event.right_carrier_identity().is_empty()
            && left_carriers.contains(event.left_carrier_identity())
            && right_carriers.contains(event.right_carrier_identity())
            && event
                .participating_carrier_identities()
                .iter()
                .all(|identity| carriers.contains(identity.as_str()))
            && !event
                .coordinate_fact()
                .coordinate_fact_identity()
                .is_empty()
            && event
                .coordinate_fact()
                .point_2d()
                .iter()
                .all(|coordinate| coordinate.is_finite())
            && bounded_parameter(event.operand_a_parameter().parameter())
            && bounded_parameter(event.operand_b_parameter().parameter())
            && event.operand_a_parameter().carrier_identity() == event.left_carrier_identity()
            && event.operand_b_parameter().carrier_identity() == event.right_carrier_identity()
            && event.operand_a_parameter().segment_identity() == event.left_segment_identity()
            && event.operand_b_parameter().segment_identity() == event.right_segment_identity()
            && !event
                .operand_a_parameter()
                .parameter_fact_identity()
                .is_empty()
            && !event
                .operand_b_parameter()
                .parameter_fact_identity()
                .is_empty()
            && !event.predicate_binding_identity().is_empty()
            && !event.predicate_bound_pair_identity().is_empty()
            && !event.segment_contract_fact_digest().is_empty()
            && !event.endpoint_source_identities().is_empty()
            && !event.endpoint_projection_fact_digests().is_empty()
            && !event.predicate_receipt_identities().is_empty()
    }));
    assert!(ledger.interval_events().iter().all(|event| {
        !event.segment_pair_identity().is_empty()
            && left_carriers.contains(event.left_carrier_identity())
            && right_carriers.contains(event.right_carrier_identity())
            && !event.collinear_relation_identity().is_empty()
            && !event.reduced_pair_identity().is_empty()
            && !event.predicate_binding_identity().is_empty()
            && !event.predicate_bound_pair_identity().is_empty()
            && !event.segment_contract_fact_digest().is_empty()
            && !event.local_frame_identity().is_empty()
            && !event.precision_basis_identity().is_empty()
            && !event
                .normalized_interval()
                .normalized_interval_identity()
                .is_empty()
            && bounded_parameter_range(event.normalized_interval().parameter_range())
            && !event
                .left_source_interval()
                .source_interval_identity()
                .is_empty()
            && event.left_source_interval().carrier_identity() == event.left_carrier_identity()
            && event.left_source_interval().segment_identity() == event.left_segment_identity()
            && bounded_parameter_range(event.left_source_interval().source_parameter_range())
            && !event
                .right_source_interval()
                .source_interval_identity()
                .is_empty()
            && event.right_source_interval().carrier_identity() == event.right_carrier_identity()
            && event.right_source_interval().segment_identity() == event.right_segment_identity()
            && bounded_parameter_range(event.right_source_interval().source_parameter_range())
    }));
    assert!(subject
        .inputs()
        .carriers
        .left()
        .iter()
        .chain(subject.inputs().carriers.right())
        .all(|carrier| {
            !carrier.source_face_identity().is_empty()
                && !carrier.source_loop_identity().is_empty()
                && !carrier.source_edge_identity().is_empty()
                && !carrier.local_frame_identity().is_empty()
                && !carrier.projection_stage_identity().is_empty()
                && !carrier.precision_basis_identity().is_empty()
        }));
}

fn assert_same_identity_set<'a>(actual: &[String], expected: impl IntoIterator<Item = &'a str>) {
    let actual = actual.iter().map(String::as_str).collect::<HashSet<_>>();
    let expected = expected.into_iter().collect::<HashSet<_>>();
    assert_eq!(actual, expected);
}

fn carrier_partition_identity_sets(
    subject: &MetabossEventExtractionSubject,
) -> (HashSet<&str>, HashSet<&str>) {
    let left = subject
        .inputs()
        .carriers
        .left()
        .iter()
        .map(|carrier| carrier.carrier_identity())
        .collect();
    let right = subject
        .inputs()
        .carriers
        .right()
        .iter()
        .map(|carrier| carrier.carrier_identity())
        .collect();
    (left, right)
}

fn bounded_parameter(parameter: f64) -> bool {
    parameter.is_finite() && (0.0..=1.0).contains(&parameter)
}

fn bounded_parameter_range(parameter_range: [f64; 2]) -> bool {
    parameter_range.into_iter().all(bounded_parameter)
}
