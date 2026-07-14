use worth_query::facade::foundation::QueryFieldKey;

fn main() {
    let key = key_fixture();
    let _ = key.terminal_aspect_projection();
    let _ = key.terminal_field_projection();
}

fn key_fixture() -> QueryFieldKey {
    panic!("fixture only")
}
