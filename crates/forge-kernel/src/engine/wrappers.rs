//! Feature implementations wrapping procedural kernels.

use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::core::config::resolve::ResolvedConfig;
use crate::operations::boolean::execute_boolean;
use crate::operations::boolean::{BooleanInput, BooleanOp};
use forge_core::DecisionLog;
use forge_core::KernelError;
use forge_signal::handles::NodeId;
use forge_topo::replay::ReplayLog;

use super::contract::{AuditLevel, EntityOriginKind, FeatureInputs, InvariantKind};
use super::traits::{Feature, FeatureOutput};

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
    euler_ops: [
        crate::engine::contract::EulerOpKind::MakeVertexFace,
        crate::engine::contract::EulerOpKind::MakeEdgeFace,
    ],
    surfaces: [crate::engine::contract::SurfaceKind::Planar],
    invariants: [InvariantKind::ManifoldEdges],
    audit: AuditLevel::Summary,
    persistent: true,
);

impl Feature for MakeCubeFeature {
    type Inputs = MakeCubeInputs;

    fn parse_inputs(
        &self,
        _raw: &HashMap<NodeId, FeatureOutput>,
    ) -> Result<MakeCubeInputs, KernelError> {
        Ok(MakeCubeInputs)
    }

    fn execute_typed(
        &self,
        _inputs: &MakeCubeInputs,
        _config: &ResolvedConfig,
    ) -> Result<FeatureOutput, KernelError> {
        let build_result = crate::mesh_builder::make_cube(self.center, self.size)?;
        let (topo, geom, brep) = build_result.into_parts();
        Ok(FeatureOutput {
            topology: topo,
            geometry: geom,
            brep,
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
    euler_ops: [
        crate::engine::contract::EulerOpKind::SplitEdge,
        crate::engine::contract::EulerOpKind::MakeEdgeFace,
    ],
    surfaces: [crate::engine::contract::SurfaceKind::Planar],
    // NOTE: Boolean operations validate topology internally via the boolean engine.
    // ManifoldEdges is NOT declared here because the engine's own validation
    // pipeline handles it. Adding it would double-validate and surface
    // engine-internal intermediate states as feature contract violations.
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
        _config: &ResolvedConfig,
    ) -> Result<FeatureOutput, KernelError> {
        let input = BooleanInput::new(
            inputs.target.topology.clone(),
            inputs.target.geometry.clone(),
            inputs.target.brep.clone(),
            inputs.tool.topology.clone(),
            inputs.tool.geometry.clone(),
            inputs.tool.brep.clone(),
            self.op,
        );

        // The inner engine handles its own execution and span logic, but we still need to
        // make sure it emits into the pipeline's active span. Because we are in an active
        // KernelSpan, execute_boolean's internal ModelingContext will delegate its logs outward!
        let envelope = execute_boolean(input);
        // We do not call ctx.absorb_sub_result(&mut envelope) here anymore because the
        // inner routines directly logged to KernelSpan! Wait, what about metrics?
        // Let's just return the envelope... wait, the trait requires returning FeatureOutput.
        // So we unpack the envelope. The pipeline executor will scoop up the active span.
        let result = envelope.into_result()?;
        let (topo, geom, brep) = result.into_states();

        Ok(FeatureOutput {
            topology: topo,
            geometry: geom,
            brep,
        })
    }

    fn dependencies(&self) -> Vec<NodeId> {
        vec![self.target, self.tool]
    }

    fn name(&self) -> &str {
        &self.name
    }
}
