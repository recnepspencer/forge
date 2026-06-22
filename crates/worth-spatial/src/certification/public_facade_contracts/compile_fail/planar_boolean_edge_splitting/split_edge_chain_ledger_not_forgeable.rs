use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitEdgeChainLedger;

fn main() {
    let _ledger = PlanarBooleanSplitEdgeChainLedger {
        ledger_identity: "ledger".to_string(),
        declaration_identity: "declaration".to_string(),
        split_request_identity: "request".to_string(),
        split_chain_validation_receipt_identity: "validation".to_string(),
        split_persistent_naming_receipt_identity: "naming".to_string(),
        split_decision_log_receipt_identity: "decisions".to_string(),
        chains: Vec::new(),
        counters: Default::default(),
    };
}
