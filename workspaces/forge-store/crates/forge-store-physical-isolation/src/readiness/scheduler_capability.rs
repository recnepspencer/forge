use super::isolation_evidence::basis::SchedulerIsolationPublicationWitness;
use super::{
    ExecutedIsolationBasis, IsolationEvidenceProjection, SchedulerIsolationCapabilityBasis,
    SchedulerIsolationProof,
};
use crate::{
    BackgroundMaintenanceIsolationAssumption, ExecutedIsolationEvidence,
    ForegroundInterferenceSurface, IsolationReadinessDenial, PhysicalIsolationCounterSnapshot,
    PhysicalStabilityAssumption,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnsupportedQoSClaim {
    P99Latency,
    P999Latency,
    HardwareQueueDepth,
    MediaQoS,
    BackgroundWorkPacing,
}

impl UnsupportedQoSClaim {
    const fn canonical_s5_non_claims() -> [Self; 5] {
        [
            Self::P99Latency,
            Self::P999Latency,
            Self::HardwareQueueDepth,
            Self::MediaQoS,
            Self::BackgroundWorkPacing,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchedulerIsolationCapability {
    basis: SchedulerIsolationCapabilityBasis,
    proof_handoff: SchedulerIsolationProof,
    counters: PhysicalIsolationCounterSnapshot,
    assumptions: [PhysicalStabilityAssumption; 4],
    foreground_interference: ForegroundInterferenceSurface,
    background_maintenance: BackgroundMaintenanceIsolationAssumption,
    unsupported_qos_claims: [UnsupportedQoSClaim; 5],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct S6StoreIsolationHandoffEvidence {
    basis: ExecutedIsolationBasis,
    counters: PhysicalIsolationCounterSnapshot,
    projection_evidence: IsolationEvidenceProjection,
    publication_witness: SchedulerIsolationPublicationWitness,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SchedulerIsolationCapabilityRequest {
    evidence: S6StoreIsolationHandoffEvidence,
}

pub fn publish_scheduler_isolation_capability_from_executed_evidence(
    closeout: ExecutedIsolationEvidence,
) -> Result<SchedulerIsolationCapability, IsolationReadinessDenial> {
    let evidence = S6StoreIsolationHandoffEvidence::from_executed_s5_closeout(closeout)?;
    publish_scheduler_isolation_capability(
        SchedulerIsolationCapabilityRequest::from_store_handoff_evidence(evidence),
    )
}

#[cfg(any(test, feature = "certification-authority"))]
pub fn publish_scheduler_isolation_capability_for_certification_test(
    wait_count: u64,
    retry_count: u64,
) -> Result<SchedulerIsolationCapability, IsolationReadinessDenial> {
    publish_scheduler_isolation_capability_from_executed_evidence(
        ExecutedIsolationEvidence::from_foreground_reservation_test_counts(
            wait_count,
            retry_count,
        )?,
    )
}

fn publish_scheduler_isolation_capability(
    request: SchedulerIsolationCapabilityRequest,
) -> Result<SchedulerIsolationCapability, IsolationReadinessDenial> {
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
    let basis = SchedulerIsolationCapabilityBasis::from_closeout_basis(
        request.evidence.basis,
        request.evidence.projection_evidence.clone(),
    );
    let proof_handoff = SchedulerIsolationProof::from_basis(basis.clone(), witness);
    Ok(SchedulerIsolationCapability {
        basis,
        proof_handoff,
        counters,
        assumptions: PhysicalStabilityAssumption::required(),
        foreground_interference: ForegroundInterferenceSurface::from_counters(counters),
        background_maintenance: BackgroundMaintenanceIsolationAssumption::from_counters(counters),
        unsupported_qos_claims: UnsupportedQoSClaim::canonical_s5_non_claims(),
    })
}

impl S6StoreIsolationHandoffEvidence {
    fn from_executed_s5_closeout(
        closeout: ExecutedIsolationEvidence,
    ) -> Result<Self, IsolationReadinessDenial> {
        if closeout.basis().executed_isolation_identity() == 0
            || closeout.proof_progression_identity() == 0
            || closeout
                .foundational_counter_receipt()
                .counter_rows()
                .is_empty()
        {
            return Err(IsolationReadinessDenial::MissingExecutedCounter);
        }
        let projection_evidence = IsolationEvidenceProjection::from_store_executed_projection(
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
        let publication_witness =
            SchedulerIsolationPublicationWitness::from_validated_store_handoff(
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

impl SchedulerIsolationCapabilityRequest {
    pub(crate) const fn from_store_handoff_evidence(
        evidence: S6StoreIsolationHandoffEvidence,
    ) -> Self {
        Self { evidence }
    }
}

impl SchedulerIsolationCapability {
    pub const fn basis(&self) -> &SchedulerIsolationCapabilityBasis {
        &self.basis
    }

    pub const fn proof_handoff(&self) -> &SchedulerIsolationProof {
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
