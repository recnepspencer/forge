use forge_query::facade::{AdmittedPreviewWorkflowFoundation, PreviewWorkflowFoundationArtifact};

fn main() {
    let _: fn(&AdmittedPreviewWorkflowFoundation) -> &PreviewWorkflowFoundationArtifact =
        AdmittedPreviewWorkflowFoundation::artifact;
}
