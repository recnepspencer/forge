use worth_query::facade::runtime::{WorthQueryDerivedArtifactBinding, WorthQueryDerivedMaterializationBundle, WorthQueryLiveArtifactBinding, WorthQueryLiveArtifactBundle};

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

fn derived_bundle_fixture() -> WorthQueryDerivedMaterializationBundle {
    panic!("fixture only")
}

fn derived_binding_fixture() -> WorthQueryDerivedArtifactBinding {
    panic!("fixture only")
}

fn live_bundle_fixture() -> WorthQueryLiveArtifactBundle {
    panic!("fixture only")
}

fn live_binding_fixture() -> WorthQueryLiveArtifactBinding {
    panic!("fixture only")
}
