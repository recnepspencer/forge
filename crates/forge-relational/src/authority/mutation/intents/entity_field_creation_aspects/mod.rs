mod authoritative_state_admission;
mod contract_validation;
mod creation_authoritative_patch;
mod field_classification;

use forge_foundational::facade::AuthoritativeRecordAspectPatch;

use crate::identity::data::KindId;
use crate::schema::data::LoweredAspectContractPlan;
use crate::storage::logic::state::EntityExtra;
use crate::transactions::data::{AspectFieldPatch, EntityAuthoritativeAspectStateDenial};

use self::authoritative_state_admission::admit_entity_creation_state;
use self::contract_validation::validate_entity_creation_fields;
use self::creation_authoritative_patch::construct_creation_patch;

#[derive(Debug, Clone)]
pub(super) struct EntityFieldCreationAspectPlan {
    pub(super) authoritative_patch: Option<AuthoritativeRecordAspectPatch>,
    pub(super) extra: EntityExtra,
}

pub(super) fn plan_entity_field_creation_aspects(
    kind_id: KindId,
    lowered_plan: Option<&LoweredAspectContractPlan>,
    fields: &AspectFieldPatch,
) -> Result<EntityFieldCreationAspectPlan, EntityAuthoritativeAspectStateDenial> {
    let Some(lowered_plan) = lowered_plan else {
        return plan_creation_without_aspects(kind_id, fields);
    };
    let validated_artifacts = validate_entity_creation_fields(lowered_plan, fields)?;
    let authoritative_state = admit_entity_creation_state(validated_artifacts.clone())?;
    let authoritative_patch = construct_creation_patch(validated_artifacts)?;
    let mut extra = EntityExtra::default();
    extra.authoritative_aspect_state = authoritative_state;

    Ok(EntityFieldCreationAspectPlan {
        authoritative_patch,
        extra,
    })
}

fn plan_creation_without_aspects(
    kind_id: KindId,
    fields: &AspectFieldPatch,
) -> Result<EntityFieldCreationAspectPlan, EntityAuthoritativeAspectStateDenial> {
    if !fields.is_empty() {
        return Err(EntityAuthoritativeAspectStateDenial::MissingAspectPlan { kind_id });
    }
    Ok(EntityFieldCreationAspectPlan {
        authoritative_patch: None,
        extra: EntityExtra::default(),
    })
}
