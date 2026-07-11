use crate::recovery_harness::{
    RecoveryPhysicsCertificationDenial, RecoveryPhysicsCertificationMatrix,
    RecoveryPhysicsCrashMatrix, RecoveryPhysicsCrashMatrixDenial,
    RecoveryPhysicsMutationSuiteEvidence, RecoveryPhysicsMutationSuiteEvidenceDenial,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPhysicsRoadmap2HarnessCertification {
    certification_matrix: RecoveryPhysicsCertificationMatrix,
    mutation_evidence: RecoveryPhysicsMutationSuiteEvidence,
}

impl RecoveryPhysicsRoadmap2HarnessCertification {
    pub fn certify_s4_ci() -> Result<Self, RecoveryPhysicsRoadmap2HarnessDenial> {
        let crash_matrix = RecoveryPhysicsCrashMatrix::roadmap_2_s4().lower()?;
        let certification_matrix = RecoveryPhysicsCertificationMatrix::certify(crash_matrix)?;
        let mutation_evidence =
            RecoveryPhysicsMutationSuiteEvidence::from_certification(&certification_matrix)?;

        Ok(Self {
            certification_matrix,
            mutation_evidence,
        })
    }

    pub const fn certification_matrix(&self) -> &RecoveryPhysicsCertificationMatrix {
        &self.certification_matrix
    }

    pub const fn mutation_evidence(&self) -> &RecoveryPhysicsMutationSuiteEvidence {
        &self.mutation_evidence
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoveryPhysicsRoadmap2HarnessDenial {
    CrashMatrix(RecoveryPhysicsCrashMatrixDenial),
    Certification(RecoveryPhysicsCertificationDenial),
    MutationEvidence(RecoveryPhysicsMutationSuiteEvidenceDenial),
}

impl From<RecoveryPhysicsCrashMatrixDenial> for RecoveryPhysicsRoadmap2HarnessDenial {
    fn from(denial: RecoveryPhysicsCrashMatrixDenial) -> Self {
        Self::CrashMatrix(denial)
    }
}

impl From<RecoveryPhysicsCertificationDenial> for RecoveryPhysicsRoadmap2HarnessDenial {
    fn from(denial: RecoveryPhysicsCertificationDenial) -> Self {
        Self::Certification(denial)
    }
}

impl From<RecoveryPhysicsMutationSuiteEvidenceDenial> for RecoveryPhysicsRoadmap2HarnessDenial {
    fn from(denial: RecoveryPhysicsMutationSuiteEvidenceDenial) -> Self {
        Self::MutationEvidence(denial)
    }
}
