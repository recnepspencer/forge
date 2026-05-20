use crate::locators::FoundationalTransitionLocator;

use super::super::primitives::{
    FoundationalBoundaryEvidenceExecutionPosture, FoundationalBoundaryEvidenceLocality,
};
use super::super::provenance::FoundationalBoundaryEvidenceProvenanceArtifact;
use super::definitions::{
    FoundationalBoundaryEvidenceCloseoutDisposition, FoundationalBoundaryEvidenceReceiptKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBoundaryEvidenceReceiptBoundary(FoundationalTransitionLocator);

impl FoundationalBoundaryEvidenceReceiptBoundary {
    pub fn transition(locator: FoundationalTransitionLocator) -> Self {
        Self(locator)
    }

    pub fn locator(&self) -> &FoundationalTransitionLocator {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBoundaryEvidencePlanningReceiptArtifact {
    boundary: FoundationalBoundaryEvidenceReceiptBoundary,
    provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
}

impl FoundationalBoundaryEvidencePlanningReceiptArtifact {
    pub(crate) fn new(
        boundary: FoundationalBoundaryEvidenceReceiptBoundary,
        provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
    ) -> Self {
        Self {
            boundary,
            provenance,
        }
    }

    pub const fn receipt_kind(&self) -> FoundationalBoundaryEvidenceReceiptKind {
        FoundationalBoundaryEvidenceReceiptKind::Planning
    }

    pub const fn execution_posture(&self) -> FoundationalBoundaryEvidenceExecutionPosture {
        FoundationalBoundaryEvidenceExecutionPosture::Planned
    }

    pub const fn locality(&self) -> FoundationalBoundaryEvidenceLocality {
        self.provenance.locality()
    }

    pub fn planned_boundary(&self) -> &FoundationalBoundaryEvidenceReceiptBoundary {
        &self.boundary
    }

    pub fn provenance(&self) -> &FoundationalBoundaryEvidenceProvenanceArtifact {
        &self.provenance
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBoundaryEvidenceCompletedReceiptArtifact {
    kind: FoundationalBoundaryEvidenceReceiptKind,
    boundary: FoundationalBoundaryEvidenceReceiptBoundary,
    provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
    closeout_disposition: Option<FoundationalBoundaryEvidenceCloseoutDisposition>,
}

impl FoundationalBoundaryEvidenceCompletedReceiptArtifact {
    pub(crate) fn new(
        kind: FoundationalBoundaryEvidenceReceiptKind,
        boundary: FoundationalBoundaryEvidenceReceiptBoundary,
        provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
        closeout_disposition: Option<FoundationalBoundaryEvidenceCloseoutDisposition>,
    ) -> Self {
        Self {
            kind,
            boundary,
            provenance,
            closeout_disposition,
        }
    }

    pub const fn receipt_kind(&self) -> FoundationalBoundaryEvidenceReceiptKind {
        self.kind
    }

    pub const fn execution_posture(&self) -> FoundationalBoundaryEvidenceExecutionPosture {
        FoundationalBoundaryEvidenceExecutionPosture::Executed
    }

    pub const fn locality(&self) -> FoundationalBoundaryEvidenceLocality {
        self.provenance.locality()
    }

    pub fn completed_boundary(&self) -> &FoundationalBoundaryEvidenceReceiptBoundary {
        &self.boundary
    }

    pub fn provenance(&self) -> &FoundationalBoundaryEvidenceProvenanceArtifact {
        &self.provenance
    }

    pub const fn closeout_disposition(
        &self,
    ) -> Option<FoundationalBoundaryEvidenceCloseoutDisposition> {
        self.closeout_disposition
    }

    pub const fn did_execute(&self) -> bool {
        self.closeout_disposition.is_none()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBoundaryEvidenceExecutedReceiptArtifact(
    FoundationalBoundaryEvidenceCompletedReceiptArtifact,
);

impl FoundationalBoundaryEvidenceExecutedReceiptArtifact {
    pub(crate) fn new(
        kind: FoundationalBoundaryEvidenceReceiptKind,
        boundary: FoundationalBoundaryEvidenceReceiptBoundary,
        provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
    ) -> Self {
        Self(FoundationalBoundaryEvidenceCompletedReceiptArtifact::new(
            kind, boundary, provenance, None,
        ))
    }

    pub const fn receipt_kind(&self) -> FoundationalBoundaryEvidenceReceiptKind {
        self.0.receipt_kind()
    }

    pub const fn execution_posture(&self) -> FoundationalBoundaryEvidenceExecutionPosture {
        self.0.execution_posture()
    }

    pub const fn locality(&self) -> FoundationalBoundaryEvidenceLocality {
        self.0.locality()
    }

    pub fn completed_boundary(&self) -> &FoundationalBoundaryEvidenceReceiptBoundary {
        self.0.completed_boundary()
    }

    pub fn provenance(&self) -> &FoundationalBoundaryEvidenceProvenanceArtifact {
        self.0.provenance()
    }

    pub const fn did_execute(&self) -> bool {
        true
    }

    pub fn completed_receipt(&self) -> &FoundationalBoundaryEvidenceCompletedReceiptArtifact {
        &self.0
    }
}
