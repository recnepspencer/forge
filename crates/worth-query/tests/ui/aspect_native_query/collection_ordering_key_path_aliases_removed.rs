use worth_query::facade::foundation::OrderingKeyPath;

fn main() {
    let key_path = key_path_fixture();
    let _ = key_path.aspect();
    let _ = key_path.field();
}

fn key_path_fixture() -> OrderingKeyPath {
    panic!("fixture only")
}
