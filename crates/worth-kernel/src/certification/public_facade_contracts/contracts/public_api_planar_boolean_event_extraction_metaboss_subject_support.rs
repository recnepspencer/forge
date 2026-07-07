#[path = "public_api_planar_boolean_collinear_relations_support/mod.rs"]
mod collinear_relation_support;
#[path = "public_api_planar_boolean_event_ledger_support.rs"]
mod event_ledger_support;
#[path = "public_api_planar_boolean_point_events_support/mod.rs"]
mod point_event_support;
#[path = "public_api_planar_boolean_event_predicate_binding_support.rs"]
mod predicate_binding_support;
#[path = "public_api_planar_boolean_common_plane_reduced_operand_pair_support.rs"]
mod reduced_pair_support;

#[path = "public_api_planar_boolean_event_extraction_metaboss_support/expected_shape.rs"]
mod expected_shape;
#[path = "public_api_planar_boolean_event_extraction_metaboss_support/subject.rs"]
mod subject;

pub(crate) use subject::MetabossEventExtractionSubject;

fn read_expected_shape(shape: &expected_shape::MetabossExpectedLedgerShape) {
    let _ = (
        shape.expected_segment_pair_breadth(),
        shape.expected_possible_segment_pair_breadth(),
        shape.expected_query_index_culled_pair_count(),
        shape.expected_point_event_count(),
        shape.expected_proper_crossing_point_count(),
        shape.expected_operand_a_endpoint_on_b_interior_point_count(),
        shape.expected_operand_b_endpoint_on_a_interior_point_count(),
        shape.expected_shared_endpoint_point_count(),
        shape.expected_interval_event_count(),
        shape.expected_partial_overlap_interval_count(),
        shape.expected_containment_overlap_interval_count(),
        shape.expected_identical_same_direction_interval_count(),
        shape.expected_identical_anti_parallel_interval_count(),
        shape.expected_collinear_relation_count(),
        shape.expected_relation_diagnostic_count(),
        shape.expected_point_group_count(),
        shape.expected_interval_group_count(),
        shape.expected_grouped_event_count(),
        shape.expected_duplicate_point_reports_suppressed(),
        shape.expected_duplicate_point_groups_merged(),
        shape.expected_duplicate_interval_groups_merged(),
    );
}

const _: () = {
    let _ = MetabossEventExtractionSubject::certify_from_pair;
    let _ = MetabossEventExtractionSubject::certify_event_carrier;
    let _ = MetabossEventExtractionSubject::inputs;
    let _ = MetabossEventExtractionSubject::policy_stop;
    let _ = MetabossEventExtractionSubject::expected;
    let _ = event_ledger_support::ledger_for_point_relation;
    let _ = read_expected_shape;
};
