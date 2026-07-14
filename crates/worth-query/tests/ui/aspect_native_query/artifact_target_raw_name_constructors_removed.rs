use worth_query::facade::runtime::{WorthQueryDerivedMaterializationTarget, WorthQueryLiveArtifactTarget};

fn main() {
    let _ = WorthQueryDerivedMaterializationTarget::new("computed.title_list");
    let _ = WorthQueryLiveArtifactTarget::new("live.task_table");
}
