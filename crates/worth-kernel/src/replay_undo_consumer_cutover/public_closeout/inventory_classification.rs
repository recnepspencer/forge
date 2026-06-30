use super::error::{
    ReplayUndoMilestoneTwelvePublicCloseoutError, ReplayUndoMilestoneTwelvePublicCloseoutErrorKind,
};
use crate::replay_undo_inventory::{
    ReplayUndoInventoryDisposition, ReplayUndoInventoryOwner, ReplayUndoInventoryReport,
    ReplayUndoInventorySourceIdentity, ReplayUndoInventorySourceKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReplayUndoPublicCloseoutClassification {
    Migrated,
    Deleted,
    Capped,
    QueryGap,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoPublicCloseoutInventoryRow {
    source_identity: ReplayUndoInventorySourceIdentity,
    source_kind: ReplayUndoInventorySourceKind,
    owner: ReplayUndoInventoryOwner,
    classification: ReplayUndoPublicCloseoutClassification,
    residue_cap: Option<usize>,
    observed_residue_count: usize,
    removal_trigger: Option<String>,
}

impl ReplayUndoPublicCloseoutInventoryRow {
    pub(crate) fn from_inventory(
        inventory: &ReplayUndoInventoryReport,
    ) -> Result<Vec<Self>, ReplayUndoMilestoneTwelvePublicCloseoutError> {
        inventory
            .rows()
            .iter()
            .map(|row| {
                let classification = match row.disposition() {
                    ReplayUndoInventoryDisposition::Migrate => {
                        ReplayUndoPublicCloseoutClassification::Migrated
                    }
                    ReplayUndoInventoryDisposition::Delete => {
                        ReplayUndoPublicCloseoutClassification::Deleted
                    }
                    ReplayUndoInventoryDisposition::Cap => {
                        ReplayUndoPublicCloseoutClassification::Capped
                    }
                    ReplayUndoInventoryDisposition::QueryGap => {
                        ReplayUndoPublicCloseoutClassification::QueryGap
                    }
                };
                require_public_closeout_metadata(
                    row.source_identity(),
                    classification,
                    row.removal_trigger(),
                    row.residue_cap(),
                )?;
                Ok(Self {
                    source_identity: row.source_identity(),
                    source_kind: row.source_kind(),
                    owner: row.owner(),
                    classification,
                    residue_cap: row.residue_cap(),
                    observed_residue_count: row.observed_residue_count(),
                    removal_trigger: row.removal_trigger().map(str::to_string),
                })
            })
            .collect()
    }

    pub const fn source_identity(&self) -> ReplayUndoInventorySourceIdentity {
        self.source_identity
    }

    pub const fn source_kind(&self) -> ReplayUndoInventorySourceKind {
        self.source_kind
    }

    pub const fn owner(&self) -> ReplayUndoInventoryOwner {
        self.owner
    }

    pub const fn classification(&self) -> ReplayUndoPublicCloseoutClassification {
        self.classification
    }

    pub const fn residue_cap(&self) -> Option<usize> {
        self.residue_cap
    }

    pub const fn observed_residue_count(&self) -> usize {
        self.observed_residue_count
    }

    pub fn removal_trigger(&self) -> Option<&str> {
        self.removal_trigger.as_deref()
    }
}

fn require_public_closeout_metadata(
    source_identity: ReplayUndoInventorySourceIdentity,
    classification: ReplayUndoPublicCloseoutClassification,
    removal_trigger: Option<&str>,
    residue_cap: Option<usize>,
) -> Result<(), ReplayUndoMilestoneTwelvePublicCloseoutError> {
    match classification {
        ReplayUndoPublicCloseoutClassification::Migrated => Ok(()),
        ReplayUndoPublicCloseoutClassification::Deleted => {
            require_removal_trigger(source_identity, removal_trigger)
        }
        ReplayUndoPublicCloseoutClassification::Capped
        | ReplayUndoPublicCloseoutClassification::QueryGap => {
            require_removal_trigger(source_identity, removal_trigger)?;
            if residue_cap.is_some() {
                Ok(())
            } else {
                Err(ReplayUndoMilestoneTwelvePublicCloseoutError::new(
                    ReplayUndoMilestoneTwelvePublicCloseoutErrorKind::MissingResidueCap,
                    format!(
                        "public replay/undo closeout row `{}` is missing a residue cap",
                        source_identity.as_str()
                    ),
                ))
            }
        }
    }
}

fn require_removal_trigger(
    source_identity: ReplayUndoInventorySourceIdentity,
    removal_trigger: Option<&str>,
) -> Result<(), ReplayUndoMilestoneTwelvePublicCloseoutError> {
    if removal_trigger.is_some_and(|trigger| !trigger.is_empty()) {
        Ok(())
    } else {
        Err(ReplayUndoMilestoneTwelvePublicCloseoutError::new(
            ReplayUndoMilestoneTwelvePublicCloseoutErrorKind::MissingRemovalTrigger,
            format!(
                "public replay/undo closeout row `{}` is missing a removal trigger",
                source_identity.as_str()
            ),
        ))
    }
}
