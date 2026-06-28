use super::cutover_basis::{CompactionCutoverBasis, CompactionGenerationIdentity};
use crate::source_precedence::CheckpointBaseAdmission;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoverableOldCompactionGeneration {
    pub(super) basis: CompactionCutoverBasis,
}

impl RecoverableOldCompactionGeneration {
    pub fn from_checkpoint_base_admission(
        generation: CompactionGenerationIdentity,
        admission: &CheckpointBaseAdmission,
    ) -> Self {
        Self {
            basis: CompactionCutoverBasis::new(
                generation,
                admission.checkpoint_id().clone(),
                admission.covered_lsn_range(),
            ),
        }
    }

    pub const fn generation(&self) -> CompactionGenerationIdentity {
        self.basis.generation()
    }
}
