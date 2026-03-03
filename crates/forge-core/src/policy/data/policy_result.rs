//! Three-state return type for geometry solvers.

use super::policy_query::PolicyQuery;
use crate::errors::KernelError;

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
