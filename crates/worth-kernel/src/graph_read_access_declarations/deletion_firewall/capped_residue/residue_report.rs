use super::super::deletion_ledger::{
    WorthGraphReadDeclarationDeletionLedgerReport, WorthGraphReadDeclarationDeletionStatus,
};
use super::super::errors::{
    WorthGraphReadDeclarationDeletionFirewallError,
    WorthGraphReadDeclarationDeletionFirewallErrorKind,
};
use super::super::stable_identity_digest::stable_digest;
use super::declaration_residue_cap_for_source_path;
use super::residue_row::WorthGraphReadDeclarationCappedResidueRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadDeclarationCappedResidueReport {
    rows: Vec<WorthGraphReadDeclarationCappedResidueRow>,
    report_digest: String,
}

impl WorthGraphReadDeclarationCappedResidueReport {
    pub(in crate::graph_read_access_declarations::deletion_firewall) fn from_deletion_report(
        deletion_report: &WorthGraphReadDeclarationDeletionLedgerReport,
    ) -> Result<Self, WorthGraphReadDeclarationDeletionFirewallError> {
        let rows = deletion_report
            .rows()
            .iter()
            .filter(|row| row.status() == WorthGraphReadDeclarationDeletionStatus::CappedResidue)
            .filter_map(|row| {
                WorthGraphReadDeclarationCappedResidueRow::from_deletion_row(
                    row,
                    declaration_residue_cap_for_source_path(row.source_path()),
                )
            })
            .collect::<Vec<_>>();
        if rows.iter().any(|row| !row.is_within_cap()) {
            return Err(WorthGraphReadDeclarationDeletionFirewallError::new(
                WorthGraphReadDeclarationDeletionFirewallErrorKind::CappedResidueCapExceeded,
            ));
        }
        let mut digest_parts = vec![
            "worth_graph_read_declaration_capped_residue_report_v1".to_string(),
            format!("row_count:{}", rows.len()),
        ];
        digest_parts.extend(rows.iter().map(|row| format!("row:{}", row.row_digest())));
        Ok(Self {
            rows,
            report_digest: stable_digest(&digest_parts),
        })
    }

    pub fn rows(&self) -> &[WorthGraphReadDeclarationCappedResidueRow] {
        &self.rows
    }

    pub const fn residue_count(&self) -> usize {
        self.rows.len()
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}
