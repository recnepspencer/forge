use worth_query::facade::read::{current, WorthQueryReadDeclaration};

fn clone_admitted_request(declaration: WorthQueryReadDeclaration) {
    let request = declaration.using(current());
    let _reused = request.clone();
}

fn main() {}
