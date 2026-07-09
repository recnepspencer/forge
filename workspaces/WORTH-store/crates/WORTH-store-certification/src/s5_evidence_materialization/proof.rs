use worth_proof::{Artifact, PhaseMarker};

use super::{
    S5ExecutedIsolationFinding, S5ExecutedIsolationRequiredCounters, S5ExecutedIsolationSourceBasis,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S5ExecutedIsolationProjectionPhase;
impl PhaseMarker for S5ExecutedIsolationProjectionPhase {}

pub type S5ProofProjectionArtifact =
    Artifact<S5ExecutedIsolationProjectionPhase, S5PhysicalIsolationProofProgression>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S5StableReadPlanProofProjection {
    basis: S5ExecutedIsolationSourceBasis,
    counters: S5ExecutedIsolationRequiredCounters,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S5PhysicalIsolationProofProgression {
    stable_read_projection: S5StableReadPlanProofProjection,
}

impl S5StableReadPlanProofProjection {
    fn from_finding(finding: &S5ExecutedIsolationFinding) -> Self {
        Self {
            basis: finding.basis().clone(),
            counters: finding.counters(),
        }
    }

    pub const fn basis(&self) -> &S5ExecutedIsolationSourceBasis {
        &self.basis
    }

    pub const fn counters(&self) -> S5ExecutedIsolationRequiredCounters {
        self.counters
    }
}

impl S5PhysicalIsolationProofProgression {
    fn checked_from_finding(finding: &S5ExecutedIsolationFinding) -> Self {
        Self {
            stable_read_projection: S5StableReadPlanProofProjection::from_finding(finding),
        }
    }

    pub const fn stable_read_projection(&self) -> &S5StableReadPlanProofProjection {
        &self.stable_read_projection
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S5PhysicalIsolationProofTrace {
    projection: S5ProofProjectionArtifact,
}

impl S5PhysicalIsolationProofTrace {
    pub(crate) fn from_finding(finding: &S5ExecutedIsolationFinding) -> Self {
        let progression = S5PhysicalIsolationProofProgression::checked_from_finding(finding);
        Self {
            projection: Artifact::new(progression),
        }
    }

    pub const fn projection(&self) -> &S5ProofProjectionArtifact {
        &self.projection
    }

    pub fn is_checked_from_executed_store_isolation(&self) -> bool {
        self.projection
            .payload()
            .stable_read_projection()
            .counters()
            .outcome_count()
            == 1
    }
}
