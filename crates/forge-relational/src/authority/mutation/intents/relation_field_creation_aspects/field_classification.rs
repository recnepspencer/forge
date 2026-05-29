use forge_foundational::facade::{
    AspectLocator, AspectValue, BoundarySourceLocator, FieldKey, LocatorAuthority,
};

use crate::schema::data::{LoweredAspectBinding, LoweredAspectPlan};
use crate::transactions::data::{AspectFieldPatchTarget, RelationAuthoritativeAspectStateDenial};

pub(super) enum RelationCreationFieldTarget<'a> {
    Scalar(&'a LoweredAspectBinding),
    StructField {
        binding_index: usize,
        field: FieldKey,
    },
}

pub(super) fn resolve_creation_field_target<'a>(
    lowered_plan: &'a LoweredAspectPlan,
    target: &AspectFieldPatchTarget,
) -> Result<RelationCreationFieldTarget<'a>, RelationAuthoritativeAspectStateDenial> {
    let Some(field) = single_creation_field(target) else {
        return Err(unsupported_target(target, "nested field paths"));
    };
    let Some((binding_index, binding)) = lowered_plan
        .executable_bindings
        .iter()
        .enumerate()
        .find(|(_, binding)| binding.contract.key() == target.aspect_key())
    else {
        return Err(unsupported_target(target, "undeclared relation aspect"));
    };

    if binding.targets_relation_scalar_field(field) {
        return Ok(RelationCreationFieldTarget::Scalar(binding));
    }
    if binding.targets_relation_struct_field(field) {
        return Ok(RelationCreationFieldTarget::StructField {
            binding_index,
            field: field.clone(),
        });
    }
    Err(unsupported_target(
        target,
        "field path not admitted by relation aspect binding",
    ))
}

pub(super) fn source_locator_for_target(target: &AspectFieldPatchTarget) -> BoundarySourceLocator {
    BoundarySourceLocator::aspect_field(target.locator().clone())
}

pub(super) fn source_locator_for_aspect_binding(
    binding: &LoweredAspectBinding,
) -> BoundarySourceLocator {
    BoundarySourceLocator::aspect(AspectLocator::new(
        LocatorAuthority::Planned,
        binding.aspect_key.clone(),
    ))
}

fn single_creation_field(target: &AspectFieldPatchTarget) -> Option<&FieldKey> {
    match target.field_path().fields() {
        [field] => Some(field),
        _ => None,
    }
}

fn unsupported_target(
    target: &AspectFieldPatchTarget,
    value_family: impl Into<String>,
) -> RelationAuthoritativeAspectStateDenial {
    RelationAuthoritativeAspectStateDenial::UnsupportedAspectValue {
        source_locator: source_locator_for_target(target),
        value_family: value_family.into(),
    }
}

pub(super) type StructCreationFieldSet = Vec<(FieldKey, AspectValue)>;
