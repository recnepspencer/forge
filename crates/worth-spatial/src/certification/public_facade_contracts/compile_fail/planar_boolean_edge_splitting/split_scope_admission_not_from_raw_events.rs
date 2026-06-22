use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanEdgeSplitScopeAdmissionInput;
use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanIntervalEvent, PlanarBooleanPointEvent, PlanarBooleanSegmentCarrier,
};

fn main() {
    let raw_point_events: Vec<PlanarBooleanPointEvent> = Vec::new();
    let raw_interval_events: Vec<PlanarBooleanIntervalEvent> = Vec::new();
    let raw_segment_carriers: Vec<PlanarBooleanSegmentCarrier> = Vec::new();

    let _ = PlanarBooleanEdgeSplitScopeAdmissionInput::from_split_request(&raw_point_events);
    let _ = PlanarBooleanEdgeSplitScopeAdmissionInput::from_split_request(&raw_interval_events);
    let _ = PlanarBooleanEdgeSplitScopeAdmissionInput::from_split_request(&raw_segment_carriers);
}
