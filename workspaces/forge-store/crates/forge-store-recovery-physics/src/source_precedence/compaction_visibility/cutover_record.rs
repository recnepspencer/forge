use super::cutover_basis::{CompactionCutoverBasis, CompactionGenerationIdentity};
use crate::CheckpointCutoverReceipt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedCompactionCutoverRecord {
    pub(super) basis: CompactionCutoverBasis,
}

impl AdmittedCompactionCutoverRecord {
    pub fn from_checkpoint_cutover_receipt(
        generation: CompactionGenerationIdentity,
        receipt: &CheckpointCutoverReceipt,
    ) -> Self {
        Self {
            basis: CompactionCutoverBasis::new(
                generation,
                receipt.checkpoint_id().clone(),
                receipt.covered_lsn_range().range(),
            ),
        }
    }

    pub const fn generation(&self) -> CompactionGenerationIdentity {
        self.basis.generation()
    }

    pub fn artifact_digest(&self) -> &str {
        self.basis.artifact_digest()
    }
}
