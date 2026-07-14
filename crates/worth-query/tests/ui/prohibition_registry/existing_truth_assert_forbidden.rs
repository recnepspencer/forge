use worth_query::facade::runtime::{WorthQueryAspectMutationBuilder, WorthQueryExistingTruthTargetBinding, WorthQueryWorkspace};

fn forbidden(mut workspace: WorthQueryWorkspace, binding: WorthQueryExistingTruthTargetBinding) {
    let _ = workspace.assert_existing(binding, |builder: WorthQueryAspectMutationBuilder| builder);
}

fn main() {}
