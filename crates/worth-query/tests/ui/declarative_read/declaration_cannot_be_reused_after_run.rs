use worth_query::facade::read::{current, WorthQueryReadDeclaration};
use worth_query::facade::runtime::WorthQueryWorkspace;

fn reuse_after_run(
    declaration: WorthQueryReadDeclaration,
    workspace: &mut WorthQueryWorkspace,
) {
    let _outcome = declaration.using(current()).run(workspace);
    let _identity = declaration.identity();
}

fn main() {}
