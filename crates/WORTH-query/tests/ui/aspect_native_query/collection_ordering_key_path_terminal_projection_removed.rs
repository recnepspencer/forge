use worth_query::facade::OrderingKeyPath;

fn main() {
    let key_path = key_path_fixture();
    let _ = key_path.terminal_aspect_projection();
    let _ = key_path.terminal_field_projection();
}

fn key_path_fixture() -> OrderingKeyPath {
    panic!("fixture only")
}
