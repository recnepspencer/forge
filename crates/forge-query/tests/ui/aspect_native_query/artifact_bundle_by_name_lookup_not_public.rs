use forge_query::facade::{
    ForgeQueryDerivedArtifactBinding, ForgeQueryDerivedMaterializationBundle,
    ForgeQueryLiveArtifactBinding, ForgeQueryLiveArtifactBundle,
};

fn main() {
    let derived_bundle = derived_bundle_fixture();
    let _ = derived_bundle.materialization_by_name("derived.example");
    let _ = derived_bundle.target_view_names();

    let derived_binding = derived_binding_fixture();
    let _ = derived_binding.materialization_by_name("derived.example");
    let _ = derived_binding.target_view_names();

    let live_bundle = live_bundle_fixture();
    let _ = live_bundle.read_by_name("live.example");
    let _ = live_bundle.target_view_names();

    let live_binding = live_binding_fixture();
    let _ = live_binding.read_by_name("live.example");
    let _ = live_binding.target_view_names();
}

fn derived_bundle_fixture() -> ForgeQueryDerivedMaterializationBundle {
    panic!("fixture only")
}

fn derived_binding_fixture() -> ForgeQueryDerivedArtifactBinding {
    panic!("fixture only")
}

fn live_bundle_fixture() -> ForgeQueryLiveArtifactBundle {
    panic!("fixture only")
}

fn live_binding_fixture() -> ForgeQueryLiveArtifactBinding {
    panic!("fixture only")
}
