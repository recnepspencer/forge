//! The Feature trait — the core extension point for implementing features.
//!
//! DOMAIN: Defines what it means to be a feature in the engine.
//! Requires `FeatureContract` as a supertrait — implementing `Feature`
//! without `FeatureContract` is a compile error.

use std::any::Any;
use std::collections::HashMap;

use forge_core::KernelError;
use forge_signal::facade::NodeId;

use crate::context::scope::OperationScope;
use super::contract::{FeatureContract, FeatureInputs};
use super::super::output::solid_envelope::SolidEnvelope;

/// A parametric feature that can be evaluated.
///
/// Requires `FeatureContract` as a supertrait — implementing `Feature`
/// without `FeatureContract` is a compile error. This is the compiler-enforced
/// guarantee that every feature declares its policies, invariants, and audit level.
///
/// # Compile-Fail: Feature without Contract
///
/// ```compile_fail
/// use forge_kernel::engine::facade::{Feature, SolidEnvelope};
/// use forge_kernel::engine::facade::FeatureInputs;
/// use forge_kernel::configuration::facade::ResolvedConfig;
/// use forge_core::KernelError;
/// use forge_signal::facade::NodeId;
/// use std::collections::HashMap;
///
/// #[derive(Debug)]
/// struct NakedFeature;
///
/// struct EmptyInputs;
/// impl FeatureInputs for EmptyInputs {
///     fn validate(&self) -> Result<(), KernelError> { Ok(()) }
/// }
///
/// // This should fail because NakedFeature does NOT implement FeatureContract.
/// impl Feature for NakedFeature {
///     type Inputs = EmptyInputs;
///     fn parse_inputs(&self, _: &HashMap<NodeId, SolidEnvelope>) -> Result<EmptyInputs, KernelError> {
///         Ok(EmptyInputs)
///     }
///     fn execute_typed(&self, _: &EmptyInputs, _: &mut OperationScope<'_>) -> Result<SolidEnvelope, KernelError> {
///         unimplemented!()
///     }
///     fn dependencies(&self) -> Vec<NodeId> { vec![] }
///     fn name(&self) -> &str { "naked" }
/// }
/// ```
pub trait Feature: FeatureContract + std::fmt::Debug + Any {
    /// The typed input DTO for this feature.
    type Inputs: FeatureInputs;

    /// Parse raw dependency outputs into typed inputs.
    ///
    /// Takes ownership of the input map — the pipeline has already
    /// performed coordinate conditioning on the geometry. Features
    /// can use `.remove()` to move data out without cloning.
    fn parse_inputs(
        &self,
        raw: HashMap<NodeId, SolidEnvelope>,
    ) -> Result<Self::Inputs, KernelError>;

    /// Execute the feature's business logic with typed inputs and an operation scope
    /// that provides both configuration and a decision recording sink.
    ///
    /// Takes ownership of inputs — features that need to mutate geometry
    /// (e.g., boolean operations) can do so in-place without cloning.
    fn execute_typed(
        &self,
        inputs: Self::Inputs,
        scope: &mut OperationScope<'_>,
    ) -> Result<forge_core::envelope::OperationResult<SolidEnvelope>, KernelError>;

    /// Return the list of input dependencies (NodeIds).
    fn dependencies(&self) -> Vec<NodeId>;

    /// Return the name of the feature (for debugging).
    fn name(&self) -> &str;
}
