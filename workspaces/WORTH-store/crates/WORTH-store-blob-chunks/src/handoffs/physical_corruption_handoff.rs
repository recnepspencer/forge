use worth_store_physical_integrity::PreDecodePhysicalDenial;

use crate::corruption::{
    observe_physical_pre_decode_denial, PhysicalCorruptionHandoffClassification,
};
use crate::BlobCorruptionDenial;

/// Cross-crate production handoff: physical-integrity pre-decode denial â†’ blob corruption rejection.
pub fn reject_physical_handoff_from_pre_decode_denial(
    denial: &PreDecodePhysicalDenial,
) -> (
    PhysicalCorruptionHandoffClassification,
    BlobCorruptionDenial,
) {
    observe_physical_pre_decode_denial(denial)
}
