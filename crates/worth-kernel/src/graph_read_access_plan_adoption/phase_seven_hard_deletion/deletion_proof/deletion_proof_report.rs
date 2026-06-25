use std::path::Path;

use crate::graph_read_access_plan_adoption::WorthGraphReadAccessExecutionReceiptAccountingPhaseSevenSeed;

use super::super::errors::{
    WorthGraphReadAccessHardDeletionError, WorthGraphReadAccessHardDeletionErrorKind,
};
use super::super::stable_digest;
use super::deletion_proof_row::WorthGraphReadAccessHardDeletionProofRow;
use super::deletion_status::WorthGraphReadAccessHardDeletionStatus;
use super::migrated_execution_target::current_migrated_execution_targets;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessHardDeletionProofReport {
    rows: Vec<WorthGraphReadAccessHardDeletionProofRow>,
    deleted_count: usize,
    capped_residue_count: usize,
    typed_query_gap_count: usize,
    unresolved_count: usize,
    report_digest: String,
}

impl WorthGraphReadAccessHardDeletionProofReport {
    pub(crate) fn from_workspace_root(
        source_seed: &WorthGraphReadAccessExecutionReceiptAccountingPhaseSevenSeed,
        workspace_root: &Path,
    ) -> Result<Self, WorthGraphReadAccessHardDeletionError> {
        let mut rows = vec![
            WorthGraphReadAccessHardDeletionProofRow::from_phase_four_cutover_proof(
                source_seed.phase_four_cutover_proof(),
            ),
        ];
        rows.extend(
            current_migrated_execution_targets()
                .iter()
                .copied()
                .map(|target| {
                    WorthGraphReadAccessHardDeletionProofRow::from_target(target, workspace_root)
                }),
        );
        let report = Self::from_rows(rows);
        if report.unresolved_count > 0 {
            return Err(WorthGraphReadAccessHardDeletionError::new(
                WorthGraphReadAccessHardDeletionErrorKind::UnresolvedMigratedExecutionPath,
            ));
        }
        Ok(report)
    }

    fn from_rows(rows: Vec<WorthGraphReadAccessHardDeletionProofRow>) -> Self {
        let deleted_count = count_status(&rows, WorthGraphReadAccessHardDeletionStatus::Deleted);
        let capped_residue_count =
            count_status(&rows, WorthGraphReadAccessHardDeletionStatus::CappedResidue);
        let typed_query_gap_count = count_status(
            &rows,
            WorthGraphReadAccessHardDeletionStatus::TypedQueryGapWithRemovalTrigger,
        );
        let unresolved_count =
            count_status(&rows, WorthGraphReadAccessHardDeletionStatus::Unresolved);
        let report_digest = stable_digest(
            &std::iter::once("worth_graph_read_access_hard_deletion_proof_report_v1".to_string())
                .chain(rows.iter().map(|row| format!("row:{}", row.row_digest())))
                .chain([
                    format!("deleted:{deleted_count}"),
                    format!("capped_residue:{capped_residue_count}"),
                    format!("typed_query_gap:{typed_query_gap_count}"),
                    format!("unresolved:{unresolved_count}"),
                ])
                .collect::<Vec<_>>(),
        );
        Self {
            rows,
            deleted_count,
            capped_residue_count,
            typed_query_gap_count,
            unresolved_count,
            report_digest,
        }
    }

    #[cfg(test)]
    pub(in crate::graph_read_access_plan_adoption::phase_seven_hard_deletion) fn from_rows_for_test(
        rows: Vec<WorthGraphReadAccessHardDeletionProofRow>,
    ) -> Self {
        Self::from_rows(rows)
    }

    pub fn rows(&self) -> &[WorthGraphReadAccessHardDeletionProofRow] {
        &self.rows
    }

    pub const fn deleted_count(&self) -> usize {
        self.deleted_count
    }

    pub const fn capped_residue_count(&self) -> usize {
        self.capped_residue_count
    }

    pub const fn typed_query_gap_count(&self) -> usize {
        self.typed_query_gap_count
    }

    pub const fn unresolved_count(&self) -> usize {
        self.unresolved_count
    }

    pub const fn all_migrated_paths_deleted_or_capped(&self) -> bool {
        self.unresolved_count == 0
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

fn count_status(
    rows: &[WorthGraphReadAccessHardDeletionProofRow],
    status: WorthGraphReadAccessHardDeletionStatus,
) -> usize {
    rows.iter().filter(|row| row.status() == status).count()
}

#[cfg(test)]
mod adversarial_deletion_proof_report {
    use super::*;

    impl WorthGraphReadAccessHardDeletionProofReport {
        pub(crate) fn with_unresolved_path_for_tests(&self) -> Self {
            let mut report = self.clone();
            report.unresolved_count = self.unresolved_count + 1;
            report.report_digest = stable_digest(&[
                "worth_graph_read_access_hard_deletion_proof_report_adversarial_unresolved_v1"
                    .to_string(),
                format!("source:{}", self.report_digest),
                format!("unresolved:{}", report.unresolved_count),
            ]);
            report
        }
    }
}
