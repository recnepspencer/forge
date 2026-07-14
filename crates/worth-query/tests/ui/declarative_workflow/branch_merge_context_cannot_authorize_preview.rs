use worth_query::facade::workflow::{
    WorthQueryBranchMergeContext, WorthQueryWorkflowDeclaration,
};

fn cannot_preview(
    declaration: WorthQueryWorkflowDeclaration,
    context: WorthQueryBranchMergeContext,
) {
    let _request = declaration.using(context);
}

fn main() {}
