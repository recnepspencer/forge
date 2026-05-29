use forge_foundational::facade::{
    AspectLocator, AspectShape, AspectValue, BoundarySourceLocator, FieldKey, LocatorAuthority,
};

use crate::schema::data::{
    LoweredAspectBinding, LoweredAspectPlan, LoweredExecutableAspectBindingKind,
};
use crate::transactions::data::{AspectFieldPatchTarget, EntityAuthoritativeAspectStateDenial};

pub(super) enum EntityCreationFieldTarget<'a> {
    Scalar(&'a LoweredAspectBinding),
    StructField {
        binding_index: usize,
        field: FieldKey,
    },
}

pub(super) fn resolve_creation_field_target<'a>(
    lowered_plan: &'a LoweredAspectPlan,
    target: &AspectFieldPatchTarget,
) -> Result<EntityCreationFieldTarget<'a>, EntityAuthoritativeAspectStateDenial> {
    let Some(field) = single_creation_field(target) else {
        return Err(unsupported_target(target, "nested field paths"));
    };
    let Some((binding_index, binding)) = lowered_plan
        .executable_bindings
        .iter()
        .enumerate()
        .find(|(_, binding)| binding.contract.key() == target.aspect_key())
    else {
        return Err(unsupported_target(target, "undeclared entity aspect"));
    };

    match &binding.binding_kind {
        LoweredExecutableAspectBindingKind::EntityFieldScalar { field: declared }
            if declared == field =>
        {
            Ok(EntityCreationFieldTarget::Scalar(binding))
        }
        LoweredExecutableAspectBindingKind::EntityFieldStruct { .. }
            if struct_contract_declares_field(binding, field) =>
        {
            Ok(EntityCreationFieldTarget::StructField {
                binding_index,
                field: field.clone(),
            })
        }
        _ => Err(unsupported_target(
            target,
            "field path not admitted by entity aspect binding",
        )),
    }
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

fn struct_contract_declares_field(binding: &LoweredAspectBinding, field: &FieldKey) -> bool {
    if !matches!(
        &binding.binding_kind,
        LoweredExecutableAspectBindingKind::EntityFieldStruct { .. }
    ) {
        return false;
    }
    let AspectShape::Struct(shape) = binding.contract.shape() else {
        return false;
    };
    shape.field(field).is_some()
}

fn unsupported_target(
    target: &AspectFieldPatchTarget,
    value_family: impl Into<String>,
) -> EntityAuthoritativeAspectStateDenial {
    EntityAuthoritativeAspectStateDenial::UnsupportedAspectValue {
        source_locator: source_locator_for_target(target),
        value_family: value_family.into(),
    }
}

pub(super) type StructCreationFieldSet = Vec<(FieldKey, AspectValue)>;
