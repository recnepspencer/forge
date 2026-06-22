use forge_foundational::facade::{
    FoundationalAuthoritativePerformanceClaim, FoundationalCounterBackedPerformanceReceipt,
    FoundationalPerformanceCounterRow,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerBinaryCounterSet {
    receipt: FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>,
    canonical_digest: String,
}

impl ForgeServerBinaryCounterSet {
    pub(crate) fn new(
        receipt: FoundationalCounterBackedPerformanceReceipt<
            FoundationalAuthoritativePerformanceClaim,
        >,
    ) -> Self {
        let canonical_digest = canonical_digest(&receipt);
        Self {
            receipt,
            canonical_digest,
        }
    }

    pub fn receipt(
        &self,
    ) -> &FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>
    {
        &self.receipt
    }

    pub fn counter(&self, name: &str) -> Option<u64> {
        self.receipt
            .counter_rows()
            .iter()
            .find(|row| row.name().as_str() == name)
            .map(FoundationalPerformanceCounterRow::observed_count)
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

fn canonical_digest(
    receipt: &FoundationalCounterBackedPerformanceReceipt<
        FoundationalAuthoritativePerformanceClaim,
    >,
) -> String {
    receipt
        .counter_rows()
        .iter()
        .map(|row| format!("{}={}", row.name().as_str(), row.observed_count()))
        .collect::<Vec<_>>()
        .join("|")
}
