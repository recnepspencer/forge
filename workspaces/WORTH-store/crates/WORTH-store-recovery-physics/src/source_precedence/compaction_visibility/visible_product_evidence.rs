use super::{
    cutover_basis::{
        require_same_cutover_basis, require_same_generation, CompactionGenerationIdentity,
    },
    AdmittedCompactionCutoverDurability, AdmittedCompactionCutoverRecord,
    RecoverableOldCompactionGeneration,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompactionVisibleProductEvidence {
    generation: CompactionGenerationIdentity,
}

impl CompactionVisibleProductEvidence {
    pub fn admit(
        generation: CompactionGenerationIdentity,
        cutover: AdmittedCompactionCutoverRecord,
        old_generation: RecoverableOldCompactionGeneration,
        durability: AdmittedCompactionCutoverDurability,
    ) -> Result<Self, CompactionVisibleProductEvidenceDenial> {
        require_same_generation(generation, cutover.generation())?;
        require_same_generation(generation, old_generation.generation())?;
        require_same_generation(generation, durability.generation())?;
        require_same_cutover_basis(&cutover.basis, &old_generation.basis)?;
        require_same_cutover_basis(&cutover.basis, &durability.basis)?;
        Ok(Self { generation })
    }

    pub const fn generation(self) -> CompactionGenerationIdentity {
        self.generation
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompactionVisibleProductEvidenceDenial {
    GenerationMismatch {
        expected: CompactionGenerationIdentity,
        observed: CompactionGenerationIdentity,
    },
    CutoverBasisMismatch,
    CutoverDurabilityArtifactMismatch,
    CutoverDurabilityRangeMismatch,
}
