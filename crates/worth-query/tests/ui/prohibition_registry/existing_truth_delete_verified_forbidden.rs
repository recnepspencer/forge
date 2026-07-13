use worth_query::facade::runtime::{WorthQueryAspectMutationBuilder, WorthQueryDeleteMutationBuilder, WorthQueryExistingTruthTargetBinding, WorthQueryWorkspace};

fn forbidden(mut workspace: WorthQueryWorkspace, binding: WorthQueryExistingTruthTargetBinding) {
    let _ = workspace.delete_existing_verified(
        binding,
        |builder: WorthQueryAspectMutationBuilder| builder,
        |builder: WorthQueryDeleteMutationBuilder| builder,
    );
}

fn main() {}
