use worth_query::facade::runtime::{
    WorthQueryDeleteMutationBuilder, WorthQueryExistingTruthTargetBinding, WorthQueryWorkspace,
};

fn forbidden(mut workspace: WorthQueryWorkspace, binding: WorthQueryExistingTruthTargetBinding) {
    let _ = workspace.delete_existing_with(binding, |builder: WorthQueryDeleteMutationBuilder| {
        builder
    });
}

fn main() {}
