use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanRetainedIntervalSplitEntry;

fn main() {
    let _ = PlanarBooleanRetainedIntervalSplitEntry {
        entry_identity: "entry".to_string(),
        source_edge_identity: "source".to_string(),
        carrier_identity: "carrier".to_string(),
        candidate_identity: "candidate".to_string(),
        event_identity: "event".to_string(),
        interval_event_kind:
            worth_spatial::facade::planar_boolean_events::PlanarBooleanIntervalEventKind::PartialOverlap,
        admitted_parameter_range: [0.0, 1.0],
        source_interval_identity: "source interval".to_string(),
        source_parameter_range: [0.0, 1.0],
        source_sense:
            worth_spatial::facade::planar_boolean_events::PlanarBooleanSourceIntervalSense::Forward,
        normalized_interval_identity: "normalized interval".to_string(),
        normalized_parameter_range: [0.0, 1.0],
        local_frame_identity: "frame".to_string(),
        precision_basis_identity: "precision".to_string(),
        participation_row_identity: "row".to_string(),
        event_group_identities: vec![],
    };
}

