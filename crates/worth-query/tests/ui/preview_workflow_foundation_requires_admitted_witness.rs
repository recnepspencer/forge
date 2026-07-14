use worth_query::facade::policy::{admit_preview_workflow_foundation_request, PreviewSessionPlanBinding, PreviewWorkflowFoundationArtifact, PreviewWorkflowFoundationRequest};

fn main() {
    let _: fn(
        &PreviewSessionPlanBinding,
        PreviewWorkflowFoundationRequest,
    ) -> PreviewWorkflowFoundationArtifact = admit_preview_workflow_foundation_request;
}
