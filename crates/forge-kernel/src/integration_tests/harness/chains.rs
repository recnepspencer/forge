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
use forge_core::KernelError;
use forge_topo::validators::validate::ValidationLevel;

use super::builders::configs::test_config;

/// A chain of operations with interleaved invariant checks.
pub struct OpChain {
    envelope: SolidEnvelope,
    config: ResolvedConfig,
    step_count: usize,
    check_after_every_op: bool,
}

impl OpChain {
    /// Start a new chain from an existing solid.
    pub fn new(envelope: SolidEnvelope) -> Self {
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
    /// The closure receives a mutable reference to the scope and
    /// must return the (possibly modified) SolidEnvelope.
    pub fn apply<F>(mut self, op_name: &str, f: F) -> Self
    where
        F: FnOnce(SolidEnvelope, &mut OperationScope<'_>) -> Result<SolidEnvelope, KernelError>,
    {
        self.step_count += 1;
        let step = self.step_count;

        let mut ctx = ModelingContext::new();
        let mut scope = OperationScope::new(&self.config, &mut ctx);

        self.envelope = f(self.envelope, &mut scope).unwrap_or_else(|e| {
            panic!("Chain step {step} ({op_name}) failed: {:?}", e);
        });

        if self.check_after_every_op {
            self.check_invariants_internal(op_name);
        }

        self
    }

    /// Run an operation that may fail, returning the error instead of panicking.
    ///
    /// Useful for testing failure paths mid-chain:
    ///
    /// ```rust,ignore
    /// let result = OpChain::new(unit_cube()?)
    ///     .apply("split", |env, scope| split(env, scope))
    ///     .try_apply("bad_op", |env, scope| bad_op(env, scope));
    /// assert!(result.is_err());
    /// ```
    pub fn try_apply<F>(mut self, _op_name: &str, f: F) -> Result<Self, KernelError>
    where
        F: FnOnce(SolidEnvelope, &mut OperationScope<'_>) -> Result<SolidEnvelope, KernelError>,
    {
        self.step_count += 1;

        let mut ctx = ModelingContext::new();
        let mut scope = OperationScope::new(&self.config, &mut ctx);

        self.envelope = f(self.envelope, &mut scope)?;

        if self.check_after_every_op {
            self.check_invariants_internal(_op_name);
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
        let arena = self.envelope.topology().arena();
        forge_topo::validators::validate::validate_topology(arena, ValidationLevel::Intermediate)
            .unwrap_or_else(|e| {
                panic!("Structural invariant violation at step {}: {:?}", self.step_count, e);
            });
        self
    }

    /// Non-panicking structural validation — returns the error if invariants fail.
    pub fn try_assert_structural(self) -> Result<Self, KernelError> {
        let arena = self.envelope.topology().arena();
        forge_topo::validators::validate::validate_topology(arena, ValidationLevel::Intermediate)?;
        Ok(self)
    }

    /// Finish the chain and return the final solid.
    pub fn finish(self) -> SolidEnvelope {
        self.envelope
    }

    /// Finish the chain with a final validation.
    pub fn finish_validated(self) -> SolidEnvelope {
        forge_topo::validators::validate::validate_topology(
            self.envelope.topology().arena(),
            ValidationLevel::Full,
        ).unwrap_or_else(|e| {
            panic!("Final validation failed: {:?}", e);
        });
        self.envelope
    }

    /// Non-panicking finish with validation — returns the error if validation fails.
    pub fn try_finish_validated(self) -> Result<SolidEnvelope, KernelError> {
        forge_topo::validators::validate::validate_topology(
            self.envelope.topology().arena(),
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
        &self.envelope
    }

    fn check_invariants_internal(&self, context: &str) {
        let arena = self.envelope.topology().arena();
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
///
/// This combines chain testing with determinism verification.
pub fn assert_chain_deterministic<F>(n: usize, chain_fn: F)
where
    F: Fn() -> Result<SolidEnvelope, KernelError>,
{
    super::determinism::assert_deterministic_n(chain_fn, n);
}
