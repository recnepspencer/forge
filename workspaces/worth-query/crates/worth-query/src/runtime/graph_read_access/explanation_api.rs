#[cfg(test)]
use super::WorthQueryGraphReadOperationRegistry;
use super::{
    access_requirements, admit_boolean_predicate_expression_for_read_graph,
    admit_query_schema_references_for_read_graph, derive_graph_read_access_shape,
    normalize_boolean_selectivity_for_access_shape, resolve_graph_read_operations_for_read_graph,
    WorthQueryBooleanExpressionAdmissionError, WorthQueryBooleanSelectivityShape,
    WorthQueryGraphReadAccessAuthorityContext, WorthQueryGraphReadAccessComplexityContract,
    WorthQueryGraphReadAccessInvalidationBasis, WorthQueryGraphReadAccessMemoryEstimateBasis,
    WorthQueryGraphReadAccessRebuildBasis, WorthQueryGraphReadAccessRequirementDerivationError,
    WorthQueryGraphReadAccessRequirementExplanationOutcome,
    WorthQueryGraphReadAccessRequirementKind, WorthQueryGraphReadAccessRequirementRow,
    WorthQueryGraphReadAccessRequirementSet, WorthQueryGraphReadAccessShape,
    WorthQueryGraphReadAccessShapeExplanation, WorthQueryGraphReadOperationCapabilityRequirement,
    WorthQueryGraphReadOperationLookup, WorthQueryGraphReadOperationOutcome,
    WorthQueryGraphReadOperationUnsupportedDenial,
    WorthQueryGraphReadSchemaReferenceAdmissionError,
};
use crate::identity::hash_parts;
use crate::runtime::{WorthQueryReadFamily, WorthQueryReadGraph};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphReadAccessShapeExplanationError {
    SchemaReferenceAdmission(WorthQueryGraphReadSchemaReferenceAdmissionError),
    BooleanExpressionAdmission(WorthQueryBooleanExpressionAdmissionError),
    OperationRequiresAccessCapabilityRegistration(
        WorthQueryGraphReadOperationCapabilityRequirement,
    ),
    OperationUnsupportedShape(WorthQueryGraphReadOperationUnsupportedDenial),
}

