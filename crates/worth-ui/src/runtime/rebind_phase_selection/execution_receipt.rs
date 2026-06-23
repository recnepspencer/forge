use crate::runtime::{
    WorthUiHeaderFrameRebindReceipt, WorthUiPageHostRebindReceipt,
    WorthUiRuntimeChangeEvidenceDigest, WorthUiRuntimeInstanceWitness,
};

use super::{WorthUiRebindPhaseSelectionCounters, WorthUiRebindPhaseSelectionRow};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiRebindPhaseExecutionReceipt {
    runtime_instance: WorthUiRuntimeInstanceWitness,
    change_evidence_digest: WorthUiRuntimeChangeEvidenceDigest,
    counters: WorthUiRebindPhaseSelectionCounters,
    rows: Vec<WorthUiRebindPhaseSelectionRow>,
    replay_digest: u64,
    header_rebind: WorthUiHeaderFrameRebindReceipt,
    page_host_rebind: WorthUiPageHostRebindReceipt,
}

impl WorthUiRebindPhaseExecutionReceipt {
    pub(crate) fn new(
        runtime_instance: WorthUiRuntimeInstanceWitness,
        change_evidence_digest: WorthUiRuntimeChangeEvidenceDigest,
        counters: WorthUiRebindPhaseSelectionCounters,
        rows: Vec<WorthUiRebindPhaseSelectionRow>,
        replay_digest: u64,
        header_rebind: WorthUiHeaderFrameRebindReceipt,
        page_host_rebind: WorthUiPageHostRebindReceipt,
    ) -> Self {
        Self {
            runtime_instance,
            change_evidence_digest,
            counters,
            rows,
            replay_digest,
            header_rebind,
            page_host_rebind,
        }
    }

    pub fn runtime_instance(&self) -> WorthUiRuntimeInstanceWitness {
        self.runtime_instance
    }

    pub fn change_evidence_digest(&self) -> WorthUiRuntimeChangeEvidenceDigest {
        self.change_evidence_digest
    }

    pub fn counters(&self) -> WorthUiRebindPhaseSelectionCounters {
        self.counters
    }

    pub fn rows(&self) -> &[WorthUiRebindPhaseSelectionRow] {
        &self.rows
    }

    pub fn replay_digest(&self) -> u64 {
        self.replay_digest
    }

    pub fn header_rebind(&self) -> &WorthUiHeaderFrameRebindReceipt {
        &self.header_rebind
    }

    pub fn page_host_rebind(&self) -> &WorthUiPageHostRebindReceipt {
        &self.page_host_rebind
    }
}
