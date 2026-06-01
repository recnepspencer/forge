use forge_foundational::facade::{AspectFieldLocator, FieldKey};

use crate::schema::data::{LoweredAspectContractBinding, LoweredAspectContractPlan};
use crate::transactions::data::EntityFieldAspectPatchDenial;

pub(super) enum EntityFieldPatchTarget<'a> {
    Scalar(&'a LoweredAspectContractBinding),
    StructField {
        binding_index: usize,
        field: FieldKey,
    },
}

pub(super) fn resolve_entity_field_patch_target<'a>(
    lowered_plan: &'a LoweredAspectContractPlan,
    target: &AspectFieldLocator,
) -> Result<EntityFieldPatchTarget<'a>, EntityFieldAspectPatchDenial> {
    let field_key = single_field_path_key(target)?;
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
    target: &AspectFieldLocator,
) -> Result<&FieldKey, EntityFieldAspectPatchDenial> {
    match target.field_path().fields() {
        [field] => Ok(field),
        _ => Err(
            EntityFieldAspectPatchDenial::UnsupportedNestedEntityFieldPath {
                field_locator: target.clone(),
            },
        ),
    }
}

#[cfg(test)]
mod tests {
    use forge_foundational::facade::{
        AspectFieldLocator, AspectKey, CanonicalFieldPath, FieldKey, LocatorAuthority,
    };

    use crate::identity::data::KindId;
    use crate::schema::data::{AspectContractPlanRevision, LoweredAspectContractPlan};
    use crate::transactions::data::EntityFieldAspectPatchDenial;

    use super::resolve_entity_field_patch_target;

    #[test]
    fn nested_entity_field_patch_denial_preserves_full_aspect_field_locator() {
        let nested_locator = AspectFieldLocator::new(
            LocatorAuthority::Planned,
            AspectKey::new("summary").expect("valid aspect key"),
            CanonicalFieldPath::new([
                FieldKey::new("title").expect("valid field key"),
                FieldKey::new("locale").expect("valid field key"),
            ])
            .expect("valid nested field path"),
        );
        let lowered_plan = LoweredAspectContractPlan {
            kind_id: KindId(1),
            plan_revision: AspectContractPlanRevision(1),
            executable_bindings: smallvec::smallvec![],
        };

        let error = match resolve_entity_field_patch_target(&lowered_plan, &nested_locator) {
            Ok(_) => panic!("nested path must be rejected before aspect lookup"),
            Err(error) => error,
        };

        assert!(matches!(
            error,
            EntityFieldAspectPatchDenial::UnsupportedNestedEntityFieldPath {
                field_locator
            } if field_locator == nested_locator
        ));
    }
}
