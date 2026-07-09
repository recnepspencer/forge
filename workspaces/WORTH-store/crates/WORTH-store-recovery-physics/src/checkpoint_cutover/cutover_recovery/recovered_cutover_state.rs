use worth_store_physical_backend::BackendDurabilityProfile;

use super::super::{
    CheckpointCutoverReceipt, CheckpointDurabilityEvidence, CheckpointValidationDenial,
    StoreOwnedCheckpointLocator,
};
use super::recovered_checkpoint_evidence::{
    RecoveredCheckpointManifestMedia, RecoveredCheckpointRoot, RecoveredCheckpointSelector,
};
use super::recovered_mismatch;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckpointCutoverCrashStage {
    BeforeCutover,
    DuringCutover,
    AfterCutover,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveredCheckpointCutoverState {
    receipt: Option<CheckpointCutoverReceipt>,
    selector: Option<RecoveredCheckpointSelector>,
    root: Option<RecoveredCheckpointRoot>,
    manifest_media: Option<RecoveredCheckpointManifestMedia>,
}

impl RecoveredCheckpointCutoverState {
    pub const fn before_cutover() -> Self {
        Self {
            receipt: None,
            selector: None,
            root: None,
            manifest_media: None,
        }
    }

    pub const fn during_cutover_without_durable_selector() -> Self {
        Self {
            receipt: None,
            selector: None,
            root: None,
            manifest_media: None,
        }
    }

    pub fn admit_selected_during_cutover<P: BackendDurabilityProfile>(
        receipt: CheckpointCutoverReceipt,
        locator: StoreOwnedCheckpointLocator,
        root: &CheckpointDurabilityEvidence<P>,
        manifest: &CheckpointDurabilityEvidence<P>,
        page_lsn_frontier: &CheckpointDurabilityEvidence<P>,
    ) -> Result<Self, CheckpointValidationDenial> {
        Self::admit_selected(receipt, locator, root, manifest, page_lsn_frontier)
    }

    pub fn admit_selected_after_cutover<P: BackendDurabilityProfile>(
        receipt: CheckpointCutoverReceipt,
        locator: StoreOwnedCheckpointLocator,
        root: &CheckpointDurabilityEvidence<P>,
        manifest: &CheckpointDurabilityEvidence<P>,
        page_lsn_frontier: &CheckpointDurabilityEvidence<P>,
    ) -> Result<Self, CheckpointValidationDenial> {
        Self::admit_selected(receipt, locator, root, manifest, page_lsn_frontier)
    }

    fn admit_selected<P: BackendDurabilityProfile>(
        receipt: CheckpointCutoverReceipt,
        locator: StoreOwnedCheckpointLocator,
        root: &CheckpointDurabilityEvidence<P>,
        manifest: &CheckpointDurabilityEvidence<P>,
        page_lsn_frontier: &CheckpointDurabilityEvidence<P>,
    ) -> Result<Self, CheckpointValidationDenial> {
        let selector = RecoveredCheckpointSelector::from_store_locator(&receipt, &locator)?;
        let root = RecoveredCheckpointRoot::from_durability_evidence(&receipt, root)?;
        let manifest_media = RecoveredCheckpointManifestMedia::from_durability_evidence(
            &receipt,
            manifest,
            page_lsn_frontier,
        )?;
        Self::selected(receipt, selector, root, manifest_media)
    }

    fn selected(
        receipt: CheckpointCutoverReceipt,
        selector: RecoveredCheckpointSelector,
        root: RecoveredCheckpointRoot,
        manifest_media: RecoveredCheckpointManifestMedia,
    ) -> Result<Self, CheckpointValidationDenial> {
        if selector.checkpoint_id() != receipt.checkpoint_id()
            || root.checkpoint_id() != receipt.checkpoint_id()
            || manifest_media.checkpoint_id() != receipt.checkpoint_id()
        {
            return Err(recovered_mismatch(&receipt));
        }
        Ok(Self {
            receipt: Some(receipt),
            selector: Some(selector),
            root: Some(root),
            manifest_media: Some(manifest_media),
        })
    }

    pub(super) const fn receipt(&self) -> Option<&CheckpointCutoverReceipt> {
        self.receipt.as_ref()
    }

    pub(super) const fn has_selected_checkpoint_basis(&self) -> bool {
        self.receipt.is_some()
            && self.selector.is_some()
            && self.root.is_some()
            && self.manifest_media.is_some()
    }
}
