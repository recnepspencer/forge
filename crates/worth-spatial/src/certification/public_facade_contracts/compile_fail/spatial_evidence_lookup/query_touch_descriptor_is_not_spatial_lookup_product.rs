use forge_query::facade::ForgeQueryGraphTouchDescriptor;
use worth_spatial::facade::workload_vocabulary::SpatialEvidenceLookupProduct;

fn requires_spatial_lookup_product(_: SpatialEvidenceLookupProduct) {}

fn query_touch_descriptor() -> ForgeQueryGraphTouchDescriptor {
    panic!("fixture never constructs a Query descriptor")
}

fn main() {
    requires_spatial_lookup_product(query_touch_descriptor());
}
