use worth_store_physical_certification::PhysicalFixtureBuilder;

fn main() {
    let _fixture = PhysicalFixtureBuilder::production_backed("label-shortcut")
        .materialize_with("label-is-not-authority");
}
