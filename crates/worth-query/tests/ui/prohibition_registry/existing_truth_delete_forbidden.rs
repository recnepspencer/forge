use worth_query::facade::runtime::{WorthQueryExistingTruthTargetBinding, WorthQueryWorkspace};

fn forbidden(mut workspace: WorthQueryWorkspace, binding: WorthQueryExistingTruthTargetBinding) {
    let _ = workspace.delete_existing(binding);
}

fn main() {}
