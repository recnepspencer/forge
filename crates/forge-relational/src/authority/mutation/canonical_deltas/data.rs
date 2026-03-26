use smallvec::SmallVec;

use crate::diagnostics::data::DiagnosticCode;
use crate::identity::data::EntityId;
use crate::payloads::data::RecordPayload;
use crate::publication::patch::data::{AspectKey, CanonicalAspectSet, RecordStructuralChange};
use crate::schema::data::{AspectPlanRevision, AspectPrecision};
use crate::transactions::data::CommitConflict;
use crate::transactions::data::ConflictClass;
use crate::transactions::data::RecordRef;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CanonicalRecordAspectDelta {
    pub(crate) target: RecordRef,
    pub(crate) kind_id: crate::identity::data::KindId,
    pub(crate) plan_revision: AspectPlanRevision,
    pub(crate) structural_change: RecordStructuralChange,
    pub(crate) changed_aspects: CanonicalAspectSet,
    pub(crate) evaluated_bindings: SmallVec<[EvaluatedAspectBinding; 4]>,
    pub(crate) contains_degraded_precision: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EvaluatedAspectBinding {
    pub(crate) aspect_key: AspectKey,
    pub(crate) changed: bool,
    pub(crate) precision: AspectPrecision,
    pub(crate) evidence: BindingEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BindingEvidence {
    JsonFieldPresenceOrValue {
        old_present: bool,
        new_present: bool,
        old_canonical_json: Option<String>,
        new_canonical_json: Option<String>,
    },
    EndpointIdentity {
        old: Option<EntityId>,
        new: Option<EntityId>,
    },
    Lifecycle {
        transition: LifecycleTransitionClass,
    },
    OpaquePayloadDigest {
        old_present: bool,
        new_present: bool,
        old_diagnostic_digest: Option<u128>,
        new_diagnostic_digest: Option<u128>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LifecycleTransitionClass {
    NoTransition,
    Create,
    Update,
    Delete,
    RetainForAudit,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum BindingEvaluationContext<'a> {
    Entity {
        structural_change: RecordStructuralChange,
        old_payload: Option<&'a RecordPayload>,
        new_payload: Option<&'a RecordPayload>,
    },
    Relation {
        structural_change: RecordStructuralChange,
        old_payload: Option<&'a RecordPayload>,
        new_payload: Option<&'a RecordPayload>,
        old_source: Option<EntityId>,
        new_source: Option<EntityId>,
        old_target: Option<EntityId>,
        new_target: Option<EntityId>,
    },
}

impl<'a> BindingEvaluationContext<'a> {
    pub(crate) fn structural_change(self) -> RecordStructuralChange {
        match self {
            Self::Entity {
                structural_change, ..
            }
            | Self::Relation {
                structural_change, ..
            } => structural_change,
        }
    }

    pub(crate) fn old_payload(self) -> Option<&'a RecordPayload> {
        match self {
            Self::Entity { old_payload, .. } | Self::Relation { old_payload, .. } => old_payload,
        }
    }

    pub(crate) fn new_payload(self) -> Option<&'a RecordPayload> {
        match self {
            Self::Entity { new_payload, .. } | Self::Relation { new_payload, .. } => new_payload,
        }
    }

    pub(crate) fn relation_endpoints(
        self,
    ) -> Option<(
        Option<EntityId>,
        Option<EntityId>,
        Option<EntityId>,
        Option<EntityId>,
    )> {
        match self {
            Self::Entity { .. } => None,
            Self::Relation {
                old_source,
                new_source,
                old_target,
                new_target,
                ..
            } => Some((old_source, new_source, old_target, new_target)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CanonicalDeltaError {
    MissingEntityAspectPlan {
        kind_id: crate::identity::data::KindId,
    },
    MissingRelationAspectPlan {
        kind_id: crate::identity::data::KindId,
    },
    InvalidLoweredBindingForRecordClass {
        aspect_key: AspectKey,
        detail: String,
    },
    SymbolicLoweredFieldName {
        aspect_key: AspectKey,
        field: crate::symbols::data::InternedString,
    },
    JsonEvidenceSerialization {
        aspect_key: AspectKey,
        detail: String,
    },
    CanonicalAspectKeyRequiresRawString {
        aspect_key: AspectKey,
    },
}

impl CanonicalDeltaError {
    pub(crate) fn to_commit_conflict(&self) -> CommitConflict {
        CommitConflict::new(ConflictClass::AspectDeltaFailure {
            detail: self.detail(),
            fields: self.fields(),
        })
    }

    fn detail(&self) -> String {
        match self {
            Self::MissingEntityAspectPlan { kind_id } => format!(
                "missing lowered entity aspect plan for kind {} during canonical delta evaluation",
                kind_id.0
            ),
            Self::MissingRelationAspectPlan { kind_id } => format!(
                "missing lowered relation aspect plan for kind {} during canonical delta evaluation",
                kind_id.0
            ),
            Self::InvalidLoweredBindingForRecordClass { detail, .. } => detail.clone(),
            Self::SymbolicLoweredFieldName { aspect_key, .. } => format!(
                "lowered aspect binding {:?} carried a symbolic field name into canonical delta evaluation",
                aspect_key
            ),
            Self::JsonEvidenceSerialization { detail, .. } => detail.clone(),
            Self::CanonicalAspectKeyRequiresRawString { aspect_key } => format!(
                "canonical aspect key {:?} must remain raw when writing aspect versions",
                aspect_key
            ),
        }
    }

    fn fields(&self) -> serde_json::Value {
        match self {
            Self::MissingEntityAspectPlan { kind_id } => serde_json::json!({
                "kind_id": kind_id.0,
                "record_class": "entity",
                "code": DiagnosticCode::AspectDeltaFailure,
            }),
            Self::MissingRelationAspectPlan { kind_id } => serde_json::json!({
                "kind_id": kind_id.0,
                "record_class": "relation",
                "code": DiagnosticCode::AspectDeltaFailure,
            }),
            Self::InvalidLoweredBindingForRecordClass { aspect_key, detail } => serde_json::json!({
                "aspect_key": aspect_key,
                "detail": detail,
            }),
            Self::SymbolicLoweredFieldName { aspect_key, field } => serde_json::json!({
                "aspect_key": aspect_key,
                "field": field,
            }),
            Self::JsonEvidenceSerialization { aspect_key, detail } => serde_json::json!({
                "aspect_key": aspect_key,
                "detail": detail,
            }),
            Self::CanonicalAspectKeyRequiresRawString { aspect_key } => serde_json::json!({
                "aspect_key": aspect_key,
            }),
        }
    }
}
