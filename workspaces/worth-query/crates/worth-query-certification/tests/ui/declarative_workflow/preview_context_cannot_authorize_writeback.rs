use worth_query::facade::workflow::{
    WorthQueryWorkflowContext, WorthQueryWritebackDeclaration,
};

fn cannot_writeback(
    declaration: WorthQueryWritebackDeclaration,
    context: WorthQueryWorkflowContext,
) {
    let _request = declaration.using(context);
}

fn main() {}
