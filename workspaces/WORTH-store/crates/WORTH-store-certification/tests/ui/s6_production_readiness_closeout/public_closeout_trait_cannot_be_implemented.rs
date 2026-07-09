use worth_store_readiness::{
    adopt_materialized_s6_certification_evidence_for_closeout, close_s6_production_readiness,
    S6MaterializedCertificationCloseoutEvidence, S6ProductionReadinessClosureInput,
    S6ReadinessCertificationCounterEvidence, S6ReadinessCertificationCounterFamily,
    S6ReadinessCertificationCounterStrength, S6ReadinessCertificationProofSummary,
    S6ReadinessCertificationProofTopology, S6ReadinessResidualDebtEvidenceKind,
    S6ReadinessResidualDebtEvidenceRow,
};

struct WORTHdCloseoutEvidence;

impl S6MaterializedCertificationCloseoutEvidence for WORTHdCloseoutEvidence {
    fn is_courtroom_evidence_over_executed_store_law(&self) -> bool {
        true
    }

    fn canonical_execution_identity_tag(&self) -> u64 {
        42
    }

    fn proof_execution_identity_tag(&self) -> u64 {
        42
    }

    fn canonical_lane_binding_mask(&self) -> u16 {
        0b111_1111_1111
    }

    fn proof_lane_binding_mask(&self) -> u16 {
        0b111_1111_1111
    }

    fn profile_count(&self) -> usize {
        6
    }

    fn profile_boundary_certification_only(&self) -> bool {
        true
    }

    fn performance_receipt_count(&self) -> usize {
        5
    }

    fn counter_strengths(&self) -> Vec<S6ReadinessCertificationCounterEvidence> {
        use S6ReadinessCertificationCounterFamily as Family;
        use S6ReadinessCertificationCounterStrength as Strength;
        [
            (
                Family::ForegroundReservation,
                Strength::CertificationOnly,
                2,
            ),
            (Family::BackgroundPacing, Strength::CertificationOnly, 2),
            (Family::QueueExecution, Strength::CertificationOnly, 13),
            (Family::FlushDurability, Strength::Exact, 1),
            (Family::LatencyInterference, Strength::Unavailable, 0),
            (
                Family::LaterReadinessHandoff,
                Strength::CertificationOnly,
                5,
            ),
            (Family::SecureIoPreservation, Strength::Exact, 2),
            (Family::AccessPolicy, Strength::Exact, 2),
            (Family::PostAdmissionViolation, Strength::Derived, 2),
            (Family::QualificationMatrix, Strength::CertificationOnly, 1),
        ]
        .into_iter()
        .map(|(family, strength, rows)| {
            S6ReadinessCertificationCounterEvidence::new(family, strength, rows)
        })
        .collect()
    }

    fn canonical_access_policy_rows(&self) -> usize {
        2
    }

    fn canonical_post_admission_violation_rows(&self) -> usize {
        2
    }

    fn proof_summary(&self) -> S6ReadinessCertificationProofSummary {
        S6ReadinessCertificationProofSummary::new(true, 5, 2, 2)
    }

    fn proof_topology(&self) -> S6ReadinessCertificationProofTopology {
        S6ReadinessCertificationProofTopology::new(
            true, true, true, true, true, true, true, true, true, true, true, true, 5, 5, 5,
        )
    }

    fn residual_debt_rows(&self) -> Vec<S6ReadinessResidualDebtEvidenceRow> {
        use S6ReadinessResidualDebtEvidenceKind::{
            DegradedBackendPosture, DeniedClaim, RebindRequired, ResidualQualificationDebt,
            StaleEvidence, UnavailableEvidence, UnsupportedBackendProfile,
        };
        [
            UnsupportedBackendProfile,
            UnavailableEvidence,
            DegradedBackendPosture,
            DeniedClaim,
            StaleEvidence,
            RebindRequired,
            ResidualQualificationDebt,
        ]
        .into_iter()
        .map(|kind| S6ReadinessResidualDebtEvidenceRow::new(kind, 1))
        .collect()
    }
}

fn main() {
    let adoption =
        adopt_materialized_s6_certification_evidence_for_closeout(&WORTHdCloseoutEvidence).unwrap();
    let closeout = close_s6_production_readiness(
        S6ProductionReadinessClosureInput::from_phase13_adoption(adoption),
    )
    .unwrap();
    let _ = closeout;
}
