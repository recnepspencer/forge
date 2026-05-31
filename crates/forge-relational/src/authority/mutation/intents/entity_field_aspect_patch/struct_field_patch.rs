use std::collections::BTreeMap;

use forge_foundational::facade::{
    AspectFieldLocator, AspectMask, AspectValue, AuthoritativeRecordAspectPatch,
    CanonicalFieldPath, FieldKey, LocatorAuthority, MutationMask,
};
use forge_proof::TransitionOutcome;

use crate::schema::data::LoweredAspectContractPlan;
use crate::transactions::data::EntityFieldAspectPatchDenial;

pub(super) fn combine_struct_field_patches(
    mut authoritative_patch: AuthoritativeRecordAspectPatch,
    lowered_plan: &LoweredAspectContractPlan,
    struct_field_sets: BTreeMap<usize, Vec<(FieldKey, AspectValue)>>,
) -> Result<AuthoritativeRecordAspectPatch, EntityFieldAspectPatchDenial> {
    for (binding_index, field_sets) in struct_field_sets {
        let binding = &lowered_plan.executable_bindings[binding_index];
        let mutation_mask = mutation_mask_for_field_sets(&field_sets);
        let field_patch = construct_struct_field_patch(
            &binding.contract,
            binding.aspect_key(),
            &mutation_mask,
            field_sets,
        )?;
        authoritative_patch = combine_authoritative_patch(authoritative_patch, field_patch)?;
    }

    Ok(authoritative_patch)
}

fn mutation_mask_for_field_sets(
    field_sets: &[(FieldKey, AspectValue)],
) -> AspectMask<MutationMask> {
    AspectMask::<MutationMask>::new(
        field_sets
            .iter()
            .map(|(field, _)| CanonicalFieldPath::single(field.clone())),
    )
}

fn construct_struct_field_patch(
    contract: &forge_foundational::facade::AspectContract,
    aspect_key: &forge_foundational::facade::AspectKey,
    mutation_mask: &AspectMask<MutationMask>,
    field_sets: Vec<(FieldKey, AspectValue)>,
) -> Result<AuthoritativeRecordAspectPatch, EntityFieldAspectPatchDenial> {
    let mut builder = forge_foundational::facade::aspects()
        .patch()
        .field_level(contract, mutation_mask);
    for (field, value) in field_sets {
        builder = builder.set_field(field, value);
    }

    match builder.finish() {
        TransitionOutcome::Success(patch) => Ok(patch),
        TransitionOutcome::Denied(denial) => {
            Err(EntityFieldAspectPatchDenial::PatchConstructionDenied {
                field_locator: first_mask_field_path(mutation_mask).map(|field_path| {
                    AspectFieldLocator::new(
                        LocatorAuthority::Planned,
                        aspect_key.clone(),
                        field_path,
                    )
                }),
                denial,
            })
        }
    }
}

fn combine_authoritative_patch(
    left: AuthoritativeRecordAspectPatch,
    right: AuthoritativeRecordAspectPatch,
) -> Result<AuthoritativeRecordAspectPatch, EntityFieldAspectPatchDenial> {
    match AuthoritativeRecordAspectPatch::combine(left, right) {
        TransitionOutcome::Success(patch) => Ok(patch),
        TransitionOutcome::Denied(denial) => {
            Err(EntityFieldAspectPatchDenial::PatchConstructionDenied {
                field_locator: None,
                denial,
            })
        }
    }
}

fn first_mask_field_path(mask: &AspectMask<MutationMask>) -> Option<CanonicalFieldPath> {
    mask.paths().iter().next().cloned()
}
