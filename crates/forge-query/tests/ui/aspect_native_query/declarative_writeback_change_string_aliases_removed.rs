use forge_query::facade::{AspectFieldKey, DeclarativeWritebackChange, DeclarativeWritebackValue};

fn main() {
    let change = DeclarativeWritebackChange::new(
        AspectFieldKey::new("title", "value").unwrap(),
        DeclarativeWritebackValue::string("Buy oat milk"),
    );
    let _ = change.aspect();
    let _ = change.field();
}
