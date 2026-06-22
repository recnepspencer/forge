use forge_query::facade::{ForgeQueryDerivedMaterializationBundle, ForgeQueryLiveArtifactBundle};

fn main() {
    let derived = derived_bundle_fixture();
    let _ = derived.includes_view_name("derived.example");

    let live = live_bundle_fixture();
    let _ = live.includes_view_name("live.example");
}

fn derived_bundle_fixture() -> ForgeQueryDerivedMaterializationBundle {
    panic!("fixture only")
}

fn live_bundle_fixture() -> ForgeQueryLiveArtifactBundle {
    panic!("fixture only")
}
