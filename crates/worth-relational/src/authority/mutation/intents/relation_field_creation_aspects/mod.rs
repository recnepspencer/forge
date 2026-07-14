mod authoritative_state_admission;
mod contract_validation;
mod creation_authoritative_patch;
mod endpoint_update_authoritative_state;
mod field_classification;

use worth_foundational::facade::AuthoritativeRecordAspectPatch;

use crate::identity::data::{EntityId, KindId};
use crate::schema::data::LoweredAspectContractPlan;
use crate::storage::logic::state::RelationExtra;
use crate::transactions::data::{AspectFieldPatch, RelationAuthoritativeAspectStateDenial};

use self::authoritative_state_admission::admit_relation_creation_state;
use self::contract_validation::validate_relation_creation_aspects;
use self::creation_authoritative_patch::construct_creation_patch;
pub(super) use self::endpoint_update_authoritative_state::apply_relation_endpoint_update_aspects;

#[derive(Debug, Clone)]
pub(super) struct RelationFieldCreationAspectPlan {
    pub(super) authoritative_patch: Option<AuthoritativeRecordAspectPatch>,
    pub(super) extra: RelationExtra,
}

pub(super) fn plan_relation_field_creation_aspects(
    kind_id: KindId,
    lowered_plan: Option<&LoweredAspectContractPlan>,
    fields: &AspectFieldPatch,
    source: EntityId,
    target: EntityId,
) -> Result<RelationFieldCreationAspectPlan, RelationAuthoritativeAspectStateDenial> {
    let Some(lowered_plan) = lowered_plan else {
        return plan_creation_without_aspects(kind_id, fields, source, target);
    };
    let validated_artifacts =
        validate_relation_creation_aspects(lowered_plan, fields, source, target)?;
    let authoritative_state = admit_relation_creation_state(validated_artifacts.clone())?;
    let authoritative_patch = construct_creation_patch(validated_artifacts)?;
    let mut extra = RelationExtra::default();
    extra.endpoints = Some(crate::storage::logic::state::RelationEndpoints { source, target });
    extra.authoritative_aspect_state = authoritative_state;

    Ok(RelationFieldCreationAspectPlan {
        authoritative_patch,
        extra,
    })
}

fn plan_creation_without_aspects(
    kind_id: KindId,
    fields: &AspectFieldPatch,
    source: EntityId,
    target: EntityId,
) -> Result<RelationFieldCreationAspectPlan, RelationAuthoritativeAspectStateDenial> {
    if !fields.is_empty() {
        return Err(RelationAuthoritativeAspectStateDenial::MissingAspectPlan { kind_id });
    }
    let mut extra = RelationExtra::default();
    extra.endpoints = Some(crate::storage::logic::state::RelationEndpoints { source, target });
    Ok(RelationFieldCreationAspectPlan {
        authoritative_patch: None,
        extra,
    })
}
