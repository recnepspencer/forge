use worth_query::facade::QueryFieldKey;

fn main() {
    let key = key_fixture();
    let _ = key.aspect();
    let _ = key.field();
}

fn key_fixture() -> QueryFieldKey {
    panic!("fixture only")
}
