//! Feature implementations wrapping procedural kernels.

use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use forge_core::KernelError;
use forge_core::DecisionLog;
use forge_signal::handles::NodeId;
use forge_topo::replay::ReplayLog;
use crate::operations::boolean::{BooleanInput, BooleanOp};
use crate::operations::boolean::execute_boolean;

use super::traits::{Feature, FeatureOutput};

/// A root feature that produces a base shape (e.g. from a sketch/extrude).
/// For Phase 2.7, we simulate this as a "Make Cube" feature since we don't have
/// full sketch->extrude logic exposed as a single function yet (it's in 2.3 pipeline).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MakeCubeFeature {
    name: String,
    center: [f64; 3],
    size: f64,
}

impl MakeCubeFeature {
    pub fn new(name: &str, center: [f64; 3], size: f64) -> Self {
        Self {
            name: name.to_string(),
            center,
            size,
        }
    }
}

impl Feature for MakeCubeFeature {
    fn evaluate(
        &self,
        _inputs: &HashMap<NodeId, FeatureOutput>,
    ) -> Result<FeatureOutput, KernelError> {
        // Use the test helper logic for now (extracted to public helper if needed)
        // Or reconstruct manually.
        // For 2.7, let's just make a cube using bsp.
        let build_result = crate::mesh_builder::make_cube(self.center, self.size)?;
        let (topo, geom) = build_result.into_parts();
        Ok(FeatureOutput {
            topology: topo,
            geometry: geom,
            decision_log: Arc::new(DecisionLog::new()),
            replay_log: Arc::new(ReplayLog::new()),
            lineage_events: Arc::new(Vec::new()),
        })
    }

    fn dependencies(&self) -> Vec<NodeId> {
        Vec::new() // Roots have no deps
    }

    fn name(&self) -> &str {
        &self.name
    }
}

/// A Boolean feature taking two inputs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BooleanFeature {
    name: String,
    op: BooleanOp,
    target: NodeId,
    tool: NodeId,
}

impl BooleanFeature {
    pub fn new(name: &str, op: BooleanOp, target: NodeId, tool: NodeId) -> Self {
        Self {
            name: name.to_string(),
            op,
            target,
            tool,
        }
    }
}

impl Feature for BooleanFeature {
    fn evaluate(
        &self,
        inputs: &HashMap<NodeId, FeatureOutput>,
    ) -> Result<FeatureOutput, KernelError> {
        let target_out = inputs.get(&self.target).ok_or(KernelError::InvalidInput {
             message: "Missing target input".into(), context: None
        })?;
        let tool_out = inputs.get(&self.tool).ok_or(KernelError::InvalidInput {
             message: "Missing tool input".into(), context: None
        })?;

        let input = BooleanInput::new(
            target_out.topology.clone(),
            target_out.geometry.clone(),
            tool_out.topology.clone(),
            tool_out.geometry.clone(),
            self.op,
        );

        let mut envelope = execute_boolean(input)?;
        let log = envelope.take_decision_log();
        let result = envelope.into_value();
        let (topo, geom, replay, lineage, _introspection) = result.into_full_parts();

        Ok(FeatureOutput {
            topology: topo,
            geometry: geom,
            decision_log: Arc::new(log),
            replay_log: Arc::new(replay),
            lineage_events: Arc::new(lineage),
        })
    }

    fn dependencies(&self) -> Vec<NodeId> {
        vec![self.target, self.tool]
    }

    fn name(&self) -> &str {
        &self.name
    }
}
