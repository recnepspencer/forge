use super::basis::S6ReadinessPublicationWitness;
use super::{
    BackgroundMaintenanceIsolationAssumption, ExecutedS5IsolationCloseout,
    ForegroundInterferenceSurface, PhysicalIsolationCounterSnapshot, PhysicalStabilityAssumption,
    S5PhysicalIsolationCloseoutBasis, S6HandoffProjectionEvidence, S6IoQosIsolationReadinessBasis,
    S6IoQosIsolationReadinessDenial, S6ReadinessProofHandoff, UnsupportedQoSClaim,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S6IoQosIsolationReadiness {
    basis: S6IoQosIsolationReadinessBasis,
    proof_handoff: S6ReadinessProofHandoff,
    counters: PhysicalIsolationCounterSnapshot,
    assumptions: [PhysicalStabilityAssumption; 4],
    foreground_interference: ForegroundInterferenceSurface,
    background_maintenance: BackgroundMaintenanceIsolationAssumption,
    unsupported_qos_claims: [UnsupportedQoSClaim; 5],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct S6StoreIsolationHandoffEvidence {
    basis: S5PhysicalIsolationCloseoutBasis,
    counters: PhysicalIsolationCounterSnapshot,
    projection_evidence: S6HandoffProjectionEvidence,
    publication_witness: S6ReadinessPublicationWitness,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct S6IoQosIsolationReadinessRequest {
    evidence: S6StoreIsolationHandoffEvidence,
}

pub fn publish_s6_io_qos_isolation_readiness_from_s5_closeout(
    closeout: ExecutedS5IsolationCloseout,
) -> Result<S6IoQosIsolationReadiness, S6IoQosIsolationReadinessDenial> {
    let evidence = S6StoreIsolationHandoffEvidence::from_executed_s5_closeout(closeout)?;
    publish_s6_io_qos_isolation_readiness(
        S6IoQosIsolationReadinessRequest::from_store_handoff_evidence(evidence),
    )
}

#[cfg(any(test, feature = "certification-authority"))]
pub fn publish_s6_io_qos_isolation_readiness_for_foreground_reservation_test(
    wait_count: u64,
    retry_count: u64,
) -> Result<S6IoQosIsolationReadiness, S6IoQosIsolationReadinessDenial> {
    publish_s6_io_qos_isolation_readiness_from_s5_closeout(
        ExecutedS5IsolationCloseout::from_foreground_reservation_test_counts(
            wait_count,
            retry_count,
        )?,
    )
}

fn publish_s6_io_qos_isolation_readiness(
    request: S6IoQosIsolationReadinessRequest,
) -> Result<S6IoQosIsolationReadiness, S6IoQosIsolationReadinessDenial> {
    let counters = request.evidence.counters;
    let witness = request.evidence.publication_witness;
    let _ = PhysicalIsolationCounterSnapshot::from_store_executed_counts(
        counters.outcome_count(),
        counters.wait_count(),
        counters.retry_count(),
        counters.latch_counter_rows(),
        counters.latch_wait_count(),
        counters.reclaim_counter_rows(),
        counters.blocked_maintenance_count(),
        counters.reclaim_block_count(),
        counters.protected_byte_footprint(),
    )?;
    let basis = S6IoQosIsolationReadinessBasis::from_closeout_basis(
        request.evidence.basis,
        request.evidence.projection_evidence.clone(),
    );
    let proof_handoff = S6ReadinessProofHandoff::from_basis(basis.clone(), witness);
    Ok(S6IoQosIsolationReadiness {
        basis,
        proof_handoff,
        counters,
        assumptions: PhysicalStabilityAssumption::s6_handoff_assumptions(),
        foreground_interference: ForegroundInterferenceSurface::from_counters(counters),
        background_maintenance: BackgroundMaintenanceIsolationAssumption::from_counters(counters),
        unsupported_qos_claims: UnsupportedQoSClaim::canonical_s5_non_claims(),
    })
}

impl S6StoreIsolationHandoffEvidence {
    fn from_executed_s5_closeout(
        closeout: ExecutedS5IsolationCloseout,
    ) -> Result<Self, S6IoQosIsolationReadinessDenial> {
        if closeout.basis().executed_isolation_identity() == 0
            || closeout.proof_progression_identity() == 0
            || closeout
                .foundational_counter_receipt()
                .counter_rows()
                .is_empty()
        {
            return Err(S6IoQosIsolationReadinessDenial::MissingExecutedCounter);
        }
        let projection_evidence = S6HandoffProjectionEvidence::from_store_executed_projection(
            closeout.foundational_counter_receipt().clone(),
            closeout.proof_progression_identity(),
        );
        let _ = PhysicalIsolationCounterSnapshot::from_store_executed_counts(
            closeout.counters().outcome_count(),
            closeout.counters().wait_count(),
            closeout.counters().retry_count(),
            closeout.counters().latch_counter_rows(),
            closeout.counters().latch_wait_count(),
            closeout.counters().reclaim_counter_rows(),
            closeout.counters().blocked_maintenance_count(),
            closeout.counters().reclaim_block_count(),
            closeout.counters().protected_byte_footprint(),
        )?;
        let publication_witness = S6ReadinessPublicationWitness::from_validated_store_handoff(
            closeout.counters(),
            &projection_evidence,
        )?;
        Ok(S6StoreIsolationHandoffEvidence {
            basis: closeout.basis(),
            counters: closeout.counters(),
            projection_evidence,
            publication_witness,
        })
    }
}

impl S6IoQosIsolationReadinessRequest {
    pub(crate) const fn from_store_handoff_evidence(
        evidence: S6StoreIsolationHandoffEvidence,
    ) -> Self {
        Self { evidence }
    }
}

impl S6IoQosIsolationReadiness {
    pub const fn basis(&self) -> &S6IoQosIsolationReadinessBasis {
        &self.basis
    }

    pub const fn proof_handoff(&self) -> &S6ReadinessProofHandoff {
        &self.proof_handoff
    }

    pub const fn counters(&self) -> PhysicalIsolationCounterSnapshot {
        self.counters
    }

    pub const fn assumptions(&self) -> &[PhysicalStabilityAssumption; 4] {
        &self.assumptions
    }

    pub const fn foreground_interference(&self) -> ForegroundInterferenceSurface {
        self.foreground_interference
    }

    pub const fn background_maintenance(&self) -> BackgroundMaintenanceIsolationAssumption {
        self.background_maintenance
    }

    pub const fn unsupported_qos_claims(&self) -> &[UnsupportedQoSClaim; 5] {
        &self.unsupported_qos_claims
    }
}
