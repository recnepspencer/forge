use super::{
    cutover_basis::{
        require_same_generation, CompactionCutoverBasis, CompactionGenerationIdentity,
    },
    AdmittedCompactionCutoverRecord, CompactionVisibleProductEvidenceDenial,
};
use crate::DurableAckReceipt;
use forge_store_physical_backend::BackendDurabilityProfile;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedCompactionCutoverDurability {
    pub(super) basis: CompactionCutoverBasis,
}

impl AdmittedCompactionCutoverDurability {
    pub fn from_durable_ack_receipt<P: BackendDurabilityProfile>(
        generation: CompactionGenerationIdentity,
        cutover: &AdmittedCompactionCutoverRecord,
        ack: &DurableAckReceipt<P>,
    ) -> Result<Self, CompactionVisibleProductEvidenceDenial> {
        require_same_generation(generation, cutover.generation())?;
        require_ack_matches_cutover_basis(ack, &cutover.basis)?;
        Ok(Self {
            basis: cutover.basis.clone(),
        })
    }

    pub const fn generation(&self) -> CompactionGenerationIdentity {
        self.basis.generation()
    }
}

fn require_ack_matches_cutover_basis<P: BackendDurabilityProfile>(
    ack: &DurableAckReceipt<P>,
    basis: &CompactionCutoverBasis,
) -> Result<(), CompactionVisibleProductEvidenceDenial> {
    if ack.ack_basis().lsn_range() != basis.covered_lsn_range() {
        return Err(CompactionVisibleProductEvidenceDenial::CutoverDurabilityRangeMismatch);
    }
    if ack.ack_basis().frame_digest().as_str() != basis.artifact_digest() {
        return Err(CompactionVisibleProductEvidenceDenial::CutoverDurabilityArtifactMismatch);
    }
    Ok(())
}
