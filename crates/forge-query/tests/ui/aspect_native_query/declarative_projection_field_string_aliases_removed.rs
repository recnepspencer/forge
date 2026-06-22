use forge_query::facade::{AspectFieldKey, DeclarativeProjectionField};

fn main() {
    let field = DeclarativeProjectionField::new(AspectFieldKey::new("identity", "id").unwrap());
    let _ = field.aspect();
    let _ = field.field();
}
