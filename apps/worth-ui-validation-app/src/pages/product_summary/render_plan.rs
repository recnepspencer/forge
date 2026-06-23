use worth_ui::facade::WorthUiPageHostSlotReceipt;

use super::projection::ValidationProductSummaryProjection;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationProductSummaryRenderPlan {
    projection: ValidationProductSummaryProjection,
}

impl ValidationProductSummaryRenderPlan {
    pub fn from_projection(projection: ValidationProductSummaryProjection) -> Self {
        Self { projection }
    }

    pub fn page_name(&self) -> &str {
        self.projection.page_name()
    }

    pub fn page_host_frame_digest(&self) -> u64 {
        self.projection.page_host_frame_digest()
    }

    pub fn active_artifact_digest(&self) -> u64 {
        self.projection.active_artifact_digest()
    }

    pub fn active_plan_digest(&self) -> u64 {
        self.projection.active_plan_digest()
    }

    pub fn capability_snapshot_digest(&self) -> u64 {
        self.projection.capability_snapshot_digest()
    }

    pub fn frame_epoch(&self) -> u64 {
        self.projection.frame_epoch()
    }

    pub fn slots(&self) -> &[WorthUiPageHostSlotReceipt] {
        self.projection.slots()
    }

    pub fn evidence(&self) -> &super::ValidationProductSummaryEvidence {
        self.projection.evidence()
    }
}
