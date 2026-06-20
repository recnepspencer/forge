use super::{
    access_requirements, admit_boolean_predicate_expression_for_read_graph,
    admit_query_schema_references_for_read_graph, derive_graph_read_access_shape,
    normalize_boolean_selectivity_for_access_shape, resolve_graph_read_operations_for_read_graph,
    ForgeQueryBooleanExpressionAdmissionError, ForgeQueryBooleanSelectivityShape,
    ForgeQueryGraphReadAccessAuthorityContext, ForgeQueryGraphReadAccessComplexityContract,
    ForgeQueryGraphReadAccessInvalidationBasis, ForgeQueryGraphReadAccessMemoryEstimateBasis,
    ForgeQueryGraphReadAccessRebuildBasis, ForgeQueryGraphReadAccessRequirementDerivationError,
    ForgeQueryGraphReadAccessRequirementExplanationOutcome,
    ForgeQueryGraphReadAccessRequirementKind, ForgeQueryGraphReadAccessRequirementRow,
    ForgeQueryGraphReadAccessRequirementSet, ForgeQueryGraphReadAccessShape,
    ForgeQueryGraphReadAccessShapeExplanation, ForgeQueryGraphReadOperationCapabilityRequirement,
    ForgeQueryGraphReadOperationOutcome, ForgeQueryGraphReadOperationRegistry,
    ForgeQueryGraphReadOperationUnsupportedDenial,
    ForgeQueryGraphReadSchemaReferenceAdmissionError,
};
use crate::identity::hash_parts;
use crate::runtime::{ForgeQueryReadFamily, ForgeQueryReadGraph};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryGraphReadAccessShapeExplanationError {
    SchemaReferenceAdmission(ForgeQueryGraphReadSchemaReferenceAdmissionError),
    BooleanExpressionAdmission(ForgeQueryBooleanExpressionAdmissionError),
    OperationRequiresAccessCapabilityRegistration(
        ForgeQueryGraphReadOperationCapabilityRequirement,
    ),
    OperationUnsupportedShape(ForgeQueryGraphReadOperationUnsupportedDenial),
}

impl ForgeQueryGraphReadAccessShapeExplanationError {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SchemaReferenceAdmission(error) => error.kind().as_str(),
            Self::BooleanExpressionAdmission(error) => error.kind().as_str(),
            Self::OperationRequiresAccessCapabilityRegistration(requirement) => {
                requirement.kind().as_str()
            }
            Self::OperationUnsupportedShape(denial) => denial.kind().as_str(),
        }
    }
}

pub fn explain_graph_read_access_shape_for_family(
    family: &ForgeQueryReadFamily,
) -> Result<ForgeQueryGraphReadAccessShapeExplanation, ForgeQueryGraphReadAccessShapeExplanationError>
{
    let authority = ForgeQueryGraphReadAccessAuthorityContext::runtime_current_compatibility();
    explain_graph_read_access_shape_for_family_in_authority(family, &authority)
}

pub fn explain_graph_read_access_shape_for_family_in_authority(
    family: &ForgeQueryReadFamily,
    authority: &ForgeQueryGraphReadAccessAuthorityContext,
) -> Result<ForgeQueryGraphReadAccessShapeExplanation, ForgeQueryGraphReadAccessShapeExplanationError>
{
    explain_graph_read_access_shape_for_graph(
        family.family_digest(),
        family.read_graph(),
        authority,
        &ForgeQueryGraphReadOperationRegistry::empty(),
    )
}

pub fn explain_graph_read_access_shape_for_family_with_operation_registry(
    family: &ForgeQueryReadFamily,
    registry: &ForgeQueryGraphReadOperationRegistry,
) -> Result<ForgeQueryGraphReadAccessShapeExplanation, ForgeQueryGraphReadAccessShapeExplanationError>
{
    let authority = ForgeQueryGraphReadAccessAuthorityContext::runtime_current_compatibility();
    explain_graph_read_access_shape_for_graph(
        family.family_digest(),
        family.read_graph(),
        &authority,
        registry,
    )
}

pub fn resolve_graph_read_operations_for_family_with_registry(
    family: &ForgeQueryReadFamily,
    registry: &ForgeQueryGraphReadOperationRegistry,
) -> Result<ForgeQueryGraphReadOperationOutcome, ForgeQueryGraphReadAccessShapeExplanationError> {
    let authority = ForgeQueryGraphReadAccessAuthorityContext::runtime_current_compatibility();
    resolve_graph_read_operations_for_graph(family.read_graph(), &authority, registry)
}

