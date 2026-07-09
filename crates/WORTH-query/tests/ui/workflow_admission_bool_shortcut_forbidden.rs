use worth_query::facade::{WorkflowDeclarationFamily, WorkflowDeclarationRequest};

fn main() {
    let _ = WorkflowDeclarationRequest::new_with_bool(
        WorkflowDeclarationFamily::MergeLoweringNarrow,
        true,
    );
}
