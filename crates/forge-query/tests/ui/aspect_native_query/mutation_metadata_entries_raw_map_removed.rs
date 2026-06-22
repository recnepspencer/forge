use std::collections::BTreeMap;

use forge_query::facade::{ForgeQueryMutationMetadata, ForgeQueryMutationMetadataValue};

fn main() {
    let metadata = metadata_fixture();
    let _: &BTreeMap<String, ForgeQueryMutationMetadataValue> = metadata.entries();
}

fn metadata_fixture() -> ForgeQueryMutationMetadata {
    panic!("fixture only")
}
