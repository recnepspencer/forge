//! Trait definitions for the feature system.

use std::any::Any;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use forge_core::KernelError;
use forge_signal::handles::NodeId;
use forge_topo::state::TopologyState;

use crate::geometry_store::GeometryStore;

/// The output of a feature evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureOutput {
    /// The resulting topology (snapshot).
    pub topology: TopologyState,
    /// The resulting geometry.
    pub geometry: GeometryStore,
}

/// A parametric feature that can be evaluated.
///
/// Features are the nodes in the dependency graph.
pub trait Feature: std::fmt::Debug + Any {
    /// Evaluate the feature given its inputs.
    ///
    /// # Arguments
    /// * `inputs` - Map of input NodeId to their FeatureOutput.
    ///
    /// # Returns
    /// * `FeatureOutput` - The result of the operation.
    fn evaluate(
        &self,
        inputs: &HashMap<NodeId, FeatureOutput>,
    ) -> Result<FeatureOutput, KernelError>;

    /// Return the list of input dependencies (NodeIds).
    fn dependencies(&self) -> Vec<NodeId>;

    /// Return the name of the feature (for debugging).
    fn name(&self) -> &str;
}
