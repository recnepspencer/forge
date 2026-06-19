use worth_spatial::facade::planar_boolean_loop_reconstruction::PlanarBooleanLoopDecisionLog;

fn main() {
    let _ = PlanarBooleanLoopDecisionLog {
        decision_log_identity: String::new(),
        request_identity: String::new(),
        split_ledger_receipt_identity: String::new(),
        rows: Vec::new(),
        lookup_index: panic!("not constructible"),
        counters: Default::default(),
    };
}
