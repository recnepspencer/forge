use worth_query::facade::runtime::{
    WorthQueryAspectMutationBuilder, WorthQueryExistingTruthTargetBinding, WorthQueryWorkspace,
};

fn forbidden(mut workspace: WorthQueryWorkspace, binding: WorthQueryExistingTruthTargetBinding) {
    let _ = workspace.update_existing_verified(
        binding,
        |builder: WorthQueryAspectMutationBuilder| builder,
        |builder: WorthQueryAspectMutationBuilder| builder,
    );
}

fn main() {}
