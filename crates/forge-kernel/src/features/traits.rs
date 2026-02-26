//! Trait definitions for the feature system.

use std::any::Any;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use forge_core::KernelError;
use forge_signal::handles::NodeId;
use forge_topo::state::TopologyState;

use crate::core::ModelingContext;
use crate::geometry_state::GeometryState;
use crate::features::pipeline::contract::{FeatureContract, FeatureInputs};

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
}

/// A parametric feature that can be evaluated.
///
/// Requires `FeatureContract` as a supertrait — implementing `Feature`
/// without `FeatureContract` is a compile error. This is the compiler-enforced
/// guarantee that every feature declares its policies, invariants, and audit level.
pub trait Feature: FeatureContract + std::fmt::Debug + Any {
    /// The typed input DTO for this feature.
    type Inputs: FeatureInputs;

    /// Parse raw dependency outputs into typed inputs.
    fn parse_inputs(&self, raw: &HashMap<NodeId, FeatureOutput>) -> Result<Self::Inputs, KernelError>;

    /// Execute the feature's business logic with typed inputs and context.
    fn execute_typed(&self, inputs: &Self::Inputs, ctx: &mut ModelingContext) -> Result<FeatureOutput, KernelError>;

    /// Return the list of input dependencies (NodeIds).
    fn dependencies(&self) -> Vec<NodeId>;

    /// Return the name of the feature (for debugging).
    fn name(&self) -> &str;
}
