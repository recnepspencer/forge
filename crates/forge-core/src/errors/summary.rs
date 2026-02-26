//! Serializable error-summary types for audit artifacts and machine-readable logs.
//!
//! These summaries are intended for audit/replay records and other serialized
//! outputs where callers need typed failure semantics without relying on
//! `Display` strings or carrying full runtime state.

use serde::{Deserialize, Serialize};

use super::schema::{
    AmbiguousResult, DiagnosticPayload, ErrorContext, KernelError, MergeError, TopologyError,
    PersistentResolutionRole, PersistentResolutionIncompatibility,
};
use crate::tracing::ResolutionQuerySummary;

/// Broad category for a serialized error summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorCategory {
    Kernel,
}

/// Optional non-kernel source error captured alongside a kernel error summary.
///
/// This is reserved for future source-chain enrichment in audit artifacts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceErrorSummary {
    Opaque {
        type_name: String,
        message: String,
    },
}

/// Top-level serializable error summary envelope.
///
/// `human_message` is a convenience field only. Consumers must use the typed
/// variant summaries (`kernel`) for critical logic.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorSummary {
    pub category: ErrorCategory,
    pub kernel: Option<KernelErrorSummary>,
    #[serde(default)]
    pub source_chain: Vec<SourceErrorSummary>,
    pub human_message: Option<String>,
}

impl From<&KernelError> for ErrorSummary {
    fn from(value: &KernelError) -> Self {
        Self {
            category: ErrorCategory::Kernel,
            kernel: Some(KernelErrorSummary::from(value)),
            source_chain: Vec::new(),
            human_message: Some(value.to_string()),
        }
    }
}

/// Serializable typed summary of `KernelError`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum KernelErrorSummary {
    TopologyViolation {
        err: TopologyErrorSummary,
        context: Option<ErrorContext>,
    },
    AmbiguousResult {
        result: AmbiguousResult,
        context: Option<ErrorContext>,
    },
    ToleranceExceeded {
        location: [f64; 3],
        margin: f64,
        message: String,
        context: Option<ErrorContext>,
    },
    PrecisionEscalation {
        bit_length: u32,
        threshold: u32,
        context: Option<ErrorContext>,
    },
    InvalidInput {
        message: String,
        context: Option<ErrorContext>,
    },
    InternalError {
        message: String,
        context: Option<ErrorContext>,
    },
    InvalidConfig {
        field: String,
        reason: String,
    },
    DiagnosticFailure {
        payload: DiagnosticPayloadSummary,
        source: Box<KernelErrorSummary>,
    },
    ReplayMismatch {
        expected: String,
        actual: String,
        context: Option<ErrorContext>,
    },
    MergeFailure(MergeErrorSummary),
}

impl From<&KernelError> for KernelErrorSummary {
    fn from(value: &KernelError) -> Self {
        match value {
            KernelError::TopologyViolation { err, context } => Self::TopologyViolation {
                err: TopologyErrorSummary::from(err),
                context: context.clone(),
            },
            KernelError::AmbiguousResult { result, context } => Self::AmbiguousResult {
                result: result.clone(),
                context: context.clone(),
            },
            KernelError::ToleranceExceeded { location, margin, message, context } => Self::ToleranceExceeded {
                location: *location,
                margin: *margin,
                message: message.clone(),
                context: context.clone(),
            },
            KernelError::PrecisionEscalation { bit_length, threshold, context } => Self::PrecisionEscalation {
                bit_length: *bit_length,
                threshold: *threshold,
                context: context.clone(),
            },
            KernelError::InvalidInput { message, context } => Self::InvalidInput {
                message: message.clone(),
                context: context.clone(),
            },
            KernelError::InternalError { message, context } => Self::InternalError {
                message: message.clone(),
                context: context.clone(),
            },
            KernelError::InvalidConfig { field, reason } => Self::InvalidConfig {
                field: field.clone(),
                reason: reason.clone(),
            },
            KernelError::DiagnosticFailure { payload, source } => Self::DiagnosticFailure {
                payload: DiagnosticPayloadSummary::from(payload),
                source: Box::new(KernelErrorSummary::from(source.as_ref())),
            },
            KernelError::ReplayMismatch { expected, actual, context } => Self::ReplayMismatch {
                expected: expected.clone(),
                actual: actual.clone(),
                context: context.clone(),
            },
            KernelError::MergeFailure(err) => Self::MergeFailure(MergeErrorSummary::from(err)),
        }
    }
}

