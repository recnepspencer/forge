//! Stub adapters for the UI MVP.
//!
//! DOMAIN: Converts raw/stub data to clean forge-ui-types domain shapes.
//! In MVP this module produces hard-coded stub data. When the kernel API is
//! wired up, only this crate changes — all consumers stay untouched.
//! DEPENDENCIES: forge-ui-types only. No egui, no theme.

use forge_ui_types::{
    ChatMessage, FeatureStatus, KernelTelemetry, MessageContent, MessageRole, UiFeature,
    UiFeatureId, UiFeatureKind, UiPlane,
};

/// Produce a hard-coded stub feature list for the MVP.
pub fn stub_feature_list() -> Vec<UiFeature> {
    vec![
        UiFeature {
            id: UiFeatureId(1),
            name: "Body".to_string(),
            kind: UiFeatureKind::MakeCube,
            status: FeatureStatus::Exact,
            children: vec![],
        },
        UiFeature {
            id: UiFeatureId(2),
            name: "Notch".to_string(),
            kind: UiFeatureKind::BooleanSubtract,
            status: FeatureStatus::NearBoundary,
            children: vec![],
        },
        UiFeature {
            id: UiFeatureId(3),
            name: "Chamfer (planned)".to_string(),
            kind: UiFeatureKind::Other("Chamfer".to_string()),
            status: FeatureStatus::Pending,
            children: vec![],
        },
    ]
}

/// Produce stub planes for the cube faces.
pub fn stub_planes() -> Vec<UiPlane> {
    vec![
        UiPlane {
            name: "+X face".to_string(),
            normal: [1.0, 0.0, 0.0],
            offset: 1.0,
            status: FeatureStatus::Exact,
        },
        UiPlane {
            name: "-X face".to_string(),
            normal: [-1.0, 0.0, 0.0],
            offset: 1.0,
            status: FeatureStatus::Exact,
        },
        UiPlane {
            name: "+Y face".to_string(),
            normal: [0.0, 1.0, 0.0],
            offset: 1.0,
            status: FeatureStatus::Exact,
        },
        UiPlane {
            name: "-Y face".to_string(),
            normal: [0.0, -1.0, 0.0],
            offset: 1.0,
            status: FeatureStatus::Exact,
        },
        UiPlane {
            name: "+Z face".to_string(),
            normal: [0.0, 0.0, 1.0],
            offset: 1.0,
            status: FeatureStatus::Exact,
        },
        UiPlane {
            name: "-Z face".to_string(),
            normal: [0.0, 0.0, -1.0],
            offset: 1.0,
            status: FeatureStatus::Exact,
        },
    ]
}

/// Produce stub chat history for the MVP.
pub fn stub_chat_history() -> Vec<ChatMessage> {
    vec![
        ChatMessage {
            role: MessageRole::System,
            content: MessageContent::Text(
                "Forge ready. You are working on **my_model.fg** (planar MVP).".to_string(),
            ),
            timestamp_secs: 0,
        },
        ChatMessage {
            role: MessageRole::Agent,
            content: MessageContent::Text(
                "I built a unit cube from 6 half-space planes. All 6 faces are classified **Exact** — no precision escalation required. What would you like to do next?".to_string(),
            ),
            timestamp_secs: 1,
        },
    ]
}

/// Produce stub kernel telemetry for the status bar.
pub fn stub_telemetry() -> KernelTelemetry {
    KernelTelemetry {
        face_count: 6,
        vertex_count: 8,
        edge_count: 12,
        last_op_ms: 1.2,
        precision_mode: "Exact".to_string(),
    }
}
