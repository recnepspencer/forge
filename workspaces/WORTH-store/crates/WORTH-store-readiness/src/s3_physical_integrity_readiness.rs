use crate::{
    S2PhysicalSubstrateReadiness, S3PhysicalIntegrityReadinessPayload, S3ReadinessDenial,
    S3ReadinessDenialKind,
};

#[derive(Debug, PartialEq, Eq)]
pub struct S3PhysicalIntegrityReadiness {
    s2_readiness: S2PhysicalSubstrateReadiness,
    payload: S3PhysicalIntegrityReadinessPayload,
}

impl S3PhysicalIntegrityReadiness {
    pub fn from_s2_bounded_residency_closeout(
        s2_readiness: S2PhysicalSubstrateReadiness,
        payload: S3PhysicalIntegrityReadinessPayload,
    ) -> Result<Self, S3ReadinessDenial> {
        if !s2_readiness.is_sealed() {
            return Err(S3ReadinessDenial::new(
                S3ReadinessDenialKind::S2ReadinessNotSealed,
            ));
        }
        require_physical_recap_matches_s2_facts(s2_readiness, payload)?;
        Ok(Self {
            s2_readiness,
            payload: payload.require_complete()?,
        })
    }

    pub const fn s2_readiness(&self) -> S2PhysicalSubstrateReadiness {
        self.s2_readiness
    }

    pub const fn payload(&self) -> S3PhysicalIntegrityReadinessPayload {
        self.payload
    }
}

fn require_physical_recap_matches_s2_facts(
    s2_readiness: S2PhysicalSubstrateReadiness,
    payload: S3PhysicalIntegrityReadinessPayload,
) -> Result<(), S3ReadinessDenial> {
    let facts = s2_readiness.facts();
    let recap = payload.physical_authority_recap();
    if recap.physical_reference_count() == facts.physical_reference_count()
        && recap.header_decode_witness_count() == facts.header_decode_witness_count()
        && recap.payload_admission_witness_count() == facts.payload_admission_witness_count()
    {
        Ok(())
    } else {
        Err(S3ReadinessDenial::new(
            S3ReadinessDenialKind::PhysicalAuthorityRecapMismatch,
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        close_s1_physical_substrate_readiness, prove_s2_physical_substrate_readiness,
        BufferPoolAuthorityRecap, IntegrityInspectionLifetimeLaw, PhysicalAuthorityRecap,
        ProtectedIntegrityViewCapability, S2BoundedCounterRecap, S2DenialBehaviorRecap,
        S2DeniedBoundaryKind, S2NoMaterializationWitness, S2PhysicalSubstrateReadiness,
        S3PhysicalIntegrityReadiness, S3PhysicalIntegrityReadinessPayload, S3ReadinessDenialKind,
        ScrubPlanningAllocationEnvelope, VerifierResidentEnvelope,
    };
    use worth_store_contracts::{
        AcceptedHandoffReadiness, HandoffEvidenceDigestSet, StableDigest, ROADMAP_2_S1_SCOPE,
    };

    #[test]
    fn readiness_denies_physical_authority_recap_mismatch() {
        let readiness = s2_readiness();
        let denial = S3PhysicalIntegrityReadiness::from_s2_bounded_residency_closeout(
            readiness,
            mismatched_payload(readiness),
        )
        .unwrap_err();

        assert_eq!(
            denial.kind(),
            S3ReadinessDenialKind::PhysicalAuthorityRecapMismatch
        );
    }

    fn mismatched_payload(
        readiness: S2PhysicalSubstrateReadiness,
    ) -> S3PhysicalIntegrityReadinessPayload {
        let facts = readiness.facts();
        S3PhysicalIntegrityReadinessPayload::from_s2_closeout_evidence(
            ProtectedIntegrityViewCapability::protected_views(1).unwrap(),
            VerifierResidentEnvelope::bounded(128, 1).unwrap(),
            ScrubPlanningAllocationEnvelope::bounded(64).unwrap(),
            IntegrityInspectionLifetimeLaw::lease_scoped(),
            S2NoMaterializationWitness::observed_zero(0, 0).unwrap(),
            S2BoundedCounterRecap::exact(128, 1, 0, 64, 0, 0).unwrap(),
            S2DenialBehaviorRecap::from_named_boundaries(&S2DeniedBoundaryKind::ALL).unwrap(),
            PhysicalAuthorityRecap::from_s1_authority(
                facts.physical_reference_count() + 1,
                facts.header_decode_witness_count(),
                facts.payload_admission_witness_count(),
            )
            .unwrap(),
            BufferPoolAuthorityRecap::s2_authority(true, true, true, true).unwrap(),
        )
    }

    fn s2_readiness() -> S2PhysicalSubstrateReadiness {
        prove_s2_physical_substrate_readiness(
            close_s1_physical_substrate_readiness(
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
