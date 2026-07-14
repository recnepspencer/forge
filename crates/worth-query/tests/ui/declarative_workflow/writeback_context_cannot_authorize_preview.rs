use worth_query::facade::workflow::{
    WorthQueryWorkflowDeclaration, WorthQueryWritebackContext,
};

fn cannot_promote(
    declaration: WorthQueryWorkflowDeclaration,
    context: WorthQueryWritebackContext,
) {
    let _request = declaration.using(context);
}

fn main() {}
