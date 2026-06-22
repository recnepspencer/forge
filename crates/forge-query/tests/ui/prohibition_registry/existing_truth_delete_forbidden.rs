use forge_query::facade::runtime::{ForgeQueryExistingTruthTargetBinding, ForgeQueryWorkspace};

fn forbidden(mut workspace: ForgeQueryWorkspace, binding: ForgeQueryExistingTruthTargetBinding) {
    let _ = workspace.delete_existing(binding);
}

fn main() {}
