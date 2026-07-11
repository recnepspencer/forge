use crate::recovery_harness::{
    RecoveryPhysicsCertificationMatrix, RecoveryPhysicsCounterExpectation,
    RecoveryPhysicsCounterKind, RecoveryPhysicsCrashLane, RecoveryPhysicsMutant,
    RecoveryPhysicsMutationSuiteLaneEvidence,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPhysicsMutationSuiteEvidence {
    rows: Vec<RecoveryPhysicsMutationSuiteLaneEvidence>,
}

impl RecoveryPhysicsMutationSuiteEvidence {
    pub fn from_certification(
        certification: &RecoveryPhysicsCertificationMatrix,
    ) -> Result<Self, RecoveryPhysicsMutationSuiteEvidenceDenial> {
        let mut rows = Vec::with_capacity(RecoveryPhysicsMutant::REQUIRED_S4_MUTANTS.len());
        for mutant in RecoveryPhysicsMutant::REQUIRED_S4_MUTANTS {
            let lane = mutant.intended_lane();
            certification
                .lane(lane)
                .ok_or(RecoveryPhysicsMutationSuiteEvidenceDenial::MissingCertifiedLane(lane))?;
            rows.push(RecoveryPhysicsMutationSuiteLaneEvidence::from_suite_lane(
                mutant,
                lane,
                mutant.failure_evidence(),
                RecoveryPhysicsCounterExpectation::exact(
                    RecoveryPhysicsCounterKind::MutationFailures,
                    1,
                ),
            ));
        }
        Ok(Self { rows })
    }

    pub fn rows(&self) -> &[RecoveryPhysicsMutationSuiteLaneEvidence] {
        &self.rows
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryPhysicsMutationSuiteEvidenceDenial {
    MissingCertifiedLane(RecoveryPhysicsCrashLane),
}
