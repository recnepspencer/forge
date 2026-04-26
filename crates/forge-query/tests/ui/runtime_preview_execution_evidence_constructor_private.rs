use forge_query::facade::{
    ForgeQueryAuthorityLane, ForgeQueryPreviewExecutionEvidence, ForgeQueryPreviewExecutionKind,
};

fn main() {
    let _forged = ForgeQueryPreviewExecutionEvidence {
        label: "preview".to_string(),
        kind: ForgeQueryPreviewExecutionKind::LivePatch,
        handle_name: "handle".to_string(),
        source_lane: ForgeQueryAuthorityLane::AuthoritativeTruth,
        preview_lane: ForgeQueryAuthorityLane::PreviewTruth,
        commit_identity: "commit".to_string(),
        aspect_paths: Vec::new(),
        execution_digest: String::new(),
    };
}
