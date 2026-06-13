use worth_ui::facade::WorthUiArtifactToPlanProvenance;

fn uninhabited<T>() -> T {
    panic!("compile-fail fixture never runs")
}

fn main() {
    let _provenance = WorthUiArtifactToPlanProvenance {
        plan_index: 0,
        identity_basis: String::new(),
        input_family: uninhabited(),
        source: uninhabited(),
        capability_reference: None,
        query_links: None,
    };
}
