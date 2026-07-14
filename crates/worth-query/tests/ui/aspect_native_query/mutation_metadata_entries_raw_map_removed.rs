use std::collections::BTreeMap;

use worth_query::facade::runtime::{WorthQueryMutationMetadata, WorthQueryMutationMetadataValue};

fn main() {
    let metadata = metadata_fixture();
    let _: &BTreeMap<String, WorthQueryMutationMetadataValue> = metadata.entries();
}

fn metadata_fixture() -> WorthQueryMutationMetadata {
    panic!("fixture only")
}
