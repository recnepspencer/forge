use forge_foundational::facade::{AspectShape, CanonicalFieldPath, FieldKey};

use crate::schema::data::{
    LoweredAspectBinding, LoweredAspectPlan, LoweredExecutableAspectBindingKind,
};
use crate::transactions::data::{AspectFieldPatchTarget, EntityFieldAspectPatchDenial};

pub(super) enum EntityFieldPatchTarget<'a> {
    Scalar(&'a LoweredAspectBinding),
    StructField {
        binding_index: usize,
        field: FieldKey,
    },
}

pub(super) fn resolve_entity_field_patch_target<'a>(
    lowered_plan: &'a LoweredAspectPlan,
    target: &AspectFieldPatchTarget,
) -> Result<EntityFieldPatchTarget<'a>, EntityFieldAspectPatchDenial> {
    let field_key = single_field_path_key(target.field_path())?;
    let Some((binding_index, binding)) = lowered_plan
        .executable_bindings
        .iter()
        .enumerate()
        .find(|(_, binding)| binding.contract.key() == target.aspect_key())
    else {
        return Err(EntityFieldAspectPatchDenial::UndeclaredEntityAspectTarget {
            field_locator: target.locator().clone(),
        });
    };

    match &binding.binding_kind {
        LoweredExecutableAspectBindingKind::EntityFieldScalar { field } if field == field_key => {
            Ok(EntityFieldPatchTarget::Scalar(binding))
        }
        LoweredExecutableAspectBindingKind::EntityFieldStruct { .. }
            if entity_struct_contract_declares_field(binding, field_key) =>
        {
            Ok(EntityFieldPatchTarget::StructField {
                binding_index,
                field: field_key.clone(),
            })
        }
        _ => Err(
            EntityFieldAspectPatchDenial::EntityAspectFieldPathMismatch {
                field_locator: target.locator().clone(),
            },
        ),
    }
}

fn single_field_path_key(
    field_path: &CanonicalFieldPath,
) -> Result<&FieldKey, EntityFieldAspectPatchDenial> {
    match field_path.fields() {
        [field] => Ok(field),
        fields => Err(
            EntityFieldAspectPatchDenial::UnsupportedNestedEntityFieldPath {
                path: fields.to_vec(),
            },
        ),
    }
}

fn entity_struct_contract_declares_field(
    binding: &LoweredAspectBinding,
    field_key: &FieldKey,
) -> bool {
    if !matches!(
        &binding.binding_kind,
        LoweredExecutableAspectBindingKind::EntityFieldStruct { .. }
    ) {
        return false;
    }
    let AspectShape::Struct(shape) = binding.contract.shape() else {
        return false;
    };
    shape.field(field_key).is_some()
}
