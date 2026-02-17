//! Core shared types for the Forge geometry kernel.
//!
//! This crate contains the common language that `forge-math`, `forge-geom`,
//! `forge-topo`, and `forge-kernel` all speak. It defines the error taxonomy,
//! policy structures, and data access traits.

use std::fmt;

use serde::{Deserialize, Serialize};

pub mod result;

pub use result::{
    DecisionKind, DecisionId, TracedDecision, DecisionLog,
    DecisionSummary, DecisionContext, EntityRef,
    KernelWarning, OperationMetrics, LineageDelta, OperationResult,
    LogLevel, log_level, log_result, log_decision_log,
};

// =========================================================================
// GEOMETRY SOURCE TRAIT (Rule 3.1)
// =========================================================================

/// Trait for providers of geometric data.
///
/// Allows lower layers (like `forge-geom`) to request data from higher layers
/// (like `forge-kernel` or `forge-topo`) without depending on them.
pub trait GeometrySource {
    /// Retrieve a plane by its handle/index.
    ///
    /// The specific index type is up to the implementation, but typically
    /// maps to `forge_topo::PlaneId` or similar. To keep this trait generic
    /// and avoid circular deps, we can use `usize` or a generic index,
    /// but for now, let's assume specific methods as needed by `implicit_vertex`.
    ///
    /// In the `implicit_vertex` case, we need 3 planes.
    /// 
    /// To be truly decoupled, the arguments here should be generic or simple types.
    /// `forge_geom` currently likely uses `PlaneRef` or `usize`.
    ///
    /// Let's define a method that returns a `Plane` (equation) given an index.
    /// Since we can't import `Plane` from `forge-geom` (circular), 
    /// we might need to define a simple Plane struct here or use [f64; 4].
    ///
    /// However, `forge-math` defines `Plane`. If `forge-core` depends upon `forge-math`,
    /// then `forge-math` cannot depend on `forge-core` for `KernelError`.
    /// This is the dependency cycle concern.
    ///
    /// **Resolution**: `forge-math` should be bottom-most. `forge-core` depends on `forge-math`.
    /// `KernelError` (in `forge-core`) cannot be used in `forge-math`.
    /// `forge-math` must return `MathError` (or similar).
    ///
    /// So `GeometrySource` can use types from `forge-math`.
    fn get_plane(&self, index: usize) -> Result<[f64; 4], KernelError>;
}


// =========================================================================
// STRUCTURED ERROR CONTEXT (Milestone 1B.1)
// =========================================================================

/// Identifies where an error originated in the kernel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorScope {
    /// Global kernel error not tied to a specific operation or entity.
    Global,
    /// Error occurred while processing a specific feature.
    Feature { feature_id: u64 },
    /// Error occurred on a specific topological entity.
    Entity {
        entity_kind: &'static str,
        index: u32,
    },
    /// Error occurred during a specific Euler operation.
    Operation {
        op_name: &'static str,
        invocation_id: u64,
    },
}

/// Machine-actionable remediation hints for an error.
#[derive(Debug, Clone, PartialEq)]
pub enum SuggestedFix {
    /// Increase a tolerance threshold.
    IncreaseThreshold {
        parameter: &'static str,
        current: f64,
        suggested: f64,
    },
    /// Reduce a geometric parameter value.
    ReduceValue {
        parameter: &'static str,
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

/// Structured diagnostic context for AI agents and UI.
#[derive(Debug, Clone, PartialEq)]
pub struct ErrorContext {
    /// Where the error happened.
    pub scope: ErrorScope,
    /// Automated remediation hints.
    pub suggested_fixes: Vec<SuggestedFix>,
    /// Human-readable detailed explanation.
    pub detail: String,
}


// =========================================================================
// KERNEL ERROR (Moved from forge-math)
// =========================================================================

/// The primary error type used across all Forge crates.
#[derive(Debug, Clone)]
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
        }
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
        }
    }
}

