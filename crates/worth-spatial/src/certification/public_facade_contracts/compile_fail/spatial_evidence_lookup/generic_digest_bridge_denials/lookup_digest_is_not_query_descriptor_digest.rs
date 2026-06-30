use std::borrow::Borrow;

use worth_spatial::facade::workload_vocabulary::{
    SpatialEvidenceLookupProductDigest, SpatialEvidenceQueryTouchDescriptorDigest,
};

fn require_query_descriptor_digest<T: Borrow<SpatialEvidenceQueryTouchDescriptorDigest>>(_: T) {}

fn lookup_digest() -> SpatialEvidenceLookupProductDigest {
    panic!("fixture never constructs a lookup digest")
}

fn main() {
    require_query_descriptor_digest(lookup_digest());
}
