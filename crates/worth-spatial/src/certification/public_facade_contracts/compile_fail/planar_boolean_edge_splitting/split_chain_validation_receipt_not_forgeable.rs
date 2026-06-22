use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanSplitChainValidationCounters, PlanarBooleanSplitChainValidationReceipt,
};

fn main() {
    let _ = PlanarBooleanSplitChainValidationReceipt {
        receipt_identity: "forged".to_string(),
        split_edge_fragment_set_identity: "fragments".to_string(),
        overlap_edge_chain_set_identity: "chains".to_string(),
        interval_subdivision_schedule_set_identity: "intervals".to_string(),
        fragment_coverage_rows: Vec::new(),
        overlap_coverage_rows: Vec::new(),
        counters: PlanarBooleanSplitChainValidationCounters::default(),
    };
}
