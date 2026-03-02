//! BooleanFeature — the engine Feature adapter for the boolean solver.
//!
//! DOMAIN: Bridges the engine's Feature trait to the boolean operation solver.
//! Lives in `operations/boolean/` because it is a boolean-domain concern,
//! not engine infrastructure.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::context::scope::OperationScope;
use crate::engine::facade::{Feature, FeatureOutput, FeatureInputs};
use crate::engine::facade::{AuditLevel, EntityOriginKind, InvariantKind};
use forge_core::KernelError;
use forge_signal::facade::NodeId;

use super::{execute_boolean, BooleanInput, BooleanOp};

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

/// Typed inputs for Boolean — target and tool feature outputs.
pub struct BooleanInputs {
    pub target: FeatureOutput,
    pub tool: FeatureOutput,
}

impl FeatureInputs for BooleanInputs {
    fn validate(&self) -> Result<(), KernelError> {
        if self.target.topology.arena().face_count() == 0 {
            return Err(KernelError::InvalidInput {
                message: "Boolean target has no faces".into(),
                context: None,
            });
        }
        if self.tool.topology.arena().face_count() == 0 {
            return Err(KernelError::InvalidInput {
                message: "Boolean tool has no faces".into(),
                context: None,
            });
        }
        Ok(())
    }
}

crate::declare_feature!(BooleanFeature,
    kind: "boolean",
    policies: [
        forge_core::PolicyKind::CoincidentGeometry,
    ],
    origins: [
        EntityOriginKind::SplitOperator,
        EntityOriginKind::CopyOperator,
    ],
    euler_ops: [
        crate::engine::facade::EulerOpKind::SplitEdge,
        crate::engine::facade::EulerOpKind::MakeEdgeFace,
    ],
    surfaces: [crate::engine::facade::SurfaceKind::Planar],
    invariants: [],
    audit: AuditLevel::Full,
    persistent: true,
);

impl Feature for BooleanFeature {
    type Inputs = BooleanInputs;

    fn parse_inputs(
        &self,
        raw: &HashMap<NodeId, FeatureOutput>,
    ) -> Result<BooleanInputs, KernelError> {
        let target = raw
            .get(&self.target)
            .ok_or(KernelError::InvalidInput {
                message: "Missing target input".into(),
                context: None,
            })?
            .clone();
        let tool = raw
            .get(&self.tool)
            .ok_or(KernelError::InvalidInput {
                message: "Missing tool input".into(),
                context: None,
            })?
            .clone();
        Ok(BooleanInputs { target, tool })
    }

    fn execute_typed(
        &self,
        inputs: &BooleanInputs,
        _scope: &mut OperationScope<'_>,
    ) -> Result<FeatureOutput, KernelError> {
        let input = BooleanInput::new(
            inputs.target.topology.clone(),
            inputs.target.geometry.clone(),
            inputs.tool.topology.clone(),
            inputs.tool.geometry.clone(),
            self.op,
        );

        let envelope = execute_boolean(input);
        let result = envelope.into_result()?;
        let (topo, geom) = result.into_states();

        Ok(FeatureOutput {
            topology: topo,
            geometry: geom,
        })
    }

    fn dependencies(&self) -> Vec<NodeId> {
        vec![self.target, self.tool]
    }

    fn name(&self) -> &str {
        &self.name
    }
}
