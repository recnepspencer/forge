use worth_query::facade::{
    materialize_query_conflict_inspection_artifact, WorthQueryDomainCapabilityTransitionOutcome,
    WorthQueryLowerRuntimeBoundaryBoundContributionTarget,
    WorthQueryMaterializationReadyWorkflowContribution, QueryConflictInspectionArtifact,
};

fn main() {
    let _workflow_inspection_materializer: fn(
        WorthQueryMaterializationReadyWorkflowContribution<
            WorthQueryLowerRuntimeBoundaryBoundContributionTarget,
        >,
    ) -> WorthQueryDomainCapabilityTransitionOutcome<QueryConflictInspectionArtifact> =
        materialize_query_conflict_inspection_artifact;
}
