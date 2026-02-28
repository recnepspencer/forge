//! Topology validation — Public API.
//!
//! DOMAIN: Re-exports from `structural` and `geometric` submodules.
//! This module owns `ValidationLevel` and the inline tests.
//!
//! DEPENDENCIES: `structural`, `geometric`

/// Validation strictness level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationLevel {
    /// No checks. Trust the operations blindly (fastest).
    None,
    /// Only check local connectivity invariants (twins, prev/next).
    /// Used for Release builds.
    Minimal,
    /// Intermediate structural checks. Allows temporary non-manifold
    /// topology mid-boolean but checks basic pointer coherence.
    Intermediate,
    /// Full global validity checks (Euler formula, loop closure).
    /// Used for Debug/Test builds.
    Full,
}

impl Default for ValidationLevel {
    fn default() -> Self {
        if cfg!(debug_assertions) {
            ValidationLevel::Full
        } else {
            ValidationLevel::Minimal
        }
    }
}

/// Topology manifold policy — controls *what world is allowed* at commit time.
///
/// This is orthogonal to `ValidationLevel` (which controls *how much to check*).
/// `ValidationLevel` = breadth/depth of checks (diagnostics vs speed).
/// `TopologyMode` = semantic policy (manifold vs NMT-intermediate constraints).
///
/// The `NmtIntermediate` skip-list is exhaustive. Any extension requires
/// a spec amendment and dedicated tests — it must never become a bypass mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopologyMode {
    /// Default. Enforces 2-manifold constraints at commit time (Doctrine D8).
    /// This runs regardless of `ValidationLevel`.
    ManifoldStrict,
    /// Permits edges with radial valence > 2 for internal pipeline checkpoints.
    ///
    /// Named skip-list (exhaustive):
    /// - SKIP: `validate_manifold_edges` (valence > 2 permitted)
    ///
    /// Note: Radial ring validation itself (closure/pointer health) STILL RUNS
    /// (for `ValidationLevel != None`) to prevent data corruption. Only the
    /// valence threshold is relaxed.
    NmtIntermediate,
}

pub use super::structural::validate_topology;
pub use super::structural::validate_topology_with_mode;
// Geometric invariant validation has moved to `forge-spatial::integrity`.
// Build with forge-spatial as a dependency and call `forge_spatial::integrity::validate_geometric_invariants`.
