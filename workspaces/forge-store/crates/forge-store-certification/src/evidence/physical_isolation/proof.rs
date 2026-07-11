use forge_proof::{Artifact, PhaseMarker};

use super::{
    ExecutedPhysicalIsolationFinding, ExecutedPhysicalIsolationRequiredCounters,
    ExecutedPhysicalIsolationSourceBasis,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S5ExecutedIsolationProjectionPhase;
impl PhaseMarker for S5ExecutedIsolationProjectionPhase {}

pub type S5ProofProjectionArtifact =
    Artifact<S5ExecutedIsolationProjectionPhase, S5PhysicalIsolationProofProgression>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S5StableReadPlanProofProjection {
    basis: ExecutedPhysicalIsolationSourceBasis,
    counters: ExecutedPhysicalIsolationRequiredCounters,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S5PhysicalIsolationProofProgression {
    stable_read_projection: S5StableReadPlanProofProjection,
}

impl S5StableReadPlanProofProjection {
    fn from_finding(finding: &ExecutedPhysicalIsolationFinding) -> Self {
        Self {
            basis: finding.basis().clone(),
            counters: finding.counters(),
        }
    }

    pub const fn basis(&self) -> &ExecutedPhysicalIsolationSourceBasis {
        &self.basis
    }

    pub const fn counters(&self) -> ExecutedPhysicalIsolationRequiredCounters {
        self.counters
    }
}

impl S5PhysicalIsolationProofProgression {
    fn checked_from_finding(finding: &ExecutedPhysicalIsolationFinding) -> Self {
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
    pub(crate) fn from_finding(finding: &ExecutedPhysicalIsolationFinding) -> Self {
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
