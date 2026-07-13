use std::collections::BTreeMap;
use worth_foundational::facade::{
    validate_aspect_value, AspectFieldLocator, AspectValue, ContractValidatedAspectArtifact,
    ContractValidationInput,
};
use worth_proof::TransitionOutcome;

use crate::schema::data::{LoweredAspectContractBinding, LoweredAspectContractPlan};
use crate::transactions::data::{AspectFieldPatch, EntityAuthoritativeAspectStateDenial};

use super::super::struct_field_value_set::StructFieldValueSet;
use super::field_classification::{
    resolve_creation_field_target, source_locator_for_aspect_binding, source_locator_for_target,
    EntityCreationFieldTarget,
};

pub(super) fn validate_entity_creation_fields(
    lowered_plan: &LoweredAspectContractPlan,
    fields: &AspectFieldPatch,
) -> Result<Vec<ContractValidatedAspectArtifact>, EntityAuthoritativeAspectStateDenial> {
    let mut scalar_artifacts = Vec::new();
    let mut struct_field_sets = BTreeMap::<usize, StructFieldValueSet>::new();

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
                    .push(field, value.clone());
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
    binding: &LoweredAspectContractBinding,
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
    binding: &LoweredAspectContractBinding,
    field_set: StructFieldValueSet,
) -> Result<ContractValidatedAspectArtifact, EntityAuthoritativeAspectStateDenial> {
    let source_locator = source_locator_for_aspect_binding(binding);
    let struct_value = field_set.into_struct_value().map_err(|_| {
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
