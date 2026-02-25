//! Error data shapes for the Forge geometry kernel.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::policy::PolicyKind;

// =========================================================================
// STRUCTURED ERROR CONTEXT (Milestone 1B.1)
// =========================================================================

/// Identifies where an error originated in the kernel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorScope {
    /// Global kernel error not tied to a specific operation or entity.
    Global,
    /// Error occurred while processing a specific feature.
    Feature { feature_id: u64 },
    /// Error occurred on a specific topological entity.
    Entity {
        entity_kind: String,
        index: u32,
    },
    /// Error occurred during a specific Euler operation.
    Operation {
        op_name: String,
        invocation_id: u64,
    },
}

/// Machine-actionable remediation hints for an error.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SuggestedFix {
    /// Increase a tolerance threshold.
    IncreaseThreshold {
        parameter: String,
        current: f64,
        suggested: f64,
    },
    /// Reduce a geometric parameter value.
    ReduceValue {
        parameter: String,
        current: f64,
        max_allowed: f64,
    },
    /// Re-run the operation with a different explicit policy.
    RetryWithPolicy { policy_kind: PolicyKind },
    /// Suggest splitting a complex operation into smaller steps.
    SplitOperation,
    /// No automatic fix available; requires manual triage.
    ManualIntervention { description: String },
}

impl fmt::Display for SuggestedFix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SuggestedFix::IncreaseThreshold { parameter, current, suggested } => {
                write!(f, "Increase {} from {:.2e} to {:.2e}", parameter, current, suggested)
            }
            SuggestedFix::ReduceValue { parameter, current, max_allowed } => {
                write!(f, "Reduce {} from {:.2e} to at most {:.2e}", parameter, current, max_allowed)
            }
            SuggestedFix::RetryWithPolicy { policy_kind } => {
                write!(f, "Retry with explicit policy: {:?}", policy_kind)
            }
            SuggestedFix::SplitOperation => {
                write!(f, "Split into smaller operations")
            }
            SuggestedFix::ManualIntervention { description } => {
                write!(f, "{}", description)
            }
        }
    }
}

/// Structured diagnostic context for AI agents and UI.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorContext {
    /// Where the error happened.
    pub scope: ErrorScope,
    /// Automated remediation hints.
    pub suggested_fixes: Vec<SuggestedFix>,
    /// Human-readable detailed explanation.
    pub detail: String,
}

// =========================================================================
// KERNEL ERROR
// =========================================================================

/// The primary error type used across all Forge crates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KernelError {
    /// A topology invariant was violated (e.g., non-manifold edge, broken loop).
    TopologyViolation {
        err: TopologyError,
        context: Option<ErrorContext>,
    },

    /// A geometric result is ambiguous and requires a policy decision.
    AmbiguousResult {
        result: AmbiguousResult,
        context: Option<ErrorContext>,
    },

    /// A tolerance threshold was exceeded during curved geometry operations.
    ToleranceExceeded {
        /// 3D location where the tolerance was exceeded
        location: [f64; 3],
        /// How close to the threshold (lower = more marginal)
        margin: f64,
        /// Description of the violation
        message: String,
        context: Option<ErrorContext>,
    },

    /// Exact arithmetic bit-length exceeded the budget.
    PrecisionEscalation {
        /// Current bit-length of the rational number
        bit_length: u32,
        /// Configured threshold
        threshold: u32,
        context: Option<ErrorContext>,
    },

    /// Invalid input provided to an operation.
    InvalidInput {
        message: String,
        context: Option<ErrorContext>,
    },

    /// Internal error — should never happen in correct code.
    InternalError {
        message: String,
        context: Option<ErrorContext>,
    },

    /// A diagnostic failure wrapping another error with replay context.
    DiagnosticFailure {
        /// Structured context for replay and debugging.
        payload: DiagnosticPayload,
        /// The underlying error that triggered diagnostics.
        source: Box<KernelError>,
    },

    /// Cross-session replay architecture mismatch.
    ///
    /// The `ReplayLog` was recorded on a different target triple than
    /// the current process. FMA and other hardware differences may cause
    /// non-deterministic results.
    ReplayMismatch {
        /// The target triple the log was recorded on.
        expected: String,
        /// The target triple of the current process.
        actual: String,
        context: Option<ErrorContext>,
    },

    /// A region merge operation failed due to a topological or geometric policy violation.
    ///
    /// Carries a structured `MergeError` describing the exact failure mode.
    /// Never downgraded to `InternalError` — the typed variant is preserved through
    /// `with_phase` so callers can match on specific failure modes.
    MergeFailure(MergeError),
}

