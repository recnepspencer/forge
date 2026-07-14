use worth_store_physical_certification::{
    FixtureMutationBoundary, LargeStoreFixtureProfile, PhysicalFixtureBuilder,
    ProductionBackedFixtureMaterialization,
};

fn main() {
    let materialization = ProductionBackedFixtureMaterialization::build_profile(
        LargeStoreFixtureProfile::StoreLargerThanMemory,
        1,
    )
    .unwrap();
    let _fixture = PhysicalFixtureBuilder::production_backed("raw-boundary")
        .materialize_with(materialization)
        .mutation_boundary(FixtureMutationBoundary::Manifest);
}
