use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanSplitSourceEdgeCarrier, PlanarBooleanSplitSourceEdgeCarrierCounters,
    PlanarBooleanSplitSourceEdgeCarrierSet,
};

fn main() {
    let _ = PlanarBooleanSplitSourceEdgeCarrierSet {
        carrier_set_identity: String::from("forged set"),
        scope_admission_identity: String::from("scope"),
        split_request_identity: String::from("request"),
        event_ledger_identity: String::from("ledger"),
        segment_carrier_set_identity: String::from("segment carriers"),
        candidate_index_product_identity: String::from("candidate index"),
        query_index_plan_digest: String::from("query plan"),
        carriers: Vec::<PlanarBooleanSplitSourceEdgeCarrier>::new(),
        carrier_offsets: unavailable_offsets(),
        source_edge_offsets: unavailable_source_offsets(),
        counters: PlanarBooleanSplitSourceEdgeCarrierCounters::default(),
    };
}

fn unavailable_offsets() -> std::collections::BTreeMap<String, usize> {
    panic!("compile-fail fixture must never construct carrier offsets")
}

fn unavailable_source_offsets() -> std::collections::BTreeMap<String, Vec<usize>> {
    panic!("compile-fail fixture must never construct source-edge offsets")
}
