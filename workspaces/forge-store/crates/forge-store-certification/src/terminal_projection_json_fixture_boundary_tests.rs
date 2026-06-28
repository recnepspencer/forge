use forge_proof::TransitionOutcome;
use forge_store_test_support::{NativeStoreAspectFixture, StoreTerminalProjectionJsonFixture};

#[test]
fn json_fixture_can_only_target_terminal_projection() {
    let fixture = NativeStoreAspectFixture::segment_header("segment-0051", 51);
    let terminal_fixture =
        StoreTerminalProjectionJsonFixture::from_boundary_fact(fixture.boundary_fact()).unwrap();

    let terminal_projection_witness = match terminal_fixture.allow_in_terminal_projection_suite() {
        TransitionOutcome::Success(witness) => witness,
        outcome => panic!("terminal JSON fixture should produce suite witness: {outcome:?}"),
    };

    let _projection = terminal_fixture.projection(terminal_projection_witness);
}
