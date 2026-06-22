use forge_query::facade::DeclarativeWritebackValue;

fn main() {
    let _ = DeclarativeWritebackValue::StructuredJson("{\"title\":\"Buy oat milk\"}".into());
}
