use crate::{
    BlobChunkQuarantine, BlobCorruptionGenerationClassification, BlobObjectClassification,
};

pub fn classify_generation_posture(
    quarantine: &BlobChunkQuarantine,
    classification: BlobObjectClassification,
) -> BlobCorruptionGenerationClassification {
    BlobCorruptionGenerationClassification::from_quarantine(quarantine, classification)
}