use forge_query::facade::runtime::{
    ForgeQueryAspectMutationBuilder, ForgeQueryExistingTruthTargetBinding, ForgeQueryWorkspace,
};

fn forbidden(mut workspace: ForgeQueryWorkspace, binding: ForgeQueryExistingTruthTargetBinding) {
    let _ = workspace.assert_existing(binding, |builder: ForgeQueryAspectMutationBuilder| builder);
}

fn main() {}
