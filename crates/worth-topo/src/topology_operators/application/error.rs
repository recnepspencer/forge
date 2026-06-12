use forge_query::facade::{
    ForgeQueryDeclarationEntryOrchestrationRefusalClass,
    ForgeQueryDeclarationEntryOrchestrationStage, ForgeQueryOrdinaryPostureKind,
    ForgeQueryRecoveryBrief, ForgeQueryRecoveryStopKind, ForgeQueryRuntimeError,
    ForgeQueryWorkspaceError,
};
use forge_relational::facade::identity::{EntityId, RelationId};
use schema::facade::platform::entities::TopologyEntityKind;
use schema::facade::platform::relations::TopologyRelationKind;

use super::super::mutation_records::TopologyMutationFamily;
#[derive(Debug)]
pub enum TopologyMutationApplicationError {
    UnsupportedFamilies(Vec<TopologyMutationFamily>),
    DeclarationEntry {
        family: TopologyMutationFamily,
        stop_class: TopologyDeclarationEntryStopClass,
        stop_stage: Option<ForgeQueryDeclarationEntryOrchestrationStage>,
        refusal_class: Option<TopologyDeclarationEntryRefusalClass>,
        recovery: Option<ForgeQueryRecoveryBrief>,
        reason: String,
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
    QueryAnchorFamilyMismatch {
        semantic_family_key: &'static str,
        query_declaration_family_key: &'static str,
    },
    RetainedSemanticAftermathMismatch {
        semantic_family_key: &'static str,
        reason: String,
    },
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum TopologyDeclarationEntryStopClass {
    Deferred,
    Denied,
    Stale,
    StaleCompletion,
    RebindRequired,
    Failed,
    Refused,
    Unsupported,
    Ambiguous,
    AsyncRequestDrift,
    AspectConflict,
    AuthorityMismatch,
    BasisMismatch,
    ExplicitNarrowingRequired,
    MissingRequiredAspect,
    PreviewCrossedResidue,
    RemaskDrift,
    ReplayDrift,
    Unavailable,
    WrongHandle,
    WrongWorld,
}

impl TopologyDeclarationEntryStopClass {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Deferred => "deferred",
            Self::Denied => "denied",
            Self::Stale => "stale",
            Self::StaleCompletion => "stale_completion",
            Self::RebindRequired => "rebind_required",
            Self::Failed => "failed",
            Self::Refused => "refused",
            Self::Unsupported => "unsupported",
            Self::Ambiguous => "ambiguous",
            Self::AsyncRequestDrift => "async_request_drift",
            Self::AspectConflict => "aspect_conflict",
            Self::AuthorityMismatch => "authority_mismatch",
            Self::BasisMismatch => "basis_mismatch",
            Self::ExplicitNarrowingRequired => "explicit_narrowing_required",
            Self::MissingRequiredAspect => "missing_required_aspect",
            Self::PreviewCrossedResidue => "preview_crossed_residue",
            Self::RemaskDrift => "remask_drift",
            Self::ReplayDrift => "replay_drift",
            Self::Unavailable => "unavailable",
            Self::WrongHandle => "wrong_handle",
            Self::WrongWorld => "wrong_world",
        }
    }
}

impl From<ForgeQueryOrdinaryPostureKind> for TopologyDeclarationEntryStopClass {
    fn from(value: ForgeQueryOrdinaryPostureKind) -> Self {
        match value {
            ForgeQueryOrdinaryPostureKind::Ambiguous => Self::Ambiguous,
            ForgeQueryOrdinaryPostureKind::AspectConflict => Self::AspectConflict,
            ForgeQueryOrdinaryPostureKind::AuthorityMismatch => Self::AuthorityMismatch,
            ForgeQueryOrdinaryPostureKind::BasisMismatch => Self::BasisMismatch,
            ForgeQueryOrdinaryPostureKind::Deferred => Self::Deferred,
            ForgeQueryOrdinaryPostureKind::Denied => Self::Denied,
            ForgeQueryOrdinaryPostureKind::ExplicitNarrowingRequired => {
                Self::ExplicitNarrowingRequired
            }
            ForgeQueryOrdinaryPostureKind::Failed => Self::Failed,
            ForgeQueryOrdinaryPostureKind::MissingRequiredAspect => Self::MissingRequiredAspect,
            ForgeQueryOrdinaryPostureKind::RebindRequired => Self::RebindRequired,
            ForgeQueryOrdinaryPostureKind::Refused => Self::Refused,
            ForgeQueryOrdinaryPostureKind::Stale => Self::Stale,
            ForgeQueryOrdinaryPostureKind::Unavailable => Self::Unavailable,
            ForgeQueryOrdinaryPostureKind::Unsupported => Self::Unsupported,
            ForgeQueryOrdinaryPostureKind::WrongHandle => Self::WrongHandle,
            ForgeQueryOrdinaryPostureKind::WrongWorld => Self::WrongWorld,
        }
    }
}

