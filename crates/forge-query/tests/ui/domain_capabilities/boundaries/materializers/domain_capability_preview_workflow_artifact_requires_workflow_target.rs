use forge_query::facade::{
    materialize_query_preview_workflow_artifact,
    ForgeQueryDomainCapabilityTransitionOutcome, ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
    ForgeQueryMaterializationReadyWorkflowContribution,
    PreviewWorkflowFoundationArtifact,
};

fn main() {
    let _preview_materializer: fn(
        ForgeQueryMaterializationReadyWorkflowContribution<
            ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
        >,
    ) -> ForgeQueryDomainCapabilityTransitionOutcome<PreviewWorkflowFoundationArtifact> =
        materialize_query_preview_workflow_artifact;
}
