use sha2::{Digest, Sha256};
use worth_store_authority::ControlStoreSelectionCoordinates;

use super::{ControlStoreSelectionIndeterminate, SelectedOperationalControlState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DivergentControlGenerationSelectionReceipt {
    selected_generation: u64,
    rejected_generation: u64,
    receipt_identity: [u8; 32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DivergentControlGenerationSelectionDenial {
    SelectedGenerationNotAdvanced,
    RejectedGenerationNotAdvanced,
    SameControlMedia,
    SameDurablePrefix,
    RejectionDidNotNameSelectedMedia,
}

impl DivergentControlGenerationSelectionReceipt {
    pub fn from_selected_generation_and_rejected_copy(
        selected: &SelectedOperationalControlState,
        rejected_copy: ControlStoreSelectionCoordinates,
        rejection: &ControlStoreSelectionIndeterminate,
    ) -> Result<Self, DivergentControlGenerationSelectionDenial> {
        let selected = selected.selected_generation();
        if selected.generation().get() <= 1 {
            return Err(DivergentControlGenerationSelectionDenial::SelectedGenerationNotAdvanced);
        }
        if rejected_copy.generation().get() <= 1 {
            return Err(DivergentControlGenerationSelectionDenial::RejectedGenerationNotAdvanced);
        }
        if selected.media_identity_fingerprint() == rejected_copy.media_identity_fingerprint() {
            return Err(DivergentControlGenerationSelectionDenial::SameControlMedia);
        }
        if selected.prefix_digest() == rejected_copy.prefix_digest() {
            return Err(DivergentControlGenerationSelectionDenial::SameDurablePrefix);
        }
        if !matches!(
            rejection,
            ControlStoreSelectionIndeterminate::SelectedMediaUnavailable {
                media_identity_fingerprint,
            } if *media_identity_fingerprint == selected.media_identity_fingerprint()
        ) {
            return Err(
                DivergentControlGenerationSelectionDenial::RejectionDidNotNameSelectedMedia,
            );
        }
        let mut digest = Sha256::new();
        digest.update(b"worth-store-divergent-control-generation-selection-v1");
        digest.update(selected.authority_identity().fingerprint());
        digest.update(selected.media_identity_fingerprint());
        digest.update(selected.generation().get().to_be_bytes());
        digest.update(selected.prefix_digest());
        digest.update(rejected_copy.media_identity_fingerprint());
        digest.update(rejected_copy.generation().get().to_be_bytes());
        digest.update(rejected_copy.prefix_digest());
        Ok(Self {
            selected_generation: selected.generation().get(),
            rejected_generation: rejected_copy.generation().get(),
            receipt_identity: digest.finalize().into(),
        })
    }

    pub const fn selected_generation(self) -> u64 {
        self.selected_generation
    }

    pub const fn rejected_generation(self) -> u64 {
        self.rejected_generation
    }

    pub const fn receipt_identity(self) -> [u8; 32] {
        self.receipt_identity
    }
}
