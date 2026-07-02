use forge_store_physical_certification::{
    LargeStoreFixtureProfile, ProductionBackedFixtureMaterialization,
    SyntheticFixtureAuthorityDenied,
};

pub fn production_backed_physical_fixture_materialization(
    profile: LargeStoreFixtureProfile,
    root_reference: u64,
) -> Result<ProductionBackedFixtureMaterialization, SyntheticFixtureAuthorityDenied> {
    ProductionBackedFixtureMaterialization::build_profile(profile, root_reference)
}
