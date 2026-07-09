use worth_query::facade::{AspectFieldKey, DeclarativeWritebackChange, DeclarativeWritebackValue};

fn main() {
    let change = DeclarativeWritebackChange::new(
        AspectFieldKey::from_authoring_parts("title", "value").unwrap(),
        DeclarativeWritebackValue::string("Buy oat milk"),
    );
    let _ = change.aspect();
    let _ = change.field();
}
