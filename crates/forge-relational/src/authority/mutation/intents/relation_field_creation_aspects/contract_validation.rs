use forge_foundational::facade::{
    aspects, validate_aspect_value, AspectValue, ContractValidatedAspectArtifact,
    ContractValidationInput, FieldKey,
};
use forge_proof::TransitionOutcome;
use std::collections::BTreeMap;

use crate::identity::data::EntityId;
use crate::schema::data::{AspectBinding, LoweredAspectBinding, LoweredAspectPlan};
use crate::transactions::data::{
    AspectFieldPatch, AspectFieldPatchTarget, RelationAuthoritativeAspectStateDenial,
};

use super::field_classification::{
    resolve_creation_field_target, source_locator_for_aspect_binding, source_locator_for_target,
    RelationCreationFieldTarget, StructCreationFieldSet,
};

pub(super) fn validate_relation_creation_aspects(
    lowered_plan: &LoweredAspectPlan,
    fields: &AspectFieldPatch,
    source: EntityId,
    target: EntityId,
) -> Result<Vec<ContractValidatedAspectArtifact>, RelationAuthoritativeAspectStateDenial> {
    let mut scalar_artifacts = Vec::new();
    let mut struct_field_sets = BTreeMap::<usize, StructCreationFieldSet>::new();

    for (target, value) in fields.iter() {
        match resolve_creation_field_target(lowered_plan, target)? {
            RelationCreationFieldTarget::Scalar(binding) => {
                scalar_artifacts.push(validate_scalar_creation_value(binding, target, value)?);
            }
            RelationCreationFieldTarget::StructField {
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
    validated_artifacts.extend(validate_endpoint_identity_aspects(
        lowered_plan,
        source,
        target,
    )?);

    Ok(validated_artifacts)
}

fn validate_scalar_creation_value(
    binding: &LoweredAspectBinding,
    target: &AspectFieldPatchTarget,
    value: &AspectValue,
) -> Result<ContractValidatedAspectArtifact, RelationAuthoritativeAspectStateDenial> {
    match validate_aspect_value(
        &binding.contract,
        ContractValidationInput::Scalar(value.clone()),
    ) {
        TransitionOutcome::Success(artifact) => Ok(artifact),
        TransitionOutcome::Denied(denial) => Err(
            RelationAuthoritativeAspectStateDenial::ContractValidationDenied {
                source_locator: source_locator_for_target(target),
                denial,
            },
        ),
    }
}

fn validate_struct_creation_value(
    binding: &LoweredAspectBinding,
    field_sets: Vec<(FieldKey, AspectValue)>,
) -> Result<ContractValidatedAspectArtifact, RelationAuthoritativeAspectStateDenial> {
    let source_locator = source_locator_for_aspect_binding(binding);
    let mut builder = aspects().vocabulary().struct_value();
    for (field, value) in field_sets {
        builder = builder.with_field(field.as_str(), value);
    }
    let struct_value = builder.finish().map_err(|_| {
        RelationAuthoritativeAspectStateDenial::StructValueConstructionDenied {
            source_locator: source_locator.clone(),
        }
    })?;

    match validate_aspect_value(
        &binding.contract,
        ContractValidationInput::Struct(struct_value),
    ) {
        TransitionOutcome::Success(artifact) => Ok(artifact),
        TransitionOutcome::Denied(denial) => Err(
            RelationAuthoritativeAspectStateDenial::StructContractValidationDenied {
                source_locator,
                denial,
            },
        ),
    }
}

pub(super) fn validate_endpoint_identity_aspects(
    lowered_plan: &LoweredAspectPlan,
    source: EntityId,
    target: EntityId,
) -> Result<Vec<ContractValidatedAspectArtifact>, RelationAuthoritativeAspectStateDenial> {
    let mut endpoint_artifacts = Vec::new();
    for binding in &lowered_plan.executable_bindings {
        match &binding.target {
            AspectBinding::RelationSourceEndpoint => {
                endpoint_artifacts.push(validate_endpoint_identity(binding, source)?);
            }
            AspectBinding::RelationTargetEndpoint => {
                endpoint_artifacts.push(validate_endpoint_identity(binding, target)?);
            }
            _ => {}
        }
    }
    Ok(endpoint_artifacts)
}

fn validate_endpoint_identity(
    binding: &LoweredAspectBinding,
    endpoint: EntityId,
) -> Result<ContractValidatedAspectArtifact, RelationAuthoritativeAspectStateDenial> {
    let value = AspectValue::EntityRef(foundational_entity_id(endpoint));
    match validate_aspect_value(&binding.contract, ContractValidationInput::Scalar(value)) {
        TransitionOutcome::Success(artifact) => Ok(artifact),
        TransitionOutcome::Denied(denial) => Err(
            RelationAuthoritativeAspectStateDenial::ContractValidationDenied {
                source_locator: source_locator_for_aspect_binding(binding),
                denial,
            },
        ),
    }
}

fn foundational_entity_id(entity_id: EntityId) -> forge_foundational::facade::EntityId {
    forge_foundational::facade::EntityId {
        partition_id: forge_foundational::facade::PartitionId(entity_id.partition_id.as_u32()),
        local_slot: forge_foundational::facade::LocalSlot(entity_id.local_slot_value()),
        generation: forge_foundational::facade::Generation(entity_id.generation_value()),
    }
}
