use worth_ui_runtime::facade::evidence::UiProjectionFactReceipt;

fn main() {
    let _ = UiProjectionFactReceipt {
        declaration_identity: unsafe { std::mem::zeroed() },
        declaration_support_authority_generation: unsafe { std::mem::zeroed() },
        query_basis_digest: "basis".into(),
        query_resolution_mode: unsafe { std::mem::zeroed() },
        projection_contract_digest: "contract".into(),
        projection_consumption_declaration_digest: "declaration".into(),
        projection_consumption_receipt_digest: "receipt".into(),
        projection_fact_set_digest: "facts".into(),
        projection_source_identity: "source".into(),
        required_measurement_dependencies: Box::new([]),
        required_query_fact_families: Box::new([]),
        required_query_fact_family_set_digest: 0,
        consumed_fact_families: Box::new([]),
        consumed_fact_family_set_digest: 0,
    };
}
