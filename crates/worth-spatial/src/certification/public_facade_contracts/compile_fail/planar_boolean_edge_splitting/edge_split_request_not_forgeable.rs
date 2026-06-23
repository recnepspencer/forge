use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanEdgeSplitRequest, PlanarBooleanEdgeSplitRequestCounters,
};

fn main() {
    let _ = PlanarBooleanEdgeSplitRequest {
        split_request_identity: String::new(),
        event_ledger_identity: String::new(),
        downstream_consumption_identity: String::new(),
        reduced_pair_identity: String::new(),
        event_extraction_request_identity: String::new(),
        segment_carrier_set_identity: String::new(),
        segment_pair_enumeration_identity: String::new(),
        candidate_index_consumption_gate_identity: String::new(),
        candidate_index_product_identity: String::new(),
        query_index_plan_digest: String::new(),
        counters: PlanarBooleanEdgeSplitRequestCounters::default(),
    };
}
