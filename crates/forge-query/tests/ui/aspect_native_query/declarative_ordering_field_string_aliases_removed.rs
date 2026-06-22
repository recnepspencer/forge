use forge_query::facade::{AspectFieldKey, DeclarativeOrderingField};

fn main() {
    let field = DeclarativeOrderingField::ascending(AspectFieldKey::new("identity", "id").unwrap());
    let _ = field.aspect();
    let _ = field.field();
}
