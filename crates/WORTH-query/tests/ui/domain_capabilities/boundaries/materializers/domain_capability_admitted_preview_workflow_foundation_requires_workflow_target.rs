use worth_query::facade::{
    materialize_admitted_preview_workflow_foundation, AdmittedPreviewWorkflowFoundation,
    WorthQueryDomainCapabilityTransitionOutcome, WorthQueryLowerRuntimeBoundaryBoundContributionTarget,
    WorthQueryMaterializationReadyWorkflowContribution,
};

fn main() {
    let _preview_materializer: fn(
        WorthQueryMaterializationReadyWorkflowContribution<
            WorthQueryLowerRuntimeBoundaryBoundContributionTarget,
        >,
    ) -> WorthQueryDomainCapabilityTransitionOutcome<AdmittedPreviewWorkflowFoundation> =
        materialize_admitted_preview_workflow_foundation;
}
