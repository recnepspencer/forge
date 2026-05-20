use crate::diagnostics::FoundationalDiagnosticLocator;
use crate::locators::BoundaryArtifactLocator;
use crate::FoundationalTransitionLocator;

use super::definitions::FoundationalBoundaryEvidenceAttachmentTargetKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoundationalBoundaryEvidenceAttachmentTarget {
    BoundaryArtifact(BoundaryArtifactLocator),
    Transition(FoundationalTransitionLocator),
    DiagnosticBundle(FoundationalDiagnosticLocator),
}

impl FoundationalBoundaryEvidenceAttachmentTarget {
    pub const fn target_kind(&self) -> FoundationalBoundaryEvidenceAttachmentTargetKind {
        match self {
            Self::BoundaryArtifact(_) => {
                FoundationalBoundaryEvidenceAttachmentTargetKind::BoundaryArtifact
            }
            Self::Transition(_) => {
                FoundationalBoundaryEvidenceAttachmentTargetKind::TransitionArtifact
            }
            Self::DiagnosticBundle(_) => {
                FoundationalBoundaryEvidenceAttachmentTargetKind::DiagnosticBundle
            }
        }
    }

    pub(crate) fn canonical_fragment(&self) -> String {
        match self {
            Self::BoundaryArtifact(locator) => format!(
                "target:boundary_artifact:{}:{:?}",
                locator.artifact_id().get(),
                locator.field()
            ),
            Self::Transition(locator) => format!("target:transition:{locator:?}"),
            Self::DiagnosticBundle(locator) => {
                format!("target:diagnostic:{}", locator.canonical_key_fragment())
            }
        }
    }
}
