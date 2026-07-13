use worth_query::facade::read::{WorthQueryReadDeclaration, WorthQueryWorkspace};

fn reuse_after_run(
    declaration: WorthQueryReadDeclaration,
    workspace: &mut WorthQueryWorkspace,
) {
    let _outcome = declaration.run(workspace);
    let _identity = declaration.identity();
}

fn main() {}