/// Specific topology invariant violations.
#[derive(Debug, Clone)]
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
    },
    /// Orientation inconsistency detected (D4 violation)
    OrientationInconsistency {
        face_index: u32,
    },
    /// An entity was referenced by a stale or invalid handle
    StaleHandle {
        entity_kind: &'static str,
        index: u32,
        expected_generation: u32,
        actual_generation: u32,
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
            TopologyError::NonManifoldEdge { edge_index } => {
                write!(f, "Edge index {} is non-manifold", edge_index)
            }
            TopologyError::OrientationInconsistency { face_index } => {
                write!(f, "Face {} has inconsistent orientation", face_index)
            }
            TopologyError::StaleHandle { entity_kind, index, expected_generation, actual_generation } => {
                write!(f, "Stale {} handle at index {} (expected gen {}, got gen {})", 
                    entity_kind, index, expected_generation, actual_generation)
            }
        }
    }
}

/// A geometric result that requires a policy decision.
///
/// This carries ONLY geometric data. No policy categories or modeling
/// concepts are allowed in the math/geom layers (Rule 2.1).
#[derive(Debug, Clone)]
pub struct AmbiguousResult {
    /// 3D location where the ambiguity occurred
    pub location: [f64; 3],
    /// Geometric metric of ambiguity (e.g. residual, distance)
    pub residual: f64,
    /// Human-readable context describing the ambiguity
    pub context: String,
}

/// Structured diagnostic context for replay and debugging.
#[derive(Debug, Clone)]
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
// POLICY TYPES (Moved from forge-kernel)
// =========================================================================

/// Categories of policy decisions the kernel may request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyKind {
    /// Two geometric entities are within tolerance of coincident
    CoincidentGeometry,
    /// Two surfaces are nearly tangent over a region
    NearTangency,
    /// A face would be created below the sliver area threshold
    SliverFace,
    /// A gap exceeds the automatic sewing threshold
    GapClosure,
    /// Precision escalation budget exceeded
    PrecisionBudget,
}

/// A query for a policy decision (Doctrine D2).
///
/// When the kernel encounters an ambiguous situation and it is mapped
/// from a geometry-layer `AmbiguousResult`, it enters this structured
/// policy request.
#[derive(Debug, Clone)]
pub struct PolicyQuery {
    /// What kind of decision is needed
    pub kind: PolicyKind,
    /// 3D location where the ambiguity occurred
    pub location: [f64; 3],
    /// How marginal this case is (lower = closer to the boundary)
    pub margin: f64,
    /// Whether the caller can override this with a policy setting
    pub overridable: bool,
}


// =========================================================================
// POLICY RESULT (Three-State Return Type for Geometry Solvers)
// =========================================================================

/// Three-state return type for geometry solvers (Doctrine D2).
///
/// Instead of returning plain `Result<T, E>`, geometry solvers return
/// `PolicyResult<T>` to distinguish between:
/// - **Success**: The math was clear, no ambiguity
/// - **Ambiguous**: Within the "ambiguity zone" — requires kernel judgment
/// - **HardError**: A genuine math failure (non-finite numbers, etc.)
///
/// The `Ambiguous` variant carries both the `PolicyQuery` (for the kernel
/// to inspect) and a `potential_value` (the solver's best guess if the
/// kernel decides to accept it).
///
/// # Example
/// ```
/// use forge_core::{PolicyResult, PolicyQuery, PolicyKind, KernelError};
///
/// fn classify_distance(dist: f64, threshold: f64) -> PolicyResult<bool> {
///     if dist > threshold * 10.0 {
///         PolicyResult::Success(false)
///     } else if dist < threshold * 0.1 {
///         PolicyResult::Success(true)
///     } else {
///         PolicyResult::Ambiguous {
///             query: PolicyQuery {
///                 kind: PolicyKind::CoincidentGeometry,
///                 location: [0.0, 0.0, 0.0],
///                 margin: dist,
///                 overridable: true,
///             },
///             potential_value: dist < threshold,
///         }
///     }
/// }
/// ```
#[derive(Debug)]
pub enum PolicyResult<T> {
    /// The math was clear, no ambiguity.
    Success(T),
    /// The math is within the "Ambiguity Zone."
    /// Requires the Kernel to look at `ModelingContext`.
    Ambiguous {
        /// Structured description of what decision is needed.
        query: PolicyQuery,
        /// The solver's best-guess value if the kernel accepts it.
        potential_value: T,
    },
    /// A genuine math failure (e.g., non-finite numbers, degenerate input).
    HardError(KernelError),
}

