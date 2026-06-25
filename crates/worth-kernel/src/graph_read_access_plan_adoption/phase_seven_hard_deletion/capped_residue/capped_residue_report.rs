use super::super::deletion_proof::WorthGraphReadAccessHardDeletionProofReport;
use super::super::errors::{
    WorthGraphReadAccessHardDeletionError, WorthGraphReadAccessHardDeletionErrorKind,
};
use super::super::stable_digest;
use super::capped_residue_row::WorthGraphReadAccessHardDeletionCappedResidueRow;
use super::residue_cap_policy::hard_deletion_residue_cap_for_source_path;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessHardDeletionCappedResidueReport {
    rows: Vec<WorthGraphReadAccessHardDeletionCappedResidueRow>,
    residue_count: usize,
    uncapped_residue_count: usize,
    report_digest: String,
}

impl WorthGraphReadAccessHardDeletionCappedResidueReport {
    pub(in crate::graph_read_access_plan_adoption::phase_seven_hard_deletion) fn from_deletion_proof(
        deletion_proof: &WorthGraphReadAccessHardDeletionProofReport,
    ) -> Result<Self, WorthGraphReadAccessHardDeletionError> {
        let rows = deletion_proof
            .rows()
            .iter()
            .filter_map(|row| {
                WorthGraphReadAccessHardDeletionCappedResidueRow::from_deletion_row(
                    row,
                    hard_deletion_residue_cap_for_source_path(row.source_path()),
                )
            })
            .collect::<Vec<_>>();
        let uncapped_residue_count = rows.iter().filter(|row| !row.is_within_cap()).count();
        if uncapped_residue_count > 0 {
            return Err(WorthGraphReadAccessHardDeletionError::new(
                WorthGraphReadAccessHardDeletionErrorKind::CappedResidueCapExceeded,
            ));
        }
        let residue_count = rows.len();
        let report_digest = stable_digest(
            &std::iter::once(
                "worth_graph_read_access_hard_deletion_capped_residue_report_v1".to_string(),
            )
            .chain(rows.iter().map(|row| format!("row:{}", row.row_digest())))
            .chain([
                format!("residue_count:{residue_count}"),
                format!("uncapped_residue_count:{uncapped_residue_count}"),
            ])
            .collect::<Vec<_>>(),
        );
        Ok(Self {
            rows,
            residue_count,
            uncapped_residue_count,
            report_digest,
        })
    }

    pub fn rows(&self) -> &[WorthGraphReadAccessHardDeletionCappedResidueRow] {
        &self.rows
    }

    pub const fn residue_count(&self) -> usize {
        self.residue_count
    }

    pub const fn uncapped_residue_count(&self) -> usize {
        self.uncapped_residue_count
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[cfg(test)]
mod adversarial_capped_residue_report {
    use super::*;

    impl WorthGraphReadAccessHardDeletionCappedResidueReport {
        pub(crate) fn with_uncapped_residue_for_tests(&self) -> Self {
            let mut report = self.clone();
            report.uncapped_residue_count = self.uncapped_residue_count + 1;
            report.report_digest = stable_digest(&[
                "worth_graph_read_access_hard_deletion_capped_residue_report_adversarial_uncapped_v1"
                    .to_string(),
                format!("source:{}", self.report_digest),
                format!("uncapped:{}", report.uncapped_residue_count),
            ]);
            report
        }
    }
}
