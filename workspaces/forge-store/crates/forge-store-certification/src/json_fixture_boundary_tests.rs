use forge_proof::TransitionOutcome;
use forge_store_test_support::{
    NativeStoreAspectFixture, StoreHostileReadmissionJsonFixture, StoreJsonFixtureBoundaryDenial,
    StoreTerminalProjectionJsonFixture,
};

#[test]
fn terminal_projection_json_fixture_denies_ordinary_native_harness() {
    let fixture = NativeStoreAspectFixture::segment_header("segment-0051", 51);
    let terminal_fixture =
        StoreTerminalProjectionJsonFixture::from_boundary_fact(fixture.boundary_fact()).unwrap();

    assert_eq!(
        terminal_fixture.allow_in_terminal_projection_suite(),
        TransitionOutcome::Denied(
            StoreJsonFixtureBoundaryDenial::TerminalProjectionJsonRequiresTerminalProjectionSuite
        )
    );
}

#[test]
fn hostile_readmission_json_fixture_denies_ordinary_native_harness() {
    let fixture = NativeStoreAspectFixture::segment_header("segment-0052", 52);
    let hostile_fixture =
        StoreHostileReadmissionJsonFixture::attacker_document(fixture.identity().clone(), ());

    assert_eq!(
        hostile_fixture.allow_in_hostile_readmission_suite(),
        TransitionOutcome::Denied(
            StoreJsonFixtureBoundaryDenial::HostileReadmissionJsonRequiresHostileReadmissionSuite
        )
    );
}
