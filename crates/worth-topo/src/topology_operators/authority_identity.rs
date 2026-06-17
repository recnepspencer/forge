use forge_query::facade::{
    ForgeQueryContinuityPriorAuthorityLabel, ForgeQueryContinuitySuccessorAuthorityLabel,
    ForgeQueryExistingTruthBindingAuthorityLabel, ForgeQueryMutationAuthorityIdentity,
};
use forge_relational::facade::identity::{EntityId, RelationId};

use crate::topology_operators::application::TopologyMutationApplicationError;

pub(crate) fn existing_entity_authority(
    entity_id: EntityId,
) -> Result<ForgeQueryMutationAuthorityIdentity, TopologyMutationApplicationError> {
    existing_truth_authority(entity_authority_label(entity_id))
}

pub(crate) fn existing_relation_authority(
    relation_id: RelationId,
) -> Result<ForgeQueryMutationAuthorityIdentity, TopologyMutationApplicationError> {
    existing_truth_authority(relation_authority_label(relation_id))
}

pub(crate) fn continuity_prior_relation_authority(
    relation_id: RelationId,
) -> Result<ForgeQueryMutationAuthorityIdentity, TopologyMutationApplicationError> {
    ForgeQueryMutationAuthorityIdentity::continuity_prior_authority(
        ForgeQueryContinuityPriorAuthorityLabel::new(relation_authority_label(relation_id))?,
    )
    .map_err(Into::into)
}

pub(crate) fn continuity_successor_relation_authority(
    relation_id: RelationId,
) -> Result<ForgeQueryMutationAuthorityIdentity, TopologyMutationApplicationError> {
    ForgeQueryMutationAuthorityIdentity::continuity_successor_authority(
        ForgeQueryContinuitySuccessorAuthorityLabel::new(format!(
            "{}:successor",
            relation_authority_label(relation_id)
        ))?,
    )
    .map_err(Into::into)
}

pub(crate) fn relation_continuity_rebind_authorities(
    relation_id: RelationId,
) -> Result<
    (
        ForgeQueryMutationAuthorityIdentity,
        ForgeQueryMutationAuthorityIdentity,
    ),
    TopologyMutationApplicationError,
> {
    Ok((
        continuity_prior_relation_authority(relation_id)?,
        continuity_successor_relation_authority(relation_id)?,
    ))
}

fn existing_truth_authority(
    label: String,
) -> Result<ForgeQueryMutationAuthorityIdentity, TopologyMutationApplicationError> {
    ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(
        ForgeQueryExistingTruthBindingAuthorityLabel::new(label)?,
    )
    .map_err(Into::into)
}

fn entity_authority_label(entity_id: EntityId) -> String {
    format!(
        "entity:{}:{}:{}",
        entity_id.partition_id.0, entity_id.local_slot.0, entity_id.generation.0
    )
}

fn relation_authority_label(relation_id: RelationId) -> String {
    format!(
        "relation:{}:{}:{}",
        relation_id.partition_id.0, relation_id.local_slot.0, relation_id.generation.0
    )
}
