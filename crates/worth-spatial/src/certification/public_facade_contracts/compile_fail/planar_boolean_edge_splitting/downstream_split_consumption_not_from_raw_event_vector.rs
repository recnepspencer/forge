use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanDownstreamSplitConsumptionInput;
use worth_spatial::facade::planar_boolean_events::PlanarBooleanPointEvent;

fn reject_raw_event_vector(point_events: Vec<PlanarBooleanPointEvent>) {
    let _ = PlanarBooleanDownstreamSplitConsumptionInput::from_raw_event_vector(point_events);
}

fn main() {}