pub fn resolve_graph_read_operations_for_family_in_authority_with_registry(
    family: &ForgeQueryReadFamily,
    authority: &ForgeQueryGraphReadAccessAuthorityContext,
    registry: &ForgeQueryGraphReadOperationRegistry,
) -> Result<ForgeQueryGraphReadOperationOutcome, ForgeQueryGraphReadAccessShapeExplanationError> {
    resolve_graph_read_operations_for_graph(family.read_graph(), authority, registry)
}

pub fn explain_boolean_selectivity_shape_for_family(
    family: &ForgeQueryReadFamily,
) -> Result<ForgeQueryBooleanSelectivityShape, ForgeQueryGraphReadAccessShapeExplanationError> {
    let authority = ForgeQueryGraphReadAccessAuthorityContext::runtime_current_compatibility();
    let explanation = explain_graph_read_access_shape_for_graph(
        family.family_digest(),
        family.read_graph(),
        &authority,
        &ForgeQueryGraphReadOperationRegistry::empty(),
    )?;
    explain_boolean_selectivity_shape_for_access_shape(
        family.read_graph(),
        explanation.access_shape().clone(),
    )
}

pub fn explain_boolean_selectivity_shape_for_family_with_operation_registry(
    family: &ForgeQueryReadFamily,
    registry: &ForgeQueryGraphReadOperationRegistry,
) -> Result<ForgeQueryBooleanSelectivityShape, ForgeQueryGraphReadAccessShapeExplanationError> {
    let authority = ForgeQueryGraphReadAccessAuthorityContext::runtime_current_compatibility();
    let explanation = explain_graph_read_access_shape_for_graph(
        family.family_digest(),
        family.read_graph(),
        &authority,
        registry,
    )?;
    explain_boolean_selectivity_shape_for_access_shape(
        family.read_graph(),
        explanation.access_shape().clone(),
    )
}

pub fn derive_graph_read_access_requirements(
    access_shape: &ForgeQueryGraphReadAccessShape,
    selectivity_shape: &ForgeQueryBooleanSelectivityShape,
) -> ForgeQueryGraphReadAccessRequirementSet {
    access_requirements::derive_graph_read_access_requirement_set(access_shape, selectivity_shape)
}

pub fn try_derive_graph_read_access_requirements(
    access_shape: &ForgeQueryGraphReadAccessShape,
    selectivity_shape: &ForgeQueryBooleanSelectivityShape,
) -> Result<
    ForgeQueryGraphReadAccessRequirementSet,
    ForgeQueryGraphReadAccessRequirementDerivationError,
> {
    access_requirements::try_derive_graph_read_access_requirement_set(
        access_shape,
        selectivity_shape,
    )
}

pub fn explain_graph_read_access_requirements_for_family(
    family: &ForgeQueryReadFamily,
) -> Result<ForgeQueryGraphReadAccessRequirementSet, ForgeQueryGraphReadAccessShapeExplanationError>
{
    let authority = ForgeQueryGraphReadAccessAuthorityContext::runtime_current_compatibility();
    explain_graph_read_access_requirements_for_family_in_authority(family, &authority)
}

pub fn explain_graph_read_access_requirements_for_family_in_authority(
    family: &ForgeQueryReadFamily,
    authority: &ForgeQueryGraphReadAccessAuthorityContext,
) -> Result<ForgeQueryGraphReadAccessRequirementSet, ForgeQueryGraphReadAccessShapeExplanationError>
{
    match explain_graph_read_access_requirement_outcome_for_family_in_authority(
        family, authority,
    )? {
        ForgeQueryGraphReadAccessRequirementExplanationOutcome::RequirementSet(requirements) => {
            Ok(requirements)
        }
        ForgeQueryGraphReadAccessRequirementExplanationOutcome::RequiresAccessCapabilityRegistration(
            requirement,
        ) => Ok(domain_operation_capability_requirement_set(requirement)),
        ForgeQueryGraphReadAccessRequirementExplanationOutcome::DeniedUnsupportedShape(denial) => {
            Err(ForgeQueryGraphReadAccessShapeExplanationError::OperationUnsupportedShape(denial))
        }
    }
}

pub fn explain_graph_read_access_requirement_outcome_for_family(
    family: &ForgeQueryReadFamily,
) -> Result<
    ForgeQueryGraphReadAccessRequirementExplanationOutcome,
    ForgeQueryGraphReadAccessShapeExplanationError,
> {
    let authority = ForgeQueryGraphReadAccessAuthorityContext::runtime_current_compatibility();
    explain_graph_read_access_requirement_outcome_for_family_in_authority(family, &authority)
}

