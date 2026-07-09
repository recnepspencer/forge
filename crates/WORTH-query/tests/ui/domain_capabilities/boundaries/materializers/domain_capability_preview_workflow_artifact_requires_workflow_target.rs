use worth_query::facade::{
    materialize_query_preview_workflow_artifact,
    WorthQueryDomainCapabilityTransitionOutcome, WorthQueryLowerRuntimeBoundaryBoundContributionTarget,
    WorthQueryMaterializationReadyWorkflowContribution,
    PreviewWorkflowFoundationArtifact,
};

fn main() {
    let _preview_materializer: fn(
        WorthQueryMaterializationReadyWorkflowContribution<
            WorthQueryLowerRuntimeBoundaryBoundContributionTarget,
        >,
    ) -> WorthQueryDomainCapabilityTransitionOutcome<PreviewWorkflowFoundationArtifact> =
        materialize_query_preview_workflow_artifact;
}
