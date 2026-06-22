use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanLoopReconstructionSplitConsumption;

fn main() {
    let _ = PlanarBooleanLoopReconstructionSplitConsumption {
        consumption_identity: String::from("forged"),
        downstream_consumption_identity: String::from("downstream"),
        split_ledger_receipt_identity: String::from("ledger"),
        split_ledger_downstream_identity: String::from("ledger-downstream"),
        split_request_identity: String::from("request"),
        workload_stage_index_identity: String::from("stage-index"),
        counters: Default::default(),
    };
}