impl WorthQueryGraphReadAccessShapeExplanationError {
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

#[cfg(test)]
pub fn explain_graph_read_access_shape_for_family(
    family: &WorthQueryReadFamily,
) -> Result<WorthQueryGraphReadAccessShapeExplanation, WorthQueryGraphReadAccessShapeExplanationError>
{
    let authority = WorthQueryGraphReadAccessAuthorityContext::runtime_current_compatibility();
    explain_graph_read_access_shape_for_family_in_authority(family, &authority)
}

#[cfg(test)]
pub fn explain_graph_read_access_shape_for_family_in_authority(
    family: &WorthQueryReadFamily,
    authority: &WorthQueryGraphReadAccessAuthorityContext,
) -> Result<WorthQueryGraphReadAccessShapeExplanation, WorthQueryGraphReadAccessShapeExplanationError>
{
    explain_graph_read_access_shape_for_graph(
        family.family_digest(),
        family.read_graph(),
        authority,
        &WorthQueryGraphReadOperationRegistry::empty(),
    )
}

#[cfg(test)]
pub(crate) fn explain_graph_read_access_shape_for_family_with_operation_lookup(
    family: &WorthQueryReadFamily,
    registry: &WorthQueryGraphReadOperationRegistry,
) -> Result<WorthQueryGraphReadAccessShapeExplanation, WorthQueryGraphReadAccessShapeExplanationError>
{
    let authority = WorthQueryGraphReadAccessAuthorityContext::runtime_current_compatibility();
    explain_graph_read_access_shape_for_graph(
        family.family_digest(),
        family.read_graph(),
        &authority,
        registry,
    )
}

#[cfg(test)]
pub(crate) fn resolve_graph_read_operations_for_family_with_operation_lookup(
    family: &WorthQueryReadFamily,
    registry: &WorthQueryGraphReadOperationRegistry,
) -> Result<WorthQueryGraphReadOperationOutcome, WorthQueryGraphReadAccessShapeExplanationError> {
    let authority = WorthQueryGraphReadAccessAuthorityContext::runtime_current_compatibility();
    resolve_graph_read_operations_for_graph(family.read_graph(), &authority, registry)
}

#[cfg(test)]
pub fn explain_boolean_selectivity_shape_for_family(
    family: &WorthQueryReadFamily,
) -> Result<WorthQueryBooleanSelectivityShape, WorthQueryGraphReadAccessShapeExplanationError> {
    let authority = WorthQueryGraphReadAccessAuthorityContext::runtime_current_compatibility();
    let explanation = explain_graph_read_access_shape_for_graph(
        family.family_digest(),
        family.read_graph(),
        &authority,
        &WorthQueryGraphReadOperationRegistry::empty(),
    )?;
    explain_boolean_selectivity_shape_for_access_shape(
        family.read_graph(),
        explanation.access_shape().clone(),
    )
}

pub fn derive_graph_read_access_requirements(
    access_shape: &WorthQueryGraphReadAccessShape,
    selectivity_shape: &WorthQueryBooleanSelectivityShape,
) -> WorthQueryGraphReadAccessRequirementSet {
    access_requirements::derive_graph_read_access_requirement_set(access_shape, selectivity_shape)
}

pub fn try_derive_graph_read_access_requirements(
    access_shape: &WorthQueryGraphReadAccessShape,
    selectivity_shape: &WorthQueryBooleanSelectivityShape,
) -> Result<
    WorthQueryGraphReadAccessRequirementSet,
    WorthQueryGraphReadAccessRequirementDerivationError,
> {
    access_requirements::try_derive_graph_read_access_requirement_set(
        access_shape,
        selectivity_shape,
    )
}

#[cfg(test)]
pub fn explain_graph_read_access_requirements_for_family(
    family: &WorthQueryReadFamily,
) -> Result<WorthQueryGraphReadAccessRequirementSet, WorthQueryGraphReadAccessShapeExplanationError>
{
    let authority = WorthQueryGraphReadAccessAuthorityContext::runtime_current_compatibility();
    explain_graph_read_access_requirements_for_family_in_authority(family, &authority)
}

#[cfg(test)]
pub fn explain_graph_read_access_requirements_for_family_in_authority(
    family: &WorthQueryReadFamily,
    authority: &WorthQueryGraphReadAccessAuthorityContext,
) -> Result<WorthQueryGraphReadAccessRequirementSet, WorthQueryGraphReadAccessShapeExplanationError>
{
    match explain_graph_read_access_requirement_outcome_for_family_in_authority(
        family, authority,
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

#[cfg(test)]
pub fn explain_graph_read_access_requirement_outcome_for_family_in_authority(
    family: &WorthQueryReadFamily,
    authority: &WorthQueryGraphReadAccessAuthorityContext,
) -> Result<
    WorthQueryGraphReadAccessRequirementExplanationOutcome,
    WorthQueryGraphReadAccessShapeExplanationError,
> {
    explain_graph_read_access_requirement_outcome_for_family_in_authority_with_operation_lookup(
        family,
        authority,
        &WorthQueryGraphReadOperationRegistry::empty(),
    )
}

pub(super) fn explain_graph_read_access_requirement_outcome_for_family_in_authority_with_operation_lookup(
    family: &WorthQueryReadFamily,
    authority: &WorthQueryGraphReadAccessAuthorityContext,
    registry: &impl WorthQueryGraphReadOperationLookup,
) -> Result<
    WorthQueryGraphReadAccessRequirementExplanationOutcome,
    WorthQueryGraphReadAccessShapeExplanationError,
> {
    let operation_resolution = match resolve_graph_read_operations_for_graph(
        family.read_graph(),
        authority,
        registry,
    )? {
        WorthQueryGraphReadOperationOutcome::Resolved(resolution) => resolution,
        WorthQueryGraphReadOperationOutcome::RequiresAccessCapabilityRegistration(requirement) => {
            return Ok(
                    WorthQueryGraphReadAccessRequirementExplanationOutcome::RequiresAccessCapabilityRegistration(
                        requirement,
                    ),
                );
        }
        WorthQueryGraphReadOperationOutcome::DeniedUnsupportedShape(denial) => {
            return Ok(
                WorthQueryGraphReadAccessRequirementExplanationOutcome::DeniedUnsupportedShape(
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
        WorthQueryGraphReadAccessRequirementExplanationOutcome::RequirementSet(
            derive_graph_read_access_requirements(&access_shape, &selectivity_shape),
        ),
    )
}

#[cfg(test)]
pub(crate) fn explain_graph_read_access_requirement_outcome_for_family_with_operation_lookup(
    family: &WorthQueryReadFamily,
    registry: &WorthQueryGraphReadOperationRegistry,
) -> Result<
    WorthQueryGraphReadAccessRequirementExplanationOutcome,
    WorthQueryGraphReadAccessShapeExplanationError,
> {
    let authority = WorthQueryGraphReadAccessAuthorityContext::runtime_current_compatibility();
    let operation_resolution = match resolve_graph_read_operations_for_graph(
        family.read_graph(),
        &authority,
        registry,
    )? {
        WorthQueryGraphReadOperationOutcome::Resolved(resolution) => resolution,
        WorthQueryGraphReadOperationOutcome::RequiresAccessCapabilityRegistration(requirement) => {
            return Ok(
                    WorthQueryGraphReadAccessRequirementExplanationOutcome::RequiresAccessCapabilityRegistration(
                        requirement,
                    ),
                );
        }
        WorthQueryGraphReadOperationOutcome::DeniedUnsupportedShape(denial) => {
            return Ok(
                WorthQueryGraphReadAccessRequirementExplanationOutcome::DeniedUnsupportedShape(
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
        WorthQueryGraphReadAccessRequirementExplanationOutcome::RequirementSet(
            derive_graph_read_access_requirements(&access_shape, &selectivity_shape),
        ),
    )
}

#[cfg(test)]
pub(crate) fn explain_graph_read_access_requirements_for_family_with_operation_lookup(
    family: &WorthQueryReadFamily,
    registry: &WorthQueryGraphReadOperationRegistry,
) -> Result<WorthQueryGraphReadAccessRequirementSet, WorthQueryGraphReadAccessShapeExplanationError>
{
    match explain_graph_read_access_requirement_outcome_for_family_with_operation_lookup(
        family, registry,
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

pub(super) fn domain_operation_capability_requirement_set(
    requirement: WorthQueryGraphReadOperationCapabilityRequirement,
) -> WorthQueryGraphReadAccessRequirementSet {
    let read_graph_digest =
        worth_foundational::facade::CanonicalDigestId::parse_hex(requirement.read_graph_digest())
            .expect("installed operation read-graph digests are canonical SHA-256 hex");
    let requirement_digest_part = requirement.digest_part();
    let access_shape_digest = hash_parts(&[
        "domain_operation_capability_registration_access_shape_v1".to_string(),
        requirement_digest_part.clone(),
    ]);
    let selectivity_shape_digest = hash_parts(&[
        "domain_operation_capability_registration_selectivity_shape_v1".to_string(),
        requirement_digest_part,
    ]);
    WorthQueryGraphReadAccessRequirementSet::new(
        read_graph_digest,
        worth_foundational::facade::CanonicalDigestId::parse_hex(&access_shape_digest)
            .expect("access-shape support digests are canonical SHA-256 hex"),
        worth_foundational::facade::CanonicalDigestId::parse_hex(&selectivity_shape_digest)
            .expect("selectivity support digests are canonical SHA-256 hex"),
        vec![WorthQueryGraphReadAccessRequirementRow::new(
            WorthQueryGraphReadAccessRequirementKind::DomainOperationCapabilityRegistration,
            WorthQueryGraphReadAccessRebuildBasis::RuntimeSupportRequired,
            WorthQueryGraphReadAccessInvalidationBasis::RuntimeLifecycleDelta,
            WorthQueryGraphReadAccessComplexityContract::LifecycleSupportAdmission,
            WorthQueryGraphReadAccessMemoryEstimateBasis::LifecycleManagedSupport,
        )
        .with_operation_capability_requirement(requirement)],
        worth_foundational::facade::CanonicalDigestWorkBudget::new(64, 16 * 1024)
            .expect("the domain-operation capability canonical budget is nonzero"),
        worth_query_installation::facade::WorthQueryCanonicalWorkEvidence::zero(),
    )
    .expect("domain-operation capability requirements fit their canonical budget")
}

pub(super) fn explain_boolean_selectivity_shape_for_access_shape(
    read_graph: &WorthQueryReadGraph,
    access_shape: WorthQueryGraphReadAccessShape,
) -> Result<WorthQueryBooleanSelectivityShape, WorthQueryGraphReadAccessShapeExplanationError> {
    let expression = admit_boolean_predicate_expression_for_read_graph(
        read_graph,
        access_shape.operation_resolution().references(),
    )
    .map_err(WorthQueryGraphReadAccessShapeExplanationError::BooleanExpressionAdmission)?;
    Ok(normalize_boolean_selectivity_for_access_shape(
        access_shape,
        expression,
    ))
}

pub(super) fn explain_graph_read_access_shape_for_graph(
    family_digest: &str,
    read_graph: &WorthQueryReadGraph,
    authority: &WorthQueryGraphReadAccessAuthorityContext,
    registry: &impl WorthQueryGraphReadOperationLookup,
) -> Result<WorthQueryGraphReadAccessShapeExplanation, WorthQueryGraphReadAccessShapeExplanationError>
{
    let operation_resolution = match resolve_graph_read_operations_for_graph(read_graph, authority, registry)? {
        WorthQueryGraphReadOperationOutcome::Resolved(resolution) => resolution,
        WorthQueryGraphReadOperationOutcome::RequiresAccessCapabilityRegistration(requirement) => {
            return Err(
                WorthQueryGraphReadAccessShapeExplanationError::OperationRequiresAccessCapabilityRegistration(
                    requirement,
                ),
            )
        }
        WorthQueryGraphReadOperationOutcome::DeniedUnsupportedShape(denial) => {
            return Err(
                WorthQueryGraphReadAccessShapeExplanationError::OperationUnsupportedShape(denial),
            )
        }
    };
    let access_shape = derive_graph_read_access_shape(operation_resolution);
    Ok(WorthQueryGraphReadAccessShapeExplanation::from_shape(
        family_digest,
        access_shape,
    ))
}

fn resolve_graph_read_operations_for_graph(
    read_graph: &WorthQueryReadGraph,
    authority: &WorthQueryGraphReadAccessAuthorityContext,
    registry: &impl WorthQueryGraphReadOperationLookup,
) -> Result<WorthQueryGraphReadOperationOutcome, WorthQueryGraphReadAccessShapeExplanationError> {
    let references = admit_query_schema_references_for_read_graph(read_graph)
        .map_err(WorthQueryGraphReadAccessShapeExplanationError::SchemaReferenceAdmission)?;
    let (basis_binding, policy_tenant_proof_binding) = authority.bind_for_read_graph(read_graph);
    Ok(resolve_graph_read_operations_for_read_graph(
        read_graph,
        references,
        basis_binding,
        policy_tenant_proof_binding,
        registry,
    ))
}
