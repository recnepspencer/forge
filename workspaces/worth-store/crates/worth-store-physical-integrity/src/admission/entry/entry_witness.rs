use crate::{IntegrityEntryBasis, ScrubEnvelopeLimits, VerifierResidentLimits};
use worth_store_contracts::PhysicalIntegrityReadinessPayload;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntegrityEntryWitness {
    basis: IntegrityEntryBasis,
}

impl IntegrityEntryWitness {
    pub(crate) const fn mint(payload: PhysicalIntegrityReadinessPayload) -> Self {
        Self {
            basis: IntegrityEntryBasis::from_payload(payload),
        }
    }

    pub const fn entry_basis(self) -> IntegrityEntryBasis {
        self.basis
    }

    pub const fn verifier_resident_limits(self) -> VerifierResidentLimits {
        self.basis.verifier_resident_limits()
    }

    pub const fn scrub_envelope_limits(self) -> ScrubEnvelopeLimits {
        self.basis.scrub_envelope_limits()
    }

    pub const fn proves_recovery_behavior(self) -> bool {
        false
    }

    pub const fn proves_blob_lifecycle(self) -> bool {
        false
    }

    pub const fn proves_repair_behavior(self) -> bool {
        false
    }

    pub const fn proves_authenticity(self) -> bool {
        false
    }

    pub const fn proves_certification_closeout(self) -> bool {
        false
    }
}