/// Serializable typed summary of `MergeError`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MergeErrorSummary {
    AmbiguousRadialSelection { edge_index: u32, valence: u32 },
    SelectedUsesNotSheetLike { edge_index: u32 },
    ProtectedUseConflict { face_index: u32, edge_index: Option<u32> },
    WouldDisconnectSheet { face_index: u32 },
    BoundaryCertificationFailed { reason: String, witness: Option<[f64; 2]> },
    PartialMergePlanRejected { step_index: Option<u32>, reason: String },
    PersistentResolutionMissing {
        role: PersistentResolutionRole,
        query: ResolutionQuerySummary,
    },
    PersistentResolutionAmbiguous {
        role: PersistentResolutionRole,
        candidate_count: u32,
        query: ResolutionQuerySummary,
    },
    PersistentResolutionIncompatible {
        role: PersistentResolutionRole,
        incompatibility: PersistentResolutionIncompatibility,
        query: ResolutionQuerySummary,
    },
    UnsupportedPersistentNmtOutput,
}

impl From<&MergeError> for MergeErrorSummary {
    fn from(value: &MergeError) -> Self {
        match value {
            MergeError::AmbiguousRadialSelection { edge_index, valence } => {
                Self::AmbiguousRadialSelection { edge_index: *edge_index, valence: *valence }
            }
            MergeError::SelectedUsesNotSheetLike { edge_index } => {
                Self::SelectedUsesNotSheetLike { edge_index: *edge_index }
            }
            MergeError::ProtectedUseConflict { face_index, edge_index } => {
                Self::ProtectedUseConflict { face_index: *face_index, edge_index: *edge_index }
            }
            MergeError::WouldDisconnectSheet { face_index } => {
                Self::WouldDisconnectSheet { face_index: *face_index }
            }
            MergeError::BoundaryCertificationFailed { reason, witness } => {
                Self::BoundaryCertificationFailed { reason: reason.clone(), witness: *witness }
            }
            MergeError::PartialMergePlanRejected { step_index, reason } => {
                Self::PartialMergePlanRejected { step_index: *step_index, reason: reason.clone() }
            }
            MergeError::PersistentResolutionMissing { role, query } => {
                Self::PersistentResolutionMissing { role: *role, query: query.clone() }
            }
            MergeError::PersistentResolutionAmbiguous { role, candidate_count, query } => {
                Self::PersistentResolutionAmbiguous {
                    role: *role,
                    candidate_count: *candidate_count,
                    query: query.clone(),
                }
            }
            MergeError::PersistentResolutionIncompatible { role, incompatibility, query } => {
                Self::PersistentResolutionIncompatible {
                    role: *role,
                    incompatibility: incompatibility.clone(),
                    query: query.clone(),
                }
            }
            MergeError::UnsupportedPersistentNmtOutput => Self::UnsupportedPersistentNmtOutput,
        }
    }
}

/// Serializable typed summary of `TopologyError`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TopologyErrorSummary {
    MissingTwin { halfedge_index: u32 },
    BrokenLoop { face_index: u32, starting_halfedge: u32 },
    EulerFormulaViolation {
        vertices: usize,
        edges: usize,
        faces: usize,
        expected_chi: i64,
        actual_chi: i64,
    },
    NonManifoldEdge { edge_index: u32, valence: usize },
    GeneralizedEulerViolation {
        shell_index: u32,
        vertices: usize,
        edges: usize,
        faces: usize,
        genus: usize,
        rings: usize,
        expected_chi: i64,
        actual_chi: i64,
    },
    OrientationInconsistency { face_index: u32 },
    StaleHandle {
        entity_kind: String,
        index: u32,
        expected_generation: u32,
        actual_generation: u32,
    },
    ZeroAreaFace { face_index: u32, computed_area: f64, threshold: f64 },
    ZeroLengthEdge { halfedge_index: u32, computed_length: f64, threshold: f64 },
    NegativeShellVolume { shell_index: u32, signed_volume: f64 },
    DegenerateLoop { face_index: u32, distinct_vertices: usize },
    LoopCorruption {
        walk_kind: String,
        seed_index: u32,
        last_visited_index: u32,
        steps_taken: usize,
        entity_bound: usize,
    },
    MissingVertexPosition { vertex_index: u32, face_index: u32 },
    NonOrientableSurface { shell_index: u32 },
    BoundaryEdgeInSolid { halfedge_index: u32, shell_index: u32 },
    InvalidOperation { detail: String },
    HierarchyViolation {
        parent_kind: String,
        parent_index: u32,
        child_kind: String,
        child_index: u32,
        detail: String,
    },
    RadialEdgeInconsistency {
        halfedge_index: u32,
        actual_edge: u32,
        seed_halfedge_index: u32,
        expected_edge: u32,
    },
}

