use worth_relational::facade::history::CommitId;
use worth_relational::facade::runtime::{RelationalRuntime, RelationalRuntimeConfig};

fn main() {
    let runtime = RelationalRuntime::new(RelationalRuntimeConfig::default());
    let _ = runtime.publish_commit_for_bridge(CommitId(1), "model");
}
