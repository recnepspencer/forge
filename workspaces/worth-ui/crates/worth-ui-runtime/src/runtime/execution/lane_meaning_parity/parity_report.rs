use crate::runtime::{
    WorthUiLaneParityCertification, WorthUiLaneParityCounters, WorthUiLaneTransitionParity,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLaneParityReport {
    certification: WorthUiLaneParityCertification,
    transitions: Vec<WorthUiLaneTransitionParity>,
    counters: WorthUiLaneParityCounters,
}

impl WorthUiLaneParityReport {
    pub(crate) fn new(
        certification: WorthUiLaneParityCertification,
        transitions: Vec<WorthUiLaneTransitionParity>,
        counters: WorthUiLaneParityCounters,
    ) -> Self {
        Self {
            certification,
            transitions,
            counters,
        }
    }

    pub fn certification(&self) -> WorthUiLaneParityCertification {
        self.certification
    }

    pub fn transitions(&self) -> &[WorthUiLaneTransitionParity] {
        &self.transitions
    }

    pub fn counters(&self) -> WorthUiLaneParityCounters {
        self.counters
    }

    pub fn certifies_activation(&self) -> bool {
        self.counters.semantic_mismatch_count() == 0
            && self.counters.visual_only_evidence_rejected_count() == 0
    }
}
