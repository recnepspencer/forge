use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanIntervalSplitCandidate;
use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanIntervalEventKind, PlanarBooleanSourceIntervalSense,
};

fn main() {
    let _ = PlanarBooleanIntervalSplitCandidate {
        candidate_identity: String::from("forged"),
        interval_event_identity: String::from("synthetic interval event"),
        interval_event_kind: PlanarBooleanIntervalEventKind::PartialOverlap,
        carrier_identity: String::from("carrier"),
        source_edge_identity: String::from("edge"),
        segment_identity: String::from("segment"),
        source_interval_identity: String::from("source interval"),
        source_parameter_range: [0.2, 0.7],
        source_sense: PlanarBooleanSourceIntervalSense::Forward,
        normalized_interval_identity: String::from("normalized interval"),
        normalized_parameter_range: [0.2, 0.7],
        local_frame_identity: String::from("frame"),
        precision_basis_identity: String::from("precision"),
        participation_row_identity: String::from("row"),
        event_group_identities: vec![String::from("event group")],
    };
}
