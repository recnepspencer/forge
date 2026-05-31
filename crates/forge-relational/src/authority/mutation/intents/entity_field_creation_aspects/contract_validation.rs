use forge_foundational::facade::{
    aspects, validate_aspect_value, AspectFieldLocator, AspectValue,
    ContractValidatedAspectArtifact, ContractValidationInput, FieldKey,
};
use forge_proof::TransitionOutcome;
use std::collections::BTreeMap;

use crate::schema::data::{LoweredAspectBinding, LoweredAspectPlan};
use crate::transactions::data::{AspectFieldPatch, EntityAuthoritativeAspectStateDenial};

use super::field_classification::{
    resolve_creation_field_target, source_locator_for_aspect_binding, source_locator_for_target,
    EntityCreationFieldTarget, StructCreationFieldSet,
};

pub(super) fn validate_entity_creation_fields(
    lowered_plan: &LoweredAspectPlan,
    fields: &AspectFieldPatch,
) -> Result<Vec<ContractValidatedAspectArtifact>, EntityAuthoritativeAspectStateDenial> {
    let mut scalar_artifacts = Vec::new();
    let mut struct_field_sets = BTreeMap::<usize, StructCreationFieldSet>::new();

    for (target, value) in fields.iter() {
        match resolve_creation_field_target(lowered_plan, target)? {
            EntityCreationFieldTarget::Scalar(binding) => {
                scalar_artifacts.push(validate_scalar_creation_value(binding, target, value)?);
            }
            EntityCreationFieldTarget::StructField {
                binding_index,
                field,
            } => {
                struct_field_sets
                    .entry(binding_index)
                    .or_default()
                    .push((field, value.clone()));
            }
        }
    }

    let mut validated_artifacts = scalar_artifacts;
    for (binding_index, field_sets) in struct_field_sets {
        let binding = &lowered_plan.executable_bindings[binding_index];
        validated_artifacts.push(validate_struct_creation_value(binding, field_sets)?);
    }

    Ok(validated_artifacts)
}

fn validate_scalar_creation_value(
    binding: &LoweredAspectBinding,
    target: &AspectFieldLocator,
    value: &AspectValue,
) -> Result<ContractValidatedAspectArtifact, EntityAuthoritativeAspectStateDenial> {
    match validate_aspect_value(
        &binding.contract,
        ContractValidationInput::Scalar(value.clone()),
    ) {
        TransitionOutcome::Success(artifact) => Ok(artifact),
        TransitionOutcome::Denied(denial) => Err(
            EntityAuthoritativeAspectStateDenial::ContractValidationDenied {
                source_locator: source_locator_for_target(target),
                denial,
            },
        ),
    }
}

fn validate_struct_creation_value(
    binding: &LoweredAspectBinding,
    field_sets: Vec<(FieldKey, AspectValue)>,
) -> Result<ContractValidatedAspectArtifact, EntityAuthoritativeAspectStateDenial> {
    let source_locator = source_locator_for_aspect_binding(binding);
    let mut builder = aspects().vocabulary().struct_value();
    for (field, value) in field_sets {
        builder = builder.with_field(field.as_str(), value);
    }
    let struct_value = builder.finish().map_err(|_| {
        EntityAuthoritativeAspectStateDenial::StructValueConstructionDenied {
            source_locator: source_locator.clone(),
        }
    })?;

    match validate_aspect_value(
        &binding.contract,
        ContractValidationInput::Struct(struct_value),
    ) {
        TransitionOutcome::Success(artifact) => Ok(artifact),
        TransitionOutcome::Denied(denial) => Err(
            EntityAuthoritativeAspectStateDenial::StructContractValidationDenied {
                source_locator,
                denial,
            },
        ),
    }
}
