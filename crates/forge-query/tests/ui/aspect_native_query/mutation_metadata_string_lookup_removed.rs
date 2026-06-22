use forge_query::facade::ForgeQueryMutationMetadata;

fn main() {
    let metadata = metadata_fixture();
    let _ = metadata.get("author");
}

fn metadata_fixture() -> ForgeQueryMutationMetadata {
    panic!("fixture only")
}
