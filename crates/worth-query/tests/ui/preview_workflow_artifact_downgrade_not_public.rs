use worth_query::facade::policy::{AdmittedPreviewWorkflowFoundation, PreviewWorkflowFoundationArtifact};

fn main() {
    let _: fn(&AdmittedPreviewWorkflowFoundation) -> &PreviewWorkflowFoundationArtifact =
        AdmittedPreviewWorkflowFoundation::artifact;
}
