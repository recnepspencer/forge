use worth_query::facade::runtime::{WorthQueryAuthorityLane, WorthQueryPreviewExecutionEvidence, WorthQueryPreviewExecutionKind};

fn main() {
    let _worthd = WorthQueryPreviewExecutionEvidence {
        label: "preview".to_string(),
        kind: WorthQueryPreviewExecutionKind::LivePatch,
        handle_name: "handle".to_string(),
        source_lane: WorthQueryAuthorityLane::AuthoritativeTruth,
        preview_lane: WorthQueryAuthorityLane::PreviewTruth,
        commit_identity: "commit".to_string(),
        aspect_paths: Vec::new(),
        execution_digest: String::new(),
    };
}
