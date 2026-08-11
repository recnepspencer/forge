use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SupportCertificationCounterSnapshot {
    coverage_row_count: u64,
    first_ship_family_count: u64,
    receipt_reuse_count: u64,
    index_probe_count: u64,
    allocation_count: u64,
    forbidden_exact_overclaim_count: u64,
    global_scan_debt_count: u64,
}

impl SupportCertificationCounterSnapshot {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        coverage_row_count: u64,
        first_ship_family_count: u64,
        receipt_reuse_count: u64,
        index_probe_count: u64,
        allocation_count: u64,
        forbidden_exact_overclaim_count: u64,
        global_scan_debt_count: u64,
    ) -> Self {
        Self {
            coverage_row_count,
            first_ship_family_count,
            receipt_reuse_count,
            index_probe_count,
            allocation_count,
            forbidden_exact_overclaim_count,
            global_scan_debt_count,
        }
    }

    pub fn coverage_row_count(&self) -> u64 {
        self.coverage_row_count
    }

    pub fn first_ship_family_count(&self) -> u64 {
        self.first_ship_family_count
    }

    pub fn receipt_reuse_count(&self) -> u64 {
        self.receipt_reuse_count
    }

    pub fn index_probe_count(&self) -> u64 {
        self.index_probe_count
    }

    pub fn allocation_count(&self) -> u64 {
        self.allocation_count
    }

    pub fn forbidden_exact_overclaim_count(&self) -> u64 {
        self.forbidden_exact_overclaim_count
    }

    pub fn global_scan_debt_count(&self) -> u64 {
        self.global_scan_debt_count
    }
}
