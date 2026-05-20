use crate::locators::{BoundaryArtifactLocator, FoundationalTransitionLocator};

use super::super::primitives::{definition, FoundationalBoundaryEvidencePrimitiveDefinition};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalBoundaryEvidenceSourceBasisKind {
    BoundaryArtifact,
    Transition,
}

pub const fn foundational_boundary_evidence_source_basis_kind_definitions(
) -> [FoundationalBoundaryEvidencePrimitiveDefinition<FoundationalBoundaryEvidenceSourceBasisKind>; 2]
{
    [
        definition(
            FoundationalBoundaryEvidenceSourceBasisKind::BoundaryArtifact,
            "boundary_artifact",
            "a provenance root that starts from a named boundary artifact locus",
            "a completed transition path or generic metadata bag",
        ),
        definition(
            FoundationalBoundaryEvidenceSourceBasisKind::Transition,
            "transition",
            "a provenance root that starts from a named transition or authority-path locus",
            "a boundary artifact attachment or generic history envelope",
        ),
    ]
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoundationalBoundaryEvidenceSourceBasis {
    BoundaryArtifact(BoundaryArtifactLocator),
    Transition(FoundationalTransitionLocator),
}

impl FoundationalBoundaryEvidenceSourceBasis {
    pub const fn boundary_artifact(locator: BoundaryArtifactLocator) -> Self {
        Self::BoundaryArtifact(locator)
    }

    pub fn transition(locator: FoundationalTransitionLocator) -> Self {
        Self::Transition(locator)
    }

    pub const fn kind(&self) -> FoundationalBoundaryEvidenceSourceBasisKind {
        match self {
            Self::BoundaryArtifact(_) => {
                FoundationalBoundaryEvidenceSourceBasisKind::BoundaryArtifact
            }
            Self::Transition(_) => FoundationalBoundaryEvidenceSourceBasisKind::Transition,
        }
    }

    pub const fn boundary_artifact_locator(&self) -> Option<BoundaryArtifactLocator> {
        match self {
            Self::BoundaryArtifact(locator) => Some(*locator),
            Self::Transition(_) => None,
        }
    }

    pub fn transition_locator(&self) -> Option<&FoundationalTransitionLocator> {
        match self {
            Self::BoundaryArtifact(_) => None,
            Self::Transition(locator) => Some(locator),
        }
    }
}
