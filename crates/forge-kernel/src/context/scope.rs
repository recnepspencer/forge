//! Operation scope — cross-cutting context bundle.
//!
//! DOMAIN: Bundles config and decision recording into a single parameter,
//! so downstream functions take one cross-cutting argument instead of many.
//! The kernel's equivalent of Express's `req` object.
//!
//! When a new cross-cutting concern is added (caching, undo, lineage),
//! add a field here. Zero downstream signature changes.
//!
//! DEPENDENCIES: `forge-core` (DecisionSink), `configuration` (ResolvedConfig),
//! `engine` (OperationSpace)

use forge_core::tracing::DecisionSink;

use crate::configuration::facade::ResolvedConfig;
use crate::engine::facade::OperationSpace;

/// Everything a kernel operation needs, bundled as one parameter.
///
/// Pass `&mut OperationScope` to any function that needs config, tracing,
/// or future cross-cutting concerns (lineage, provenance).
///
/// Designed for extensibility: adding a new field here changes zero
/// downstream signatures.
pub struct OperationScope<'a> {
    /// Resolved tolerance, precision, and policy configuration.
    pub config: &'a ResolvedConfig,
    /// Decision recording sink — typed, declarative, no global state.
    pub sink: &'a mut dyn DecisionSink,
    /// Numerical conditioning space — provides local↔world coordinate lens.
    ///
    /// Identity by default (zero cost). The pipeline sets this when the
    /// feature's `ConditioningMode` requires non-trivial conditioning.
    /// Features can use `scope.op_space.to_local()` / `scope.op_space.to_world()`
    /// for per-point transforms during execution.
    pub op_space: &'a OperationSpace,
}

/// Static identity OperationSpace used as the default when no conditioning is needed.
/// Avoids lifetime issues — lives for `'static` so it can be borrowed by any scope.
static IDENTITY_OP_SPACE: std::sync::LazyLock<OperationSpace> =
    std::sync::LazyLock::new(OperationSpace::identity);

impl<'a> OperationScope<'a> {
    /// Create a new operation scope with identity conditioning (default).
    ///
    /// Backward-compatible: existing callers don't need to provide an OperationSpace.
    pub fn new(config: &'a ResolvedConfig, sink: &'a mut dyn DecisionSink) -> Self {
        Self {
            config,
            sink,
            op_space: &IDENTITY_OP_SPACE,
        }
    }

    /// Create a scope with explicit numerical conditioning.
    ///
    /// Used by the pipeline when the feature's `ConditioningMode` requires
    /// non-trivial coordinate conditioning.
    pub fn with_conditioning(
        config: &'a ResolvedConfig,
        sink: &'a mut dyn DecisionSink,
        op_space: &'a OperationSpace,
    ) -> Self {
        Self {
            config,
            sink,
            op_space,
        }
    }
}
