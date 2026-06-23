use crate::runtime::{WorthUiRuntimeChangeEvidenceDigest, WorthUiRuntimeInstanceWitness};

use super::{WorthUiProjectionRebindCounters, WorthUiProjectionRebindRowReceipt};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiProjectionRebindBatchReceipt {
    runtime_instance: WorthUiRuntimeInstanceWitness,
    change_evidence_digest: WorthUiRuntimeChangeEvidenceDigest,
    counters: WorthUiProjectionRebindCounters,
    rows: Vec<WorthUiProjectionRebindRowReceipt>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiProjectionRebindBatchAggregationDenial {
    EmptyBatch,
    RuntimeEvidenceMismatch,
}

impl WorthUiProjectionRebindBatchReceipt {
    pub(crate) fn single_row(
        runtime_instance: WorthUiRuntimeInstanceWitness,
        change_evidence_digest: WorthUiRuntimeChangeEvidenceDigest,
        counters: WorthUiProjectionRebindCounters,
        row: WorthUiProjectionRebindRowReceipt,
    ) -> Self {
        Self {
            runtime_instance,
            change_evidence_digest,
            counters,
            rows: vec![row],
        }
    }

    #[cfg(test)]
    pub(crate) fn from_rows_for_test(
        runtime_instance: WorthUiRuntimeInstanceWitness,
        change_evidence_digest: WorthUiRuntimeChangeEvidenceDigest,
        counters: WorthUiProjectionRebindCounters,
        rows: impl IntoIterator<Item = WorthUiProjectionRebindRowReceipt>,
    ) -> Self {
        Self {
            runtime_instance,
            change_evidence_digest,
            counters,
            rows: rows.into_iter().collect(),
        }
    }

    pub fn aggregate(
        receipts: impl IntoIterator<Item = Self>,
    ) -> Result<WorthUiProjectionRebindBatchReceipt, WorthUiProjectionRebindBatchAggregationDenial>
    {
        let mut receipts = receipts.into_iter();
        let first = receipts
            .next()
            .ok_or(WorthUiProjectionRebindBatchAggregationDenial::EmptyBatch)?;
        let runtime_instance = first.runtime_instance;
        let change_evidence_digest = first.change_evidence_digest;
        let mut counters = vec![first.counters];
        let mut rows = first.rows;
        for receipt in receipts {
            if receipt.runtime_instance != runtime_instance
                || receipt.change_evidence_digest != change_evidence_digest
            {
                return Err(WorthUiProjectionRebindBatchAggregationDenial::RuntimeEvidenceMismatch);
            }
            counters.push(receipt.counters);
            rows.extend(receipt.rows);
        }
        Ok(Self {
            runtime_instance,
            change_evidence_digest,
            counters: WorthUiProjectionRebindCounters::aggregate(counters),
            rows,
        })
    }

    pub fn runtime_instance(&self) -> WorthUiRuntimeInstanceWitness {
        self.runtime_instance
    }

    pub fn change_evidence_digest(&self) -> WorthUiRuntimeChangeEvidenceDigest {
        self.change_evidence_digest
    }

    pub fn counters(&self) -> WorthUiProjectionRebindCounters {
        self.counters
    }

    pub fn rows(&self) -> &[WorthUiProjectionRebindRowReceipt] {
        &self.rows
    }
}
