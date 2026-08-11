use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SupportDomainCertificationCounterSnapshot {
    scenario_row_count: u64,
    certified_semantic_row_count: u64,
    explicit_debt_row_count: u64,
    index_probe_count: u64,
    receipt_reuse_count: u64,
    allocation_count: u64,
    physical_readiness_debt_count: u64,
}

impl SupportDomainCertificationCounterSnapshot {
    pub fn new(
        scenario_row_count: u64,
        certified_semantic_row_count: u64,
        explicit_debt_row_count: u64,
        index_probe_count: u64,
        receipt_reuse_count: u64,
        allocation_count: u64,
        physical_readiness_debt_count: u64,
    ) -> Self {
        Self {
            scenario_row_count,
            certified_semantic_row_count,
            explicit_debt_row_count,
            index_probe_count,
            receipt_reuse_count,
            allocation_count,
            physical_readiness_debt_count,
        }
    }

    pub fn scenario_row_count(&self) -> u64 {
        self.scenario_row_count
    }

    pub fn certified_semantic_row_count(&self) -> u64 {
        self.certified_semantic_row_count
    }

    pub fn explicit_debt_row_count(&self) -> u64 {
        self.explicit_debt_row_count
    }

    pub fn index_probe_count(&self) -> u64 {
        self.index_probe_count
    }

    pub fn receipt_reuse_count(&self) -> u64 {
        self.receipt_reuse_count
    }

    pub fn allocation_count(&self) -> u64 {
        self.allocation_count
    }

    pub fn physical_readiness_debt_count(&self) -> u64 {
        self.physical_readiness_debt_count
    }
}
