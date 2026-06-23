use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanPointSplitCandidate;

fn main() {
    let _ = PlanarBooleanPointSplitCandidate {
        candidate_identity: String::from("forged"),
        point_event_identity: String::from("synthetic point event"),
        point_event_kind: unavailable_point_event_kind(),
        carrier_identity: String::from("carrier"),
        source_edge_identity: String::from("edge"),
        segment_identity: String::from("segment"),
        coordinate_fact: unavailable_coordinate_fact(),
        parameter_fact_identity: String::from("parameter fact"),
        parameter: 0.5,
        participation_row_identity: String::from("row"),
        start_source_endpoint_identity: String::from("start endpoint"),
        start_projected_endpoint_fact_identity: String::from("start projection"),
        end_source_endpoint_identity: String::from("end endpoint"),
        end_projected_endpoint_fact_identity: String::from("end projection"),
    };
}

fn unavailable_point_event_kind(
) -> worth_spatial::facade::planar_boolean_events::PlanarBooleanPointEventKind {
    panic!("compile-fail fixture must never construct point event kind")
}

fn unavailable_coordinate_fact(
) -> worth_spatial::facade::planar_boolean_events::PlanarBooleanPointEventCoordinateFact {
    panic!("compile-fail fixture must never construct coordinate fact")
}
