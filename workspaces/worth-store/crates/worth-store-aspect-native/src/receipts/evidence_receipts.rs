use worth_foundational::{
    FoundationalBoundaryEvidenceCompletedReceiptArtifact,
    FoundationalBoundaryEvidenceExecutedReceiptArtifact, FoundationalDiagnosticExplanationBundle,
    FoundationalDiagnosticSupportReport,
};

use crate::StorePhysicalBoundaryWitness;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreCompletedBoundaryReceiptEvidence {
    receipt: FoundationalBoundaryEvidenceCompletedReceiptArtifact,
    physical_witness: StorePhysicalBoundaryWitness,
}

impl StoreCompletedBoundaryReceiptEvidence {
    pub const fn new(
        receipt: FoundationalBoundaryEvidenceCompletedReceiptArtifact,
        physical_witness: StorePhysicalBoundaryWitness,
    ) -> Self {
        Self {
            receipt,
            physical_witness,
        }
    }

    pub const fn receipt(&self) -> &FoundationalBoundaryEvidenceCompletedReceiptArtifact {
        &self.receipt
    }

    pub const fn physical_witness(&self) -> StorePhysicalBoundaryWitness {
        self.physical_witness
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreExecutedBoundaryReceiptEvidence {
    receipt: FoundationalBoundaryEvidenceExecutedReceiptArtifact,
    physical_witness: StorePhysicalBoundaryWitness,
}

impl StoreExecutedBoundaryReceiptEvidence {
    pub const fn new(
        receipt: FoundationalBoundaryEvidenceExecutedReceiptArtifact,
        physical_witness: StorePhysicalBoundaryWitness,
    ) -> Self {
        Self {
            receipt,
            physical_witness,
        }
    }

    pub const fn receipt(&self) -> &FoundationalBoundaryEvidenceExecutedReceiptArtifact {
        &self.receipt
    }

    pub const fn physical_witness(&self) -> StorePhysicalBoundaryWitness {
        self.physical_witness
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreDiagnosticSupportReportEvidence {
    diagnostic: FoundationalDiagnosticSupportReport,
    physical_witness: StorePhysicalBoundaryWitness,
}

impl StoreDiagnosticSupportReportEvidence {
    pub fn new(
        diagnostic: FoundationalDiagnosticSupportReport,
        physical_witness: StorePhysicalBoundaryWitness,
    ) -> Self {
        Self {
            diagnostic,
            physical_witness,
        }
    }

    pub const fn diagnostic(&self) -> &FoundationalDiagnosticSupportReport {
        &self.diagnostic
    }

    pub const fn physical_witness(&self) -> StorePhysicalBoundaryWitness {
        self.physical_witness
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreDiagnosticExplanationBundleEvidence {
    diagnostic: FoundationalDiagnosticExplanationBundle,
    physical_witness: StorePhysicalBoundaryWitness,
}

impl StoreDiagnosticExplanationBundleEvidence {
    pub fn new(
        diagnostic: FoundationalDiagnosticExplanationBundle,
        physical_witness: StorePhysicalBoundaryWitness,
    ) -> Self {
        Self {
            diagnostic,
            physical_witness,
        }
    }

    pub const fn diagnostic(&self) -> &FoundationalDiagnosticExplanationBundle {
        &self.diagnostic
    }

    pub const fn physical_witness(&self) -> StorePhysicalBoundaryWitness {
        self.physical_witness
    }
}
