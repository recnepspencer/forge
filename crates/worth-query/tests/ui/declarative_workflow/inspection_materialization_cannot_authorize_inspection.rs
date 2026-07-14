use worth_query::facade::inspection::{declare, WorthQueryInspectionMaterialization};

fn cannot_inspect_diagnostics(materialization: &WorthQueryInspectionMaterialization) {
    let _declaration = declare(materialization);
}

fn main() {}
