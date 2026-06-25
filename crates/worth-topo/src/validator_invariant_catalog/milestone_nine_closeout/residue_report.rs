use crate::validator_invariant_catalog::milestone_nine_closeout::{
    WorthTopologyMilestoneNineDeletionDisposition, WorthTopologyMilestoneNineDeletionLedgerReport,
};
use crate::validator_invariant_catalog::WorthTopologyOperatorCertificationCutoverCloseout;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthTopologyMilestoneNineResidueStatus {
    CappedByDeletionLedger,
    StaleWithoutDeletionLedger,
    UncappedAuthority,
}

impl WorthTopologyMilestoneNineResidueStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CappedByDeletionLedger => "capped-by-deletion-ledger",
            Self::StaleWithoutDeletionLedger => "stale-without-deletion-ledger",
            Self::UncappedAuthority => "uncapped-authority",
        }
    }

    pub const fn is_closed(self) -> bool {
        matches!(self, Self::CappedByDeletionLedger)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyMilestoneNineResidueAuditRow {
    source_path: String,
    source_residue_digest: String,
    deletion_ledger_row_digest: Option<String>,
    status: WorthTopologyMilestoneNineResidueStatus,
    row_digest: String,
}

impl WorthTopologyMilestoneNineResidueAuditRow {
    pub(in crate::validator_invariant_catalog) fn new(
        source_path: impl Into<String>,
        source_residue_digest: impl Into<String>,
        deletion_ledger_row_digest: Option<String>,
        status: WorthTopologyMilestoneNineResidueStatus,
    ) -> Self {
        let source_path = source_path.into();
        let source_residue_digest = source_residue_digest.into();
        let ledger_digest = deletion_ledger_row_digest.as_deref().unwrap_or("none");
        let row_digest = [
            "worth-topo-milestone-nine-residue-audit-row-v1",
            source_path.as_str(),
            source_residue_digest.as_str(),
            ledger_digest,
            status.as_str(),
        ]
        .join("|");
        Self {
            source_path,
            source_residue_digest,
            deletion_ledger_row_digest,
            status,
            row_digest,
        }
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub fn source_residue_digest(&self) -> &str {
        &self.source_residue_digest
    }

    pub fn deletion_ledger_row_digest(&self) -> Option<&str> {
        self.deletion_ledger_row_digest.as_deref()
    }

    pub const fn status(&self) -> WorthTopologyMilestoneNineResidueStatus {
        self.status
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyMilestoneNineResidueAuditReport {
    rows: Vec<WorthTopologyMilestoneNineResidueAuditRow>,
    report_digest: String,
}

impl WorthTopologyMilestoneNineResidueAuditReport {
    pub(in crate::validator_invariant_catalog) fn from_cutover_and_deletion_ledger(
        cutover: &WorthTopologyOperatorCertificationCutoverCloseout,
        deletion_ledger: &WorthTopologyMilestoneNineDeletionLedgerReport,
    ) -> Self {
        let rows = cutover
            .old_expectation_residue()
            .rows()
            .iter()
            .map(|residue| {
                let ledger_row = deletion_ledger
                    .rows()
                    .iter()
                    .find(|row| row.source_path() == residue.source_path());
                let status = match ledger_row {
                    Some(row)
                        if row.disposition()
                            == WorthTopologyMilestoneNineDeletionDisposition::CappedResidue =>
                    {
                        WorthTopologyMilestoneNineResidueStatus::CappedByDeletionLedger
                    }
                    Some(_) => WorthTopologyMilestoneNineResidueStatus::UncappedAuthority,
                    None => WorthTopologyMilestoneNineResidueStatus::StaleWithoutDeletionLedger,
                };
                WorthTopologyMilestoneNineResidueAuditRow::new(
                    residue.source_path(),
                    residue.row_digest(),
                    ledger_row.map(|row| row.row_digest().to_string()),
                    status,
                )
            })
            .collect::<Vec<_>>();
        Self::from_rows(rows)
    }

    pub(in crate::validator_invariant_catalog) fn from_rows(
        rows: impl IntoIterator<Item = WorthTopologyMilestoneNineResidueAuditRow>,
    ) -> Self {
        let rows = rows.into_iter().collect::<Vec<_>>();
        let mut digest_parts = vec![
            "worth-topo-milestone-nine-residue-audit-report-v1".to_string(),
            format!("row-count:{}", rows.len()),
        ];
        digest_parts.extend(rows.iter().map(|row| format!("row:{}", row.row_digest())));
        Self {
            rows,
            report_digest: digest_parts.join("|"),
        }
    }

    pub fn rows(&self) -> &[WorthTopologyMilestoneNineResidueAuditRow] {
        &self.rows
    }

    pub fn uncapped_authority_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| {
                row.status() == WorthTopologyMilestoneNineResidueStatus::UncappedAuthority
            })
            .count()
    }

    pub fn stale_residue_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| {
                row.status() == WorthTopologyMilestoneNineResidueStatus::StaleWithoutDeletionLedger
            })
            .count()
    }

    pub fn capped_residue_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.status().is_closed())
            .count()
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}
