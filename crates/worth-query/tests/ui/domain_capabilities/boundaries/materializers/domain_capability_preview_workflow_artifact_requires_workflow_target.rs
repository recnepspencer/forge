use worth_query::facade::policy::PreviewWorkflowFoundationArtifact;
use worth_query::facade::runtime::{materialize_query_preview_workflow_artifact, WorthQueryDomainCapabilityTransitionOutcome, WorthQueryLowerRuntimeBoundaryBoundContributionTarget, WorthQueryMaterializationReadyWorkflowContribution};

fn main() {
    let _preview_materializer: fn(
        WorthQueryMaterializationReadyWorkflowContribution<
            WorthQueryLowerRuntimeBoundaryBoundContributionTarget,
        >,
    ) -> WorthQueryDomainCapabilityTransitionOutcome<PreviewWorkflowFoundationArtifact> =
        materialize_query_preview_workflow_artifact;
}
