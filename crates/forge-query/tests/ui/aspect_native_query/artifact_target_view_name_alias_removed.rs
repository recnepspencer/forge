use forge_query::facade::{
    ForgeQueryDerivedMaterializationTarget, ForgeQueryLiveArtifactTarget,
};

fn main() {
    let derived_target = derived_target_fixture();
    let live_target = live_target_fixture();

    let _ = derived_target.view_name();
    let _ = live_target.view_name();
}

fn derived_target_fixture() -> ForgeQueryDerivedMaterializationTarget {
    panic!("fixture only")
}

fn live_target_fixture() -> ForgeQueryLiveArtifactTarget {
    panic!("fixture only")
}
