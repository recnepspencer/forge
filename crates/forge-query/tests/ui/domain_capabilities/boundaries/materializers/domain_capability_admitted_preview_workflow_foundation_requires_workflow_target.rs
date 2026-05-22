use forge_query::facade::{
    materialize_admitted_preview_workflow_foundation, AdmittedPreviewWorkflowFoundation,
    ForgeQueryDomainCapabilityTransitionOutcome, ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
    ForgeQueryMaterializationReadyWorkflowContribution,
};

fn main() {
    let _preview_materializer: fn(
        ForgeQueryMaterializationReadyWorkflowContribution<
            ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
        >,
    ) -> ForgeQueryDomainCapabilityTransitionOutcome<AdmittedPreviewWorkflowFoundation> =
        materialize_admitted_preview_workflow_foundation;
}
