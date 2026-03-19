use smallvec::SmallVec;

use crate::identity::data::EntityId;
use crate::payloads::data::RecordPayload;
use crate::publication::patch::data::{AspectKey, CanonicalAspectSet, RecordStructuralChange};
use crate::schema::data::{
    AspectPlanRevision, AspectPrecision, LoweredAspectComparator, LoweredAspectExtractor,
    LoweredAspectPlan,
};
use crate::symbols::data::InternedString;
use crate::transactions::data::RecordRef;
use crate::transactions::data::{
    AspectEvaluationTrace, AspectEvaluationTraceRow, AspectLifecycleTransitionClass,
    AspectTraceEvidence,
};

use super::outcomes::RecordMutation;
use super::MutationWorkspace;

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
        old_canonical_hash: Option<u64>,
        new_canonical_hash: Option<u64>,
    },
    EndpointIdentity {
        old: Option<EntityId>,
        new: Option<EntityId>,
    },
    Lifecycle {
        transition: LifecycleTransitionClass,
    },
    OpaquePayload {
        old_hash: Option<u128>,
        new_hash: Option<u128>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LifecycleTransitionClass {
    None,
    Create,
    Update,
    Delete,
    RetainForAudit,
}

impl CanonicalRecordAspectDelta {
    pub(crate) fn evaluation_trace(&self) -> AspectEvaluationTrace {
        AspectEvaluationTrace {
            target: self.target.clone(),
            kind_id: self.kind_id,
            plan_revision: self.plan_revision,
            structural_change: self.structural_change,
            changed_aspects: self.changed_aspects.clone(),
            contains_degraded_precision: self.contains_degraded_precision,
            binding_rows: self
                .evaluated_bindings
                .iter()
                .map(EvaluatedAspectBinding::trace_row)
                .collect(),
        }
    }
}

impl EvaluatedAspectBinding {
    fn trace_row(&self) -> AspectEvaluationTraceRow {
        AspectEvaluationTraceRow {
            aspect_key: self.aspect_key.clone(),
            changed: self.changed,
            precision: self.precision,
            evidence: self.evidence.trace_evidence(),
        }
    }
}

impl BindingEvidence {
    fn trace_evidence(&self) -> AspectTraceEvidence {
        match self {
            Self::JsonFieldPresenceOrValue {
                old_present,
                new_present,
                old_canonical_hash,
                new_canonical_hash,
            } => AspectTraceEvidence::JsonFieldPresenceOrValue {
                old_present: *old_present,
                new_present: *new_present,
                old_canonical_hash: *old_canonical_hash,
                new_canonical_hash: *new_canonical_hash,
            },
            Self::EndpointIdentity { old, new } => AspectTraceEvidence::EndpointIdentity {
                old: *old,
                new: *new,
            },
            Self::Lifecycle { transition } => AspectTraceEvidence::Lifecycle {
                transition: transition.trace_transition(),
            },
            Self::OpaquePayload { old_hash, new_hash } => AspectTraceEvidence::OpaquePayload {
                old_hash: *old_hash,
                new_hash: *new_hash,
            },
        }
    }
}

impl LifecycleTransitionClass {
    fn trace_transition(self) -> AspectLifecycleTransitionClass {
        match self {
            Self::None => AspectLifecycleTransitionClass::None,
            Self::Create => AspectLifecycleTransitionClass::Create,
            Self::Update => AspectLifecycleTransitionClass::Update,
            Self::Delete => AspectLifecycleTransitionClass::Delete,
            Self::RetainForAudit => AspectLifecycleTransitionClass::RetainForAudit,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct EntityState<'a> {
    payload: Option<&'a RecordPayload>,
}

#[derive(Debug, Clone, Copy)]
struct RelationState<'a> {
    source: Option<EntityId>,
    target: Option<EntityId>,
    payload: Option<&'a RecordPayload>,
}

pub(crate) fn canonical_delta_for_mutation(
    mutation: &RecordMutation,
    workspace: &MutationWorkspace<'_>,
) -> CanonicalRecordAspectDelta {
    match mutation {
        RecordMutation::EntityCreated {
            entity_id,
            kind_id,
            payload,
        } => evaluate_entity_delta(
            workspace,
            *entity_id,
            *kind_id,
            RecordStructuralChange::Created,
            EntityState { payload: None },
            EntityState {
                payload: Some(payload),
            },
        ),
        RecordMutation::EntityUpdated {
            entity_id,
            kind_id,
            old_payload,
            new_payload,
        } => evaluate_entity_delta(
            workspace,
            *entity_id,
            *kind_id,
            RecordStructuralChange::Updated,
            EntityState {
                payload: Some(old_payload),
            },
            EntityState {
                payload: Some(new_payload),
            },
        ),
        RecordMutation::EntityDeleted {
            entity_id,
            kind_id,
            payload,
        } => evaluate_entity_delta(
            workspace,
            *entity_id,
            *kind_id,
            RecordStructuralChange::Deleted,
            EntityState {
                payload: Some(payload),
            },
            EntityState { payload: None },
        ),
        RecordMutation::RelationCreated {
            relation_id,
            kind_id,
            source,
            target,
            payload,
        } => evaluate_relation_delta(
            workspace,
            *relation_id,
            *kind_id,
            RecordStructuralChange::Created,
            RelationState {
                source: None,
                target: None,
                payload: None,
            },
            RelationState {
                source: Some(*source),
                target: Some(*target),
                payload: payload.as_ref(),
            },
        ),
        RecordMutation::RelationDeleted {
            relation_id,
            kind_id,
            source,
            target,
            payload,
        } => evaluate_relation_delta(
            workspace,
            *relation_id,
            *kind_id,
            RecordStructuralChange::Deleted,
            RelationState {
                source: Some(*source),
                target: Some(*target),
                payload: payload.as_ref(),
            },
            RelationState {
                source: None,
                target: None,
                payload: None,
            },
        ),
        RecordMutation::RelationRetainedForAudit {
            relation_id,
            kind_id,
            source,
            target,
            payload,
        } => evaluate_relation_delta(
            workspace,
            *relation_id,
            *kind_id,
            RecordStructuralChange::RetainedForAudit,
            RelationState {
                source: Some(*source),
                target: Some(*target),
                payload: payload.as_ref(),
            },
            RelationState {
                source: Some(*source),
                target: Some(*target),
                payload: payload.as_ref(),
            },
        ),
    }
}

fn evaluate_entity_delta(
    workspace: &MutationWorkspace<'_>,
    entity_id: crate::identity::data::EntityId,
    kind_id: crate::identity::data::KindId,
    structural_change: RecordStructuralChange,
    old_state: EntityState<'_>,
    new_state: EntityState<'_>,
) -> CanonicalRecordAspectDelta {
    let plan = workspace
        .entity_aspect_plan(kind_id)
        .unwrap_or_else(|| panic!("missing lowered entity aspect plan for kind {}", kind_id.0));
    let evaluated_bindings = evaluate_bindings(
        plan,
        structural_change,
        old_state.payload,
        new_state.payload,
        None,
        None,
        None,
        None,
    );
    assemble_delta(
        RecordRef::Entity(entity_id),
        kind_id,
        plan,
        structural_change,
        evaluated_bindings,
    )
}

fn evaluate_relation_delta(
    workspace: &MutationWorkspace<'_>,
    relation_id: crate::identity::data::RelationId,
    kind_id: crate::identity::data::KindId,
    structural_change: RecordStructuralChange,
    old_state: RelationState<'_>,
    new_state: RelationState<'_>,
) -> CanonicalRecordAspectDelta {
    let plan = workspace.relation_aspect_plan(kind_id).unwrap_or_else(|| {
        panic!(
            "missing lowered relation aspect plan for kind {}",
            kind_id.0
        )
    });
    let evaluated_bindings = evaluate_bindings(
        plan,
        structural_change,
        old_state.payload,
        new_state.payload,
        old_state.source,
        new_state.source,
        old_state.target,
        new_state.target,
    );
    assemble_delta(
        RecordRef::Relation(relation_id),
        kind_id,
        plan,
        structural_change,
        evaluated_bindings,
    )
}

fn assemble_delta(
    target: RecordRef,
    kind_id: crate::identity::data::KindId,
    plan: &LoweredAspectPlan,
    structural_change: RecordStructuralChange,
    evaluated_bindings: SmallVec<[EvaluatedAspectBinding; 4]>,
) -> CanonicalRecordAspectDelta {
    let changed_aspects = CanonicalAspectSet::new(
        evaluated_bindings
            .iter()
            .filter(|binding| binding.changed)
            .map(|binding| binding.aspect_key.clone()),
    );
    let contains_degraded_precision = evaluated_bindings
        .iter()
        .any(|binding| binding.precision == AspectPrecision::Opaque);
    CanonicalRecordAspectDelta {
        target,
        kind_id,
        plan_revision: plan.plan_revision,
        structural_change,
        changed_aspects,
        evaluated_bindings,
        contains_degraded_precision,
    }
}

fn evaluate_bindings(
    plan: &LoweredAspectPlan,
    structural_change: RecordStructuralChange,
    old_payload: Option<&RecordPayload>,
    new_payload: Option<&RecordPayload>,
    old_source: Option<EntityId>,
    new_source: Option<EntityId>,
    old_target: Option<EntityId>,
    new_target: Option<EntityId>,
) -> SmallVec<[EvaluatedAspectBinding; 4]> {
    let mut evaluated = SmallVec::new();
    let lifecycle_transition = lifecycle_transition(structural_change);
    for binding in &plan.executable_bindings {
        let (evidence, changed) = match (&binding.extractor, binding.comparator) {
            (
                LoweredAspectExtractor::EntityJsonField { field }
                | LoweredAspectExtractor::RelationJsonField { field },
                LoweredAspectComparator::JsonScalarEquality,
            ) => evaluate_json_field(old_payload, new_payload, raw_field_name(field)),
            (
                LoweredAspectExtractor::RelationSourceEndpoint,
                LoweredAspectComparator::EndpointIdentityEquality,
            ) => {
                let evidence = BindingEvidence::EndpointIdentity {
                    old: old_source,
                    new: new_source,
                };
                let changed = binding_evidence_changed(&evidence);
                (evidence, changed)
            }
            (
                LoweredAspectExtractor::RelationTargetEndpoint,
                LoweredAspectComparator::EndpointIdentityEquality,
            ) => {
                let evidence = BindingEvidence::EndpointIdentity {
                    old: old_target,
                    new: new_target,
                };
                let changed = binding_evidence_changed(&evidence);
                (evidence, changed)
            }
            (
                LoweredAspectExtractor::LifecycleTransition,
                LoweredAspectComparator::LifecycleTransitionEquality,
            ) => {
                let evidence = BindingEvidence::Lifecycle {
                    transition: lifecycle_transition,
                };
                let changed = binding_evidence_changed(&evidence);
                (evidence, changed)
            }
            (
                LoweredAspectExtractor::OpaqueWholePayloadBytes,
                LoweredAspectComparator::OpaquePayloadByteEquality,
            ) => evaluate_opaque_payload(old_payload, new_payload),
            _ => continue,
        };
        if changed {
            evaluated.push(EvaluatedAspectBinding {
                aspect_key: binding.aspect_key.clone(),
                changed,
                precision: binding.precision,
                evidence,
            });
        }
    }
    evaluated
}

fn raw_field_name(value: &InternedString) -> &str {
    match value {
        InternedString::Raw(raw) => raw.as_str(),
        InternedString::Symbol(_) => {
            unreachable!("lowered aspect plans should not contain symbolic field names")
        }
    }
}

fn evaluate_json_field(
    old_payload: Option<&RecordPayload>,
    new_payload: Option<&RecordPayload>,
    field_name: &str,
) -> (BindingEvidence, bool) {
    let old_value = extract_json_field(old_payload, field_name);
    let new_value = extract_json_field(new_payload, field_name);
    let changed = (old_value.is_some() || new_value.is_some()) && old_value != new_value;
    let evidence = BindingEvidence::JsonFieldPresenceOrValue {
        old_present: old_value.is_some(),
        new_present: new_value.is_some(),
        old_canonical_hash: changed
            .then(|| old_value.map(hash_json_value_u64))
            .flatten(),
        new_canonical_hash: changed
            .then(|| new_value.map(hash_json_value_u64))
            .flatten(),
    };
    (evidence, changed)
}

fn extract_json_field<'a>(
    payload: Option<&'a RecordPayload>,
    field_name: &str,
) -> Option<&'a serde_json::Value> {
    payload
        .and_then(RecordPayload::as_json)
        .and_then(serde_json::Value::as_object)
        .and_then(|object| object.get(field_name))
}

