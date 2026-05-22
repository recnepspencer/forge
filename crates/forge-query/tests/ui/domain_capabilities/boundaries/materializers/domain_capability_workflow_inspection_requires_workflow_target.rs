use forge_query::facade::{
    materialize_query_conflict_inspection_artifact, ForgeQueryDomainCapabilityTransitionOutcome,
    ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
    ForgeQueryMaterializationReadyWorkflowContribution, QueryConflictInspectionArtifact,
};

fn main() {
    let _workflow_inspection_materializer: fn(
        ForgeQueryMaterializationReadyWorkflowContribution<
            ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
        >,
    ) -> ForgeQueryDomainCapabilityTransitionOutcome<QueryConflictInspectionArtifact> =
        materialize_query_conflict_inspection_artifact;
}
