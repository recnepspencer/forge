use crate::evidence::UiConstraintPropagationEdgeFamily;
use crate::evidence::UiLayoutOperatorSpecialInputRequirement;

use super::admission_parts::ConstraintAuthorityContext;

pub(super) fn classify_special_input_requirements(
    context: &ConstraintAuthorityContext<'_>,
) -> Vec<UiConstraintPropagationEdgeFamily> {
    context
        .special_input_requirements
        .iter()
        .copied()
        .map(family_for_requirement)
        .collect()
}

pub(super) fn family_for_requirement(
    requirement: UiLayoutOperatorSpecialInputRequirement,
) -> UiConstraintPropagationEdgeFamily {
    match requirement {
        UiLayoutOperatorSpecialInputRequirement::ViewportExtent => {
            UiConstraintPropagationEdgeFamily::ViewportInput
        }
        UiLayoutOperatorSpecialInputRequirement::ScrollViewportExtent => {
            UiConstraintPropagationEdgeFamily::ScrollViewportInput
        }
        UiLayoutOperatorSpecialInputRequirement::PortalAnchorRect => {
            UiConstraintPropagationEdgeFamily::PortalAnchorInput
        }
    }
}
