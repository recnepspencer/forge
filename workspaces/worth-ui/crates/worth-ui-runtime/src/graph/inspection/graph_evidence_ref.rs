use crate::declaration::{UiAspectName, UiDeclarationIdentity};
use crate::graph::{UiGraphMountEligibilityIdentity, UiGraphNodeIdentity};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiGraphEvidenceRef {
    GraphNode(UiGraphNodeIdentity),
    Declaration(UiDeclarationIdentity),
    MountEligibility(UiGraphMountEligibilityIdentity),
    Aspect(UiAspectName),
    Page(UiGraphNodeIdentity),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum UiGraphEvidenceRefKind {
    GraphNode,
    Declaration,
    MountEligibility,
    Aspect,
    Page,
}

impl UiGraphEvidenceRef {
    pub const fn kind(&self) -> UiGraphEvidenceRefKind {
        match self {
            Self::GraphNode(_) => UiGraphEvidenceRefKind::GraphNode,
            Self::Declaration(_) => UiGraphEvidenceRefKind::Declaration,
            Self::MountEligibility(_) => UiGraphEvidenceRefKind::MountEligibility,
            Self::Aspect(_) => UiGraphEvidenceRefKind::Aspect,
            Self::Page(_) => UiGraphEvidenceRefKind::Page,
        }
    }
}
