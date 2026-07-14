use worth_store_physical_integrity::PreDecodePhysicalDenial;

use super::classify_physical_handoff::{
    classify_and_reject_physical_handoff, PhysicalCorruptionHandoffClassification,
};
use crate::BlobCorruptionDenial;

/// Production observation entry: physical pre-decode denial does not mint blob authority.
pub fn observe_physical_pre_decode_denial(
    denial: &PreDecodePhysicalDenial,
) -> (
    PhysicalCorruptionHandoffClassification,
    BlobCorruptionDenial,
) {
    classify_and_reject_physical_handoff(denial)
}
