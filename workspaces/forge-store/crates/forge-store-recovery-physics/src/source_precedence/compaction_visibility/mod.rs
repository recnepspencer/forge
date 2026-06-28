mod artifact_residue;
mod cutover_basis;
mod cutover_durability;
mod cutover_record;
mod old_generation_recoverability;
mod visible_product_evidence;

pub use artifact_residue::{
    CompactionArtifactResidueReason, CompactionArtifactResidueRejection,
    CompactionCutoverRecoveryPosture, CompactionGenerationVisibility,
};
pub use cutover_basis::CompactionGenerationIdentity;
pub use cutover_durability::AdmittedCompactionCutoverDurability;
pub use cutover_record::AdmittedCompactionCutoverRecord;
pub use old_generation_recoverability::RecoverableOldCompactionGeneration;
pub use visible_product_evidence::{
    CompactionVisibleProductEvidence, CompactionVisibleProductEvidenceDenial,
};
