pub(super) fn consequence_for(
    target: crate::graph::UiGraphNodeIdentity,
    neighborhood: &crate::evidence::UiAllocationNeighborhood,
    impact: Option<&crate::runtime::WorthUiReplacementImpactClassification>,
) -> super::UiReplanWidenReason {
    if target == neighborhood.root_graph_node_identity() {
        return super::UiReplanWidenReason::SharedAncestorRequirement;
    }
    match impact.map(crate::runtime::WorthUiReplacementImpactClassification::impact) {
        Some(crate::runtime::WorthUiReplacementImpact::StructuralReplacement(_))
        | Some(crate::runtime::WorthUiReplacementImpact::LaneAffecting { .. }) => {
            super::UiReplanWidenReason::ConstraintPropagationCrossing
        }
        Some(crate::runtime::WorthUiReplacementImpact::BroadReplacement(_)) => {
            super::UiReplanWidenReason::SharedAncestorRequirement
        }
        Some(crate::runtime::WorthUiReplacementImpact::NoOp)
        | Some(crate::runtime::WorthUiReplacementImpact::LocalSubtree(_))
        | None => super::UiReplanWidenReason::MeasurementBasisReach,
    }
}
