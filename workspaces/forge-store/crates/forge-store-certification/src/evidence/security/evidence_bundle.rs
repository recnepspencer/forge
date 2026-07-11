use super::{
    S51CertificationCloseoutInput, S51CloseoutApiAdoptionEvidence,
    S51CloseoutBoundaryEvidencePublication, S51CloseoutCounterMatrix,
    S51CloseoutPerformanceReceipts,
};

#[derive(Debug)]
pub struct S51CertificationCloseoutEvidence {
    input: S51CertificationCloseoutInput,
    counter_matrix: S51CloseoutCounterMatrix,
    performance_receipts: S51CloseoutPerformanceReceipts,
    boundary_evidence: S51CloseoutBoundaryEvidencePublication,
    api_adoption: S51CloseoutApiAdoptionEvidence,
}

impl S51CertificationCloseoutEvidence {
    pub(crate) fn new(
        input: S51CertificationCloseoutInput,
        counter_matrix: S51CloseoutCounterMatrix,
        performance_receipts: S51CloseoutPerformanceReceipts,
        boundary_evidence: S51CloseoutBoundaryEvidencePublication,
        api_adoption: S51CloseoutApiAdoptionEvidence,
    ) -> Self {
        Self {
            input,
            counter_matrix,
            performance_receipts,
            boundary_evidence,
            api_adoption,
        }
    }

    pub const fn input(&self) -> &S51CertificationCloseoutInput {
        &self.input
    }

    pub const fn counter_matrix(&self) -> S51CloseoutCounterMatrix {
        self.counter_matrix
    }

    pub const fn performance_receipts(&self) -> &S51CloseoutPerformanceReceipts {
        &self.performance_receipts
    }

    pub const fn boundary_evidence(&self) -> &S51CloseoutBoundaryEvidencePublication {
        &self.boundary_evidence
    }

    pub const fn api_adoption(&self) -> S51CloseoutApiAdoptionEvidence {
        self.api_adoption
    }

    pub const fn readiness_construction_attempts(&self) -> u64 {
        0
    }
}
