use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanEdgeSplitRequest, PlanarBooleanEdgeSplitRequestInput,
};
use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanIntervalEvent, PlanarBooleanPointEvent, PlanarBooleanSegmentCarrier,
};

fn main() {
    let raw_point_events: Vec<PlanarBooleanPointEvent> = Vec::new();
    let raw_interval_events: Vec<PlanarBooleanIntervalEvent> = Vec::new();
    let raw_segment_carriers: Vec<PlanarBooleanSegmentCarrier> = Vec::new();

    let _ = PlanarBooleanEdgeSplitRequest::admit(PlanarBooleanEdgeSplitRequestInput::new(
        &raw_point_events,
        &raw_interval_events,
        &raw_segment_carriers,
    ));
}
