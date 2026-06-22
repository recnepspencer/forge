use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitSourceEdgeCarrierRecoveryInput;
use worth_spatial::facade::planar_boolean_events::PlanarBooleanSegmentCarrier;

fn main() {
    let raw_segment_carriers: Vec<PlanarBooleanSegmentCarrier> = Vec::new();
    let _ = PlanarBooleanSplitSourceEdgeCarrierRecoveryInput::from_scope_and_event_ledger(
        &raw_segment_carriers,
        &raw_segment_carriers,
    );
}
