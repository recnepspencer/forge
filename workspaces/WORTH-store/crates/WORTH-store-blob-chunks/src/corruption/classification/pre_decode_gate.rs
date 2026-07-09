use worth_store_physical_integrity::{PreDecodePhysicalDenial, PreDecodePhysicalDenialKind};

use super::damage_case::BlobDamageCase;
use super::damage_decision_table::{
    classify_damage_case_from_detection_context,
    classify_streaming_read_damage_from_checksum_match, damage_case_for_localization_denial,
    map_pre_decode_denial_kind, LocalizationEligibilityCase,
};
use crate::corruption::types::{BlobCorruptionDetectionSource, BlobCorruptionPlacementClass};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobDamageEvidence {
    StreamingChecksumMismatch,
    DetectionContext {
        source: BlobCorruptionDetectionSource,
        placement: BlobCorruptionPlacementClass,
    },
    OrdinalNotInFrontier,
    GenerationFrontierMismatch,
    PhysicalPreDecode(PreDecodePhysicalDenialKind),
}

pub fn classify_blob_damage_before_decode(evidence: BlobDamageEvidence) -> BlobDamageCase {
    match evidence {
        BlobDamageEvidence::StreamingChecksumMismatch => BlobDamageCase::ChecksumMismatch,
        BlobDamageEvidence::DetectionContext { source, placement } => {
            classify_damage_case_from_detection_context(source, placement)
        }
        BlobDamageEvidence::OrdinalNotInFrontier => {
            damage_case_for_localization_denial(LocalizationEligibilityCase::OrdinalNotInFrontier)
        }
        BlobDamageEvidence::GenerationFrontierMismatch => damage_case_for_localization_denial(
            LocalizationEligibilityCase::GenerationFrontierMismatch,
        ),
        BlobDamageEvidence::PhysicalPreDecode(kind) => map_pre_decode_denial_kind(kind),
    }
}

pub fn classify_streaming_damage_before_decode(checksums_match: bool) -> Option<BlobDamageCase> {
    classify_streaming_read_damage_from_checksum_match(checksums_match)
        .map(|_| classify_blob_damage_before_decode(BlobDamageEvidence::StreamingChecksumMismatch))
}

pub fn classify_physical_damage_before_decode(denial: &PreDecodePhysicalDenial) -> BlobDamageCase {
    classify_blob_damage_before_decode(BlobDamageEvidence::PhysicalPreDecode(denial.kind()))
}
