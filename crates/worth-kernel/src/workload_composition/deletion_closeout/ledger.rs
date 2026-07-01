use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::phase_fifteen_deleted_surfaces::current_phase_fifteen_deleted_surface_rows;
use crate::workload_composition::{
    ConflictBatchAdmissionDisposition, ConflictBatchAdmissionInventory,
    ConflictBatchAdmissionInventoryError, ConflictBatchAdmissionOwner,
    ConflictBatchAdmissionReplacementPhase,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthTouchedGraphConflictDeletionDisposition {
    DeletedAuthority,
    CappedResidue,
    CertificationOnlyFence,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphConflictDeletionLedgerRow {
    source_path: String,
    surface_name: String,
    owner: ConflictBatchAdmissionOwner,
    disposition: WorthTouchedGraphConflictDeletionDisposition,
    blocker: String,
    removal_trigger: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphConflictDeletionLedger {
    rows: Vec<WorthTouchedGraphConflictDeletionLedgerRow>,
    ledger_digest: String,
}

impl WorthTouchedGraphConflictDeletionLedgerRow {
    pub(super) fn explicit(
        source_path: String,
        surface_name: String,
        owner: ConflictBatchAdmissionOwner,
        disposition: WorthTouchedGraphConflictDeletionDisposition,
        blocker: String,
        removal_trigger: String,
    ) -> Self {
        Self {
            source_path,
            surface_name,
            owner,
            disposition,
            blocker,
            removal_trigger,
        }
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub fn surface_name(&self) -> &str {
        &self.surface_name
    }

    pub const fn owner(&self) -> ConflictBatchAdmissionOwner {
        self.owner
    }

    pub const fn disposition(&self) -> WorthTouchedGraphConflictDeletionDisposition {
        self.disposition
    }

    pub fn blocker(&self) -> &str {
        &self.blocker
    }

    pub fn removal_trigger(&self) -> &str {
        &self.removal_trigger
    }
}

impl WorthTouchedGraphConflictDeletionLedger {
    pub(crate) fn from_inventory(
        inventory: &ConflictBatchAdmissionInventory,
    ) -> Result<Self, ConflictBatchAdmissionInventoryError> {
        let mut rows = expected_deletion_ledger_rows(inventory)?;
        rows.sort_by(|left, right| {
            left.source_path
                .cmp(&right.source_path)
                .then(left.surface_name.cmp(&right.surface_name))
        });
        let parts = rows
            .iter()
            .map(|row| {
                format!(
                    "{}:{}:{:?}:{:?}:{}:{}",
                    row.source_path,
                    row.surface_name,
                    row.owner,
                    row.disposition,
                    row.blocker,
                    row.removal_trigger
                )
            })
            .chain(std::iter::once(
                "worth-kernel:touched-graph-conflict-deletion-ledger:v1".to_string(),
            ))
            .collect::<Vec<_>>();
        let ledger_digest = truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts);
        Ok(Self {
            rows,
            ledger_digest,
        })
    }

    pub fn rows(&self) -> &[WorthTouchedGraphConflictDeletionLedgerRow] {
        &self.rows
    }

    pub fn ledger_digest(&self) -> &str {
        &self.ledger_digest
    }
}

pub(crate) fn expected_deletion_ledger_rows(
    inventory: &ConflictBatchAdmissionInventory,
) -> Result<Vec<WorthTouchedGraphConflictDeletionLedgerRow>, ConflictBatchAdmissionInventoryError> {
    let mut rows = inventory
        .rows()
        .iter()
        .filter(|row| {
            row.replacement_phase()
                == ConflictBatchAdmissionReplacementPhase::PhaseTwelveFirewallDeletion
        })
        .map(|row| {
            let disposition = match row.disposition() {
                ConflictBatchAdmissionDisposition::Migrate
                | ConflictBatchAdmissionDisposition::Delete => {
                    WorthTouchedGraphConflictDeletionDisposition::DeletedAuthority
                }
                ConflictBatchAdmissionDisposition::Cap => {
                    WorthTouchedGraphConflictDeletionDisposition::CappedResidue
                }
                ConflictBatchAdmissionDisposition::CertificationOnly => {
                    WorthTouchedGraphConflictDeletionDisposition::CertificationOnlyFence
                }
                ConflictBatchAdmissionDisposition::QueryGap => {
                    return Err(
                        ConflictBatchAdmissionInventoryError::SourceFirewallViolation(format!(
                            "phase 12 closeout cannot bind query gap `{}`",
                            row.surface_name()
                        )),
                    )
                }
            };
            Ok(WorthTouchedGraphConflictDeletionLedgerRow::explicit(
                row.source_path().to_string(),
                row.surface_name().to_string(),
                row.owner(),
                disposition,
                row.blocker().to_string(),
                row.removal_trigger().to_string(),
            ))
        })
        .collect::<Result<Vec<_>, _>>()?;
    rows.extend(current_phase_fifteen_deleted_surface_rows());
    Ok(rows)
}
