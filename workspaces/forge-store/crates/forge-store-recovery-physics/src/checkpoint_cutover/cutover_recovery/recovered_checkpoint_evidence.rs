use forge_store_physical_backend::BackendDurabilityProfile;
use forge_store_physical_format::PhysicalReference;

use super::super::{
    CheckpointCutoverReceipt, CheckpointDurabilityEvidence, CheckpointDurabilityRole, CheckpointId,
    CheckpointValidationDenial, StoreOwnedCheckpointLocator,
};
use super::recovered_mismatch;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveredCheckpointSelector {
    checkpoint_id: CheckpointId,
}

impl RecoveredCheckpointSelector {
    pub(crate) fn from_store_locator(
        receipt: &CheckpointCutoverReceipt,
        locator: &StoreOwnedCheckpointLocator,
    ) -> Result<Self, CheckpointValidationDenial> {
        if locator.checkpoint_id() != receipt.checkpoint_id() {
            return Err(recovered_mismatch(receipt));
        }
        Ok(Self {
            checkpoint_id: receipt.checkpoint_id().clone(),
        })
    }

    pub(super) fn checkpoint_id(&self) -> &CheckpointId {
        &self.checkpoint_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveredCheckpointRoot {
    checkpoint_id: CheckpointId,
    root_reference: PhysicalReference,
}

impl RecoveredCheckpointRoot {
    pub(crate) fn from_durability_evidence<P: BackendDurabilityProfile>(
        receipt: &CheckpointCutoverReceipt,
        evidence: &CheckpointDurabilityEvidence<P>,
    ) -> Result<Self, CheckpointValidationDenial> {
        if evidence.role() != CheckpointDurabilityRole::Root
            || evidence.checkpoint_id() != receipt.checkpoint_id()
        {
            return Err(recovered_mismatch(receipt));
        }
        Ok(Self {
            checkpoint_id: receipt.checkpoint_id().clone(),
            root_reference: evidence.root_reference(),
        })
    }

    pub(super) fn checkpoint_id(&self) -> &CheckpointId {
        &self.checkpoint_id
    }

    pub const fn root_reference(&self) -> PhysicalReference {
        self.root_reference
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveredCheckpointManifestMedia {
    checkpoint_id: CheckpointId,
}

impl RecoveredCheckpointManifestMedia {
    pub(crate) fn from_durability_evidence<P: BackendDurabilityProfile>(
        receipt: &CheckpointCutoverReceipt,
        manifest: &CheckpointDurabilityEvidence<P>,
        page_lsn_frontier: &CheckpointDurabilityEvidence<P>,
    ) -> Result<Self, CheckpointValidationDenial> {
        if manifest.role() != CheckpointDurabilityRole::Manifest
            || page_lsn_frontier.role() != CheckpointDurabilityRole::PageLsnFrontier
            || manifest.checkpoint_id() != receipt.checkpoint_id()
            || page_lsn_frontier.checkpoint_id() != receipt.checkpoint_id()
        {
            return Err(recovered_mismatch(receipt));
        }
        Ok(Self {
            checkpoint_id: receipt.checkpoint_id().clone(),
        })
    }

    pub(super) fn checkpoint_id(&self) -> &CheckpointId {
        &self.checkpoint_id
    }
}
