mod admit_readmission;
mod classify_generation_posture;
mod classify_physical_handoff;
mod localize_damage;
mod observe_physical_pre_decode;
mod seal_quarantine;

pub use classify_physical_handoff::{
    classify_and_reject_physical_handoff, reject_physical_handoff_as_blob_authority,
    PhysicalCorruptionHandoffClassification,
};
pub use observe_physical_pre_decode::observe_physical_pre_decode_denial;

pub(in crate::corruption) use admit_readmission::verify_current_store_authority_for_readmission;
pub use classify_generation_posture::classify_generation_posture;
pub use localize_damage::{
    from_detected_source, from_read_corruption, from_streaming_read_request,
};
pub use seal_quarantine::{seal, seal_quarantine_from_localization};