impl fmt::Display for KernelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KernelError::TopologyViolation { err, .. } => write!(f, "Topology violation: {}", err),
            KernelError::AmbiguousResult { result, .. } => {
                write!(f, "Ambiguous result at [{:.6}, {:.6}, {:.6}]: {}", 
                    result.location[0], result.location[1], result.location[2], result.context)
            }
            KernelError::ToleranceExceeded {
                location,
                margin,
                message,
                ..
            } => {
                write!(
                    f,
                    "Tolerance exceeded at [{:.6}, {:.6}, {:.6}] (margin: {:.2e}): {}",
                    location[0], location[1], location[2], margin, message
                )
            }
            KernelError::PrecisionEscalation {
                bit_length,
                threshold,
                ..
            } => {
                write!(
                    f,
                    "Precision escalation: {} bits exceeds {} bit threshold",
                    bit_length, threshold
                )
            }
            KernelError::InvalidInput { message, .. } => write!(f, "Invalid input: {}", message),
            KernelError::InternalError { message, .. } => write!(f, "Internal error: {}", message),
            KernelError::DiagnosticFailure { payload, source } => {
                write!(
                    f,
                    "Diagnostic failure in '{}' (hash: {:#x}, seed: {}): {}",
                    payload.operation, payload.state_hash, payload.seed, source
                )
            }
            KernelError::ReplayMismatch { expected, actual, .. } => {
                write!(
                    f,
                    "Replay architecture mismatch: log recorded on '{}', current process is '{}'",
                    expected, actual
                )
            }
            KernelError::MergeFailure(err) => write!(f, "Merge failure: {}", err),
        }
    }
}

impl KernelError {
    /// Extract the structured error context, if any variant carries one.
    pub fn get_context(&self) -> Option<&ErrorContext> {
        match self {
            KernelError::TopologyViolation { context, .. }
            | KernelError::AmbiguousResult { context, .. }
            | KernelError::ToleranceExceeded { context, .. }
            | KernelError::PrecisionEscalation { context, .. }
            | KernelError::InvalidInput { context, .. }
            | KernelError::InternalError { context, .. }
            | KernelError::ReplayMismatch { context, .. } => context.as_ref(),
            KernelError::DiagnosticFailure { source, .. } => source.get_context(),
            // MergeError fields are self-describing; no separate ErrorContext attached.
            KernelError::MergeFailure(_) => None,
        }
    }

    /// Wrap this error with a phase label, prefixing its detail string.
    ///
    /// For `MergeFailure`, the typed `MergeError` structure is preserved.
    /// `PartialMergePlanRejected` has its `reason` prefixed with the phase label.
    /// Other `MergeError` variants are re-wrapped with an outer `PartialMergePlanRejected`
    /// carrying the phase context. This ensures typed semantics are never lost.
    pub fn with_phase(mut self, phase: &str) -> Self {
        match &mut self {
            KernelError::TopologyViolation { context, .. }
            | KernelError::AmbiguousResult { context, .. }
            | KernelError::ToleranceExceeded { context, .. }
            | KernelError::PrecisionEscalation { context, .. }
            | KernelError::InvalidInput { context, .. }
            | KernelError::InternalError { context, .. }
            | KernelError::ReplayMismatch { context, .. } => {
                let ctx = context.get_or_insert_with(|| ErrorContext {
                    scope: ErrorScope::Global,
                    suggested_fixes: Vec::new(),
                    detail: String::new(),
                });
                if ctx.detail.is_empty() {
                    ctx.detail = format!("Failed during phase '{}'", phase);
                } else {
                    ctx.detail = format!("[{}] {}", phase, ctx.detail);
                }
            }
            KernelError::DiagnosticFailure { source, .. } => {
                let new_source = (**source).clone();
                *source = Box::new(new_source.with_phase(phase));
            }
            // Preserve typed MergeError structure exactly.
            // Only PartialMergePlanRejected gets a phase prefix on its reason field —
            // it is the only general-purpose carrier. All other MergeError variants
            // already carry fully structured fields (edge_index, valence, witness, etc.)
            // that are more informative than a phase string. Do not convert them.
            KernelError::MergeFailure(merge_err) => {
                if let MergeError::PartialMergePlanRejected { reason, .. } = merge_err {
                    *reason = format!("[{}] {}", phase, reason);
                }
                // All other MergeError variants: left completely untouched.
            }
        }
        self
    }
}

