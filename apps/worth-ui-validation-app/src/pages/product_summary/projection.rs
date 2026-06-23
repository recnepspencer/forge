use worth_ui::facade::{
    WorthUiActiveRuntimeObservation, WorthUiPageHostFrameReceipt, WorthUiPageHostPlan,
    WorthUiPageHostSlotReceipt,
};

use super::evidence_summary::ValidationProductSummaryEvidence;
use crate::reload::ValidationReloadEvidenceEntry;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationProductSummaryProjection {
    active: WorthUiActiveRuntimeObservation,
    page_host_frame: WorthUiPageHostFrameReceipt,
    evidence: ValidationProductSummaryEvidence,
}

impl ValidationProductSummaryProjection {
    pub fn from_runtime_receipts(
        active: WorthUiActiveRuntimeObservation,
        page_host_plan: &WorthUiPageHostPlan,
        latest_evidence: Option<&ValidationReloadEvidenceEntry>,
    ) -> Self {
        Self {
            active,
            page_host_frame: page_host_plan.execute_frame(),
            evidence: ValidationProductSummaryEvidence::from_latest_entry(latest_evidence),
        }
    }

    pub fn page_name(&self) -> &str {
        self.page_host_frame.page_name()
    }

    pub fn page_host_frame_digest(&self) -> u64 {
        self.page_host_frame.frame_digest()
    }

    pub fn active_artifact_digest(&self) -> u64 {
        self.active.artifact_digest()
    }

    pub fn active_plan_digest(&self) -> u64 {
        self.active.active_plan_digest()
    }

    pub fn capability_snapshot_digest(&self) -> u64 {
        self.active.snapshot_digest()
    }

    pub fn frame_epoch(&self) -> u64 {
        self.active.frame_epoch().as_u64()
    }

    pub fn slots(&self) -> &[WorthUiPageHostSlotReceipt] {
        self.page_host_frame.slots()
    }

    pub fn evidence(&self) -> &ValidationProductSummaryEvidence {
        &self.evidence
    }
}
