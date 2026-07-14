use worth_store_aspect_native::StoreAspectBoundaryFact;

use crate::NativeStoreAspectFixture;

pub fn physical_isolation_boundary_fact(label: &str, segment: u64) -> StoreAspectBoundaryFact {
    NativeStoreAspectFixture::segment_header(label, segment)
        .boundary_fact()
        .clone()
}
