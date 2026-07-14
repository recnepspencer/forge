use crate::{DamageClassification, IntegrityEvidenceOutcome, QuarantineRecord};
use worth_store_contracts::StableDigest;

pub(crate) fn quarantine_evidence_outcome(record: &QuarantineRecord) -> IntegrityEvidenceOutcome {
    match record.damage_classification() {
        DamageClassification::IntactPhysicalBoundary(_) => {
            IntegrityEvidenceOutcome::IntactPhysicalBoundary
        }
        DamageClassification::RebuildableDerivedDamage(_) => {
            IntegrityEvidenceOutcome::RebuildableDerivedDamage
        }
        DamageClassification::QuarantinedPhysicalDamage(_) => {
            IntegrityEvidenceOutcome::QuarantinedPhysicalDamage
        }
        DamageClassification::UnrecoverableAuthorityDamage(_) => {
            IntegrityEvidenceOutcome::UnrecoverableAuthorityDamage
        }
        DamageClassification::IndeterminatePhysicalDamage(_) => {
            IntegrityEvidenceOutcome::IndeterminatePhysicalDamage
        }
    }
}

pub(crate) fn quarantine_evidence_denial_count(record: &QuarantineRecord) -> u8 {
    match record.damage_classification() {
        DamageClassification::IntactPhysicalBoundary(_) => 0,
        DamageClassification::RebuildableDerivedDamage(_)
        | DamageClassification::QuarantinedPhysicalDamage(_)
        | DamageClassification::UnrecoverableAuthorityDamage(_)
        | DamageClassification::IndeterminatePhysicalDamage(_) => 1,
    }
}

pub(crate) fn quarantine_receipt_claim_basis(record: &QuarantineRecord) -> StableDigest {
    record.receipt().foundational_basis().digest().clone()
}
