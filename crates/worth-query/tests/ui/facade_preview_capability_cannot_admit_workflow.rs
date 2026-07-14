use worth_query::facade::foundation::WorthQueryApplicationFacade;
use worth_query::facade::runtime::{WorkflowAuthorityTargetFamily, WorkflowBudgetClass, WorkflowCostClass, WorkflowDeclarationFamily, WorkflowDeclarationRequest, WorkflowFreshnessPolicy};

fn main() {
    let facade = WorthQueryApplicationFacade::runtime_backed_default();
    let preview = facade.preview_query_capability().unwrap();
    let binding: worth_query::facade::runtime::WorkflowContextBinding = todo!();
    let request = WorkflowDeclarationRequest::new(
        WorkflowDeclarationFamily::ConflictInspectionNarrow,
        WorkflowAuthorityTargetFamily::QueryInspection,
        WorkflowCostClass::InspectionNarrow,
        WorkflowBudgetClass::InspectionBounded,
        WorkflowFreshnessPolicy::ExactBasis,
    );

    let _ = preview.capability().admit_declaration(&binding, request);
}
