use crate::{UiEvidenceBudget, UiInspectionScope, UiInspectionTarget};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum UiInspectionRelevanceOutcome {
    Matched,
    EmptyLocal,
    UnsupportedScope { scope: UiInspectionScope },
    ContradictoryRequest,
    BudgetExceeded { budget: UiEvidenceBudget },
    NotApplicableToTarget { target: UiInspectionTargetClass },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum UiInspectionTargetClass {
    ProductRoot,
    DeclaredSurface,
    GraphNodeIdentity,
    PublishedAspect,
    ConsumedAspect,
    DeclarationIdentity,
    AuthoredSourceProvenance,
    ObligationGraphNode,
    ObligationTouch,
    ObligationEvidenceHandle,
}

impl UiInspectionTargetClass {
    pub fn from_target(target: &UiInspectionTarget) -> Self {
        match target {
            UiInspectionTarget::ProductRoot => Self::ProductRoot,
            UiInspectionTarget::DeclaredSurface { .. } => Self::DeclaredSurface,
            UiInspectionTarget::GraphNodeIdentity { .. } => Self::GraphNodeIdentity,
            UiInspectionTarget::PublishedAspect { .. } => Self::PublishedAspect,
            UiInspectionTarget::ConsumedAspect { .. } => Self::ConsumedAspect,
            UiInspectionTarget::DeclarationIdentity { .. } => Self::DeclarationIdentity,
            UiInspectionTarget::AuthoredSourceProvenance { .. } => Self::AuthoredSourceProvenance,
            UiInspectionTarget::ObligationGraphNode { .. } => Self::ObligationGraphNode,
            UiInspectionTarget::ObligationTouch { .. } => Self::ObligationTouch,
            UiInspectionTarget::ObligationEvidenceHandle { .. } => Self::ObligationEvidenceHandle,
        }
    }
}
