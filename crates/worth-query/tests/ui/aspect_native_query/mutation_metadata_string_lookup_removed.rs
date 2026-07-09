use worth_query::facade::WorthQueryMutationMetadata;

fn main() {
    let metadata = metadata_fixture();
    let _ = metadata.get("author");
}

fn metadata_fixture() -> WorthQueryMutationMetadata {
    panic!("fixture only")
}
