use worth_query::facade::history::WorthQueryHistoricalPathDeclaration;

fn run_without_context(declaration: WorthQueryHistoricalPathDeclaration) {
    let _outcome = declaration.run(todo!());
}

fn main() {}
