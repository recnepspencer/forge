use crate::validator_invariant_catalog::selected_validator_enforcement::loop_wiring::{
    WorthTopologyLoopWiringHalfEdgeWitnessRow, WorthTopologyLoopWiringLoopWitnessRow,
};
#[cfg(test)]
use crate::validator_invariant_catalog::selection_from_touched_closure::WorthTopologySelectedLegalityObligationRow;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthTopologyLoopWiringAdmittedLocalFacts {
    selected_obligation_digest: String,
    admitted_fact_receipt_digest: String,
    loop_rows: Vec<WorthTopologyLoopWiringLoopWitnessRow>,
    half_edge_rows: Vec<WorthTopologyLoopWiringHalfEdgeWitnessRow>,
    rejected_outside_loop_fact_count: usize,
    rejected_outside_half_edge_fact_count: usize,
    direct_materialized_report_row_read_count: usize,
    projection_consumed_fact_receipt_count: usize,
}

impl WorthTopologyLoopWiringAdmittedLocalFacts {
    #[cfg(test)]
    pub(in crate::validator_invariant_catalog) fn from_selected_obligation_and_rows(
        selected_obligation: &WorthTopologySelectedLegalityObligationRow,
        admitted_fact_receipt_digest: impl Into<String>,
        loop_rows: impl IntoIterator<Item = WorthTopologyLoopWiringLoopWitnessRow>,
        half_edge_rows: impl IntoIterator<Item = WorthTopologyLoopWiringHalfEdgeWitnessRow>,
    ) -> Self {
        Self::from_selected_obligation_rows_and_rejected_counts(
            selected_obligation,
            admitted_fact_receipt_digest,
            loop_rows,
            half_edge_rows,
            0,
            0,
        )
    }

    #[cfg(test)]
    pub(in crate::validator_invariant_catalog) fn from_selected_obligation_rows_and_rejected_counts(
        selected_obligation: &WorthTopologySelectedLegalityObligationRow,
        admitted_fact_receipt_digest: impl Into<String>,
        loop_rows: impl IntoIterator<Item = WorthTopologyLoopWiringLoopWitnessRow>,
        half_edge_rows: impl IntoIterator<Item = WorthTopologyLoopWiringHalfEdgeWitnessRow>,
        rejected_outside_loop_fact_count: usize,
        rejected_outside_half_edge_fact_count: usize,
    ) -> Self {
        Self {
            selected_obligation_digest: selected_obligation.row_digest().to_string(),
            admitted_fact_receipt_digest: admitted_fact_receipt_digest.into(),
            loop_rows: loop_rows.into_iter().collect(),
            half_edge_rows: half_edge_rows.into_iter().collect(),
            rejected_outside_loop_fact_count,
            rejected_outside_half_edge_fact_count,
            direct_materialized_report_row_read_count: 0,
            projection_consumed_fact_receipt_count: 0,
        }
    }

    #[cfg(test)]
    pub(in crate::validator_invariant_catalog) fn from_unbound_selected_obligation_digest_for_tests(
        selected_obligation_digest: impl Into<String>,
        admitted_fact_receipt_digest: impl Into<String>,
        loop_rows: impl IntoIterator<Item = WorthTopologyLoopWiringLoopWitnessRow>,
        half_edge_rows: impl IntoIterator<Item = WorthTopologyLoopWiringHalfEdgeWitnessRow>,
    ) -> Self {
        Self {
            selected_obligation_digest: selected_obligation_digest.into(),
            admitted_fact_receipt_digest: admitted_fact_receipt_digest.into(),
            loop_rows: loop_rows.into_iter().collect(),
            half_edge_rows: half_edge_rows.into_iter().collect(),
            rejected_outside_loop_fact_count: 0,
            rejected_outside_half_edge_fact_count: 0,
            direct_materialized_report_row_read_count: 0,
            projection_consumed_fact_receipt_count: 0,
        }
    }

    pub fn selected_obligation_digest(&self) -> &str {
        &self.selected_obligation_digest
    }

    pub fn admitted_fact_receipt_digest(&self) -> &str {
        &self.admitted_fact_receipt_digest
    }

    pub fn loop_rows(&self) -> &[WorthTopologyLoopWiringLoopWitnessRow] {
        &self.loop_rows
    }

    pub fn half_edge_rows(&self) -> &[WorthTopologyLoopWiringHalfEdgeWitnessRow] {
        &self.half_edge_rows
    }

    pub const fn rejected_outside_loop_fact_count(&self) -> usize {
        self.rejected_outside_loop_fact_count
    }

    pub const fn rejected_outside_half_edge_fact_count(&self) -> usize {
        self.rejected_outside_half_edge_fact_count
    }

    pub const fn direct_materialized_report_row_read_count(&self) -> usize {
        self.direct_materialized_report_row_read_count
    }

    pub const fn projection_consumed_fact_receipt_count(&self) -> usize {
        self.projection_consumed_fact_receipt_count
    }
}
