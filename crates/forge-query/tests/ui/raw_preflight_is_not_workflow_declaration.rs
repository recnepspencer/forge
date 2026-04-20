use forge_query::facade::{
    admit_query_workflow_declaration, ExecutionPreflightBundle, WorkflowAuthorityTargetFamily,
    WorkflowBudgetClass, WorkflowCostClass, WorkflowDeclarationFamily,
    WorkflowDeclarationRequest, WorkflowFreshnessPolicy,
};

fn main() {
    let preflight: ExecutionPreflightBundle = todo!();
    let _ = admit_query_workflow_declaration(
        &preflight,
        WorkflowDeclarationRequest::new(
            WorkflowDeclarationFamily::MergeLoweringNarrow,
            WorkflowAuthorityTargetFamily::RelationalMerge,
            WorkflowCostClass::MergeLoweringNarrow,
            WorkflowBudgetClass::AuthorityTargetBounded,
            WorkflowFreshnessPolicy::ExactBasis,
        ),
    );
}