impl std::error::Error for KernelError {}

impl From<forge_math::MathError> for KernelError {
    fn from(err: forge_math::MathError) -> Self {
        match err {
            forge_math::MathError::PrecisionEscalation {
                bit_length,
                threshold,
            } => KernelError::PrecisionEscalation {
                bit_length,
                threshold,
                context: None,
            },
            forge_math::MathError::InvalidInput(msg) => KernelError::InvalidInput { 
                message: msg, 
                context: None 
            },
            forge_math::MathError::InternalError(msg) => KernelError::InternalError { 
                message: msg, 
                context: None 
            },
            forge_math::MathError::Ambiguous {
                location,
                residual,
                context,
            } => KernelError::AmbiguousResult {
                result: AmbiguousResult {
                    location,
                    residual,
                    context,
                },
                context: None,
            },
        }
    }
}

// =========================================================================
// TOPOLOGY ERROR
// =========================================================================

/// Specific topology invariant violations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TopologyError {
    /// A halfedge is missing its twin (non-manifold or broken mesh)
    MissingTwin {
        halfedge_index: u32,
    },
    /// A face loop doesn't close (following `next` doesn't return to start)
    BrokenLoop {
        face_index: u32,
        starting_halfedge: u32,
    },
    /// Euler formula V - E + F ≠ 2 for a genus-0 solid
    EulerFormulaViolation {
        vertices: usize,
        edges: usize,
        faces: usize,
        expected_chi: i64,
        actual_chi: i64,
    },
    /// A non-manifold edge was detected (more than 2 faces sharing an edge)
    NonManifoldEdge {
        edge_index: u32,
        valence: usize,
    },
    /// Generalized Euler formula violation for genus-aware validation.
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
    /// Orientation inconsistency detected (D4 violation)
    OrientationInconsistency {
        face_index: u32,
    },
    /// An entity was referenced by a stale or invalid handle
    StaleHandle {
        entity_kind: String,
        index: u32,
        expected_generation: u32,
        actual_generation: u32,
    },
    /// A face has near-zero area (geometric degeneracy).
    ZeroAreaFace {
        face_index: u32,
        computed_area: f64,
        threshold: f64,
    },
    /// An edge has near-zero length.
    ZeroLengthEdge {
        halfedge_index: u32,
        computed_length: f64,
        threshold: f64,
    },
    /// A shell has the wrong signed volume (normals point inward).
    NegativeShellVolume {
        shell_index: u32,
        signed_volume: f64,
    },
    /// A loop has fewer than 3 distinct vertices.
    DegenerateLoop {
        face_index: u32,
        distinct_vertices: usize,
    },
    /// A topology walk exceeded its entity-count bound (corrupted next/prev chain).
    LoopCorruption {
        /// What kind of walk was being performed.
        walk_kind: String,
        /// The entity where the walk started.
        seed_index: u32,
        /// The last entity visited before the bound was hit.
        last_visited_index: u32,
        /// How many steps were taken.
        steps_taken: usize,
        /// The upper bound that was exceeded.
        entity_bound: usize,
    },
    /// A vertex referenced by a face has no geometry (position) available.
    MissingVertexPosition {
        vertex_index: u32,
        face_index: u32,
    },
    /// A shell has non-orientable surface topology (Möbius strip, Klein bottle).
    /// The kernel targets orientable 2-manifolds only.
    NonOrientableSurface {
        shell_index: u32,
    },
    /// A boundary edge (self-radial, no face across the gap) was found in a
    /// solid shell. Solid shells must be watertight.
    /// A boundary edge (self-radial, no face across the gap) was found in a
    /// solid shell. Solid shells must be watertight.
    BoundaryEdgeInSolid {
        halfedge_index: u32,
        shell_index: u32,
    },
    /// An operation attempted something topologically invalid.
    InvalidOperation {
        detail: String,
    },
    /// A parent-child hierarchy invariant was violated (Solid→Lump→Region→Shell).
    HierarchyViolation {
        /// The kind of the parent entity (e.g. "Region", "Lump", "Body").
        parent_kind: String,
        /// Arena index of the parent entity.
        parent_index: u32,
        /// The kind of the child entity (e.g. "Shell", "Region", "Lump").
        child_kind: String,
        /// Arena index of the child entity.
        child_index: u32,
        /// What went wrong.
        detail: String,
    },
    /// Two halfedges in the same radial ring reference different `EdgeId`s.
    /// A radial ring represents all face-uses of a single geometric edge;
    /// mixed edge entities indicate structural corruption.
    RadialEdgeInconsistency {
        /// The halfedge that disagrees with the ring seed.
        halfedge_index: u32,
        /// The edge entity found on the disagreeing halfedge.
        actual_edge: u32,
        /// The seed halfedge that defines the expected edge.
        seed_halfedge_index: u32,
        /// The edge entity on the ring seed.
        expected_edge: u32,
    },
}

