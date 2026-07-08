use forge_store_physical_integrity::{PhysicalDamageHandoffEvidence, PreDecodePhysicalDenial};

use crate::corruption::classification::{classify_blob_damage_before_decode, BlobDamageEvidence};
use crate::{BlobCorruptionDenial, BlobDamageCase};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalCorruptionHandoffClassification {
    damage_case: BlobDamageCase,
}

impl PhysicalCorruptionHandoffClassification {
    pub fn classify_from_handoff_evidence(
        evidence: PhysicalDamageHandoffEvidence,
    ) -> PhysicalCorruptionHandoffClassification {
        PhysicalCorruptionHandoffClassification {
            damage_case: classify_blob_damage_before_decode(BlobDamageEvidence::PhysicalPreDecode(
                evidence.denial_kind(),
            )),
        }
    }

    pub fn classify_from_pre_decode_denial(
        denial: &PreDecodePhysicalDenial,
    ) -> PhysicalCorruptionHandoffClassification {
        Self::classify_from_handoff_evidence(denial.handoff_evidence())
    }

    pub const fn damage_case(self) -> BlobDamageCase {
        self.damage_case
    }
}

pub fn classify_and_reject_physical_handoff(
    denial: &PreDecodePhysicalDenial,
) -> (
    PhysicalCorruptionHandoffClassification,
    BlobCorruptionDenial,
) {
    let classification =
        PhysicalCorruptionHandoffClassification::classify_from_pre_decode_denial(denial);
    let rejection = reject_physical_handoff_as_blob_authority(&classification);
    (classification, rejection)
}

pub const fn reject_physical_handoff_as_blob_authority(
    classification: &PhysicalCorruptionHandoffClassification,
) -> BlobCorruptionDenial {
    BlobCorruptionDenial::LowerPhysicalEvidenceRejected {
        damage_case: classification.damage_case(),
    }
}
