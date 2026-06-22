use forge_query::facade::runtime::{
    ForgeQueryAspectMutationBuilder, ForgeQueryExistingTruthTargetBinding, ForgeQueryWorkspace,
};

fn forbidden(mut workspace: ForgeQueryWorkspace, binding: ForgeQueryExistingTruthTargetBinding) {
    let _ = workspace.update_existing(binding, |builder: ForgeQueryAspectMutationBuilder| builder);
}

fn main() {}
