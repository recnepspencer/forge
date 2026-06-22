use crate::runtime::{
    WorthUiExecutionPlanInspection, WorthUiLaneInspection, WorthUiPlanNodeInspection,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPlanInspectionSurface {
    plan_digest: u64,
    nodes: Vec<WorthUiPlanNodeInspection>,
    lanes: Vec<WorthUiLaneInspection>,
}

impl WorthUiPlanInspectionSurface {
    pub(crate) fn from_inspection(inspection: &WorthUiExecutionPlanInspection) -> Self {
        Self {
            plan_digest: inspection.plan_digest().raw(),
            nodes: inspection.nodes().to_vec(),
            lanes: inspection.lanes().to_vec(),
        }
    }

    pub(crate) fn absent(active_plan_digest: u64) -> Self {
        Self {
            plan_digest: active_plan_digest,
            nodes: Vec::new(),
            lanes: Vec::new(),
        }
    }

    pub fn plan_digest(&self) -> u64 {
        self.plan_digest
    }

    pub fn nodes(&self) -> &[WorthUiPlanNodeInspection] {
        &self.nodes
    }

    pub fn lanes(&self) -> &[WorthUiLaneInspection] {
        &self.lanes
    }
}
