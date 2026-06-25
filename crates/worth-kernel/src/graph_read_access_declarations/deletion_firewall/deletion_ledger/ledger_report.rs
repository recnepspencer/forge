use std::path::Path;

use crate::graph_read_access_inventory::WorthGraphReadDeletionLedgerItem;

use super::super::errors::{
    WorthGraphReadDeclarationDeletionFirewallError,
    WorthGraphReadDeclarationDeletionFirewallErrorKind,
};
use super::super::stable_identity_digest::stable_digest;
use super::ledger_row::{
    WorthGraphReadDeclarationDeletionLedgerRow, WorthGraphReadDeclarationDeletionStatus,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadDeclarationDeletionLedgerReport {
    rows: Vec<WorthGraphReadDeclarationDeletionLedgerRow>,
    deleted_count: usize,
    capped_residue_count: usize,
    report_digest: String,
}

impl WorthGraphReadDeclarationDeletionLedgerReport {
    pub(in crate::graph_read_access_declarations::deletion_firewall) fn from_deletion_items(
        deletion_items: &[WorthGraphReadDeletionLedgerItem],
        workspace_root: &Path,
    ) -> Result<Self, WorthGraphReadDeclarationDeletionFirewallError> {
        if deletion_items.is_empty() {
            return Err(error(
                WorthGraphReadDeclarationDeletionFirewallErrorKind::MissingDeletionLedgerItem,
            ));
        }
        let rows = deletion_items
            .iter()
            .map(|item| {
                WorthGraphReadDeclarationDeletionLedgerRow::from_deletion_item(item, workspace_root)
            })
            .collect::<Vec<_>>();
        if rows.iter().any(|row| {
            row.status() == WorthGraphReadDeclarationDeletionStatus::CappedResidue
                && row.blocker().is_none()
        }) {
            return Err(error(
                WorthGraphReadDeclarationDeletionFirewallErrorKind::DeletionResidueRequiresBlocker,
            ));
        }
        let deleted_count = rows
            .iter()
            .filter(|row| row.status() == WorthGraphReadDeclarationDeletionStatus::Deleted)
            .count();
        let capped_residue_count = rows.len() - deleted_count;
        let mut digest_parts = vec![
            "worth_graph_read_declaration_deletion_ledger_report_v1".to_string(),
            format!("deleted_count:{deleted_count}"),
            format!("capped_residue_count:{capped_residue_count}"),
        ];
        digest_parts.extend(rows.iter().map(|row| format!("row:{}", row.row_digest())));
        Ok(Self {
            rows,
            deleted_count,
            capped_residue_count,
            report_digest: stable_digest(&digest_parts),
        })
    }

    pub fn rows(&self) -> &[WorthGraphReadDeclarationDeletionLedgerRow] {
        &self.rows
    }

    pub const fn deleted_count(&self) -> usize {
        self.deleted_count
    }

    pub const fn capped_residue_count(&self) -> usize {
        self.capped_residue_count
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

const fn error(
    kind: WorthGraphReadDeclarationDeletionFirewallErrorKind,
) -> WorthGraphReadDeclarationDeletionFirewallError {
    WorthGraphReadDeclarationDeletionFirewallError::new(kind)
}
