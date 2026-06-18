use forge_query::facade::runtime::{ForgeQueryExistingTruthTargetBinding, ForgeQueryWorkspace};

fn forbidden(workspace: ForgeQueryWorkspace, binding: ForgeQueryExistingTruthTargetBinding) {
    let _ = workspace.probe_existing(binding, ["shape.kind"]);
}

fn main() {}
