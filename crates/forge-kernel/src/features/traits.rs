//! Trait definitions for the feature system.

use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

use forge_core::KernelError;
use forge_core::DecisionLog;
use forge_signal::handles::NodeId;
use forge_topo::state::TopologyState;
use forge_topo::replay::ReplayLog;
use forge_topo::lineage::LineageEvent;

use crate::geometry_state::GeometryState;

/// The output of a feature evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureOutput {
    /// The resulting topology (snapshot).
    pub topology: TopologyState,
    /// The resulting geometry.
    pub geometry: GeometryState,
    /// Traced decisions from this evaluation.
    pub decision_log: Arc<DecisionLog>,
    /// Replay log recording each pipeline phase.
    pub replay_log: Arc<ReplayLog>,
    /// Lineage events emitted during the operation.
    pub lineage_events: Arc<Vec<LineageEvent>>,
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
