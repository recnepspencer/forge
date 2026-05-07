use forge_query::facade::{ForgeQueryRuntimeError, ForgeQueryWorkspaceError};
use forge_relational::facade::identity::{EntityId, RelationId};
use worth_schema::facade::{WorthTopologyEntityKind, WorthTopologyRelationKind};

use crate::query::WorthTopologyQuerySurfaceError;

use super::super::types::WorthTopologyEditFamily;
use super::WorthTopologyEditApplicationMode;

#[derive(Debug)]
pub enum WorthTopologyQueryEditExecutionError {
    UnsupportedMode(WorthTopologyEditApplicationMode),
    UnsupportedFamilies(Vec<WorthTopologyEditFamily>),
    MissingCreatedEntityReference(String),
    MissingExistingEntityBinding(EntityId),
    MissingExistingRelationBinding(RelationId),
    CreatedEntityKindMismatch {
        create_key: String,
        expected: WorthTopologyEntityKind,
        actual: WorthTopologyEntityKind,
    },
    ExistingEntityKindMismatch {
        entity_id: EntityId,
        expected: WorthTopologyEntityKind,
        actual: WorthTopologyEntityKind,
    },
    ExistingRelationKindMismatch {
        relation_id: RelationId,
        expected: WorthTopologyRelationKind,
        actual: WorthTopologyRelationKind,
    },
    ExistingRelationSourceMismatch {
        relation_id: RelationId,
        expected_source_entity_id: EntityId,
        actual_source_identity: String,
    },
    ExistingEntityOutgoingRelationCountMismatch {
        entity_id: EntityId,
        relation_kind: WorthTopologyRelationKind,
        expected: usize,
        actual: usize,
    },
    ExistingEntityIncomingRelationCountMismatch {
        entity_id: EntityId,
        relation_kind: WorthTopologyRelationKind,
        expected: usize,
        actual: usize,
    },
    ExistingHalfEdgesNotOnSameEdge {
        relation_id: RelationId,
        source_half_edge_id: EntityId,
        target_half_edge_id: EntityId,
        source_edge_identity: String,
        target_edge_identity: String,
    },
    ExistingHalfEdgesNotOnSameLoop {
        relation_id: RelationId,
        source_half_edge_id: EntityId,
        target_half_edge_id: EntityId,
        source_loop_identity: String,
        target_loop_identity: String,
    },
    Query(ForgeQueryRuntimeError),
    Surface(WorthTopologyQuerySurfaceError),
    MaterializedDecode(String),
    UnexpectedInspectionFamily,
}

impl std::fmt::Display for WorthTopologyQueryEditExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedMode(mode) => write!(
                f,
                "worth topology query edit execution does not admit mode `{mode:?}` yet"
            ),
            Self::UnsupportedFamilies(families) => write!(
                f,
                "worth topology query edit execution does not admit families `{families:?}` yet"
            ),
            Self::MissingCreatedEntityReference(create_key) => write!(
                f,
                "worth topology query edit execution is missing same-batch created entity `{create_key}`"
            ),
            Self::MissingExistingEntityBinding(entity_id) => write!(
                f,
                "worth topology query edit execution is missing live query binding for authoritative entity `{entity_id:?}`"
            ),
            Self::MissingExistingRelationBinding(relation_id) => write!(
                f,
                "worth topology query edit execution is missing live query binding for authoritative relation `{relation_id:?}`"
            ),
            Self::CreatedEntityKindMismatch {
                create_key,
                expected,
                actual,
            } => write!(
                f,
                "worth topology query edit execution expected created entity `{create_key}` to be `{}`, found `{}`",
                expected.kind_name(),
                actual.kind_name()
            ),
            Self::ExistingEntityKindMismatch {
                entity_id,
                expected,
                actual,
            } => write!(
                f,
                "worth topology query edit execution expected authoritative entity `{entity_id:?}` to be `{}`, found `{}`",
                expected.kind_name(),
                actual.kind_name()
            ),
            Self::ExistingRelationKindMismatch {
                relation_id,
                expected,
                actual,
            } => write!(
                f,
                "worth topology query edit execution expected authoritative relation `{relation_id:?}` to be `{}`, found `{}`",
                expected.kind_name(),
                actual.kind_name()
            ),
            Self::ExistingRelationSourceMismatch {
                relation_id,
                expected_source_entity_id,
                actual_source_identity,
            } => write!(
                f,
                "worth topology query edit execution expected authoritative relation `{relation_id:?}` to originate from halfedge `{expected_source_entity_id:?}`, found query source identity `{actual_source_identity}`"
            ),
            Self::ExistingEntityOutgoingRelationCountMismatch {
                entity_id,
                relation_kind,
                expected,
                actual,
            } => write!(
                f,
                "worth topology query edit execution expected authoritative entity `{entity_id:?}` to have exactly {expected} outgoing `{}` relation(s), found {actual}",
                relation_kind.kind_name()
            ),
            Self::ExistingEntityIncomingRelationCountMismatch {
                entity_id,
                relation_kind,
                expected,
                actual,
            } => write!(
                f,
                "worth topology query edit execution expected authoritative entity `{entity_id:?}` to have exactly {expected} incoming `{}` relation(s), found {actual}",
                relation_kind.kind_name()
            ),
            Self::ExistingHalfEdgesNotOnSameEdge {
                relation_id,
                source_half_edge_id,
                target_half_edge_id,
                source_edge_identity,
                target_edge_identity,
            } => write!(
                f,
                "worth topology query edit execution expected radial splice relation `{relation_id:?}` to keep halfedges `{source_half_edge_id:?}` and `{target_half_edge_id:?}` on the same edge, found source edge `{source_edge_identity}` and target edge `{target_edge_identity}`"
            ),
            Self::ExistingHalfEdgesNotOnSameLoop {
                relation_id,
                source_half_edge_id,
                target_half_edge_id,
                source_loop_identity,
                target_loop_identity,
            } => write!(
                f,
                "worth topology query edit execution expected loop-successor relation `{relation_id:?}` to keep halfedges `{source_half_edge_id:?}` and `{target_half_edge_id:?}` on the same loop, found source loop `{source_loop_identity}` and target loop `{target_loop_identity}`"
            ),
            Self::Query(error) => write!(f, "{error}"),
            Self::Surface(error) => write!(f, "{error}"),
            Self::MaterializedDecode(message) => write!(f, "{message}"),
            Self::UnexpectedInspectionFamily => write!(
                f,
                "worth topology query edit execution expected batch-write receipt inspection"
            ),
        }
    }
}

impl std::error::Error for WorthTopologyQueryEditExecutionError {}

impl From<ForgeQueryRuntimeError> for WorthTopologyQueryEditExecutionError {
    fn from(value: ForgeQueryRuntimeError) -> Self {
        Self::Query(value)
    }
}

impl From<ForgeQueryWorkspaceError> for WorthTopologyQueryEditExecutionError {
    fn from(value: ForgeQueryWorkspaceError) -> Self {
        Self::Query(ForgeQueryRuntimeError::Workspace(value))
    }
}

impl From<WorthTopologyQuerySurfaceError> for WorthTopologyQueryEditExecutionError {
    fn from(value: WorthTopologyQuerySurfaceError) -> Self {
        Self::Surface(value)
    }
}
