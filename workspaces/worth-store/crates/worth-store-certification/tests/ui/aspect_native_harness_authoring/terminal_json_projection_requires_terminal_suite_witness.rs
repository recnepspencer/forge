use worth_store_test_support::{NativeStoreAspectFixture, StoreTerminalProjectionJsonFixture};

fn main() {
    let fixture = NativeStoreAspectFixture::segment_header("segment-0063", 63);
    let terminal_fixture =
        StoreTerminalProjectionJsonFixture::from_boundary_fact(fixture.boundary_fact()).unwrap();

    let _projection = terminal_fixture.projection();
}
