use forge_foundational::facade::{
    AspectFieldLocator, AspectLocator, AspectValue, BoundarySourceLocator, FieldKey,
    LocatorAuthority,
};

use crate::schema::data::{LoweredAspectBinding, LoweredAspectPlan};
use crate::transactions::data::{
    AspectFieldTargetRejectionReason, EntityAuthoritativeAspectStateDenial,
};

pub(super) enum EntityCreationFieldTarget<'a> {
    Scalar(&'a LoweredAspectBinding),
    StructField {
        binding_index: usize,
        field: FieldKey,
    },
}

pub(super) fn resolve_creation_field_target<'a>(
    lowered_plan: &'a LoweredAspectPlan,
    target: &AspectFieldLocator,
) -> Result<EntityCreationFieldTarget<'a>, EntityAuthoritativeAspectStateDenial> {
    let Some(field) = single_creation_field(target) else {
        return Err(unsupported_target(
            target,
            AspectFieldTargetRejectionReason::NestedFieldPath,
        ));
    };
    let Some((binding_index, binding)) = lowered_plan
        .executable_bindings
        .iter()
        .enumerate()
        .find(|(_, binding)| binding.contract.key() == target.aspect().aspect_key())
    else {
        return Err(unsupported_target(
            target,
            AspectFieldTargetRejectionReason::UndeclaredAspect,
        ));
    };

    if binding.targets_entity_scalar_field(field) {
        return Ok(EntityCreationFieldTarget::Scalar(binding));
    }
    if binding.targets_entity_struct_field(field) {
        return Ok(EntityCreationFieldTarget::StructField {
            binding_index,
            field: field.clone(),
        });
    }
    Err(unsupported_target(
        target,
        AspectFieldTargetRejectionReason::FieldPathNotAdmittedByAspectBinding,
    ))
}

pub(super) fn source_locator_for_target(target: &AspectFieldLocator) -> BoundarySourceLocator {
    BoundarySourceLocator::aspect_field(target.clone())
}

pub(super) fn source_locator_for_aspect_binding(
    binding: &LoweredAspectBinding,
) -> BoundarySourceLocator {
    BoundarySourceLocator::aspect(AspectLocator::new(
        LocatorAuthority::Planned,
        binding.aspect_key().clone(),
    ))
}

fn single_creation_field(target: &AspectFieldLocator) -> Option<&FieldKey> {
    match target.field_path().fields() {
        [field] => Some(field),
        _ => None,
    }
}

fn unsupported_target(
    target: &AspectFieldLocator,
    reason: AspectFieldTargetRejectionReason,
) -> EntityAuthoritativeAspectStateDenial {
    EntityAuthoritativeAspectStateDenial::UnsupportedAspectFieldTarget {
        target: target.clone(),
        reason,
    }
}

pub(super) type StructCreationFieldSet = Vec<(FieldKey, AspectValue)>;
