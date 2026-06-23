use super::counters::PlanarBooleanSplitChainValidationCounters;
use super::coverage_row::{
    PlanarBooleanOverlapChainCoverageRow, PlanarBooleanSplitFragmentCoverageRow,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanSplitChainValidationReceipt {
    receipt_identity: String,
    split_edge_fragment_set_identity: String,
    overlap_edge_chain_set_identity: String,
    interval_subdivision_schedule_set_identity: String,
    fragment_coverage_rows: Vec<PlanarBooleanSplitFragmentCoverageRow>,
    overlap_coverage_rows: Vec<PlanarBooleanOverlapChainCoverageRow>,
    counters: PlanarBooleanSplitChainValidationCounters,
}

impl PlanarBooleanSplitChainValidationReceipt {
    pub(crate) fn new(
        receipt_identity: String,
        split_edge_fragment_set_identity: String,
        overlap_edge_chain_set_identity: String,
        interval_subdivision_schedule_set_identity: String,
        fragment_coverage_rows: Vec<PlanarBooleanSplitFragmentCoverageRow>,
        overlap_coverage_rows: Vec<PlanarBooleanOverlapChainCoverageRow>,
        counters: PlanarBooleanSplitChainValidationCounters,
    ) -> Self {
        Self {
            receipt_identity,
            split_edge_fragment_set_identity,
            overlap_edge_chain_set_identity,
            interval_subdivision_schedule_set_identity,
            fragment_coverage_rows,
            overlap_coverage_rows,
            counters,
        }
    }

    pub fn receipt_identity(&self) -> &str {
        &self.receipt_identity
    }
    pub fn split_edge_fragment_set_identity(&self) -> &str {
        &self.split_edge_fragment_set_identity
    }
    pub fn overlap_edge_chain_set_identity(&self) -> &str {
        &self.overlap_edge_chain_set_identity
    }
    pub fn interval_subdivision_schedule_set_identity(&self) -> &str {
        &self.interval_subdivision_schedule_set_identity
    }
    pub fn fragment_coverage_rows(&self) -> &[PlanarBooleanSplitFragmentCoverageRow] {
        &self.fragment_coverage_rows
    }
    pub fn overlap_coverage_rows(&self) -> &[PlanarBooleanOverlapChainCoverageRow] {
        &self.overlap_coverage_rows
    }
    pub fn counters(&self) -> PlanarBooleanSplitChainValidationCounters {
        self.counters
    }
    pub fn certifies_split_chain_integrity(&self) -> bool {
        self.counters.gaps_rejected() == 0
            && self.counters.overlaps_rejected() == 0
            && self.counters.dangling_references_rejected() == 0
            && self.counters.mismatched_interval_basis_rejected() == 0
            && self.counters.foreign_chain_sets_rejected() == 0
            && self.counters.out_of_interval_references_rejected() == 0
            && self.counters.denied_chains() == 0
            && self.counters.fragment_schedules_checked() == self.fragment_coverage_rows.len()
    }
}
