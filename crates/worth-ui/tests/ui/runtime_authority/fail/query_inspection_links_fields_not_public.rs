use worth_ui::facade::WorthUiQueryInspectionLinks;

fn uninhabited<T>() -> T {
    panic!("compile-fail fixture never runs")
}

fn main() {
    let _links = WorthUiQueryInspectionLinks {
        binding_identity: uninhabited(),
        support_admission_digest: String::new(),
        basis_capability_digest: String::new(),
        live_compatibility_digest: String::new(),
        inspection_digest: String::new(),
        projection_consumption_digest: String::new(),
        async_result_state_digest: String::new(),
        recovery_digest: String::new(),
        preservation_receipt: None,
        required_surfaces: Vec::new(),
    };
}
