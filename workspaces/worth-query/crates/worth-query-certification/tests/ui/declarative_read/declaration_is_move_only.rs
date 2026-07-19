use worth_query::facade::read::WorthQueryReadDeclaration;

fn duplicate_declaration(declaration: WorthQueryReadDeclaration) {
    let _copy = declaration.clone();
}

fn main() {}
