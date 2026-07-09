//! Public boundary contracts for host-domain integration.

/// Contract marker for the reactive dependency graph managed by `worth-signal`.
///
/// Guarantees:
/// - Dependency edges form a DAG.
/// - Evaluation order is deterministic.
/// - Cycles are rejected as invalid input.
///
/// Non-goals:
/// - Storing or mutating host structural graphs (e.g., host graph cycles).
/// - Owning host-domain numerics or acceleration structures.
#[derive(Debug, Clone, Copy, Default)]
pub struct DependencyGraphContract;

/// Contract marker for host structural state boundaries.
///
/// Host domains own structural graphs (possibly cyclic) externally and provide
/// opaque snapshots/views to signal compute closures.
#[derive(Debug, Clone, Copy, Default)]
pub struct StructuralStateBoundaryContract;

/// Contract marker for zero-indirection compute-path integration.
///
/// `worth-signal` does not require per-field reactive lookups during compute.
/// Compute closures may consume pre-packed host snapshots directly.
#[derive(Debug, Clone, Copy, Default)]
pub struct RawPathComputeContract;
