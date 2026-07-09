use crate::{S6MaterializedCertificationAdoptionReceipt, S6ReadinessCertificationProofTopology};

use super::{
    denial::S6ProductionReadinessClosureDenial, input::S6LaterMilestoneNonClaimBoundaries,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct S6ProductionReadinessProof {
    checked_phase13_execution: bool,
    checked_readmission_boundaries: usize,
    checked_topology: S6ReadinessCertificationProofTopology,
    later_handoff_boundaries: usize,
}

impl S6ProductionReadinessProof {
    pub(crate) fn from_phase13_adoption(
        adoption: &S6MaterializedCertificationAdoptionReceipt,
        non_claims: &S6LaterMilestoneNonClaimBoundaries,
    ) -> Result<Self, S6ProductionReadinessClosureDenial> {
        if !adoption.proof().checked_execution()
            || adoption.proof().readmission_boundaries() != 5
            || !adoption
                .proof_topology()
                .is_checked_for_closeout(adoption.proof())
        {
            return Err(S6ProductionReadinessClosureDenial::Phase13EvidenceCannotSatisfyReadiness);
        }
        Ok(Self {
            checked_phase13_execution: adoption.proof().checked_execution(),
            checked_readmission_boundaries: adoption.proof().readmission_boundaries(),
            checked_topology: adoption.proof_topology(),
            later_handoff_boundaries: non_claims.later_handoff_boundary_count(),
        })
    }

    pub const fn checked_phase13_execution(&self) -> bool {
        self.checked_phase13_execution
    }

    pub const fn checked_readmission_boundaries(&self) -> usize {
        self.checked_readmission_boundaries
    }

    pub const fn later_handoff_boundaries(&self) -> usize {
        self.later_handoff_boundaries
    }

    pub const fn checked_topology(&self) -> S6ReadinessCertificationProofTopology {
        self.checked_topology
    }

    pub const fn is_checked_for_s6_closeout(&self) -> bool {
        self.checked_phase13_execution
            && self.checked_readmission_boundaries == 5
            && self.checked_topology.is_checked_for_closeout(
                crate::S6ReadinessCertificationProofSummary::new(
                    self.checked_phase13_execution,
                    self.checked_readmission_boundaries,
                    self.checked_topology.readiness_readmission_boundaries(),
                    self.checked_topology.executed_readmission_boundaries(),
                ),
            )
            && self.later_handoff_boundaries == 4
    }
}
