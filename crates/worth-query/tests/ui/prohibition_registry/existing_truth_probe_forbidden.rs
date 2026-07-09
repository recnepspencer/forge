use worth_query::facade::runtime::{WorthQueryExistingTruthTargetBinding, WorthQueryWorkspace};

fn forbidden(workspace: WorthQueryWorkspace, binding: WorthQueryExistingTruthTargetBinding) {
    let _ = workspace.probe_existing(binding, ["shape.kind"]);
}

fn main() {}
