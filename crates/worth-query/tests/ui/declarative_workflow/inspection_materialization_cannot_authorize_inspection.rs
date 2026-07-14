use worth_query::facade::inspection::{inspect, WorthQueryInspectionMaterialization};

fn cannot_inspect_diagnostics(materialization: &WorthQueryInspectionMaterialization) {
    let _declaration = inspect(materialization);
}

fn main() {}
