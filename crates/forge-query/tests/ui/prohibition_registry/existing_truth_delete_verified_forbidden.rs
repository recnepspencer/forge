use forge_query::facade::runtime::{
    ForgeQueryAspectMutationBuilder, ForgeQueryDeleteMutationBuilder,
    ForgeQueryExistingTruthTargetBinding, ForgeQueryWorkspace,
};

fn forbidden(mut workspace: ForgeQueryWorkspace, binding: ForgeQueryExistingTruthTargetBinding) {
    let _ = workspace.delete_existing_verified(
        binding,
        |builder: ForgeQueryAspectMutationBuilder| builder,
        |builder: ForgeQueryDeleteMutationBuilder| builder,
    );
}

fn main() {}
