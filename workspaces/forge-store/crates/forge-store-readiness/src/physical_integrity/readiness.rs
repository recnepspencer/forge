use crate::PhysicalSubstrateReadiness;
use forge_store_contracts::{
    PhysicalIntegrityReadinessDenial, PhysicalIntegrityReadinessDenialKind,
    PhysicalIntegrityReadinessPayload,
};

#[derive(Debug, PartialEq, Eq)]
pub struct PhysicalIntegrityReadiness {
    s2_readiness: PhysicalSubstrateReadiness,
    payload: PhysicalIntegrityReadinessPayload,
}

impl PhysicalIntegrityReadiness {
    pub fn from_s2_bounded_residency_closeout(
        s2_readiness: PhysicalSubstrateReadiness,
        payload: PhysicalIntegrityReadinessPayload,
    ) -> Result<Self, PhysicalIntegrityReadinessDenial> {
        if !s2_readiness.is_sealed() {
            return Err(PhysicalIntegrityReadinessDenial::new(
                PhysicalIntegrityReadinessDenialKind::S2ReadinessNotSealed,
            ));
        }
        require_physical_recap_matches_s2_facts(s2_readiness, payload)?;
        Ok(Self {
            s2_readiness,
            payload: payload.require_complete()?,
        })
    }

    pub const fn s2_readiness(&self) -> PhysicalSubstrateReadiness {
        self.s2_readiness
    }

    pub const fn payload(&self) -> PhysicalIntegrityReadinessPayload {
        self.payload
    }
}

fn require_physical_recap_matches_s2_facts(
    s2_readiness: PhysicalSubstrateReadiness,
    payload: PhysicalIntegrityReadinessPayload,
) -> Result<(), PhysicalIntegrityReadinessDenial> {
    let facts = s2_readiness.facts();
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

#[cfg(test)]
mod tests {
    use crate::{
        close_physical_substrate_readiness, prove_physical_substrate_readiness,
        PhysicalIntegrityReadiness, PhysicalSubstrateReadiness,
    };
    use forge_store_contracts::{
        AcceptedHandoffReadiness, BoundedCounterRecap, BufferPoolAuthorityRecap,
        DenialBehaviorRecap, DeniedBoundaryKind, HandoffEvidenceDigestSet,
        IntegrityInspectionLifetimeLaw, NoMaterializationWitness, PhysicalAuthorityRecap,
        PhysicalIntegrityReadinessDenialKind, PhysicalIntegrityReadinessPayload,
        ProtectedIntegrityViewCapability, ScrubPlanningAllocationEnvelope, StableDigest,
        VerifierResidentEnvelope, ROADMAP_2_S1_SCOPE,
    };

    #[test]
    fn readiness_denies_physical_authority_recap_mismatch() {
        let readiness = s2_readiness();
        let denial = PhysicalIntegrityReadiness::from_s2_bounded_residency_closeout(
            readiness,
            mismatched_payload(readiness),
        )
        .unwrap_err();

        assert_eq!(
            denial.kind(),
            PhysicalIntegrityReadinessDenialKind::PhysicalAuthorityRecapMismatch
        );
    }

    fn mismatched_payload(
        readiness: PhysicalSubstrateReadiness,
    ) -> PhysicalIntegrityReadinessPayload {
        let facts = readiness.facts();
        PhysicalIntegrityReadinessPayload::from_s2_closeout_evidence(
            ProtectedIntegrityViewCapability::protected_views(1).unwrap(),
            VerifierResidentEnvelope::bounded(128, 1).unwrap(),
            ScrubPlanningAllocationEnvelope::bounded(64).unwrap(),
            IntegrityInspectionLifetimeLaw::lease_scoped(),
            NoMaterializationWitness::observed_zero(0, 0).unwrap(),
            BoundedCounterRecap::exact(128, 1, 0, 64, 0, 0).unwrap(),
            DenialBehaviorRecap::from_named_boundaries(&DeniedBoundaryKind::ALL).unwrap(),
            PhysicalAuthorityRecap::from_s1_authority(
                facts.physical_reference_count() + 1,
                facts.header_decode_witness_count(),
                facts.payload_admission_witness_count(),
            )
            .unwrap(),
            BufferPoolAuthorityRecap::s2_authority(true, true, true, true).unwrap(),
        )
    }

    fn s2_readiness() -> PhysicalSubstrateReadiness {
        prove_physical_substrate_readiness(
            close_physical_substrate_readiness(
                AcceptedHandoffReadiness::from_s0_artifacts(
                    ROADMAP_2_S1_SCOPE,
                    HandoffEvidenceDigestSet::new(
                        StableDigest::new("sha256:s3-readiness-backend").unwrap(),
                        StableDigest::new("sha256:s3-readiness-deferred").unwrap(),
                        StableDigest::new("sha256:s3-readiness-harness").unwrap(),
                        StableDigest::new("sha256:s3-readiness-terms").unwrap(),
                        StableDigest::new("sha256:s3-readiness-audit").unwrap(),
                        StableDigest::new("sha256:s3-readiness-complexity").unwrap(),
                        StableDigest::new("sha256:s3-readiness-provenance").unwrap(),
                    ),
                )
                .unwrap(),
            )
            .unwrap(),
        )
        .unwrap()
    }
}