impl fmt::Display for TopologyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TopologyError::MissingTwin { halfedge_index } => {
                write!(f, "Halfedge index {} is missing its twin", halfedge_index)
            }
            TopologyError::BrokenLoop { face_index, starting_halfedge } => {
                write!(f, "Face {} has a broken loop starting at halfedge {}", face_index, starting_halfedge)
            }
            TopologyError::EulerFormulaViolation {
                vertices,
                edges,
                faces,
                expected_chi,
                actual_chi,
            } => {
                write!(
                    f,
                    "Euler formula violation: V={} E={} F={}, χ={} (expected {})",
                    vertices, edges, faces, actual_chi, expected_chi
                )
            }
            TopologyError::NonManifoldEdge { edge_index, valence } => {
                write!(f, "Edge index {} is non-manifold (valence {})", edge_index, valence)
            }
            TopologyError::GeneralizedEulerViolation {
                shell_index, vertices, edges, faces, genus, rings, expected_chi, actual_chi,
            } => {
                write!(
                    f,
                    "Generalized Euler violation in shell {}: V={} E={} F={} G={} R={}, χ={} (expected {})",
                    shell_index, vertices, edges, faces, genus, rings, actual_chi, expected_chi
                )
            }
            TopologyError::OrientationInconsistency { face_index } => {
                write!(f, "Face {} has inconsistent orientation", face_index)
            }
            TopologyError::StaleHandle { entity_kind, index, expected_generation, actual_generation } => {
                write!(f, "Stale {} handle at index {} (expected gen {}, got gen {})", 
                    entity_kind, index, expected_generation, actual_generation)
            }
            TopologyError::ZeroAreaFace { face_index, computed_area, threshold } => {
                write!(f, "Face {} has near-zero area {:.2e} (threshold: {:.2e})",
                    face_index, computed_area, threshold)
            }
            TopologyError::ZeroLengthEdge { halfedge_index, computed_length, threshold } => {
                write!(f, "Edge {} has near-zero length {:.2e} (threshold: {:.2e})",
                    halfedge_index, computed_length, threshold)
            }
            TopologyError::NegativeShellVolume { shell_index, signed_volume } => {
                write!(f, "Shell {} has negative signed volume {:.6e} (normals point inward)",
                    shell_index, signed_volume)
            }
            TopologyError::DegenerateLoop { face_index, distinct_vertices } => {
                write!(f, "Face {} has degenerate loop with only {} distinct vertices (need >= 3)",
                    face_index, distinct_vertices)
            }
            TopologyError::LoopCorruption { walk_kind, seed_index, last_visited_index, steps_taken, entity_bound } => {
                write!(f, "Loop corruption in {}: seed={}, last={}, steps={}/{}",
                    walk_kind, seed_index, last_visited_index, steps_taken, entity_bound)
            }
            TopologyError::MissingVertexPosition { vertex_index, face_index } => {
                write!(f, "Vertex {} referenced by face {} has no position",
                    vertex_index, face_index)
            }
            TopologyError::NonOrientableSurface { shell_index } => {
                write!(f, "Shell {} has non-orientable surface topology (kernel targets orientable 2-manifolds only)",
                    shell_index)
            }
            TopologyError::BoundaryEdgeInSolid { halfedge_index, shell_index } => {
                write!(f, "Halfedge {} is a boundary edge in solid shell {} (solid shells must be watertight)",
                    halfedge_index, shell_index)
            }
            TopologyError::InvalidOperation { detail } => {
                write!(f, "Invalid operation: {}", detail)
            }
            TopologyError::HierarchyViolation { parent_kind, parent_index, child_kind, child_index, detail } => {
                write!(f, "Hierarchy violation: {} {} → {} {}: {}",
                    parent_kind, parent_index, child_kind, child_index, detail)
            }
            TopologyError::RadialEdgeInconsistency { halfedge_index, actual_edge, seed_halfedge_index, expected_edge } => {
                write!(f, "Radial ring edge-entity inconsistency: he[{}].edge = {} but ring seed he[{}].edge = {}",
                    halfedge_index, actual_edge, seed_halfedge_index, expected_edge)
            }
        }
    }
}

