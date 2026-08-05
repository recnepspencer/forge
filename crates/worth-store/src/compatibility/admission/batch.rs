use super::*;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct CompatibilityAdmissionBatch {
    pub(in crate::compatibility::admission) read_receipts:
        BTreeMap<ReceiptKey, ReadCompatibilityReceipt>,
    pub(in crate::compatibility::admission) write_receipts:
        BTreeMap<ReceiptKey, WriteCompatibilityReceipt>,
    pub(in crate::compatibility::admission) counters: CompatibilityAdmissionCounters,
}
impl CompatibilityAdmissionBatch {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn counters(&self) -> &CompatibilityAdmissionCounters {
        &self.counters
    }

    pub(crate) fn counters_mut(&mut self) -> &mut CompatibilityAdmissionCounters {
        &mut self.counters
    }
}
