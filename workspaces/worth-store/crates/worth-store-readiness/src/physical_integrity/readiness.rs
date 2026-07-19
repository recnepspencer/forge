use crate::PhysicalSubstrateReadiness;
use worth_store_contracts::{
    PhysicalIntegrityReadinessDenial, PhysicalIntegrityReadinessDenialKind,
    PhysicalIntegrityReadinessPayload,
};

#[derive(Debug, PartialEq, Eq)]
pub struct PhysicalIntegrityReadiness {
    physical_substrate_readiness: PhysicalSubstrateReadiness,
    payload: PhysicalIntegrityReadinessPayload,
}

impl PhysicalIntegrityReadiness {
    pub fn from_physical_substrate_bounded_residency_closeout(
        physical_substrate_readiness: PhysicalSubstrateReadiness,
        payload: PhysicalIntegrityReadinessPayload,
    ) -> Result<Self, PhysicalIntegrityReadinessDenial> {
        if !physical_substrate_readiness.is_sealed() {
            return Err(PhysicalIntegrityReadinessDenial::new(
                PhysicalIntegrityReadinessDenialKind::PhysicalSubstrateReadinessNotSealed,
            ));
        }
        require_physical_recap_matches_physical_substrate_facts(
            physical_substrate_readiness,
            payload,
        )?;
        Ok(Self {
            physical_substrate_readiness,
            payload: payload.require_complete()?,
        })
    }

    pub const fn physical_substrate_readiness(&self) -> PhysicalSubstrateReadiness {
        self.physical_substrate_readiness
    }

    pub const fn payload(&self) -> PhysicalIntegrityReadinessPayload {
        self.payload
    }
}

fn require_physical_recap_matches_physical_substrate_facts(
    physical_substrate_readiness: PhysicalSubstrateReadiness,
    payload: PhysicalIntegrityReadinessPayload,
) -> Result<(), PhysicalIntegrityReadinessDenial> {
    let facts = physical_substrate_readiness.facts();
    let recap = payload.physical_authority_recap();
    if recap.physical_reference_count() == facts.physical_reference_count()
        && recap.header_decode_witness_count() == facts.header_decode_witness_count()
        && recap.payload_admission_witness_count() == facts.payload_admission_witness_count()
    {
        Ok(())
    } else {
        Err(PhysicalIntegrityReadinessDenial::new(
            PhysicalIntegrityReadinessDenialKind::PhysicalAuthorityRecapMismatch,
        ))
    }
}
