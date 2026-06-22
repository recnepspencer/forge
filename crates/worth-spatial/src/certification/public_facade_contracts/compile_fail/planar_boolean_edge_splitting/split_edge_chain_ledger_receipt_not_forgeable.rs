use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitEdgeChainLedgerReceipt;

fn main() {
    let _receipt = PlanarBooleanSplitEdgeChainLedgerReceipt {
        receipt_identity: "receipt".to_string(),
        ledger_identity: "ledger".to_string(),
        downstream_consumption_identity: "downstream".to_string(),
        split_request_identity: "request".to_string(),
        split_chain_validation_receipt_identity: "validation".to_string(),
        split_persistent_naming_receipt_identity: "naming".to_string(),
        split_decision_log_receipt_identity: "decisions".to_string(),
        chain_identities: Vec::new(),
        counters: Default::default(),
    };
}
