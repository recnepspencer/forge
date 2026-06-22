use forge_query::facade::{ForgeQueryDerivedMaterializationTarget, ForgeQueryLiveArtifactTarget};

fn main() {
    let _ = ForgeQueryDerivedMaterializationTarget::new("computed.title_list");
    let _ = ForgeQueryLiveArtifactTarget::new("live.task_table");
}
