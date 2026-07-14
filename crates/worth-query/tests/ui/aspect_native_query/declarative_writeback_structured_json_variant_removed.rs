use worth_query::facade::foundation::DeclarativeWritebackValue;

fn main() {
    let _ = DeclarativeWritebackValue::StructuredJson("{\"title\":\"Buy oat milk\"}".into());
}
