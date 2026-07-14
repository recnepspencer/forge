use worth_query::facade::foundation::{AspectFieldKey, DeclarativeProjectionField};

fn main() {
    let field = DeclarativeProjectionField::new(AspectFieldKey::from_authoring_parts("identity", "id").unwrap());
    let _ = field.aspect();
    let _ = field.field();
}
