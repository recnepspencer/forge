use worth_query::facade::aggregate::WorthQueryCountDeclaration;
use worth_query::facade::runtime::WorthQueryWorkspace;

fn run_without_context(
    declaration: WorthQueryCountDeclaration,
    workspace: &mut WorthQueryWorkspace,
) {
    let _outcome = declaration.run(workspace);
}

fn main() {}
