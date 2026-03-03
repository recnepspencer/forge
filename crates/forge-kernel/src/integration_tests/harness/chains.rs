//! Chain / mutation sequence testing infrastructure.
//!
//! DOMAIN: Production kernels die from long operation chains where
//! operator A corrupts a tiny pointer and operator D's assertion
//! catches it 12 ops later. This module provides fluent chain builders
//! that validate after EVERY step using the production validator.
//!
//! ```rust,ignore
//! OpChain::new(unit_cube()?)
//!     .split_all_edges()
//!     .assert_valid()
//!     .split_all_edges()  // second round — catches pointer rot
//!     .assert_valid()
//!     .finish();
//! ```

use crate::configuration::facade::ResolvedConfig;
use crate::context::ModelingContext;
use crate::context::scope::OperationScope;
use crate::engine::facade::SolidEnvelope;
use forge_core::envelope::OperationResult;
use forge_core::KernelError;
use forge_topo::validators::validate::ValidationLevel;

use super::builders::configs::test_config;

/// A chain of operations with interleaved invariant checks.
///
/// Each step produces an `OperationResult<SolidEnvelope>`. The chain
/// accumulates metadata across all steps using `absorb_metadata`, so the
/// final `OperationResult` carries the merged decision log, warnings,
/// metrics, and lineage from every step.
pub struct OpChain {
    envelope: OperationResult<SolidEnvelope>,
    config: ResolvedConfig,
    step_count: usize,
    check_after_every_op: bool,
}

impl OpChain {
    /// Start a new chain from an existing solid.
    pub fn new(envelope: OperationResult<SolidEnvelope>) -> Self {
        Self {
            envelope,
            config: test_config(),
            step_count: 0,
            check_after_every_op: true,
        }
    }

    /// Use a custom config for subsequent operations.
    pub fn with_config(mut self, config: ResolvedConfig) -> Self {
        self.config = config;
        self
    }

    /// Disable automatic invariant checks after every op.
    /// You can still call `.assert_valid()` manually.
    pub fn no_auto_check(mut self) -> Self {
        self.check_after_every_op = false;
        self
    }

    /// Run an arbitrary operation on the current solid.
    ///
    /// The closure receives the current `SolidEnvelope` and a scope,
    /// and must return an `OperationResult<SolidEnvelope>`.
    /// Metadata from the result is absorbed into the chain's running total.
    pub fn apply<F>(mut self, op_name: &str, f: F) -> Self
    where
        F: FnOnce(SolidEnvelope, &mut OperationScope<'_>) -> Result<OperationResult<SolidEnvelope>, KernelError>,
    {
        self.step_count += 1;
        let step = self.step_count;

        let mut ctx = ModelingContext::new();
        let mut scope = OperationScope::new(&self.config, &mut ctx);

        // Cloned for the operation; self.envelope remains owned.
        let solid = self.envelope.get_value().clone();
        
        let mut result = f(solid, &mut scope).unwrap_or_else(|e| {
            panic!("Chain step {step} ({op_name}) failed: {:?}", e);
        });

        // 1. Merge the metadata from this operation into our persistent chain envelope.
        self.envelope.absorb_metadata(&mut result);
        
        // 2. Map the value to the new solid produced by the operation.
        // `map` preserves all metadata (including the stuff we just absorbed).
        self.envelope = self.envelope.map(|_| result.into_value());

        if self.check_after_every_op {
            self.check_invariants_internal(op_name);
        }

        self
    }

    /// Run an operation that may fail, returning the error instead of panicking.
    pub fn try_apply<F>(mut self, op_name: &str, f: F) -> Result<Self, KernelError>
    where
        F: FnOnce(SolidEnvelope, &mut OperationScope<'_>) -> Result<OperationResult<SolidEnvelope>, KernelError>,
    {
        self.step_count += 1;

        let mut ctx = ModelingContext::new();
        let mut scope = OperationScope::new(&self.config, &mut ctx);

        let solid = self.envelope.get_value().clone();
        let mut result = f(solid, &mut scope)?;
        
        self.envelope.absorb_metadata(&mut result);
        self.envelope = self.envelope.map(|_| result.into_value());

        if self.check_after_every_op {
            self.check_invariants_internal(op_name);
        }

        Ok(self)
    }

    /// Explicitly validate all topology invariants at this point in the chain.
    pub fn assert_valid(self) -> Self {
        self.check_invariants_internal("explicit check");
        self
    }

    /// Validate structural invariants only (no Euler formula check).
    pub fn assert_structural(self) -> Self {
        let arena = self.envelope.get_value().topology().arena();
        forge_topo::validators::validate::validate_topology(arena, ValidationLevel::Intermediate)
            .unwrap_or_else(|e| {
                panic!("Structural invariant violation at step {}: {:?}", self.step_count, e);
            });
        self
    }

    /// Non-panicking structural validation — returns the error if invariants fail.
    pub fn try_assert_structural(self) -> Result<Self, KernelError> {
        let arena = self.envelope.get_value().topology().arena();
        forge_topo::validators::validate::validate_topology(arena, ValidationLevel::Intermediate)?;
        Ok(self)
    }

    /// Finish the chain and return the final solid.
    pub fn finish(self) -> OperationResult<SolidEnvelope> {
        self.envelope
    }

    /// Finish the chain with a final validation.
    pub fn finish_validated(self) -> OperationResult<SolidEnvelope> {
        forge_topo::validators::validate::validate_topology(
            self.envelope.get_value().topology().arena(),
            ValidationLevel::Full,
        ).unwrap_or_else(|e| {
            panic!("Final validation failed: {:?}", e);
        });
        self.envelope
    }

    /// Non-panicking finish with validation — returns the error if validation fails.
    pub fn try_finish_validated(self) -> Result<OperationResult<SolidEnvelope>, KernelError> {
        forge_topo::validators::validate::validate_topology(
            self.envelope.get_value().topology().arena(),
            ValidationLevel::Full,
        )?;
        Ok(self.envelope)
    }

    /// Get the current step count.
    pub fn steps(&self) -> usize {
        self.step_count
    }

    /// Get a reference to the current solid (for mid-chain inspection).
    pub fn peek(&self) -> &SolidEnvelope {
        self.envelope.get_value()
    }

    /// Get a reference to the OperationResult (for mid-chain inspection of metadata).
    pub fn envelope(&self) -> &OperationResult<SolidEnvelope> {
        &self.envelope
    }

    fn check_invariants_internal(&self, context: &str) {
        let arena = self.envelope.get_value().topology().arena();
        let step = self.step_count;

        if let Err(e) = forge_topo::validators::validate::validate_topology(arena, ValidationLevel::Full) {
            panic!(
                "Invariant violation at chain step {} ({}): {:?}",
                step, context, e
            );
        }
    }
}

/// Run the same sequence of operations N times and assert all produce
/// identical structural hashes.
pub fn assert_chain_deterministic<F>(n: usize, chain_fn: F)
where
    F: Fn() -> Result<OperationResult<SolidEnvelope>, KernelError>,
{
    super::determinism::assert_deterministic_n(chain_fn, n);
}
