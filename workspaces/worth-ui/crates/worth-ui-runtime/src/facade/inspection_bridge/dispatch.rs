use worth_ui_inspection::{UiInspectionQuery, UiInspectionScope, UiInspectionTarget};

/// Named inspection dispatch lane — teaches valid transition order before boundary routing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InspectionDispatchLane {
    MeasurementScope,
    PlanningScope,
    ProductRootOrDeclaredSurface,
    AuthoredLookup,
    GraphNodeIdentity,
    AspectEvidence,
    RetainedObligation,
    UnsupportedTarget,
}

pub fn classify_inspection_dispatch(query: &UiInspectionQuery) -> InspectionDispatchLane {
    if query.scope() == UiInspectionScope::Measurement {
        return InspectionDispatchLane::MeasurementScope;
    }
    if query.scope() == UiInspectionScope::Planning {
        return InspectionDispatchLane::PlanningScope;
    }
    match query.target() {
        UiInspectionTarget::ProductRoot | UiInspectionTarget::DeclaredSurface { .. } => {
            InspectionDispatchLane::ProductRootOrDeclaredSurface
        }
        UiInspectionTarget::DeclarationIdentity { .. }
        | UiInspectionTarget::AuthoredSourceProvenance { .. } => {
            InspectionDispatchLane::AuthoredLookup
        }
        UiInspectionTarget::GraphNodeIdentity { .. } => InspectionDispatchLane::GraphNodeIdentity,
        UiInspectionTarget::PublishedAspect { .. } | UiInspectionTarget::ConsumedAspect { .. } => {
            InspectionDispatchLane::AspectEvidence
        }
        UiInspectionTarget::ObligationGraphNode { .. }
        | UiInspectionTarget::ObligationTouch { .. }
        | UiInspectionTarget::ObligationEvidenceHandle { .. } => {
            InspectionDispatchLane::RetainedObligation
        }
        _ => InspectionDispatchLane::UnsupportedTarget,
    }
}
