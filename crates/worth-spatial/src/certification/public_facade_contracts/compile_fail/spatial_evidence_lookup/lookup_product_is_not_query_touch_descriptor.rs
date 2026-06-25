use forge_query::facade::ForgeQueryGraphTouchDescriptor;
use worth_spatial::facade::workload_vocabulary::SpatialEvidenceLookupProduct;

fn requires_query_touch_descriptor(_: ForgeQueryGraphTouchDescriptor) {}

fn spatial_lookup_product() -> SpatialEvidenceLookupProduct {
    panic!("fixture never constructs a lookup product")
}

fn main() {
    requires_query_touch_descriptor(spatial_lookup_product());
}