pub fn explain_graph_read_access_requirement_outcome_for_family_in_authority(
    family: &ForgeQueryReadFamily,
    authority: &ForgeQueryGraphReadAccessAuthorityContext,
) -> Result<
    ForgeQueryGraphReadAccessRequirementExplanationOutcome,
    ForgeQueryGraphReadAccessShapeExplanationError,
> {
    explain_graph_read_access_requirement_outcome_for_family_in_authority_with_operation_registry(
        family,
        authority,
        &ForgeQueryGraphReadOperationRegistry::empty(),
    )
}

fn explain_graph_read_access_requirement_outcome_for_family_in_authority_with_operation_registry(
    family: &ForgeQueryReadFamily,
    authority: &ForgeQueryGraphReadAccessAuthorityContext,
    registry: &ForgeQueryGraphReadOperationRegistry,
) -> Result<
    ForgeQueryGraphReadAccessRequirementExplanationOutcome,
    ForgeQueryGraphReadAccessShapeExplanationError,
> {
    let operation_resolution = match resolve_graph_read_operations_for_graph(
        family.read_graph(),
        authority,
        registry,
    )? {
        ForgeQueryGraphReadOperationOutcome::Resolved(resolution) => resolution,
        ForgeQueryGraphReadOperationOutcome::RequiresAccessCapabilityRegistration(requirement) => {
            return Ok(
                    ForgeQueryGraphReadAccessRequirementExplanationOutcome::RequiresAccessCapabilityRegistration(
                        requirement,
                    ),
                );
        }
        ForgeQueryGraphReadOperationOutcome::DeniedUnsupportedShape(denial) => {
            return Ok(
                ForgeQueryGraphReadAccessRequirementExplanationOutcome::DeniedUnsupportedShape(
                    denial,
                ),
            );
        }
    };
    let access_shape = derive_graph_read_access_shape(operation_resolution);
    let selectivity_shape = explain_boolean_selectivity_shape_for_access_shape(
        family.read_graph(),
        access_shape.clone(),
    )?;
    Ok(
        ForgeQueryGraphReadAccessRequirementExplanationOutcome::RequirementSet(
            derive_graph_read_access_requirements(&access_shape, &selectivity_shape),
        ),
    )
}

pub fn explain_graph_read_access_requirement_outcome_for_family_with_operation_registry(
    family: &ForgeQueryReadFamily,
    registry: &ForgeQueryGraphReadOperationRegistry,
) -> Result<
    ForgeQueryGraphReadAccessRequirementExplanationOutcome,
    ForgeQueryGraphReadAccessShapeExplanationError,
> {
    let authority = ForgeQueryGraphReadAccessAuthorityContext::runtime_current_compatibility();
    let operation_resolution = match resolve_graph_read_operations_for_graph(
        family.read_graph(),
        &authority,
        registry,
    )? {
        ForgeQueryGraphReadOperationOutcome::Resolved(resolution) => resolution,
        ForgeQueryGraphReadOperationOutcome::RequiresAccessCapabilityRegistration(requirement) => {
            return Ok(
                    ForgeQueryGraphReadAccessRequirementExplanationOutcome::RequiresAccessCapabilityRegistration(
                        requirement,
                    ),
                );
        }
        ForgeQueryGraphReadOperationOutcome::DeniedUnsupportedShape(denial) => {
            return Ok(
                ForgeQueryGraphReadAccessRequirementExplanationOutcome::DeniedUnsupportedShape(
                    denial,
                ),
            );
        }
    };
    let access_shape = derive_graph_read_access_shape(operation_resolution);
    let selectivity_shape = explain_boolean_selectivity_shape_for_access_shape(
        family.read_graph(),
        access_shape.clone(),
    )?;
    Ok(
        ForgeQueryGraphReadAccessRequirementExplanationOutcome::RequirementSet(
            derive_graph_read_access_requirements(&access_shape, &selectivity_shape),
        ),
    )
}

pub fn explain_graph_read_access_requirements_for_family_with_operation_registry(
    family: &ForgeQueryReadFamily,
    registry: &ForgeQueryGraphReadOperationRegistry,
) -> Result<ForgeQueryGraphReadAccessRequirementSet, ForgeQueryGraphReadAccessShapeExplanationError>
{
    match explain_graph_read_access_requirement_outcome_for_family_with_operation_registry(
        family, registry,
    )? {
        ForgeQueryGraphReadAccessRequirementExplanationOutcome::RequirementSet(requirements) => {
            Ok(requirements)
        }
        ForgeQueryGraphReadAccessRequirementExplanationOutcome::RequiresAccessCapabilityRegistration(
            requirement,
        ) => Ok(domain_operation_capability_requirement_set(requirement)),
        ForgeQueryGraphReadAccessRequirementExplanationOutcome::DeniedUnsupportedShape(denial) => {
            Err(ForgeQueryGraphReadAccessShapeExplanationError::OperationUnsupportedShape(denial))
        }
    }
}

