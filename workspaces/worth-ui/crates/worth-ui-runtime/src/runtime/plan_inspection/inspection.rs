use crate::runtime::{
    WorthUiArtifactToPlanProvenance, WorthUiExecutionPlanDigest, WorthUiLaneInspection,
    WorthUiPlanInspectionCounters, WorthUiPlanNodeInspection, WorthUiRuntimeHandle,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiExecutionPlanInspection {
    plan_digest: WorthUiExecutionPlanDigest,
    nodes: Vec<WorthUiPlanNodeInspection>,
    lanes: Vec<WorthUiLaneInspection>,
    provenance: Vec<WorthUiArtifactToPlanProvenance>,
    counters: WorthUiPlanInspectionCounters,
}

impl WorthUiExecutionPlanInspection {
    pub(crate) fn new(
        plan_digest: WorthUiExecutionPlanDigest,
        nodes: Vec<WorthUiPlanNodeInspection>,
        lanes: Vec<WorthUiLaneInspection>,
        provenance: Vec<WorthUiArtifactToPlanProvenance>,
        counters: WorthUiPlanInspectionCounters,
    ) -> Self {
        Self {
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
