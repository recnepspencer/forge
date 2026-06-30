use std::borrow::Borrow;

use worth_spatial::facade::workload_vocabulary::{
    SpatialEvidenceLookupProductDigest, SpatialEvidenceQueryTouchDescriptorDigest,
};

fn require_lookup_product_digest<T: Borrow<SpatialEvidenceLookupProductDigest>>(_: T) {}

fn query_digest() -> SpatialEvidenceQueryTouchDescriptorDigest {
    panic!("fixture never constructs a query digest")
}

fn main() {
    require_lookup_product_digest(query_digest());
}
