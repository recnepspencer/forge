use forge_store_physical_certification::PhysicalFixtureBuilder;
use forge_store_physical_format::PersistedPhysicalLayout;

fn main() {
    let raw_layout = PersistedPhysicalLayout::builder().build();
    let _fixture = PhysicalFixtureBuilder::production_backed("raw-layout")
        .materialize_with(raw_layout);
}
