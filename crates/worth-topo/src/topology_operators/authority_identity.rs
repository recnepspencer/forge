use forge_query::facade::{
    ForgeQueryContinuityPriorAuthorityLabel, ForgeQueryContinuitySuccessorAuthorityLabel,
    ForgeQueryExistingTruthBindingAuthorityLabel, ForgeQueryMutationAuthorityIdentity,
    ForgeQueryWorkspaceError,
};
use forge_relational::facade::identity::{EntityId, RelationId};

pub(crate) fn existing_entity_authority(
    entity_id: EntityId,
) -> Result<ForgeQueryMutationAuthorityIdentity, ForgeQueryWorkspaceError> {
    existing_truth_authority(format!("{entity_id:?}"))
}

pub(crate) fn existing_relation_authority(
    relation_id: RelationId,
) -> Result<ForgeQueryMutationAuthorityIdentity, ForgeQueryWorkspaceError> {
    existing_truth_authority(format!("{relation_id:?}"))
}

pub(crate) fn continuity_prior_relation_authority(
    relation_id: RelationId,
) -> Result<ForgeQueryMutationAuthorityIdentity, ForgeQueryWorkspaceError> {
    ForgeQueryMutationAuthorityIdentity::continuity_prior_authority(
        ForgeQueryContinuityPriorAuthorityLabel::new(format!("{relation_id:?}"))?,
    )
}

pub(crate) fn continuity_successor_relation_authority(
    relation_id: RelationId,
) -> Result<ForgeQueryMutationAuthorityIdentity, ForgeQueryWorkspaceError> {
    ForgeQueryMutationAuthorityIdentity::continuity_successor_authority(
        ForgeQueryContinuitySuccessorAuthorityLabel::new(format!("{relation_id:?}:successor"))?,
    )
}

fn existing_truth_authority(
    label: String,
) -> Result<ForgeQueryMutationAuthorityIdentity, ForgeQueryWorkspaceError> {
    ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(
        ForgeQueryExistingTruthBindingAuthorityLabel::new(label)?,
    )
}
