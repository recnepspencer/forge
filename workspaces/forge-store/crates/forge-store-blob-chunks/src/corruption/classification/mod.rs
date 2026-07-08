mod damage_case;
mod damage_decision_table;
mod generation_posture;
mod pre_decode_gate;

pub use damage_case::BlobDamageCase;
pub(crate) use damage_decision_table::{
    classify_localization_eligibility_from_frontier_match, LocalizationEligibilityCase,
};
pub(in crate::corruption) use pre_decode_gate::classify_physical_damage_before_decode;
pub use pre_decode_gate::{
    classify_blob_damage_before_decode, classify_streaming_damage_before_decode, BlobDamageEvidence,
};

pub use generation_posture::{
    AuthoritativeBlobCorruptionPosture, BlobCorruptionGenerationClassification,
    DerivedBlobCorruptionRebuildReadiness,
};
