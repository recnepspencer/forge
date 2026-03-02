//! Operation scope — cross-cutting context bundle.
//!
//! DOMAIN: Bundles config and decision recording into a single parameter,
//! so downstream functions take one cross-cutting argument instead of many.
//! The kernel's equivalent of Express's `req` object.
//!
//! When a new cross-cutting concern is added (caching, undo, lineage),
//! add a field here. Zero downstream signature changes.
//!
//! DEPENDENCIES: `forge-core` (DecisionSink), `configuration` (ResolvedConfig)

use forge_core::tracing::{DecisionSink, NullSink};

use crate::configuration::facade::ResolvedConfig;

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
}

impl<'a> OperationScope<'a> {
    /// Create a new operation scope.
    pub fn new(config: &'a ResolvedConfig, sink: &'a mut dyn DecisionSink) -> Self {
        Self { config, sink }
    }

    /// Create a scope with a NullSink for testing.
    ///
    /// Returns a `(NullSink, OperationScope)` pair — the caller must keep
    /// the `NullSink` alive for the scope's lifetime.
    ///
    /// # Example
    /// ```ignore
    /// let cfg = resolve_config(&KernelConfig::default(), None, None, None).unwrap();
    /// let mut null = NullSink;
    /// let mut scope = OperationScope::new(&cfg, &mut null);
    /// make_cube([0.0; 3], 1.0, &mut scope).unwrap();
    /// ```
    pub fn null_sink() -> NullSink {
        NullSink
    }
}

