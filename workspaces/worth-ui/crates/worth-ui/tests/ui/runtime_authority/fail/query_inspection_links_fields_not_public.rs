use worth_ui::facade::WorthUiQueryInspectionLinks;

fn uninhabited<T>() -> T {
    panic!("compile-fail fixture never runs")
}

fn main() {
    let _links = WorthUiQueryInspectionLinks {
        binding_identity: uninhabited(),
        posture: uninhabited(),
        preservation_receipt: None,
        required_surfaces: Vec::new(),
    };
}
