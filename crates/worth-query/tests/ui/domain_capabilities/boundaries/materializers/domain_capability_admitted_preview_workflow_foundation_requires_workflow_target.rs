use worth_query::facade::policy::AdmittedPreviewWorkflowFoundation;
use worth_query::facade::runtime::{materialize_admitted_preview_workflow_foundation, WorthQueryDomainCapabilityTransitionOutcome, WorthQueryLowerRuntimeBoundaryBoundContributionTarget, WorthQueryMaterializationReadyWorkflowContribution};

fn main() {
    let _preview_materializer: fn(
        WorthQueryMaterializationReadyWorkflowContribution<
            WorthQueryLowerRuntimeBoundaryBoundContributionTarget,
        >,
    ) -> WorthQueryDomainCapabilityTransitionOutcome<AdmittedPreviewWorkflowFoundation> =
        materialize_admitted_preview_workflow_foundation;
}
