use crate::courtroom::recovery::harness::{
    RecoveryPhysicsCertificationMatrix, RecoveryPhysicsCounterExpectation,
    RecoveryPhysicsCounterKind, RecoveryPhysicsCrashLane, RecoveryPhysicsMutant,
    RecoveryPhysicsMutationFailureEvidence, RecoveryPhysicsMutationSuiteLaneEvidence,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPhysicsMutationValidationMatrix {
    rows: Vec<RecoveryPhysicsMutationValidationRow>,
}

impl RecoveryPhysicsMutationValidationMatrix {
    pub fn validate(
        certification: &RecoveryPhysicsCertificationMatrix,
        evidence: &[RecoveryPhysicsMutationSuiteLaneEvidence],
    ) -> Result<Self, RecoveryPhysicsMutationValidationDenial> {
        let mut rows = Vec::with_capacity(RecoveryPhysicsMutant::REQUIRED_S4_MUTANTS.len());
        for mutant in RecoveryPhysicsMutant::REQUIRED_S4_MUTANTS {
            let lane = mutant.intended_lane();
            if certification.lane(lane).is_none() {
                return Err(RecoveryPhysicsMutationValidationDenial::MissingLane(lane));
            }
            let suite_evidence = evidence.iter().find(|row| row.mutant() == mutant).ok_or(
                RecoveryPhysicsMutationValidationDenial::MissingEvidence(mutant),
            )?;
            if suite_evidence.lane() != lane {
                return Err(RecoveryPhysicsMutationValidationDenial::WrongLane {
                    mutant,
                    expected: lane,
                    actual: suite_evidence.lane(),
                });
            }
            if suite_evidence.failure_evidence() != mutant.failure_evidence() {
                return Err(RecoveryPhysicsMutationValidationDenial::WrongEvidence(
                    mutant,
                ));
            }
            if suite_evidence.counter().kind() != RecoveryPhysicsCounterKind::MutationFailures
                || suite_evidence.counter().expected() != 1
            {
                return Err(RecoveryPhysicsMutationValidationDenial::WrongCounter(
                    mutant,
                ));
            }
            rows.push(RecoveryPhysicsMutationValidationRow {
                mutant,
                lane,
                failure_evidence: suite_evidence.failure_evidence(),
                counter: suite_evidence.counter(),
            });
        }
        Ok(Self { rows })
    }

    pub fn rows(&self) -> &[RecoveryPhysicsMutationValidationRow] {
        &self.rows
    }

    pub fn all_required_mutants_failed(&self) -> bool {
        self.rows.len() == RecoveryPhysicsMutant::REQUIRED_S4_MUTANTS.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryPhysicsMutationValidationRow {
    mutant: RecoveryPhysicsMutant,
    lane: RecoveryPhysicsCrashLane,
    failure_evidence: RecoveryPhysicsMutationFailureEvidence,
    counter: RecoveryPhysicsCounterExpectation,
}

impl RecoveryPhysicsMutationValidationRow {
    pub const fn mutant(&self) -> RecoveryPhysicsMutant {
        self.mutant
    }

    pub const fn lane(&self) -> RecoveryPhysicsCrashLane {
        self.lane
    }

    pub const fn failure_evidence(&self) -> RecoveryPhysicsMutationFailureEvidence {
        self.failure_evidence
    }

    pub const fn counter(&self) -> RecoveryPhysicsCounterExpectation {
        self.counter
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryPhysicsMutationValidationDenial {
    MissingLane(RecoveryPhysicsCrashLane),
    MissingEvidence(RecoveryPhysicsMutant),
    WrongLane {
        mutant: RecoveryPhysicsMutant,
        expected: RecoveryPhysicsCrashLane,
        actual: RecoveryPhysicsCrashLane,
    },
    WrongEvidence(RecoveryPhysicsMutant),
    WrongCounter(RecoveryPhysicsMutant),
}
