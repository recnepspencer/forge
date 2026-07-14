use worth_query::facade::foundation::ExecutionPreflightBundle;
use worth_query::facade::runtime::{admit_query_workflow_declaration, WorkflowAuthorityTargetFamily, WorkflowBudgetClass, WorkflowCostClass, WorkflowDeclarationFamily, WorkflowDeclarationRequest, WorkflowFreshnessPolicy};

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
