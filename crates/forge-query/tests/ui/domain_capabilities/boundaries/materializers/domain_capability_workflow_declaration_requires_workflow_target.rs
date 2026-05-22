use forge_query::facade::{
    materialize_query_workflow_declaration, ForgeQueryDomainCapabilityTransitionOutcome,
    ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
    ForgeQueryMaterializationReadyWorkflowContribution, QueryWorkflowDeclaration,
};

fn main() {
    let _workflow_materializer: fn(
        ForgeQueryMaterializationReadyWorkflowContribution<
            ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
        >,
    ) -> ForgeQueryDomainCapabilityTransitionOutcome<QueryWorkflowDeclaration> =
        materialize_query_workflow_declaration;
}
