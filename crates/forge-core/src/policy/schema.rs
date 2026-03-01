//! Policy data shapes for the Forge geometry kernel.

use serde::{Deserialize, Serialize};

use crate::errors::{AmbiguousResult, KernelError};

// =========================================================================
// POLICY KIND
// =========================================================================

/// Categories of policy decisions the kernel may request.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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

// =========================================================================
// POLICY QUERY
// =========================================================================

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
            PolicyResult::Ambiguous { query, .. } => Err(KernelError::AmbiguousResult {
                result: AmbiguousResult {
                    location: query.location,
                    residual: query.margin,
                    context: format!("Policy decision required: {:?}", query.kind),
                },
                context: None,
            }),
            PolicyResult::HardError(e) => Err(e),
        }
    }

    /// Convert to a standard `Result`, accepting the potential value on ambiguity.
    ///
    /// Use this when the caller trusts the solver's best guess.
    pub fn into_result_accepting(self) -> Result<T, KernelError> {
        match self {
            PolicyResult::Success(v) => Ok(v),
            PolicyResult::Ambiguous {
                potential_value, ..
            } => Ok(potential_value),
            PolicyResult::HardError(e) => Err(e),
        }
    }
}

impl<T> From<T> for PolicyResult<T> {
    fn from(value: T) -> Self {
        PolicyResult::Success(value)
    }
}

// =========================================================================
// VALIDATION CHECKPOINT
// =========================================================================

/// Checkpoints where invariant validation can be triggered.
///
/// This is a shared contract type used by both the configuration system
/// (to specify which checkpoints are active) and the proof system
/// (to execute validation at those checkpoints).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ValidationCheckpoint {
    /// After MutableDraft::commit() — structural invariants.
    PostCommit,
    /// After every Boolean operation.
    PostBoolean,
    /// After every feature evaluation.
    PostFeature,
    /// After STEP/IGES import healing.
    PostImport,
    /// On explicit request only.
    OnDemand,
}
