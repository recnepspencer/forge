use worth_query::facade::foundation::{AspectFieldKey, DeclarativeOrderingField};

fn main() {
    let field = DeclarativeOrderingField::ascending(AspectFieldKey::from_authoring_parts("identity", "id").unwrap());
    let _ = field.aspect();
    let _ = field.field();
}
