use worth_query::facade::foundation::{DeclarativeWritebackChange, DeclarativeWritebackValue};

fn main() {
    let _ = DeclarativeWritebackChange::new(
        "title",
        "value",
        DeclarativeWritebackValue::string("Buy oat milk"),
    );
}
