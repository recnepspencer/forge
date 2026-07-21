use crate::runtime::{
    WorthUiArtifactToPlanProvenance, WorthUiExecutionPlanDigest, WorthUiLaneInspection,
    WorthUiPlanInspectionCounters, WorthUiPlanNodeInspection, WorthUiRuntimeHandle,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiExecutionPlanInspection {
    active_artifact_digest: u64,
    handle_basis_digest: u64,
    handle_arena_identity: crate::runtime::WorthUiHandleArenaIdentity,
    lowering_identity: crate::runtime::planning::WorthUiExecutionPlanLoweringIdentity,
    plan_digest: WorthUiExecutionPlanDigest,
    nodes: Vec<WorthUiPlanNodeInspection>,
    lanes: Vec<WorthUiLaneInspection>,
    provenance: Vec<WorthUiArtifactToPlanProvenance>,
    counters: WorthUiPlanInspectionCounters,
}

#[cfg(any(test, feature = "certification-support"))]
pub(crate) struct WorthUiExecutionPlanInspectionInput {
    pub(crate) active_artifact_digest: u64,
    pub(crate) handle_basis_digest: u64,
    pub(crate) handle_arena_identity: crate::runtime::WorthUiHandleArenaIdentity,
    pub(crate) lowering_identity: crate::runtime::planning::WorthUiExecutionPlanLoweringIdentity,
    pub(crate) plan_digest: WorthUiExecutionPlanDigest,
    pub(crate) nodes: Vec<WorthUiPlanNodeInspection>,
    pub(crate) lanes: Vec<WorthUiLaneInspection>,
    pub(crate) provenance: Vec<WorthUiArtifactToPlanProvenance>,
    pub(crate) counters: WorthUiPlanInspectionCounters,
}

impl WorthUiExecutionPlanInspection {
    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn new(input: WorthUiExecutionPlanInspectionInput) -> Self {
        Self {
            active_artifact_digest: input.active_artifact_digest,
            handle_basis_digest: input.handle_basis_digest,
            handle_arena_identity: input.handle_arena_identity,
            lowering_identity: input.lowering_identity,
            plan_digest: input.plan_digest,
            nodes: input.nodes,
            lanes: input.lanes,
            provenance: input.provenance,
            counters: input.counters,
        }
    }

    pub fn plan_digest(&self) -> WorthUiExecutionPlanDigest {
        self.plan_digest
    }

    pub fn active_artifact_digest(&self) -> u64 {
        self.active_artifact_digest
    }

    pub fn handle_basis_digest(&self) -> u64 {
        self.handle_basis_digest
    }

    pub fn handle_arena_identity(&self) -> crate::runtime::WorthUiHandleArenaIdentity {
        self.handle_arena_identity
    }

    pub(crate) fn lowering_identity(
        &self,
    ) -> &crate::runtime::planning::WorthUiExecutionPlanLoweringIdentity {
        &self.lowering_identity
    }

    pub fn nodes(&self) -> &[WorthUiPlanNodeInspection] {
        &self.nodes
    }

    pub fn lanes(&self) -> &[WorthUiLaneInspection] {
        &self.lanes
    }

    pub fn provenance(&self) -> &[WorthUiArtifactToPlanProvenance] {
        &self.provenance
    }

    pub fn counters(&self) -> WorthUiPlanInspectionCounters {
        self.counters
    }

    pub fn node_for_runtime_handle(
        &self,
        runtime_handle: WorthUiRuntimeHandle,
    ) -> Option<&WorthUiPlanNodeInspection> {
        self.nodes
            .iter()
            .find(|node| node.runtime_handle() == runtime_handle)
    }
}
