use crate::declaration::{UiAspectName, UiDeclarationIdentity};
use crate::graph::{UiGraphNodeIdentity, UiMountedReceiptIdentity};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiGraphEvidenceRef {
    GraphNode(UiGraphNodeIdentity),
    Declaration(UiDeclarationIdentity),
    MountedReceipt(UiMountedReceiptIdentity),
    Aspect(UiAspectName),
    Page(UiGraphNodeIdentity),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum UiGraphEvidenceRefKind {
    GraphNode,
    Declaration,
    MountedReceipt,
    Aspect,
    Page,
}

impl UiGraphEvidenceRef {
    pub const fn kind(&self) -> UiGraphEvidenceRefKind {
        match self {
            Self::GraphNode(_) => UiGraphEvidenceRefKind::GraphNode,
            Self::Declaration(_) => UiGraphEvidenceRefKind::Declaration,
            Self::MountedReceipt(_) => UiGraphEvidenceRefKind::MountedReceipt,
            Self::Aspect(_) => UiGraphEvidenceRefKind::Aspect,
            Self::Page(_) => UiGraphEvidenceRefKind::Page,
        }
    }
}
