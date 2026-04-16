use forge_query::facade::{
    admit_preview_workflow_foundation, PreviewSessionPlanBinding, PreviewWorkflowFoundationArtifact,
};

fn main() {
    let _: fn(&PreviewSessionPlanBinding) -> PreviewWorkflowFoundationArtifact =
        admit_preview_workflow_foundation;
}
