use std::collections::BTreeMap;

use forge_foundational::facade::AuthoritativeRecordAspectPatch;

use crate::identity::data::KindId;
use crate::schema::data::LoweredAspectContractPlan;
use crate::transactions::data::{AspectFieldPatch, EntityFieldAspectPatchDenial};

use self::scalar_whole_aspect_patch::{
    construct_scalar_whole_aspect_patch, validate_entity_scalar_field_value,
};
use self::struct_field_patch::combine_struct_field_patches;
use self::target_resolution::{resolve_entity_field_patch_target, EntityFieldPatchTarget};
use super::struct_field_value_set::StructFieldValueSet;

mod scalar_whole_aspect_patch;
mod struct_field_patch;
mod target_resolution;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EntityFieldAspectPatchPlan {
    pub(super) authoritative_patch: AuthoritativeRecordAspectPatch,
}

pub(crate) fn plan_entity_field_aspect_patch(
    kind_id: KindId,
    lowered_plan: Option<&LoweredAspectContractPlan>,
    field_patch: &AspectFieldPatch,
) -> Result<EntityFieldAspectPatchPlan, EntityFieldAspectPatchDenial> {
    reject_empty_field_patch_plan(field_patch)?;
    let lowered_plan =
        lowered_plan.ok_or(EntityFieldAspectPatchDenial::MissingAspectPlan { kind_id })?;
    let planned_updates = classify_entity_field_patch_targets(lowered_plan, field_patch)?;
    let authoritative_patch =
        build_authoritative_entity_field_patch(lowered_plan, planned_updates)?;

    Ok(EntityFieldAspectPatchPlan {
        authoritative_patch,
    })
}

struct PlannedEntityFieldUpdates {
    scalar_sets: Vec<forge_foundational::facade::ContractValidatedAspectArtifact>,
    struct_field_sets: BTreeMap<usize, StructFieldValueSet>,
}

fn reject_empty_field_patch_plan(
    field_patch: &AspectFieldPatch,
) -> Result<(), EntityFieldAspectPatchDenial> {
    if field_patch.is_empty() {
        return Err(EntityFieldAspectPatchDenial::EmptyAuthoritativePatchPlan);
    }
    Ok(())
}

fn classify_entity_field_patch_targets(
    lowered_plan: &LoweredAspectContractPlan,
    field_patch: &AspectFieldPatch,
) -> Result<PlannedEntityFieldUpdates, EntityFieldAspectPatchDenial> {
    let mut planned_updates = PlannedEntityFieldUpdates {
        scalar_sets: Vec::new(),
        struct_field_sets: BTreeMap::new(),
    };

    for (target, value) in field_patch.iter() {
        match resolve_entity_field_patch_target(lowered_plan, target)? {
            EntityFieldPatchTarget::Scalar(binding) => {
                let validated = validate_entity_scalar_field_value(
                    target.field_path(),
                    binding,
                    value.clone(),
                )?;
                planned_updates.scalar_sets.push(validated);
            }
            EntityFieldPatchTarget::StructField {
                binding_index,
                field,
            } => {
                planned_updates
                    .struct_field_sets
                    .entry(binding_index)
                    .or_default()
                    .push(field, value.clone());
            }
        }
    }

    Ok(planned_updates)
}

fn build_authoritative_entity_field_patch(
    lowered_plan: &LoweredAspectContractPlan,
    planned_updates: PlannedEntityFieldUpdates,
) -> Result<AuthoritativeRecordAspectPatch, EntityFieldAspectPatchDenial> {
    let authoritative_patch = construct_scalar_whole_aspect_patch(planned_updates.scalar_sets)?;
    combine_struct_field_patches(
        authoritative_patch,
        lowered_plan,
        planned_updates.struct_field_sets,
    )
}
