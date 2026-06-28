use forge_store_test_support::{
    require_native_store_aspect_fixture, NativeStoreAspectFixture,
    StoreTerminalProjectionJsonFixture,
};

fn main() {
    let fixture = NativeStoreAspectFixture::segment_header("segment-0061", 61);
    let terminal_fixture =
        StoreTerminalProjectionJsonFixture::from_boundary_fact(fixture.boundary_fact()).unwrap();

    let _fact = require_native_store_aspect_fixture(&terminal_fixture);
}
