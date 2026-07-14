use worth_query::facade::workflow::{
    WorthQueryBranchMergeDeclaration, WorthQueryWorkflowContext,
};

fn cannot_merge(
    declaration: WorthQueryBranchMergeDeclaration,
    context: WorthQueryWorkflowContext,
) {
    let _request = declaration.using(context);
}

fn main() {}
