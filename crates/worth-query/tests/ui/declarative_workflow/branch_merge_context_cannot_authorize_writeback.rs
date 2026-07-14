use worth_query::facade::workflow::{
    WorthQueryBranchMergeContext, WorthQueryWritebackDeclaration,
};

fn cannot_writeback(
    declaration: WorthQueryWritebackDeclaration,
    context: WorthQueryBranchMergeContext,
) {
    let _request = declaration.using(context);
}

fn main() {}
