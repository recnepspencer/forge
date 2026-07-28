use crate::runtime::{
    WorthUiFrameWorkScope, WorthUiHandleResolutionEvidence, WorthUiOrdinaryFrameTarget,
    WorthUiOrdinaryLaneCertification, WorthUiOrdinaryLaneCounters, WorthUiOrdinaryLaneTouchReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiOrdinaryLaneFrameReceipt {
    target: WorthUiOrdinaryFrameTarget,
    touch: WorthUiOrdinaryLaneTouchReceipt,
    counters: WorthUiOrdinaryLaneCounters,
    certification: WorthUiOrdinaryLaneCertification,
    resolution_evidence: Option<WorthUiHandleResolutionEvidence>,
    work_scope: WorthUiFrameWorkScope,
}

pub(super) struct WorthUiOrdinaryLaneFrameReceiptInput {
    pub(super) target: WorthUiOrdinaryFrameTarget,
    pub(super) touch: WorthUiOrdinaryLaneTouchReceipt,
    pub(super) counters: WorthUiOrdinaryLaneCounters,
    pub(super) certification: WorthUiOrdinaryLaneCertification,
    pub(super) requested_breadth: usize,
}

impl WorthUiOrdinaryLaneFrameReceipt {
    pub(super) fn new(input: WorthUiOrdinaryLaneFrameReceiptInput) -> Self {
        let executed_breadth = input.touch.row_count();
        Self {
            target: input.target,
            touch: input.touch,
            counters: input.counters,
            certification: input.certification,
            resolution_evidence: None,
            work_scope: WorthUiFrameWorkScope::new(
                input.requested_breadth as u64,
                executed_breadth as u64,
            ),
        }
    }

    pub fn target(&self) -> WorthUiOrdinaryFrameTarget {
        self.target
    }

    pub fn touch(&self) -> &WorthUiOrdinaryLaneTouchReceipt {
        &self.touch
    }

    pub fn counters(&self) -> WorthUiOrdinaryLaneCounters {
        self.counters
    }

    pub fn certification(&self) -> WorthUiOrdinaryLaneCertification {
        self.certification
    }

    pub(crate) fn with_resolution_evidence(
        mut self,
        evidence: WorthUiHandleResolutionEvidence,
    ) -> Self {
        self.resolution_evidence = Some(evidence);
        self
    }

    pub fn resolution_evidence(&self) -> Option<WorthUiHandleResolutionEvidence> {
        self.resolution_evidence
    }

    pub fn work_scope(&self) -> WorthUiFrameWorkScope {
        self.work_scope
    }

    pub fn visual_inspection_cost(&self) -> worth_ui_inspection::UiVisualInspectionCostReceipt {
        worth_ui_inspection::UiVisualInspectionCostReceipt::default()
    }
}
