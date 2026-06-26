//! Forge UI domain types.
//!
//! DOMAIN: UI-owned shapes that are stable contracts between the kernel and
//! the presentation layer. These types are never the raw kernel API shapes.
//! DEPENDENCIES: serde only.

/// A unique identifier for a UI-level feature node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct UiFeatureId(pub u64);

/// Status of a feature in the kernel, as the UI cares about it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum FeatureStatus {
    /// Resolved exactly — no ambiguity.
    Exact,
    /// Resolved but close to a tolerance boundary.
    NearBoundary,
    /// Failed with a human-readable message.
    Error(String),
    /// Not yet evaluated.
    Pending,
}

/// A feature node as shown in the feature tree.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UiFeature {
    pub id: UiFeatureId,
    pub name: String,
    pub kind: UiFeatureKind,
    pub status: FeatureStatus,
    /// Child features (nested ops).
    pub children: Vec<UiFeature>,
}

/// The kind of geometric operation this feature represents.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum UiFeatureKind {
    MakeCube,
    Plane,
    BooleanUnion,
    BooleanSubtract,
    BooleanIntersect,
    Other(String),
}

/// A planar face as the UI wants to interact with it.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UiPlane {
    pub name: String,
    /// Unit normal vector.
    pub normal: [f32; 3],
    /// Signed offset from origin: `dot(normal, p) = offset`.
    pub offset: f32,
    pub status: FeatureStatus,
}

/// A chat message role.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum MessageRole {
    User,
    Agent,
    System,
}

/// The content body of a chat message.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum MessageContent {
    Text(String),
    CodeBlock { language: String, source: String },
    KernelEvent(String),
}

/// A single chat message.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: MessageContent,
    /// Wall-clock epoch seconds (for display only — never used for ordering).
    pub timestamp_secs: u64,
}

/// Kernel telemetry summary for the status bar.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct KernelTelemetry {
    pub face_count: usize,
    pub vertex_count: usize,
    pub edge_count: usize,
    pub last_op_ms: f64,
    pub precision_mode: String,
}
