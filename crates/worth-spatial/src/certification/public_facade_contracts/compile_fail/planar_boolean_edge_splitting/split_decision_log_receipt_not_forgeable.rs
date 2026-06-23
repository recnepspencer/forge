use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitDecisionLogReceipt;

fn main() {
    let _receipt = PlanarBooleanSplitDecisionLogReceipt {
        receipt_identity: "receipt".to_string(),
        query_declaration_identity: "query".to_string(),
        lowered_plan_identity: "plan".to_string(),
        split_chain_validation_receipt_identity: "validation".to_string(),
        split_persistent_naming_receipt_identity: "naming".to_string(),
        decision_rows: Vec::new(),
        lookup_index: panic!("not public"),
        counters: Default::default(),
    };
}