fn domain_operation_capability_requirement_set(
    requirement: ForgeQueryGraphReadOperationCapabilityRequirement,
) -> ForgeQueryGraphReadAccessRequirementSet {
    let read_graph_digest = requirement.read_graph_digest().to_string();
    let requirement_digest_part = requirement.digest_part();
    let access_shape_digest = hash_parts(&[
        "domain_operation_capability_registration_access_shape_v1".to_string(),
        requirement_digest_part.clone(),
    ]);
    let selectivity_shape_digest = hash_parts(&[
        "domain_operation_capability_registration_selectivity_shape_v1".to_string(),
        requirement_digest_part,
    ]);
    ForgeQueryGraphReadAccessRequirementSet::new(
        read_graph_digest,
        access_shape_digest,
        selectivity_shape_digest,
        vec![ForgeQueryGraphReadAccessRequirementRow::new(
            ForgeQueryGraphReadAccessRequirementKind::DomainOperationCapabilityRegistration,
            ForgeQueryGraphReadAccessRebuildBasis::RuntimeSupportRequired,
            ForgeQueryGraphReadAccessInvalidationBasis::RuntimeLifecycleDelta,
            ForgeQueryGraphReadAccessComplexityContract::LifecycleSupportAdmission,
            ForgeQueryGraphReadAccessMemoryEstimateBasis::LifecycleManagedSupport,
        )
        .with_operation_capability_requirement(requirement)],
    )
}

fn explain_boolean_selectivity_shape_for_access_shape(
    read_graph: &ForgeQueryReadGraph,
    access_shape: ForgeQueryGraphReadAccessShape,
) -> Result<ForgeQueryBooleanSelectivityShape, ForgeQueryGraphReadAccessShapeExplanationError> {
    let expression = admit_boolean_predicate_expression_for_read_graph(
        read_graph,
        access_shape.operation_resolution().references(),
    )
    .map_err(ForgeQueryGraphReadAccessShapeExplanationError::BooleanExpressionAdmission)?;
    Ok(normalize_boolean_selectivity_for_access_shape(
        access_shape,
        expression,
    ))
}

fn explain_graph_read_access_shape_for_graph(
    family_digest: &str,
    read_graph: &ForgeQueryReadGraph,
    authority: &ForgeQueryGraphReadAccessAuthorityContext,
    registry: &ForgeQueryGraphReadOperationRegistry,
) -> Result<ForgeQueryGraphReadAccessShapeExplanation, ForgeQueryGraphReadAccessShapeExplanationError>
{
    let operation_resolution = match resolve_graph_read_operations_for_graph(read_graph, authority, registry)? {
        ForgeQueryGraphReadOperationOutcome::Resolved(resolution) => resolution,
        ForgeQueryGraphReadOperationOutcome::RequiresAccessCapabilityRegistration(requirement) => {
            return Err(
                ForgeQueryGraphReadAccessShapeExplanationError::OperationRequiresAccessCapabilityRegistration(
                    requirement,
                ),
            )
        }
        ForgeQueryGraphReadOperationOutcome::DeniedUnsupportedShape(denial) => {
            return Err(
                ForgeQueryGraphReadAccessShapeExplanationError::OperationUnsupportedShape(denial),
            )
        }
    };
    let access_shape = derive_graph_read_access_shape(operation_resolution);
    Ok(ForgeQueryGraphReadAccessShapeExplanation::from_shape(
        family_digest,
        access_shape,
    ))
}

fn resolve_graph_read_operations_for_graph(
    read_graph: &ForgeQueryReadGraph,
    authority: &ForgeQueryGraphReadAccessAuthorityContext,
    registry: &ForgeQueryGraphReadOperationRegistry,
) -> Result<ForgeQueryGraphReadOperationOutcome, ForgeQueryGraphReadAccessShapeExplanationError> {
    let references = admit_query_schema_references_for_read_graph(read_graph)
        .map_err(ForgeQueryGraphReadAccessShapeExplanationError::SchemaReferenceAdmission)?;
    let (basis_binding, policy_tenant_proof_binding) = authority.bind_for_read_graph(read_graph);
    Ok(resolve_graph_read_operations_for_read_graph(
        read_graph,
        references,
        basis_binding,
        policy_tenant_proof_binding,
        registry,
    ))
}
