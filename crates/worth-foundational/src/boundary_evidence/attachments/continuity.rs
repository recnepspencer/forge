use crate::diagnostics::FoundationalDiagnosticLocator;

use super::super::lineage::{
    FoundationalBoundaryEvidenceAttestedLineageArtifact,
    FoundationalBoundaryEvidenceBranchLocalLineageArtifact,
    FoundationalBoundaryEvidenceLineageSubject, FoundationalBoundaryEvidencePartialLineageArtifact,
    FoundationalBoundaryEvidencePromotedLineageArtifact,
    FoundationalBoundaryEvidenceReconstructedEquivalenceArtifact,
    FoundationalBoundaryEvidenceReplayDerivedLineageArtifact,
    FoundationalBoundaryEvidenceRestoredLineageArtifact,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoundationalBoundaryEvidenceObjectContinuityAttachment {
    Attested(FoundationalBoundaryEvidenceAttestedLineageArtifact),
    BranchLocal(FoundationalBoundaryEvidenceBranchLocalLineageArtifact),
    Promoted(FoundationalBoundaryEvidencePromotedLineageArtifact),
    ReplayDerived(FoundationalBoundaryEvidenceReplayDerivedLineageArtifact),
    Restored(FoundationalBoundaryEvidenceRestoredLineageArtifact),
    Reconstructed(FoundationalBoundaryEvidenceReconstructedEquivalenceArtifact),
    Partial(FoundationalBoundaryEvidencePartialLineageArtifact),
}

impl FoundationalBoundaryEvidenceObjectContinuityAttachment {
    pub(crate) fn canonical_fragment(&self) -> String {
        match self {
            Self::Attested(artifact) => {
                format!("continuity:attested:{:?}", artifact.outcome_kind())
            }
            Self::BranchLocal(artifact) => {
                format!("continuity:branch_local:{:?}", artifact.outcome_kind())
            }
            Self::Promoted(artifact) => {
                format!("continuity:promoted:{:?}", artifact.outcome_kind())
            }
            Self::ReplayDerived(artifact) => {
                format!("continuity:replay:{:?}", artifact.outcome_kind())
            }
            Self::Restored(artifact) => {
                format!("continuity:restored:{:?}", artifact.outcome_kind())
            }
            Self::Reconstructed(artifact) => {
                format!("continuity:reconstructed:{:?}", artifact.outcome_kind())
            }
            Self::Partial(artifact) => format!("continuity:partial:{:?}", artifact.outcome_kind()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBoundaryEvidenceLocatorContinuityAttachment {
    stable_object: FoundationalBoundaryEvidenceLineageSubject,
    from_locator: FoundationalDiagnosticLocator,
    to_locator: FoundationalDiagnosticLocator,
}

impl FoundationalBoundaryEvidenceLocatorContinuityAttachment {
    pub(crate) fn new(
        stable_object: FoundationalBoundaryEvidenceLineageSubject,
        from_locator: FoundationalDiagnosticLocator,
        to_locator: FoundationalDiagnosticLocator,
    ) -> Self {
        Self {
            stable_object,
            from_locator,
            to_locator,
        }
    }

    pub const fn stable_object(&self) -> FoundationalBoundaryEvidenceLineageSubject {
        self.stable_object
    }

    pub fn from_locator(&self) -> &FoundationalDiagnosticLocator {
        &self.from_locator
    }

    pub fn to_locator(&self) -> &FoundationalDiagnosticLocator {
        &self.to_locator
    }

    pub(crate) fn canonical_fragment(&self) -> String {
        format!(
            "locator_continuity:{}:{}:{}",
            self.stable_object.handle().get(),
            self.from_locator.canonical_key_fragment(),
            self.to_locator.canonical_key_fragment()
        )
    }
}
