use worth_store_physical_certification::{
    LargeStoreFixtureProfile, PhysicalFixtureBuilder, ProductionBackedFixtureMaterialization,
};

fn main() {
    let materialization = ProductionBackedFixtureMaterialization::build_profile(
        LargeStoreFixtureProfile::StoreLargerThanMemory,
        1,
    )
    .unwrap();
    let _fixture = PhysicalFixtureBuilder::production_backed("missing-boundary")
        .materialize_with(materialization)
        .and_reopen_through_physical_authority();
}
