//! Trait definitions for the feature system.

use std::any::Any;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use forge_core::KernelError;
use forge_signal::handles::NodeId;
use forge_topo::state::TopologyState;

use crate::brep::state::BrepState;
use crate::core::config::resolve::ResolvedConfig;
use crate::engine::contract::{FeatureContract, FeatureInputs};
use crate::geometry_state::GeometryState;

/// The output of a feature evaluation.
///
/// Domain-only data — audit metadata (decisions, replay, lineage) lives
/// in the `OperationResult` envelope that wraps this, not in the output itself.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureOutput {
    /// The resulting topology (snapshot).
    pub topology: TopologyState,
    /// The resulting geometry.
    pub geometry: GeometryState,
    /// The resulting B-Rep data.
    pub brep: BrepState,
}

/// A parametric feature that can be evaluated.
///
/// Requires `FeatureContract` as a supertrait — implementing `Feature`
/// without `FeatureContract` is a compile error. This is the compiler-enforced
/// guarantee that every feature declares its policies, invariants, and audit level.
///
/// # Compile-Fail: Feature without Contract
///
/// ```compile_fail
/// use forge_kernel::features::traits::{Feature, FeatureOutput};
/// use forge_kernel::features::pipeline::contract::FeatureInputs;
/// use forge_kernel::core::config::resolve::ResolvedConfig;
/// use forge_core::KernelError;
/// use forge_signal::handles::NodeId;
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
///     fn parse_inputs(&self, _: &HashMap<NodeId, FeatureOutput>) -> Result<EmptyInputs, KernelError> {
///         Ok(EmptyInputs)
///     }
///     fn execute_typed(&self, _: &EmptyInputs, _: &ResolvedConfig) -> Result<FeatureOutput, KernelError> {
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
    fn parse_inputs(
        &self,
        raw: &HashMap<NodeId, FeatureOutput>,
    ) -> Result<Self::Inputs, KernelError>;

    /// Execute the feature's business logic with typed inputs and the resolved configuration.
    fn execute_typed(
        &self,
        inputs: &Self::Inputs,
        config: &ResolvedConfig,
    ) -> Result<FeatureOutput, KernelError>;

    /// Return the list of input dependencies (NodeIds).
    fn dependencies(&self) -> Vec<NodeId>;

    /// Return the name of the feature (for debugging).
    fn name(&self) -> &str;
}
