use forge_store_test_support::{NativeStoreAspectFixture, StoreHostileReadmissionJsonFixture};

fn main() {
    let fixture = NativeStoreAspectFixture::segment_header("segment-0064", 64);
    let hostile_fixture =
        StoreHostileReadmissionJsonFixture::attacker_document(fixture.identity().clone(), ());

    let _attacker_document = hostile_fixture.into_attacker_document();
}
