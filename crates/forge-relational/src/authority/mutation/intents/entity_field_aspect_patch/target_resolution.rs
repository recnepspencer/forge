use forge_foundational::facade::{AspectFieldLocator, CanonicalFieldPath, FieldKey};

use crate::schema::data::{LoweredAspectBinding, LoweredAspectPlan};
use crate::transactions::data::EntityFieldAspectPatchDenial;

pub(super) enum EntityFieldPatchTarget<'a> {
    Scalar(&'a LoweredAspectBinding),
    StructField {
        binding_index: usize,
        field: FieldKey,
    },
}

pub(super) fn resolve_entity_field_patch_target<'a>(
    lowered_plan: &'a LoweredAspectPlan,
    target: &AspectFieldLocator,
) -> Result<EntityFieldPatchTarget<'a>, EntityFieldAspectPatchDenial> {
    let field_key = single_field_path_key(target.field_path())?;
    let Some((binding_index, binding)) = lowered_plan
        .executable_bindings
        .iter()
        .enumerate()
        .find(|(_, binding)| binding.contract.key() == target.aspect().aspect_key())
    else {
        return Err(EntityFieldAspectPatchDenial::UndeclaredEntityAspectTarget {
            field_locator: target.clone(),
        });
    };

    if binding.targets_entity_scalar_field(field_key) {
        return Ok(EntityFieldPatchTarget::Scalar(binding));
    }
    if binding.targets_entity_struct_field(field_key) {
        return Ok(EntityFieldPatchTarget::StructField {
            binding_index,
            field: field_key.clone(),
        });
    }
    Err(
        EntityFieldAspectPatchDenial::EntityAspectFieldPathMismatch {
            field_locator: target.clone(),
        },
    )
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
