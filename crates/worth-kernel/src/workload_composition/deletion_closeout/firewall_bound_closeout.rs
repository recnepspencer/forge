use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::ledger::WorthTouchedGraphConflictDeletionLedger;
use crate::workload_composition::{
    current_conflict_batch_admission_inventory,
    current_worth_touched_graph_conflict_source_firewall_report,
    WorthTouchedGraphConflictSourceFirewallReport,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthTouchedGraphConflictDeletionCloseoutErrorKind {
    SourceFirewallViolation,
    MissingDeletionRows,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphConflictDeletionCloseoutError {
    kind: WorthTouchedGraphConflictDeletionCloseoutErrorKind,
    detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphConflictDeletionCloseout {
    inventory_digest: String,
    source_firewall_report_digest: String,
    deletion_ledger: WorthTouchedGraphConflictDeletionLedger,
    closeout_digest: String,
}

pub fn current_worth_touched_graph_conflict_deletion_closeout(
) -> Result<WorthTouchedGraphConflictDeletionCloseout, WorthTouchedGraphConflictDeletionCloseoutError>
{
    let inventory = current_conflict_batch_admission_inventory().map_err(|error| {
        WorthTouchedGraphConflictDeletionCloseoutError::new(
            WorthTouchedGraphConflictDeletionCloseoutErrorKind::MissingDeletionRows,
            format!("phase 15 deletion inventory did not load: {error:?}"),
        )
    })?;
    let firewall_report =
        current_worth_touched_graph_conflict_source_firewall_report().map_err(|error| {
            WorthTouchedGraphConflictDeletionCloseoutError::new(
                WorthTouchedGraphConflictDeletionCloseoutErrorKind::SourceFirewallViolation,
                format!("phase 15 source firewall did not load: {error:?}"),
            )
        })?;
    closeout_from_products(&inventory, &firewall_report)
}

impl WorthTouchedGraphConflictDeletionCloseoutError {
    fn new(kind: WorthTouchedGraphConflictDeletionCloseoutErrorKind, detail: String) -> Self {
        Self { kind, detail }
    }

    pub const fn kind(&self) -> WorthTouchedGraphConflictDeletionCloseoutErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

impl WorthTouchedGraphConflictDeletionCloseout {
    pub fn inventory_digest(&self) -> &str {
        &self.inventory_digest
    }

    pub fn source_firewall_report_digest(&self) -> &str {
        &self.source_firewall_report_digest
    }

    pub const fn deletion_ledger(&self) -> &WorthTouchedGraphConflictDeletionLedger {
        &self.deletion_ledger
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }
}

pub(crate) fn closeout_from_products(
    inventory: &crate::workload_composition::ConflictBatchAdmissionInventory,
    firewall_report: &WorthTouchedGraphConflictSourceFirewallReport,
) -> Result<WorthTouchedGraphConflictDeletionCloseout, WorthTouchedGraphConflictDeletionCloseoutError>
{
    if !firewall_report.violations().is_empty() {
        return Err(WorthTouchedGraphConflictDeletionCloseoutError::new(
            WorthTouchedGraphConflictDeletionCloseoutErrorKind::SourceFirewallViolation,
            "phase 15 deletion closeout found forbidden source-firewall relapse".to_string(),
        ));
    }
    let deletion_ledger = WorthTouchedGraphConflictDeletionLedger::from_inventory(inventory)
        .map_err(|error| {
            WorthTouchedGraphConflictDeletionCloseoutError::new(
                WorthTouchedGraphConflictDeletionCloseoutErrorKind::MissingDeletionRows,
                format!("phase 15 deletion ledger could not bind inventory rows: {error:?}"),
            )
        })?;
    if deletion_ledger.rows().is_empty() {
        return Err(WorthTouchedGraphConflictDeletionCloseoutError::new(
            WorthTouchedGraphConflictDeletionCloseoutErrorKind::MissingDeletionRows,
            "phase 15 deletion closeout requires concrete deleted or capped surfaces".to_string(),
        ));
    }
    let inventory_digest = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &inventory
            .rows()
            .iter()
            .map(|row| {
                format!(
                    "{}:{}:{:?}:{:?}",
                    row.source_path(),
                    row.surface_name(),
                    row.replacement_phase(),
                    row.disposition()
                )
            })
            .chain(std::iter::once(
                "worth-kernel:touched-graph-conflict-phase-fifteen-deletion-input:v1".to_string(),
            ))
            .collect::<Vec<_>>(),
    );
    let source_firewall_report_digest = firewall_report.report_digest().to_string();
    let closeout_digest = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-kernel:touched-graph-conflict-deletion-closeout:v1".to_string(),
            format!("inventory:{inventory_digest}"),
            format!("source-firewall:{source_firewall_report_digest}"),
            format!("deletion-ledger:{}", deletion_ledger.ledger_digest()),
        ],
    );
    Ok(WorthTouchedGraphConflictDeletionCloseout {
        inventory_digest,
        source_firewall_report_digest,
        deletion_ledger,
        closeout_digest,
    })
}
