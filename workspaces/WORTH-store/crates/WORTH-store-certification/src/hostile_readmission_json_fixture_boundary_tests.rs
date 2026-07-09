use worth_foundational::{AspectLocator, BoundarySourceLocator, LocatorAuthority};
use worth_proof::TransitionOutcome;
use worth_store_aspect_native::{
    readmit_external_terminal_projection_document_as_store_aspect_state,
    StoreTerminalProjectionDenial,
};
use worth_store_test_support::{NativeStoreAspectFixture, StoreHostileReadmissionJsonFixture};

#[test]
fn hostile_readmission_tests_use_json_only_as_attacker_input() {
    let fixture = NativeStoreAspectFixture::segment_header("segment-0052", 52);
    let hostile_fixture = StoreHostileReadmissionJsonFixture::attacker_document(
        fixture.identity().clone(),
        serde_json::json!({
            "segment": "segment-0052",
            "generation": 52
        }),
    );
    let hostile_readmission_witness = match hostile_fixture.allow_in_hostile_readmission_suite() {
        TransitionOutcome::Success(witness) => witness,
        outcome => panic!("hostile JSON fixture should produce suite witness: {outcome:?}"),
    };

    let readmission = match readmit_external_terminal_projection_document_as_store_aspect_state(
        hostile_fixture.identity().clone(),
        hostile_fixture.into_attacker_document(hostile_readmission_witness),
        fixture.contract().clone(),
        source_locator(&fixture),
        fixture.physical_witness(),
    ) {
        TransitionOutcome::Success(readmission) => readmission,
        outcome => panic!("hostile JSON input should readmit through native Store: {outcome:?}"),
    };
    let rebuilt = readmission.rebuild_store_boundary_fact().unwrap();

    assert_eq!(rebuilt.identity(), fixture.identity());
    assert_eq!(
        rebuilt.authority_input().physical_witness(),
        fixture.physical_witness()
    );
}

#[test]
fn hostile_readmission_denial_is_explicit_for_non_native_output() {
    let fixture = NativeStoreAspectFixture::segment_header("segment-0053", 53);
    let hostile_fixture = StoreHostileReadmissionJsonFixture::attacker_document(
        fixture.identity().clone(),
        serde_json::json!({
            "segment": "segment-0053",
            "generation": "not-a-native-u64"
        }),
    );
    let hostile_readmission_witness = match hostile_fixture.allow_in_hostile_readmission_suite() {
        TransitionOutcome::Success(witness) => witness,
        outcome => panic!("hostile JSON fixture should produce suite witness: {outcome:?}"),
    };

    let denial = readmit_external_terminal_projection_document_as_store_aspect_state(
        hostile_fixture.identity().clone(),
        hostile_fixture.into_attacker_document(hostile_readmission_witness),
        fixture.contract().clone(),
        source_locator(&fixture),
        fixture.physical_witness(),
    );

    assert!(matches!(
        denial,
        TransitionOutcome::Denied(StoreTerminalProjectionDenial::JsonCompatibilityDenied(_))
    ));
}

fn source_locator(fixture: &NativeStoreAspectFixture) -> BoundarySourceLocator {
    BoundarySourceLocator::Aspect(AspectLocator::new(
        LocatorAuthority::Projected,
        fixture.identity().aspect_key().clone(),
    ))
}
