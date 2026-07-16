use super::explanation_api::{
    domain_operation_capability_requirement_set,
    explain_boolean_selectivity_shape_for_access_shape,
    explain_graph_read_access_requirement_outcome_for_family_in_authority_with_operation_lookup,
    explain_graph_read_access_shape_for_graph,
};
use super::{
    WorthQueryBooleanSelectivityShape, WorthQueryGraphReadAccessAuthorityContext,
    WorthQueryGraphReadAccessRequirementExplanationOutcome,
    WorthQueryGraphReadAccessRequirementSet, WorthQueryGraphReadAccessShapeExplanation,
    WorthQueryGraphReadAccessShapeExplanationError, WorthQueryGraphReadOperationLookup,
};
use crate::runtime::WorthQueryReadFamily;

pub(crate) fn explain_graph_read_access_shape_for_family_in_authority_with_lookup(
    family: &WorthQueryReadFamily,
    authority: &WorthQueryGraphReadAccessAuthorityContext,
    lookup: &impl WorthQueryGraphReadOperationLookup,
) -> Result<WorthQueryGraphReadAccessShapeExplanation, WorthQueryGraphReadAccessShapeExplanationError>
{
    explain_graph_read_access_shape_for_graph(
        family.family_digest(),
        family.read_graph(),
        authority,
        lookup,
    )
}

pub(crate) fn explain_boolean_selectivity_shape_for_family_in_authority_with_lookup(
    family: &WorthQueryReadFamily,
    authority: &WorthQueryGraphReadAccessAuthorityContext,
    lookup: &impl WorthQueryGraphReadOperationLookup,
) -> Result<WorthQueryBooleanSelectivityShape, WorthQueryGraphReadAccessShapeExplanationError> {
    let explanation = explain_graph_read_access_shape_for_graph(
        family.family_digest(),
        family.read_graph(),
        authority,
        lookup,
    )?;
    explain_boolean_selectivity_shape_for_access_shape(
        family.read_graph(),
        explanation.access_shape().clone(),
    )
}

pub(crate) fn explain_graph_read_access_requirements_for_family_in_authority_with_lookup(
    family: &WorthQueryReadFamily,
    authority: &WorthQueryGraphReadAccessAuthorityContext,
    lookup: &impl WorthQueryGraphReadOperationLookup,
) -> Result<WorthQueryGraphReadAccessRequirementSet, WorthQueryGraphReadAccessShapeExplanationError>
{
    match explain_graph_read_access_requirement_outcome_for_family_in_authority_with_operation_lookup(
        family, authority, lookup,
    )? {
        WorthQueryGraphReadAccessRequirementExplanationOutcome::RequirementSet(requirements) => {
            Ok(requirements)
        }
        WorthQueryGraphReadAccessRequirementExplanationOutcome::RequiresAccessCapabilityRegistration(
            requirement,
        ) => Ok(domain_operation_capability_requirement_set(requirement)),
        WorthQueryGraphReadAccessRequirementExplanationOutcome::DeniedUnsupportedShape(denial) => {
            Err(WorthQueryGraphReadAccessShapeExplanationError::OperationUnsupportedShape(denial))
        }
    }
}
