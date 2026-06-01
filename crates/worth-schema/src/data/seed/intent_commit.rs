use forge_relational::facade::history::BranchId;
use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::transactions::TransactionCommitError;

use crate::data::authority::{RawTopologyIntent, TopologyAuthority, TopologyAuthorityError};

use super::SeededTopologyCommit;

#[derive(Debug)]
pub enum TopologyIntentCommitError {
    DuplicateCreateKey(crate::data::authority::CreateKey),
    DuplicateLiveEntityLabel(crate::data::authority::CreateKey),
    MissingCreatedEntity(crate::data::authority::CreateKey),
    UnsupportedIdentityEntityMutation(forge_relational::facade::identity::EntityId),
    UnsupportedIdentityRelationMutation(forge_relational::facade::identity::RelationId),
    MissingEntity(forge_relational::facade::identity::EntityId),
    MissingRelation(forge_relational::facade::identity::RelationId),
    EntityKindMismatch {
        entity_id: forge_relational::facade::identity::EntityId,
        expected: crate::data::entities::EntityKind,
        found: crate::data::entities::EntityKind,
    },
    RelationShapeMismatch {
        relation_id: forge_relational::facade::identity::RelationId,
        expected_kind: crate::data::relations::RelationKind,
        found_kind: crate::data::relations::RelationKind,
        expected_source: forge_relational::facade::identity::EntityId,
        found_source: forge_relational::facade::identity::EntityId,
        expected_target: forge_relational::facade::identity::EntityId,
        found_target: forge_relational::facade::identity::EntityId,
    },
    ReadSnapshot(String),
    Commit(TransactionCommitError),
}

impl From<TopologyAuthorityError> for TopologyIntentCommitError {
    fn from(value: TopologyAuthorityError) -> Self {
        match value {
            TopologyAuthorityError::DuplicateCreateKey(key) => Self::DuplicateCreateKey(key),
            TopologyAuthorityError::DuplicateLiveEntityLabel(key) => {
                Self::DuplicateLiveEntityLabel(key)
            }
            TopologyAuthorityError::MissingCreatedEntity(key) => Self::MissingCreatedEntity(key),
            TopologyAuthorityError::UnsupportedIdentityEntityMutation(entity_id) => {
                Self::UnsupportedIdentityEntityMutation(entity_id)
            }
            TopologyAuthorityError::UnsupportedIdentityRelationMutation(relation_id) => {
                Self::UnsupportedIdentityRelationMutation(relation_id)
            }
            TopologyAuthorityError::MissingEntity(entity_id) => Self::MissingEntity(entity_id),
            TopologyAuthorityError::MissingRelation(relation_id) => {
                Self::MissingRelation(relation_id)
            }
            TopologyAuthorityError::EntityKindMismatch {
                entity_id,
                expected,
                found,
            } => Self::EntityKindMismatch {
                entity_id,
                expected,
                found,
            },
            TopologyAuthorityError::RelationShapeMismatch {
                relation_id,
                expected_kind,
                found_kind,
                expected_source,
                found_source,
                expected_target,
                found_target,
            } => Self::RelationShapeMismatch {
                relation_id,
                expected_kind,
                found_kind,
                expected_source,
                found_source,
                expected_target,
                found_target,
            },
            TopologyAuthorityError::ReadSnapshot(message) => Self::ReadSnapshot(message),
            TopologyAuthorityError::Commit(error) => Self::Commit(error),
        }
    }
}

impl std::fmt::Display for TopologyIntentCommitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateCreateKey(key) => write!(f, "duplicate create key `{}`", key.as_str()),
            Self::DuplicateLiveEntityLabel(key) => {
                write!(f, "duplicate live entity label `{}`", key.as_str())
            }
            Self::MissingCreatedEntity(key) => {
                write!(f, "missing created entity reference `{}`", key.as_str())
            }
            Self::UnsupportedIdentityEntityMutation(entity_id) => {
                write!(f, "unsupported identity entity mutation for `{entity_id:?}`")
            }
            Self::UnsupportedIdentityRelationMutation(relation_id) => {
                write!(f, "unsupported identity relation mutation for `{relation_id:?}`")
            }
            Self::MissingEntity(entity_id) => write!(f, "missing entity `{entity_id:?}`"),
            Self::MissingRelation(relation_id) => write!(f, "missing relation `{relation_id:?}`"),
            Self::EntityKindMismatch {
                entity_id,
                expected,
                found,
            } => write!(
                f,
                "entity kind mismatch for `{entity_id:?}`: expected `{expected:?}`, found `{found:?}`"
            ),
            Self::RelationShapeMismatch {
                relation_id,
                expected_kind,
                found_kind,
                expected_source,
                found_source,
                expected_target,
                found_target,
            } => write!(
                f,
                "relation shape mismatch for `{relation_id:?}`: expected kind `{expected_kind:?}` source `{expected_source:?}` target `{expected_target:?}`, found kind `{found_kind:?}` source `{found_source:?}` target `{found_target:?}`"
            ),
            Self::ReadSnapshot(message) => f.write_str(message),
            Self::Commit(error) => write!(f, "{error:?}"),
        }
    }
}

impl std::error::Error for TopologyIntentCommitError {}

pub fn commit_topology_intent(
    runtime: &mut RelationalRuntime,
    intent: RawTopologyIntent,
) -> Result<SeededTopologyCommit, TopologyIntentCommitError> {
    let verified = TopologyAuthority::new(runtime)
        .apply_topology_intent_traced(intent)
        .map(|traced| traced.into_primary_result())
        .map_err(|failure| TopologyIntentCommitError::from(failure.into_error()))?;
    Ok(SeededTopologyCommit::from_verified_commit(verified))
}

pub fn commit_topology_intent_on_branch(
    runtime: &mut RelationalRuntime,
    intent: RawTopologyIntent,
    branch_id: BranchId,
) -> Result<SeededTopologyCommit, TopologyIntentCommitError> {
    let verified = TopologyAuthority::new(runtime)
        .apply_topology_intent_on_branch_traced(intent, branch_id)
        .map(|traced| traced.into_primary_result())
        .map_err(|failure| TopologyIntentCommitError::from(failure.into_error()))?;
    Ok(SeededTopologyCommit::from_verified_commit(verified))
}
