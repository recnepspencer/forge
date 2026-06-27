use forge_store_test_support::{
    require_native_store_aspect_fixture, NativeStoreAspectFixture,
    StoreHostileReadmissionJsonFixture,
};

fn main() {
    let fixture = NativeStoreAspectFixture::segment_header("segment-0062", 62);
    let hostile_fixture =
        StoreHostileReadmissionJsonFixture::attacker_document(fixture.identity().clone(), ());

    let _fact = require_native_store_aspect_fixture(&hostile_fixture);
}
