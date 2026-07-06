mod damage_case;
mod damage_decision_table;
mod generation_posture;

pub use damage_case::BlobDamageCase;
pub(crate) use damage_decision_table::{
    classify_damage_case_from_detection_context,
    classify_localization_eligibility_from_frontier_match,
    classify_streaming_read_damage_from_checksum_match,
    damage_case_for_localization_denial, map_pre_decode_denial_kind,
    LocalizationEligibilityCase,
};

pub use generation_posture::{
    AuthoritativeBlobCorruptionPosture, BlobCorruptionGenerationClassification,
    DerivedBlobCorruptionRebuildReadiness,
};