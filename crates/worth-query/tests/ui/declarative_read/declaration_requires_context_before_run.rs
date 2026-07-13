use worth_query::facade::read::WorthQueryReadDeclaration;
use worth_query::facade::runtime::WorthQueryWorkspace;

fn run_without_context(declaration: WorthQueryReadDeclaration, workspace: &mut WorthQueryWorkspace) {
    let _outcome = declaration.run(workspace);
}

fn main() {}
