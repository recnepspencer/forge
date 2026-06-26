use worth_ui::facade::WorthUiLaneAdmission;

fn uninhabited<T>() -> T {
    panic!("compile-fail fixture never runs")
}

fn main() {
    let _admission = WorthUiLaneAdmission {
        rows: Vec::new(),
        query_support_links: Vec::new(),
        plan_input_basis_digest: 0,
        support_digest: 0,
        counters: uninhabited(),
    };
}
