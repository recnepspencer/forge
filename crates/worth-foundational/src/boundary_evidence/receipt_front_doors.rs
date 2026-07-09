use super::provenance::FoundationalBoundaryEvidenceProvenanceArtifact;
use super::receipts::{
    FoundationalBoundaryEvidenceCloseoutDisposition,
    FoundationalBoundaryEvidenceCompletedReceiptArtifact,
    FoundationalBoundaryEvidenceExecutedReceiptArtifact,
    FoundationalBoundaryEvidencePlanningReceiptArtifact,
    FoundationalBoundaryEvidenceReceiptBoundary, FoundationalBoundaryEvidenceReceiptKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FoundationalBoundaryEvidenceReceiptFrontDoor;

impl FoundationalBoundaryEvidenceReceiptFrontDoor {
    pub fn admission(
        self,
        boundary: FoundationalBoundaryEvidenceReceiptBoundary,
    ) -> FoundationalBoundaryEvidenceExecutedReceiptStep {
        FoundationalBoundaryEvidenceExecutedReceiptStep::new(
            FoundationalBoundaryEvidenceReceiptKind::Admission,
            boundary,
        )
    }

    pub fn planning(
        self,
        boundary: FoundationalBoundaryEvidenceReceiptBoundary,
    ) -> FoundationalBoundaryEvidencePlanningReceiptStep {
        FoundationalBoundaryEvidencePlanningReceiptStep::new(boundary)
    }

    pub fn execution(
        self,
        boundary: FoundationalBoundaryEvidenceReceiptBoundary,
    ) -> FoundationalBoundaryEvidenceExecutedReceiptStep {
        FoundationalBoundaryEvidenceExecutedReceiptStep::new(
            FoundationalBoundaryEvidenceReceiptKind::Execution,
            boundary,
        )
    }

    pub fn publication(
        self,
        boundary: FoundationalBoundaryEvidenceReceiptBoundary,
    ) -> FoundationalBoundaryEvidenceExecutedReceiptStep {
        FoundationalBoundaryEvidenceExecutedReceiptStep::new(
            FoundationalBoundaryEvidenceReceiptKind::Publication,
            boundary,
        )
    }

    pub fn restoration(
        self,
        boundary: FoundationalBoundaryEvidenceReceiptBoundary,
    ) -> FoundationalBoundaryEvidenceExecutedReceiptStep {
        FoundationalBoundaryEvidenceExecutedReceiptStep::new(
            FoundationalBoundaryEvidenceReceiptKind::Restoration,
            boundary,
        )
    }

    pub fn support_publication(
        self,
        boundary: FoundationalBoundaryEvidenceReceiptBoundary,
    ) -> FoundationalBoundaryEvidenceExecutedReceiptStep {
        FoundationalBoundaryEvidenceExecutedReceiptStep::new(
            FoundationalBoundaryEvidenceReceiptKind::SupportPublication,
            boundary,
        )
    }

    pub fn checkpoint_resume(
        self,
        boundary: FoundationalBoundaryEvidenceReceiptBoundary,
    ) -> FoundationalBoundaryEvidenceExecutedReceiptStep {
        FoundationalBoundaryEvidenceExecutedReceiptStep::new(
            FoundationalBoundaryEvidenceReceiptKind::CheckpointResume,
            boundary,
        )
    }

    pub fn blocked_closeout(
        self,
        boundary: FoundationalBoundaryEvidenceReceiptBoundary,
    ) -> FoundationalBoundaryEvidenceCloseoutReceiptStep {
        FoundationalBoundaryEvidenceCloseoutReceiptStep::new(
            FoundationalBoundaryEvidenceReceiptKind::Closeout,
            boundary,
            FoundationalBoundaryEvidenceCloseoutDisposition::Blocked,
        )
    }

    pub fn denied_closeout(
        self,
        boundary: FoundationalBoundaryEvidenceReceiptBoundary,
    ) -> FoundationalBoundaryEvidenceCloseoutReceiptStep {
        FoundationalBoundaryEvidenceCloseoutReceiptStep::new(
            FoundationalBoundaryEvidenceReceiptKind::Closeout,
            boundary,
            FoundationalBoundaryEvidenceCloseoutDisposition::Denied,
        )
    }
}

#[derive(Debug, Clone)]
pub struct FoundationalBoundaryEvidencePlanningReceiptStep {
    boundary: FoundationalBoundaryEvidenceReceiptBoundary,
}

impl FoundationalBoundaryEvidencePlanningReceiptStep {
    fn new(boundary: FoundationalBoundaryEvidenceReceiptBoundary) -> Self {
        Self { boundary }
    }

    pub fn with_provenance(
        self,
        provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
    ) -> FoundationalBoundaryEvidencePlanningReceiptArtifact {
        FoundationalBoundaryEvidencePlanningReceiptArtifact::new(self.boundary, provenance)
    }
}

#[derive(Debug, Clone)]
pub struct FoundationalBoundaryEvidenceExecutedReceiptStep {
    kind: FoundationalBoundaryEvidenceReceiptKind,
    boundary: FoundationalBoundaryEvidenceReceiptBoundary,
}

impl FoundationalBoundaryEvidenceExecutedReceiptStep {
    fn new(
        kind: FoundationalBoundaryEvidenceReceiptKind,
        boundary: FoundationalBoundaryEvidenceReceiptBoundary,
    ) -> Self {
        Self { kind, boundary }
    }

    pub fn with_provenance(
        self,
        provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
    ) -> FoundationalBoundaryEvidenceExecutedReceiptArtifact {
        FoundationalBoundaryEvidenceExecutedReceiptArtifact::new(
            self.kind,
            self.boundary,
            provenance,
        )
    }
}

#[derive(Debug, Clone)]
pub struct FoundationalBoundaryEvidenceCloseoutReceiptStep {
    kind: FoundationalBoundaryEvidenceReceiptKind,
    boundary: FoundationalBoundaryEvidenceReceiptBoundary,
    closeout_disposition: FoundationalBoundaryEvidenceCloseoutDisposition,
}

impl FoundationalBoundaryEvidenceCloseoutReceiptStep {
    fn new(
        kind: FoundationalBoundaryEvidenceReceiptKind,
        boundary: FoundationalBoundaryEvidenceReceiptBoundary,
        closeout_disposition: FoundationalBoundaryEvidenceCloseoutDisposition,
    ) -> Self {
        Self {
            kind,
            boundary,
            closeout_disposition,
        }
    }

    pub fn with_provenance(
        self,
        provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
    ) -> FoundationalBoundaryEvidenceCompletedReceiptArtifact {
        FoundationalBoundaryEvidenceCompletedReceiptArtifact::new(
            self.kind,
            self.boundary,
            provenance,
            Some(self.closeout_disposition),
        )
    }
}
