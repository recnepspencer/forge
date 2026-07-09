use worth_query::facade::{
    materialize_query_workflow_declaration, WorthQueryDomainCapabilityTransitionOutcome,
    WorthQueryLowerRuntimeBoundaryBoundContributionTarget,
    WorthQueryMaterializationReadyWorkflowContribution, QueryWorkflowDeclaration,
};

fn main() {
    let _workflow_materializer: fn(
        WorthQueryMaterializationReadyWorkflowContribution<
            WorthQueryLowerRuntimeBoundaryBoundContributionTarget,
        >,
    ) -> WorthQueryDomainCapabilityTransitionOutcome<QueryWorkflowDeclaration> =
        materialize_query_workflow_declaration;
}
