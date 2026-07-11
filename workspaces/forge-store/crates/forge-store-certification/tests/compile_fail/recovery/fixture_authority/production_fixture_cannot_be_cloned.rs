use forge_store_physical_certification::{
    FixtureCapabilityDeclaration, FixtureMutationBoundary, LargeStoreFixtureProfile,
    PhysicalFixtureBuilder, ProductionBackedFixtureMaterialization,
};

fn main() {
    let materialization = ProductionBackedFixtureMaterialization::build_profile(
        LargeStoreFixtureProfile::StoreLargerThanMemory,
        1,
    )
    .unwrap();
    let fixture = PhysicalFixtureBuilder::production_backed("clone-fixture")
        .materialize_with(materialization)
        .capability(FixtureCapabilityDeclaration::for_mutation_boundary(
            FixtureMutationBoundary::Manifest,
        ))
        .and_reopen_through_physical_authority()
        .unwrap();
    let _copied = fixture.clone();
}
