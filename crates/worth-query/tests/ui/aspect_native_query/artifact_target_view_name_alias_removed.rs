use worth_query::facade::{
    WorthQueryDerivedMaterializationTarget, WorthQueryLiveArtifactTarget,
};

fn main() {
    let derived_target = derived_target_fixture();
    let live_target = live_target_fixture();

    let _ = derived_target.view_name();
    let _ = live_target.view_name();
}

fn derived_target_fixture() -> WorthQueryDerivedMaterializationTarget {
    panic!("fixture only")
}

fn live_target_fixture() -> WorthQueryLiveArtifactTarget {
    panic!("fixture only")
}