impl<T> PolicyResult<T> {
    /// Returns `true` if the result is `Success`.
    pub fn is_success(&self) -> bool {
        matches!(self, PolicyResult::Success(_))
    }

    /// Returns `true` if the result requires a policy decision.
    pub fn is_ambiguous(&self) -> bool {
        matches!(self, PolicyResult::Ambiguous { .. })
    }

    /// Returns `true` if the result is a hard error.
    pub fn is_hard_error(&self) -> bool {
        matches!(self, PolicyResult::HardError(_))
    }

    /// Convert to a standard `Result`, treating ambiguity as an error.
    ///
    /// Use this when the caller cannot handle ambiguity and wants to
    /// escalate it as a `KernelError::AmbiguousResult`.
    pub fn into_result_strict(self) -> Result<T, KernelError> {
        match self {
            PolicyResult::Success(v) => Ok(v),
            PolicyResult::Ambiguous { query, .. } => {
                Err(KernelError::AmbiguousResult {
                    result: AmbiguousResult {
                        location: query.location,
                        residual: query.margin,
                        context: format!("Policy decision required: {:?}", query.kind),
                    },
                    context: None,
                })
            }
            PolicyResult::HardError(e) => Err(e),
        }
    }

    /// Convert to a standard `Result`, accepting the potential value on ambiguity.
    ///
    /// Use this when the caller trusts the solver's best guess.
    pub fn into_result_accepting(self) -> Result<T, KernelError> {
        match self {
            PolicyResult::Success(v) => Ok(v),
            PolicyResult::Ambiguous { potential_value, .. } => Ok(potential_value),
            PolicyResult::HardError(e) => Err(e),
        }
    }
}

impl<T> From<T> for PolicyResult<T> {
    fn from(value: T) -> Self {
        PolicyResult::Success(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_is_success() {
        let r: PolicyResult<i32> = PolicyResult::Success(42);
        assert!(r.is_success());
        assert!(!r.is_ambiguous());
        assert!(!r.is_hard_error());
    }

    #[test]
    fn ambiguous_carries_query_and_value() {
        let r: PolicyResult<f64> = PolicyResult::Ambiguous {
            query: PolicyQuery {
                kind: PolicyKind::CoincidentGeometry,
                location: [1.0, 2.0, 3.0],
                margin: 1e-9,
                overridable: true,
            },
            potential_value: 0.5,
        };
        assert!(r.is_ambiguous());
    }

    #[test]
    fn hard_error_is_hard_error() {
        let r: PolicyResult<i32> = PolicyResult::HardError(
            KernelError::InvalidInput {
                message: "bad".to_string(),
                context: None,
            },
        );
        assert!(r.is_hard_error());
    }

    #[test]
    fn into_result_strict_rejects_ambiguity() {
        let r: PolicyResult<i32> = PolicyResult::Ambiguous {
            query: PolicyQuery {
                kind: PolicyKind::NearTangency,
                location: [0.0; 3],
                margin: 1e-8,
                overridable: true,
            },
            potential_value: 99,
        };
        assert!(r.into_result_strict().is_err());
    }

    #[test]
    fn into_result_accepting_uses_potential_value() {
        let r: PolicyResult<i32> = PolicyResult::Ambiguous {
            query: PolicyQuery {
                kind: PolicyKind::NearTangency,
                location: [0.0; 3],
                margin: 1e-8,
                overridable: true,
            },
            potential_value: 99,
        };
        assert_eq!(r.into_result_accepting().unwrap(), 99);
    }

    #[test]
    fn from_impl_wraps_in_success() {
        let r: PolicyResult<i32> = 42.into();
        assert!(r.is_success());
    }
}
