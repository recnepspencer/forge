use crate::runtime::{
    WorthUiArtifactToPlanProvenance, WorthUiExecutionPlanDigest, WorthUiLaneInspection,
    WorthUiPlanInspectionCounters, WorthUiPlanNodeInspection, WorthUiRuntimeHandle,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiExecutionPlanInspection {
    active_artifact_digest: u64,
    handle_basis_digest: u64,
    plan_digest: WorthUiExecutionPlanDigest,
    nodes: Vec<WorthUiPlanNodeInspection>,
    lanes: Vec<WorthUiLaneInspection>,
    provenance: Vec<WorthUiArtifactToPlanProvenance>,
    counters: WorthUiPlanInspectionCounters,
}

impl WorthUiExecutionPlanInspection {
    pub(crate) fn new(
        active_artifact_digest: u64,
        handle_basis_digest: u64,
        plan_digest: WorthUiExecutionPlanDigest,
        nodes: Vec<WorthUiPlanNodeInspection>,
        lanes: Vec<WorthUiLaneInspection>,
        provenance: Vec<WorthUiArtifactToPlanProvenance>,
        counters: WorthUiPlanInspectionCounters,
    ) -> Self {
        Self {
            active_artifact_digest,
            handle_basis_digest,
            plan_digest,
            nodes,
            lanes,
            provenance,
            counters,
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
