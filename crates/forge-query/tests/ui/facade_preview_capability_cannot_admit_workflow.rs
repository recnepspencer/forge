use forge_query::facade::{
    ForgeQueryApplicationFacade, WorkflowAuthorityTargetFamily, WorkflowBudgetClass,
    WorkflowCostClass, WorkflowDeclarationFamily, WorkflowDeclarationRequest,
    WorkflowFreshnessPolicy,
};

fn main() {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let preview = facade.preview_query_capability().unwrap();
    let binding: forge_query::facade::WorkflowContextBinding = todo!();
    let request = WorkflowDeclarationRequest::new(
        WorkflowDeclarationFamily::ConflictInspectionNarrow,
        WorkflowAuthorityTargetFamily::QueryInspection,
        WorkflowCostClass::InspectionNarrow,
        WorkflowBudgetClass::InspectionBounded,
        WorkflowFreshnessPolicy::ExactBasis,
    );

    let _ = preview.capability().admit_declaration(&binding, request);
}
