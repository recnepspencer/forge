use crate::{CheckpointId, WalLsnRange};

use super::visible_product_evidence::CompactionVisibleProductEvidenceDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompactionGenerationIdentity {
    generation: u64,
}

impl CompactionGenerationIdentity {
    pub const fn new(generation: u64) -> Self {
        Self { generation }
    }

    pub const fn generation(self) -> u64 {
        self.generation
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct CompactionCutoverBasis {
    generation: CompactionGenerationIdentity,
    checkpoint_id: CheckpointId,
    covered_lsn_range: WalLsnRange,
    artifact_digest: String,
}

impl CompactionCutoverBasis {
    pub(super) fn new(
        generation: CompactionGenerationIdentity,
        checkpoint_id: CheckpointId,
        covered_lsn_range: WalLsnRange,
    ) -> Self {
        let artifact_digest =
            compaction_cutover_artifact_digest(generation, &checkpoint_id, covered_lsn_range);
        Self {
            generation,
            checkpoint_id,
            covered_lsn_range,
            artifact_digest,
        }
    }

    pub(super) const fn generation(&self) -> CompactionGenerationIdentity {
        self.generation
    }

    pub(super) fn artifact_digest(&self) -> &str {
        &self.artifact_digest
    }

    pub(super) const fn covered_lsn_range(&self) -> WalLsnRange {
        self.covered_lsn_range
    }
}

pub(super) fn require_same_generation(
    expected: CompactionGenerationIdentity,
    observed: CompactionGenerationIdentity,
) -> Result<(), CompactionVisibleProductEvidenceDenial> {
    if expected != observed {
        return Err(CompactionVisibleProductEvidenceDenial::GenerationMismatch {
            expected,
            observed,
        });
    }
    Ok(())
}

pub(super) fn require_same_cutover_basis(
    expected: &CompactionCutoverBasis,
    observed: &CompactionCutoverBasis,
) -> Result<(), CompactionVisibleProductEvidenceDenial> {
    if expected != observed {
        return Err(CompactionVisibleProductEvidenceDenial::CutoverBasisMismatch);
    }
    Ok(())
}

fn compaction_cutover_artifact_digest(
    generation: CompactionGenerationIdentity,
    checkpoint_id: &CheckpointId,
    covered_lsn_range: WalLsnRange,
) -> String {
    format!(
        "s4-compaction-cutover:generation={}:checkpoint={}:start={}:end={}",
        generation.generation(),
        checkpoint_id.digest().as_str(),
        covered_lsn_range.start().get(),
        covered_lsn_range.end_exclusive().get()
    )
}
