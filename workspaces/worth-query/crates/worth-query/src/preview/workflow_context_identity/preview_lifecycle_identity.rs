use worth_runtime_bridge::facade::{BridgePreviewLifecycleStateKind, BridgePreviewSessionIdentity};

pub(in crate::preview) fn preview_session_identity_record_label(
    identity: &BridgePreviewSessionIdentity,
) -> &str {
    identity.terminal_projection_for_reporting()
}

pub(crate) fn preview_lifecycle_state_label(kind: BridgePreviewLifecycleStateKind) -> &'static str {
    match kind {
        BridgePreviewLifecycleStateKind::Declared => "declared",
        BridgePreviewLifecycleStateKind::Admitted => "admitted",
        BridgePreviewLifecycleStateKind::Active => "active",
        BridgePreviewLifecycleStateKind::Discarded => "discarded",
        BridgePreviewLifecycleStateKind::Promoted => "promoted",
    }
}
