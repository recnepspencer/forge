use forge_query::facade::{
    ForgeQueryDeclarationEntryOrchestrationRefusalClass,
    ForgeQueryDeclarationEntryOrchestrationStage, ForgeQueryRuntimeError, ForgeQueryWorkspaceError,
};
use forge_relational::facade::identity::{EntityId, RelationId};
use schema::facade::platform::entities::TopologyEntityKind;
use schema::facade::platform::relations::TopologyRelationKind;

use super::super::contracts::TopologyEditFamily;
use super::TopologyEditApplicationMode;

#[derive(Debug)]
pub enum TopologyOperatorExecutionError {
    UnsupportedMode(TopologyEditApplicationMode),
    UnsupportedFamilies(Vec<TopologyEditFamily>),
    DeclarationEntryRequired {
        family: TopologyEditFamily,
        reason: &'static str,
    },
    DeclarationEntryProgramRequired {
        families: Vec<TopologyEditFamily>,
        reason: &'static str,
    },
    DeclarationEntry {
        family: TopologyEditFamily,
        stop_class: TopologyDeclarationEntryStopClass,
        stop_stage: ForgeQueryDeclarationEntryOrchestrationStage,
        refusal_class: Option<TopologyDeclarationEntryRefusalClass>,
        reason: &'static str,
    },
    MissingCreatedEntityReference(String),
    MissingExistingEntityBinding(EntityId),
    MissingExistingRelationBinding(RelationId),
    CreatedEntityKindMismatch {
        create_key: String,
        expected: TopologyEntityKind,
        actual: TopologyEntityKind,
    },
    ExistingEntityKindMismatch {
        entity_id: EntityId,
        expected: TopologyEntityKind,
        actual: TopologyEntityKind,
    },
    ExistingRelationKindMismatch {
        relation_id: RelationId,
        expected: TopologyRelationKind,
        actual: TopologyRelationKind,
    },
    ExistingRelationSourceMismatch {
        relation_id: RelationId,
        expected_source_entity_id: EntityId,
        actual_source_identity: String,
    },
    ExistingEntityOutgoingRelationCountMismatch {
        entity_id: EntityId,
        relation_kind: TopologyRelationKind,
        expected: usize,
        actual: usize,
    },
    ExistingEntityIncomingRelationCountMismatch {
        entity_id: EntityId,
        relation_kind: TopologyRelationKind,
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
    MaterializedDecode(String),
    UnexpectedInspectionFamily,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TopologyDeclarationEntryStopClass {
    Deferred,
    Denied,
    Stale,
    RebindRequired,
    Failed,
    Refused,
}

impl TopologyDeclarationEntryStopClass {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Deferred => "deferred",
            Self::Denied => "denied",
            Self::Stale => "stale",
            Self::RebindRequired => "rebind_required",
            Self::Failed => "failed",
            Self::Refused => "refused",
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TopologyDeclarationEntryRefusalClass {
    UnsupportedAutomation,
    ExplicitIntentRequired,
    StrongerProofRequired,
    AuthorityTransitionRequired,
    ExpensiveWorkNotAdmittedByDefault,
    PreparedButNotExecutedContinuation,
}

impl TopologyDeclarationEntryRefusalClass {
    const fn as_str(self) -> &'static str {
        match self {
            Self::UnsupportedAutomation => "unsupported_automation",
            Self::ExplicitIntentRequired => "explicit_intent_required",
            Self::StrongerProofRequired => "stronger_proof_required",
            Self::AuthorityTransitionRequired => "authority_transition_required",
            Self::ExpensiveWorkNotAdmittedByDefault => "expensive_work_not_admitted_by_default",
            Self::PreparedButNotExecutedContinuation => "prepared_but_not_executed_continuation",
        }
    }
}

impl From<ForgeQueryDeclarationEntryOrchestrationRefusalClass>
    for TopologyDeclarationEntryRefusalClass
{
    fn from(value: ForgeQueryDeclarationEntryOrchestrationRefusalClass) -> Self {
        match value {
            ForgeQueryDeclarationEntryOrchestrationRefusalClass::UnsupportedAutomation => {
                Self::UnsupportedAutomation
            }
            ForgeQueryDeclarationEntryOrchestrationRefusalClass::ExplicitIntentRequired => {
                Self::ExplicitIntentRequired
            }
            ForgeQueryDeclarationEntryOrchestrationRefusalClass::StrongerProofRequired => {
                Self::StrongerProofRequired
            }
            ForgeQueryDeclarationEntryOrchestrationRefusalClass::AuthorityTransitionRequired => {
                Self::AuthorityTransitionRequired
            }
            ForgeQueryDeclarationEntryOrchestrationRefusalClass::ExpensiveWorkNotAdmittedByDefault => {
                Self::ExpensiveWorkNotAdmittedByDefault
            }
            ForgeQueryDeclarationEntryOrchestrationRefusalClass::PreparedButNotExecutedContinuation => {
                Self::PreparedButNotExecutedContinuation
            }
        }
    }
}

impl std::fmt::Display for TopologyOperatorExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedMode(mode) => write!(
                f,
                "topology query edit execution does not admit mode `{mode:?}` yet"
            ),
            Self::UnsupportedFamilies(families) => write!(
                f,
                "topology query edit execution does not admit families `{families:?}` yet"
            ),
            Self::DeclarationEntryRequired { family, reason } => write!(
                f,
                "topology query edit execution requires declaration-entry canonical input for family `{family:?}`: {reason}"
            ),
            Self::DeclarationEntryProgramRequired { families, reason } => write!(
                f,
                "topology query edit execution requires declaration-entry canonical grouped input for families `{families:?}`: {reason}"
            ),
            Self::DeclarationEntry {
                family,
                stop_class,
                stop_stage,
                refusal_class,
                reason,
            } => {
                write!(
                    f,
                    "topology query declaration entry orchestration for family `{family:?}` stopped as `{}` at stage `{stop_stage:?}`",
                    stop_class.as_str(),
                )?;
                if let Some(refusal_class) = refusal_class {
                    write!(f, " with refusal class `{}`", refusal_class.as_str())?;
                }
                write!(f, ": {reason}")
            }
            Self::MissingCreatedEntityReference(create_key) => write!(
                f,
                "topology query edit execution is missing same-batch created entity `{create_key}`"
            ),
            Self::MissingExistingEntityBinding(entity_id) => write!(
                f,
                "topology query edit execution is missing live query binding for authoritative entity `{entity_id:?}`"
            ),
            Self::MissingExistingRelationBinding(relation_id) => write!(
                f,
                "topology query edit execution is missing live query binding for authoritative relation `{relation_id:?}`"
            ),
            Self::CreatedEntityKindMismatch {
                create_key,
                expected,
                actual,
            } => write!(
                f,
                "topology query edit execution expected created entity `{create_key}` to be `{}`, found `{}`",
                expected.kind_name(),
                actual.kind_name()
            ),
            Self::ExistingEntityKindMismatch {
                entity_id,
                expected,
                actual,
            } => write!(
                f,
                "topology query edit execution expected authoritative entity `{entity_id:?}` to be `{}`, found `{}`",
                expected.kind_name(),
                actual.kind_name()
            ),
            Self::ExistingRelationKindMismatch {
                relation_id,
                expected,
                actual,
            } => write!(
                f,
                "topology query edit execution expected authoritative relation `{relation_id:?}` to be `{}`, found `{}`",
                expected.kind_name(),
                actual.kind_name()
            ),
            Self::ExistingRelationSourceMismatch {
                relation_id,
                expected_source_entity_id,
                actual_source_identity,
            } => write!(
                f,
                "topology query edit execution expected authoritative relation `{relation_id:?}` to originate from halfedge `{expected_source_entity_id:?}`, found query source identity `{actual_source_identity}`"
            ),
            Self::ExistingEntityOutgoingRelationCountMismatch {
                entity_id,
                relation_kind,
                expected,
                actual,
            } => write!(
                f,
                "topology query edit execution expected authoritative entity `{entity_id:?}` to have exactly {expected} outgoing `{}` relation(s), found {actual}",
                relation_kind.kind_name()
            ),
            Self::ExistingEntityIncomingRelationCountMismatch {
                entity_id,
                relation_kind,
                expected,
                actual,
            } => write!(
                f,
                "topology query edit execution expected authoritative entity `{entity_id:?}` to have exactly {expected} incoming `{}` relation(s), found {actual}",
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
                "topology query edit execution expected radial splice relation `{relation_id:?}` to keep halfedges `{source_half_edge_id:?}` and `{target_half_edge_id:?}` on the same edge, found source edge `{source_edge_identity}` and target edge `{target_edge_identity}`"
            ),
            Self::ExistingHalfEdgesNotOnSameLoop {
                relation_id,
                source_half_edge_id,
                target_half_edge_id,
                source_loop_identity,
                target_loop_identity,
            } => write!(
                f,
                "topology query edit execution expected loop-successor relation `{relation_id:?}` to keep halfedges `{source_half_edge_id:?}` and `{target_half_edge_id:?}` on the same loop, found source loop `{source_loop_identity}` and target loop `{target_loop_identity}`"
            ),
            Self::Query(error) => write!(f, "{error}"),
            Self::MaterializedDecode(message) => write!(f, "{message}"),
            Self::UnexpectedInspectionFamily => write!(
                f,
                "topology query edit execution expected batch-write receipt inspection"
            ),
        }
    }
}

impl std::error::Error for TopologyOperatorExecutionError {}

impl From<ForgeQueryRuntimeError> for TopologyOperatorExecutionError {
    fn from(value: ForgeQueryRuntimeError) -> Self {
        Self::Query(value)
    }
}

impl From<ForgeQueryWorkspaceError> for TopologyOperatorExecutionError {
    fn from(value: ForgeQueryWorkspaceError) -> Self {
        Self::Query(ForgeQueryRuntimeError::Workspace(value))
    }
}
