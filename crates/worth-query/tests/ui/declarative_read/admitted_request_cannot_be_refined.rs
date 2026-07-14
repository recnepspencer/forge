use worth_query::facade::read::{current, WorthQueryReadDeclaration};

fn refine_after_context(declaration: WorthQueryReadDeclaration) {
    let request = declaration.using(current());
    let _second_context = request.using(current());
}

fn main() {}
