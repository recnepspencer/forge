use forge_store_physical_integrity::PreDecodePhysicalDenial;

use super::super::classification::map_pre_decode_denial_kind;
use crate::BlobDamageCase;
use crate::BlobCorruptionDenial;

/// Maps physical pre-decode denial to blob damage case for handoff classification.
/// Physical evidence cannot mint blob localization or quarantine authority.
pub fn classify_physical_pre_decode_damage(
    denial: &PreDecodePhysicalDenial,
) -> BlobDamageCase {
    map_pre_decode_denial_kind(denial.kind())
}

pub const fn reject_physical_evidence_as_blob_corruption_authority(
    _denial: &PreDecodePhysicalDenial,
) -> BlobCorruptionDenial {
    BlobCorruptionDenial::LowerPhysicalEvidenceRejected
}