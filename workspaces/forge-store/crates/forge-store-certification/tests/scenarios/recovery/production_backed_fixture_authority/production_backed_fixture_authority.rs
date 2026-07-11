use forge_store_physical_certification::{
    FixtureCapabilityDeclaration, FixtureMutationBoundary, FixtureProfileNonClaim,
    LargeStoreFixtureProfile, PhysicalFixtureBuilder, SyntheticFixtureAuthorityDenied,
};
use forge_store_test_support::production_backed_physical_fixture_materialization;

#[test]
fn production_backed_fixture_reopens_through_physical_authority() {
    let fixture = production_backed_fixture(1, LargeStoreFixtureProfile::StoreLargerThanMemory);

    assert_eq!(
        fixture.manifest().profile(),
        LargeStoreFixtureProfile::StoreLargerThanMemory
    );
    assert_eq!(
        fixture.manifest().semantic_digest(),
        fixture.authority_receipt().semantic_digest()
    );
    assert_eq!(fixture.manifest().source().root_reference(), 1);
    assert_eq!(fixture.authority_receipt().source().root_reference(), 1);
    assert!(fixture
        .authority_receipt()
        .reopened_through_physical_authority());
    assert_eq!(
        fixture
            .authority_receipt()
            .construction_proof()
            .basis()
            .basis()
            .value()
            .semantic_digest(),
        fixture.manifest().semantic_digest()
    );
    assert!(fixture
        .manifest()
        .scale()
        .declares_larger_than_memory_shape());
    assert_eq!(
        fixture
            .manifest()
            .artifact_catalog()
            .root_manifest_candidates(),
        1
    );
    assert_eq!(fixture.manifest().artifact_catalog().persisted_pages(), 1);
    assert_eq!(fixture.manifest().artifact_catalog().persisted_extents(), 1);
    assert_eq!(
        fixture
            .manifest()
            .artifact_catalog()
            .discovered_references(),
        4
    );
    assert_eq!(
        declared_capability_boundaries(&fixture),
        expected_declared_boundaries()
    );
    assert_eq!(
        manifest_mutation_boundaries(&fixture),
        expected_declared_boundaries()
    );
}

#[test]
fn fixture_manifest_digest_is_reopen_stable_and_identity_sensitive() {
    let first = production_backed_fixture(1, LargeStoreFixtureProfile::CheckpointHeavy);
    let repeated = production_backed_fixture(1, LargeStoreFixtureProfile::CheckpointHeavy);
    let different_root = production_backed_fixture(2, LargeStoreFixtureProfile::CheckpointHeavy);

    assert_eq!(
        first.manifest().semantic_digest(),
        repeated.manifest().semantic_digest()
    );
    assert_ne!(
        first.manifest().semantic_digest(),
        different_root.manifest().semantic_digest()
    );
}

#[test]
fn all_large_store_profiles_declare_scale_relative_to_budget() {
    for profile in LargeStoreFixtureProfile::ALL {
        let fixture = production_backed_fixture(profile as u64 + 10, profile);
        let scale = fixture.manifest().scale();

        assert_eq!(scale.profile(), profile);
        assert_eq!(fixture.authority_receipt().profile(), profile);
        assert_eq!(fixture.authority_receipt().scale(), scale);
        assert!(scale.declares_larger_than_memory_shape());
        assert!(scale.declared_store_bytes() > 0);
        assert!(scale.resident_memory_budget_bytes() > 0);

        if profile == LargeStoreFixtureProfile::BlobLargerThanMemoryReadiness {
            assert_eq!(
                scale.non_claim(),
                Some(FixtureProfileNonClaim::BlobCorrectnessNotCertified)
            );
        }
    }
}

#[test]
fn fixtures_expose_only_declared_mutation_boundaries() {
    let fixture = production_backed_fixture(3, LargeStoreFixtureProfile::CompactionHeavy);

    for boundary in declared_capability_boundaries(&fixture) {
        fixture.require_mutation_boundary(boundary).unwrap();
    }
    let denial = fixture
        .require_mutation_boundary(FixtureMutationBoundary::TenantMetadata)
        .unwrap_err();

    assert_eq!(
        denial,
        SyntheticFixtureAuthorityDenied::UndeclaredMutationBoundary(
            FixtureMutationBoundary::TenantMetadata
        )
    );
}

#[test]
fn invalid_root_identity_is_structured_denial_not_panic() {
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

fn production_backed_fixture(
    root_reference: u64,
    profile: LargeStoreFixtureProfile,
) -> forge_store_physical_certification::ProductionBackedPhysicalFixture {
    PhysicalFixtureBuilder::production_backed("phase9-production-backed")
        .materialize_with(
            production_backed_physical_fixture_materialization(profile, root_reference).unwrap(),
        )
        .capability(FixtureCapabilityDeclaration::for_mutation_boundary(
            FixtureMutationBoundary::Manifest,
        ))
        .capability(FixtureCapabilityDeclaration::for_mutation_boundary(
            FixtureMutationBoundary::PageImage,
        ))
        .capability(FixtureCapabilityDeclaration::for_mutation_boundary(
            FixtureMutationBoundary::WalFrame,
        ))
        .and_reopen_through_physical_authority()
        .unwrap()
}

fn declared_capability_boundaries(
    fixture: &forge_store_physical_certification::ProductionBackedPhysicalFixture,
) -> Vec<FixtureMutationBoundary> {
    sorted_boundaries(
        fixture
            .manifest()
            .capability_declarations()
            .iter()
            .map(FixtureCapabilityDeclaration::mutation_boundary),
    )
}

fn manifest_mutation_boundaries(
    fixture: &forge_store_physical_certification::ProductionBackedPhysicalFixture,
) -> Vec<FixtureMutationBoundary> {
    sorted_boundaries(fixture.manifest().mutation_boundaries().iter())
}

fn expected_declared_boundaries() -> Vec<FixtureMutationBoundary> {
    sorted_boundaries([
        FixtureMutationBoundary::Manifest,
        FixtureMutationBoundary::PageImage,
        FixtureMutationBoundary::WalFrame,
    ])
}

fn sorted_boundaries(
    boundaries: impl IntoIterator<Item = FixtureMutationBoundary>,
) -> Vec<FixtureMutationBoundary> {
    let mut boundaries: Vec<_> = boundaries.into_iter().collect();
    boundaries.sort_unstable();
    boundaries
}
