use worth_query::facade::inspection::{WorthQueryInspectionDeclaration, WorthQueryWorkspace};

fn cannot_run_without_context(
    declaration: WorthQueryInspectionDeclaration,
    workspace: &WorthQueryWorkspace,
) {
    let _outcome = declaration.run(workspace);
}

fn main() {}
