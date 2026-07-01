use forge_store_physical_certification::{
    FixtureCapabilityDeclaration, FixtureMutationBoundary, LargeStoreFixtureProfile,
    PhysicalFixtureBuilder, SyntheticFixtureAuthorityDenied,
};
use forge_store_test_support::production_backed_physical_fixture_materialization;

#[test]
fn invalid_root_reference_is_denied_by_materialization_admission() {
    let denial = production_backed_physical_fixture_materialization(
        LargeStoreFixtureProfile::StoreLargerThanMemory,
        0,
    )
    .unwrap_err();

    assert_eq!(
        denial,
        SyntheticFixtureAuthorityDenied::InvalidRootReference(0)
    );
}

#[test]
fn undeclared_mutation_boundary_is_denied_by_reopened_fixture_authority() {
    let materialization = production_backed_physical_fixture_materialization(
        LargeStoreFixtureProfile::StoreLargerThanMemory,
        1,
    )
    .unwrap();
    let fixture = PhysicalFixtureBuilder::production_backed("runtime-denial")
        .materialize_with(materialization)
        .capability(FixtureCapabilityDeclaration::for_mutation_boundary(
            FixtureMutationBoundary::Manifest,
        ))
        .and_reopen_through_physical_authority()
        .unwrap();

    let denial = fixture
        .require_mutation_boundary(FixtureMutationBoundary::PageImage)
        .unwrap_err();

    assert_eq!(
        denial,
        SyntheticFixtureAuthorityDenied::UndeclaredMutationBoundary(
            FixtureMutationBoundary::PageImage
        )
    );
}
