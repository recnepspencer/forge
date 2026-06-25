use super::gap::SpatialEvidenceQueryGapRow;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SpatialEvidenceQueryLoweringCounters {
    query_descriptor_count: usize,
    operating_world_descriptor_count: usize,
    query_gap_count: usize,
    broad_ledger_scan_count: usize,
}

impl SpatialEvidenceQueryLoweringCounters {
    pub(super) fn from_descriptors(gap_rows: &[SpatialEvidenceQueryGapRow]) -> Self {
        Self {
            query_descriptor_count: 1,
            operating_world_descriptor_count: 1,
            query_gap_count: gap_rows.len(),
            broad_ledger_scan_count: 0,
        }
    }

    pub fn query_descriptor_count(self) -> usize {
        self.query_descriptor_count
    }

    pub fn operating_world_descriptor_count(self) -> usize {
        self.operating_world_descriptor_count
    }

    pub fn query_gap_count(self) -> usize {
        self.query_gap_count
    }

    pub fn broad_ledger_scan_count(self) -> usize {
        self.broad_ledger_scan_count
    }
}
