use super::{HarnessEvidenceBundle, HarnessEvidenceFamily, HarnessOperationReceipt};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HarnessEvidenceLedger {
    records: Vec<HarnessOperationReceipt>,
}

impl HarnessEvidenceLedger {
    pub(crate) fn empty() -> Self {
        Self::default()
    }

    pub(crate) fn push(&mut self, receipt: HarnessOperationReceipt) {
        self.records.push(receipt);
    }

    pub fn records(&self) -> &[HarnessOperationReceipt] {
        &self.records
    }

    pub fn record_for_step(&self, step_index: usize) -> Option<&HarnessOperationReceipt> {
        self.records
            .iter()
            .find(|receipt| receipt.step_index() == step_index)
    }

    pub fn contains_family_at_step(
        &self,
        step_index: usize,
        family: HarnessEvidenceFamily,
    ) -> bool {
        self.record_for_step(step_index)
            .is_some_and(|receipt| receipt.contains(family))
    }

    pub(crate) fn aggregate_evidence(&self) -> HarnessEvidenceBundle {
        let mut aggregate = HarnessEvidenceBundle::empty();
        for receipt in &self.records {
            aggregate.merge_step_evidence(receipt.evidence().clone());
        }
        aggregate
    }
}
