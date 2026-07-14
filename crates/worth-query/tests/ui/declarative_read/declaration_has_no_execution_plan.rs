use worth_query::facade::read::WorthQueryReadDeclaration;

fn inspect_pre_context_plan(declaration: WorthQueryReadDeclaration) {
    let _plan = declaration.execution_plan();
}

fn main() {}