impl From<ForgeQueryRecoveryStopKind> for TopologyDeclarationEntryStopClass {
    fn from(value: ForgeQueryRecoveryStopKind) -> Self {
        match value {
            ForgeQueryRecoveryStopKind::Ambiguous => Self::Ambiguous,
            ForgeQueryRecoveryStopKind::AsyncRequestDrift => Self::AsyncRequestDrift,
            ForgeQueryRecoveryStopKind::AspectConflict => Self::AspectConflict,
            ForgeQueryRecoveryStopKind::AuthorityMismatch => Self::AuthorityMismatch,
            ForgeQueryRecoveryStopKind::BasisMismatch => Self::BasisMismatch,
            ForgeQueryRecoveryStopKind::ContributionDenied
            | ForgeQueryRecoveryStopKind::DeclarationDenied => Self::Denied,
            ForgeQueryRecoveryStopKind::Deferred => Self::Deferred,
            ForgeQueryRecoveryStopKind::Failed => Self::Failed,
            ForgeQueryRecoveryStopKind::MissingRequiredAspect => Self::MissingRequiredAspect,
            ForgeQueryRecoveryStopKind::PreviewCrossedResidue => Self::PreviewCrossedResidue,
            ForgeQueryRecoveryStopKind::RebindRequired => Self::RebindRequired,
            ForgeQueryRecoveryStopKind::RemaskDrift => Self::RemaskDrift,
            ForgeQueryRecoveryStopKind::ReplayDrift => Self::ReplayDrift,
            ForgeQueryRecoveryStopKind::Refused => Self::Refused,
            ForgeQueryRecoveryStopKind::Stale => Self::Stale,
            ForgeQueryRecoveryStopKind::StaleCompletion => Self::StaleCompletion,
            ForgeQueryRecoveryStopKind::Unavailable => Self::Unavailable,
            ForgeQueryRecoveryStopKind::Unsupported => Self::Unsupported,
            ForgeQueryRecoveryStopKind::WrongHandle => Self::WrongHandle,
            ForgeQueryRecoveryStopKind::WrongWorld => Self::WrongWorld,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum TopologyDeclarationEntryRefusalClass {
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

impl std::fmt::Display for TopologyMutationApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedFamilies(families) => write!(
                f,
                "topology query mutation application does not admit families `{families:?}` yet"
            ),
            Self::DeclarationEntry {
                family,
                stop_class,
                stop_stage,
                refusal_class,
                recovery,
                reason,
            } => {
                write!(
                    f,
                    "topology query declaration entry orchestration for family `{family:?}` stopped as `{}`",
                    stop_class.as_str(),
                )?;
                if let Some(stop_stage) = stop_stage {
                    write!(f, " at stage `{stop_stage:?}`")?;
                }
                if let Some(refusal_class) = refusal_class {
                    write!(f, " with refusal class `{}`", refusal_class.as_str())?;
                }
                if let Some(recovery) = recovery {
                    write!(
                        f,
                        " owned by `{:?}` recommending `{:?}`",
                        recovery.authority_surface(),
                        recovery.recommended_action()
                    )?;
                }
                write!(f, ": {reason}")
            }
            Self::MissingCreatedEntityReference(create_key) => write!(
                f,
                "topology query mutation application is missing same-mutation-set created entity `{create_key}`"
            ),
            Self::MissingExistingEntityBinding(entity_id) => write!(
                f,
                "topology query mutation application is missing live query binding for authoritative entity `{entity_id:?}`"
            ),
            Self::MissingExistingRelationBinding(relation_id) => write!(
                f,
                "topology query mutation application is missing live query binding for authoritative relation `{relation_id:?}`"
            ),
            Self::CreatedEntityKindMismatch {
                create_key,
                expected,
                actual,
            } => write!(
                f,
                "topology query mutation application expected created entity `{create_key}` to be `{}`, found `{}`",
                expected.kind_name(),
                actual.kind_name()
            ),
            Self::ExistingEntityKindMismatch {
                entity_id,
                expected,
                actual,
            } => write!(
                f,
                "topology query mutation application expected authoritative entity `{entity_id:?}` to be `{}`, found `{}`",
                expected.kind_name(),
                actual.kind_name()
            ),
            Self::ExistingRelationKindMismatch {
                relation_id,
                expected,
                actual,
            } => write!(
                f,
                "topology query mutation application expected authoritative relation `{relation_id:?}` to be `{}`, found `{}`",
                expected.kind_name(),
                actual.kind_name()
            ),
            Self::ExistingRelationSourceMismatch {
                relation_id,
                expected_source_entity_id,
                actual_source_identity,
            } => write!(
                f,
                "topology query mutation application expected authoritative relation `{relation_id:?}` to originate from halfedge `{expected_source_entity_id:?}`, found query source identity `{actual_source_identity}`"
            ),
            Self::ExistingEntityOutgoingRelationCountMismatch {
                entity_id,
                relation_kind,
                expected,
                actual,
            } => write!(
                f,
                "topology query mutation application expected authoritative entity `{entity_id:?}` to have exactly {expected} outgoing `{}` relation(s), found {actual}",
                relation_kind.kind_name()
            ),
            Self::ExistingEntityIncomingRelationCountMismatch {
                entity_id,
                relation_kind,
                expected,
                actual,
            } => write!(
                f,
                "topology query mutation application expected authoritative entity `{entity_id:?}` to have exactly {expected} incoming `{}` relation(s), found {actual}",
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
                "topology query mutation application expected radial splice relation `{relation_id:?}` to keep halfedges `{source_half_edge_id:?}` and `{target_half_edge_id:?}` on the same edge, found source edge `{source_edge_identity}` and target edge `{target_edge_identity}`"
            ),
            Self::ExistingHalfEdgesNotOnSameLoop {
                relation_id,
                source_half_edge_id,
                target_half_edge_id,
                source_loop_identity,
                target_loop_identity,
            } => write!(
                f,
                "topology query mutation application expected loop-successor relation `{relation_id:?}` to keep halfedges `{source_half_edge_id:?}` and `{target_half_edge_id:?}` on the same loop, found source loop `{source_loop_identity}` and target loop `{target_loop_identity}`"
            ),
            Self::Query(error) => write!(f, "{error}"),
            Self::MaterializedDecode(message) => write!(f, "{message}"),
            Self::QueryAnchorFamilyMismatch {
                semantic_family_key,
                query_declaration_family_key,
            } => write!(
                f,
                "topology query mutation application refused to project local aftermath for semantic family `{semantic_family_key}` from Query declaration family `{query_declaration_family_key}`"
            ),
            Self::RetainedSemanticAftermathMismatch {
                semantic_family_key,
                reason,
            } => write!(
                f,
                "topology query mutation application retained Query semantic aftermath that did not match the declared topology mutation sequence for `{semantic_family_key}`: {reason}"
            ),
        }
    }
}

impl std::error::Error for TopologyMutationApplicationError {}

#[cfg_attr(not(test), allow(dead_code))]
impl TopologyMutationApplicationError {
    pub(crate) fn is_declaration_entry_stop(&self) -> bool {
        matches!(self, Self::DeclarationEntry { .. })
    }

    pub(crate) fn declaration_entry_stop_class(&self) -> Option<TopologyDeclarationEntryStopClass> {
        match self {
            Self::DeclarationEntry { stop_class, .. } => Some(*stop_class),
            _ => None,
        }
    }

    pub(crate) fn declaration_entry_recovery_brief(&self) -> Option<&ForgeQueryRecoveryBrief> {
        match self {
            Self::DeclarationEntry { recovery, .. } => recovery.as_ref(),
            _ => None,
        }
    }
}

impl From<ForgeQueryRuntimeError> for TopologyMutationApplicationError {
    fn from(value: ForgeQueryRuntimeError) -> Self {
        Self::Query(value)
    }
}

impl From<ForgeQueryWorkspaceError> for TopologyMutationApplicationError {
    fn from(value: ForgeQueryWorkspaceError) -> Self {
        Self::Query(ForgeQueryRuntimeError::Workspace(value))
    }
}