fn binding_evidence_changed(evidence: &BindingEvidence) -> bool {
    match evidence {
        BindingEvidence::JsonFieldPresenceOrValue {
            old_present,
            new_present,
            old_canonical_hash,
            new_canonical_hash,
        } => (*old_present || *new_present) && old_canonical_hash != new_canonical_hash,
        BindingEvidence::EndpointIdentity { old, new } => old != new,
        BindingEvidence::Lifecycle { transition } => *transition != LifecycleTransitionClass::None,
        BindingEvidence::OpaquePayload { old_hash, new_hash } => old_hash != new_hash,
    }
}

fn evaluate_opaque_payload(
    old_payload: Option<&RecordPayload>,
    new_payload: Option<&RecordPayload>,
) -> (BindingEvidence, bool) {
    let changed = old_payload != new_payload;
    let evidence = BindingEvidence::OpaquePayload {
        old_hash: changed
            .then(|| old_payload.map(hash_payload_u128))
            .flatten(),
        new_hash: changed
            .then(|| new_payload.map(hash_payload_u128))
            .flatten(),
    };
    (evidence, changed)
}

fn lifecycle_transition(structural_change: RecordStructuralChange) -> LifecycleTransitionClass {
    match structural_change {
        RecordStructuralChange::Created => LifecycleTransitionClass::Create,
        RecordStructuralChange::Updated => LifecycleTransitionClass::Update,
        RecordStructuralChange::Deleted => LifecycleTransitionClass::Delete,
        RecordStructuralChange::RetainedForAudit => LifecycleTransitionClass::RetainForAudit,
    }
}

fn hash_json_value_u64(value: &serde_json::Value) -> u64 {
    hash_bytes_u64(
        &serde_json::to_vec(value)
            .expect("canonical json value serialization must succeed for aspect hashing"),
    )
}

fn hash_payload_u128(payload: &RecordPayload) -> u128 {
    let bytes = match payload {
        RecordPayload::StructuredJson(value) => serde_json::to_vec(value)
            .expect("canonical payload json serialization must succeed for aspect hashing"),
        RecordPayload::OpaqueBytes(bytes) => bytes.clone(),
    };
    hash_bytes_u128(&bytes)
}

fn hash_bytes_u64(bytes: &[u8]) -> u64 {
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;

    let mut hash = FNV_OFFSET;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

fn hash_bytes_u128(bytes: &[u8]) -> u128 {
    const FNV_OFFSET: u128 = 0x6c62272e07bb014262b821756295c58d;
    const FNV_PRIME: u128 = 0x0000000001000000000000000000013B;

    let mut hash = FNV_OFFSET;
    for byte in bytes {
        hash ^= u128::from(*byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}
