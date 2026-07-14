use worth_query::facade::runtime::{WorkflowDeclarationFamily, WorkflowDeclarationRequest};

fn main() {
    let _ = WorkflowDeclarationRequest::new_with_bool(
        WorkflowDeclarationFamily::MergeLoweringNarrow,
        true,
    );
}