// =========================================================================
// AMBIGUOUS RESULT
// =========================================================================

/// A geometric result that requires a policy decision.
///
/// This carries ONLY geometric data. No policy categories or modeling
/// concepts are allowed in the math/geom layers (Rule 2.1).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmbiguousResult {
    /// 3D location where the ambiguity occurred.
    pub location: [f64; 3],
    /// Geometric metric of ambiguity (e.g. residual, distance).
    pub residual: f64,
    /// Human-readable context describing the ambiguity.
    pub context: String,
}

// =========================================================================
// DIAGNOSTIC PAYLOAD
// =========================================================================

/// Structured diagnostic context for replay and debugging.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticPayload {
    /// The operation that was executing when the failure occurred.
    pub operation: String,
    /// The topology state hash at the time of failure.
    pub state_hash: u128,
    /// The RNG seed at the time of failure.
    pub seed: u64,
    /// Additional human-readable context.
    pub context: String,
}

// =========================================================================
// MERGE ERROR
// =========================================================================

/// Structured failure reasons for `MergeSheetRegion` and related NMT merge operations.
///
/// Wrapped by `KernelError::MergeFailure`. Never downgraded to `InternalError`.
/// Fields use raw indices (`u32`) for serialisability and trace output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MergeError {
    /// A high-valence edge neighborhood is ambiguous and requires an explicit
    /// `RadialUseSelector` to disambiguate which pair of face-uses to merge.
    AmbiguousRadialSelection { edge_index: u32, valence: u32 },

    /// The selected radial uses at the given edge do not form a sheet-like
    /// (locally planar, merging) pair.
    SelectedUsesNotSheetLike { edge_index: u32 },

    /// An explicitly protected radial use conflicts with the merge selection.
    ProtectedUseConflict { edge_index: u32 },

    /// Merging the selected face group would disconnect the sheet topology.
    WouldDisconnectSheet { face_index: u32 },

    /// The Epic A boundary certifier rejected the merge boundary.
    BoundaryCertificationFailed {
        reason: String,
        /// 2D witness point (projected face plane space) where the rejection occurred.
        witness: Option<[f64; 2]>,
    },

    /// The merge plan could not be fully executed.
    ///
    /// `step_index: None`    — rejected during plan construction, before execution began.
    /// `step_index: Some(n)` — rejected at execution step `n` (0-indexed).
    PartialMergePlanRejected { step_index: Option<u32>, reason: String },

    /// Persistent NMT output is not implemented in this milestone.
    UnsupportedPersistentNmtOutput,
}

impl fmt::Display for MergeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MergeError::AmbiguousRadialSelection { edge_index, valence } => {
                write!(
                    f,
                    "Ambiguous radial selection at edge {}: valence {} requires explicit RadialUseSelector",
                    edge_index, valence
                )
            }
            MergeError::SelectedUsesNotSheetLike { edge_index } => {
                write!(f, "Selected radial uses at edge {} are not sheet-like", edge_index)
            }
            MergeError::ProtectedUseConflict { edge_index } => {
                write!(f, "Protected radial use conflict at edge {}", edge_index)
            }
            MergeError::WouldDisconnectSheet { face_index } => {
                write!(f, "Merging face {} would disconnect the sheet", face_index)
            }
            MergeError::BoundaryCertificationFailed { reason, witness } => {
                match witness {
                    Some(w) => write!(
                        f,
                        "Boundary certification failed at [{:.6}, {:.6}]: {}",
                        w[0], w[1], reason
                    ),
                    None => write!(f, "Boundary certification failed: {}", reason),
                }
            }
            MergeError::PartialMergePlanRejected { step_index, reason } => {
                match step_index {
                    Some(n) => write!(f, "Merge plan rejected at step {}: {}", n, reason),
                    None => write!(f, "Merge plan rejected during construction: {}", reason),
                }
            }
            MergeError::UnsupportedPersistentNmtOutput => {
                write!(f, "Persistent NMT output is not supported in this milestone")
            }
        }
    }
}
