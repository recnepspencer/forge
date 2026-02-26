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
use crate::core::ModelingContext;

use super::traits::{Feature, FeatureOutput};
use super::pipeline::contract::{
    AuditLevel, EntityOriginKind, InvariantKind, FeatureInputs,
};

// ── MakeCube ──────────────────────────────────────────────────────────────

/// A root feature that produces a base shape (e.g. from a sketch/extrude).
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

/// Typed inputs for MakeCube — empty (root feature, no dependencies).
pub struct MakeCubeInputs;

impl FeatureInputs for MakeCubeInputs {
    fn validate(&self) -> Result<(), KernelError> {
        Ok(())
    }
}

crate::declare_feature!(MakeCubeFeature,
    kind: "make_cube",
    policies: [],
    origins: [EntityOriginKind::EulerOperator],
    invariants: [InvariantKind::ManifoldEdges],
    audit: AuditLevel::Summary,
);

impl Feature for MakeCubeFeature {
    type Inputs = MakeCubeInputs;

    fn parse_inputs(&self, _raw: &HashMap<NodeId, FeatureOutput>) -> Result<MakeCubeInputs, KernelError> {
        Ok(MakeCubeInputs)
    }

    fn execute_typed(&self, _inputs: &MakeCubeInputs, _ctx: &mut ModelingContext) -> Result<FeatureOutput, KernelError> {
        let build_result = crate::mesh_builder::make_cube(self.center, self.size)?;
        let (topo, geom) = build_result.into_parts();
        Ok(FeatureOutput {
            topology: topo,
            geometry: geom,
        })
    }

    fn dependencies(&self) -> Vec<NodeId> {
        Vec::new()
    }

    fn name(&self) -> &str {
        &self.name
    }
}

// ── Boolean ───────────────────────────────────────────────────────────────

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
    // NOTE: Boolean operations validate topology internally via the boolean engine.
    // ManifoldEdges is NOT declared here because the engine's own validation
    // pipeline handles it. Adding it would double-validate and surface
    // engine-internal intermediate states as feature contract violations.
    invariants: [],
    audit: AuditLevel::Full,
);

impl Feature for BooleanFeature {
    type Inputs = BooleanInputs;

    fn parse_inputs(&self, raw: &HashMap<NodeId, FeatureOutput>) -> Result<BooleanInputs, KernelError> {
        let target = raw.get(&self.target).ok_or(KernelError::InvalidInput {
            message: "Missing target input".into(),
            context: None,
        })?.clone();
        let tool = raw.get(&self.tool).ok_or(KernelError::InvalidInput {
            message: "Missing tool input".into(),
            context: None,
        })?.clone();
        Ok(BooleanInputs { target, tool })
    }

    fn execute_typed(&self, inputs: &BooleanInputs, ctx: &mut ModelingContext) -> Result<FeatureOutput, KernelError> {
        let input = BooleanInput::new(
            inputs.target.topology.clone(),
            inputs.target.geometry.clone(),
            inputs.tool.topology.clone(),
            inputs.tool.geometry.clone(),
            self.op,
        );

        let mut envelope = execute_boolean(input);
        ctx.absorb_sub_result(&mut envelope);
        let result = envelope.into_result()?;
        let (topo, geom) = result.into_topo_geom();

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
