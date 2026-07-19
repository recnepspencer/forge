use crate::memory_workspace::WorthQueryEntity;
use crate::runtime::{
    WorthQueryAuthoritativeMutationObligationDispatch, WorthQueryGraphObligationAttachmentEvidence,
};

use super::{WorthQueryReadExecutionProduct, WorthQueryReadReceipt};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryReadResult {
    rows: Vec<WorthQueryEntity>,
    receipt: WorthQueryReadReceipt,
}

impl WorthQueryReadResult {
    pub fn rows(&self) -> &[WorthQueryEntity] {
        &self.rows
    }

    pub fn receipt(&self) -> &WorthQueryReadReceipt {
        &self.receipt
    }

    pub fn graph_obligation_dispatch(
        &self,
    ) -> Option<&WorthQueryAuthoritativeMutationObligationDispatch> {
        self.receipt.graph_obligation_dispatch()
    }

    pub fn graph_obligation_evidence(&self) -> Option<WorthQueryGraphObligationAttachmentEvidence> {
        self.receipt.graph_obligation_evidence()
    }

    pub fn graph_obligation_envelope_digest(&self) -> Option<&str> {
        self.receipt.graph_obligation_envelope_digest()
    }

    pub(in crate::runtime) fn new(
        rows: Vec<WorthQueryEntity>,
        receipt: WorthQueryReadReceipt,
    ) -> Self {
        Self { rows, receipt }
    }

    #[cfg(test)]
    pub(crate) fn test_only(rows: Vec<WorthQueryEntity>, receipt: WorthQueryReadReceipt) -> Self {
        Self { rows, receipt }
    }
}

impl WorthQueryReadExecutionProduct for WorthQueryReadResult {
    fn receipt(&self) -> &WorthQueryReadReceipt {
        &self.receipt
    }

    fn receipt_mut(&mut self) -> &mut WorthQueryReadReceipt {
        &mut self.receipt
    }

    fn output_cardinality(&self) -> usize {
        self.rows.len()
    }
}
