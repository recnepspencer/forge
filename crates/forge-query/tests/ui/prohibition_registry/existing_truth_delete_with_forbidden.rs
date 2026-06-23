use forge_query::facade::runtime::{
    ForgeQueryDeleteMutationBuilder, ForgeQueryExistingTruthTargetBinding, ForgeQueryWorkspace,
};

fn forbidden(mut workspace: ForgeQueryWorkspace, binding: ForgeQueryExistingTruthTargetBinding) {
    let _ = workspace.delete_existing_with(binding, |builder: ForgeQueryDeleteMutationBuilder| {
        builder
    });
}

fn main() {}
