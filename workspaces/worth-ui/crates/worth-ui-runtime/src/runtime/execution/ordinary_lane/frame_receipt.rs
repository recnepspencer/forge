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

impl WorthUiOrdinaryLaneFrameReceipt {
    pub(crate) fn new(
        target: WorthUiOrdinaryFrameTarget,
        touch: WorthUiOrdinaryLaneTouchReceipt,
        counters: WorthUiOrdinaryLaneCounters,
        certification: WorthUiOrdinaryLaneCertification,
        requested_breadth: usize,
    ) -> Self {
        let executed_breadth = touch.row_count();
        Self {
            target,
            touch,
            counters,
            certification,
            resolution_evidence: None,
            work_scope: WorthUiFrameWorkScope::new(
                requested_breadth as u64,
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
}
