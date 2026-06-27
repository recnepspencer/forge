use crate::replay_undo_inventory::{
    ReplayUndoInventoryDisposition, ReplayUndoInventoryOwner, ReplayUndoInventoryReport,
    ReplayUndoInventorySourceIdentity,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::error::{ReplayUndoHardDeletionError, ReplayUndoHardDeletionErrorKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReplayUndoHardDeletionDisposition {
    Deleted,
    CappedResidue,
    QueryGap,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoHardDeletionLedgerRow {
    source_identity: ReplayUndoInventorySourceIdentity,
    owner: ReplayUndoInventoryOwner,
    disposition: ReplayUndoHardDeletionDisposition,
    removal_trigger: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoHardDeletionLedger {
    rows: Vec<ReplayUndoHardDeletionLedgerRow>,
    ledger_digest: String,
}

impl ReplayUndoHardDeletionLedger {
    pub(crate) fn from_inventory(
        inventory: &ReplayUndoInventoryReport,
    ) -> Result<Self, ReplayUndoHardDeletionError> {
        let rows = inventory
            .rows()
            .iter()
            .filter_map(|row| {
                let disposition = match row.disposition() {
                    ReplayUndoInventoryDisposition::Migrate => return None,
                    ReplayUndoInventoryDisposition::Delete => {
                        ReplayUndoHardDeletionDisposition::Deleted
                    }
                    ReplayUndoInventoryDisposition::Cap => {
                        ReplayUndoHardDeletionDisposition::CappedResidue
                    }
                    ReplayUndoInventoryDisposition::QueryGap => {
                        ReplayUndoHardDeletionDisposition::QueryGap
                    }
                };
                let removal_trigger = row.removal_trigger().ok_or_else(|| {
                    ReplayUndoHardDeletionError::new(
                        ReplayUndoHardDeletionErrorKind::MissingHardDeletionRemovalTrigger,
                        format!(
                            "replay/undo hard-deletion row `{}` is missing a removal trigger",
                            row.source_identity().as_str()
                        ),
                    )
                });
                Some(
                    removal_trigger.map(|trigger| ReplayUndoHardDeletionLedgerRow {
                        source_identity: row.source_identity(),
                        owner: row.owner(),
                        disposition,
                        removal_trigger: trigger.to_string(),
                    }),
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        let ledger_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &ledger_digest_parts(&rows),
        );
        Ok(Self {
            rows,
            ledger_digest,
        })
    }

    pub fn rows(&self) -> &[ReplayUndoHardDeletionLedgerRow] {
        &self.rows
    }

    pub fn ledger_digest(&self) -> &str {
        &self.ledger_digest
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }
}

impl ReplayUndoHardDeletionLedgerRow {
    pub const fn source_identity(&self) -> ReplayUndoInventorySourceIdentity {
        self.source_identity
    }

    pub const fn owner(&self) -> ReplayUndoInventoryOwner {
        self.owner
    }

    pub const fn disposition(&self) -> ReplayUndoHardDeletionDisposition {
        self.disposition
    }

    pub fn removal_trigger(&self) -> &str {
        &self.removal_trigger
    }
}

fn ledger_digest_parts(rows: &[ReplayUndoHardDeletionLedgerRow]) -> Vec<String> {
    let mut parts = vec!["worth-kernel:replay-undo-hard-deletion-ledger:v1".to_string()];
    parts.extend(rows.iter().map(|row| {
        format!(
            "{}:{:?}:{:?}:{}",
            row.source_identity.as_str(),
            row.owner,
            row.disposition,
            row.removal_trigger
        )
    }));
    parts
}