impl From<&TopologyError> for TopologyErrorSummary {
    fn from(value: &TopologyError) -> Self {
        match value {
            TopologyError::MissingTwin { halfedge_index } => Self::MissingTwin { halfedge_index: *halfedge_index },
            TopologyError::BrokenLoop { face_index, starting_halfedge } => {
                Self::BrokenLoop { face_index: *face_index, starting_halfedge: *starting_halfedge }
            }
            TopologyError::EulerFormulaViolation { vertices, edges, faces, expected_chi, actual_chi } => {
                Self::EulerFormulaViolation {
                    vertices: *vertices,
                    edges: *edges,
                    faces: *faces,
                    expected_chi: *expected_chi,
                    actual_chi: *actual_chi,
                }
            }
            TopologyError::NonManifoldEdge { edge_index, valence } => {
                Self::NonManifoldEdge { edge_index: *edge_index, valence: *valence }
            }
            TopologyError::GeneralizedEulerViolation { shell_index, vertices, edges, faces, genus, rings, expected_chi, actual_chi } => {
                Self::GeneralizedEulerViolation {
                    shell_index: *shell_index,
                    vertices: *vertices,
                    edges: *edges,
                    faces: *faces,
                    genus: *genus,
                    rings: *rings,
                    expected_chi: *expected_chi,
                    actual_chi: *actual_chi,
                }
            }
            TopologyError::OrientationInconsistency { face_index } => {
                Self::OrientationInconsistency { face_index: *face_index }
            }
            TopologyError::StaleHandle { entity_kind, index, expected_generation, actual_generation } => {
                Self::StaleHandle {
                    entity_kind: entity_kind.clone(),
                    index: *index,
                    expected_generation: *expected_generation,
                    actual_generation: *actual_generation,
                }
            }
            TopologyError::ZeroAreaFace { face_index, computed_area, threshold } => {
                Self::ZeroAreaFace { face_index: *face_index, computed_area: *computed_area, threshold: *threshold }
            }
            TopologyError::ZeroLengthEdge { halfedge_index, computed_length, threshold } => {
                Self::ZeroLengthEdge { halfedge_index: *halfedge_index, computed_length: *computed_length, threshold: *threshold }
            }
            TopologyError::NegativeShellVolume { shell_index, signed_volume } => {
                Self::NegativeShellVolume { shell_index: *shell_index, signed_volume: *signed_volume }
            }
            TopologyError::DegenerateLoop { face_index, distinct_vertices } => {
                Self::DegenerateLoop { face_index: *face_index, distinct_vertices: *distinct_vertices }
            }
            TopologyError::LoopCorruption { walk_kind, seed_index, last_visited_index, steps_taken, entity_bound } => {
                Self::LoopCorruption {
                    walk_kind: walk_kind.clone(),
                    seed_index: *seed_index,
                    last_visited_index: *last_visited_index,
                    steps_taken: *steps_taken,
                    entity_bound: *entity_bound,
                }
            }
            TopologyError::MissingVertexPosition { vertex_index, face_index } => {
                Self::MissingVertexPosition { vertex_index: *vertex_index, face_index: *face_index }
            }
            TopologyError::NonOrientableSurface { shell_index } => {
                Self::NonOrientableSurface { shell_index: *shell_index }
            }
            TopologyError::BoundaryEdgeInSolid { halfedge_index, shell_index } => {
                Self::BoundaryEdgeInSolid { halfedge_index: *halfedge_index, shell_index: *shell_index }
            }
            TopologyError::InvalidOperation { detail } => {
                Self::InvalidOperation { detail: detail.clone() }
            }
            TopologyError::HierarchyViolation { parent_kind, parent_index, child_kind, child_index, detail } => {
                Self::HierarchyViolation {
                    parent_kind: parent_kind.clone(),
                    parent_index: *parent_index,
                    child_kind: child_kind.clone(),
                    child_index: *child_index,
                    detail: detail.clone(),
                }
            }
            TopologyError::RadialEdgeInconsistency { halfedge_index, actual_edge, seed_halfedge_index, expected_edge } => {
                Self::RadialEdgeInconsistency {
                    halfedge_index: *halfedge_index,
                    actual_edge: *actual_edge,
                    seed_halfedge_index: *seed_halfedge_index,
                    expected_edge: *expected_edge,
                }
            }
        }
    }
}

/// Serializable summary of diagnostic replay payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticPayloadSummary {
    pub operation: String,
    pub state_hash: u128,
    pub seed: u64,
    pub context: String,
}

impl From<&DiagnosticPayload> for DiagnosticPayloadSummary {
    fn from(value: &DiagnosticPayload) -> Self {
        Self {
            operation: value.operation.clone(),
            state_hash: value.state_hash,
            seed: value.seed,
            context: value.context.clone(),
        }
    }
}
