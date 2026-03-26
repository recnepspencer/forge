use std::collections::BTreeMap;
use std::sync::Arc;

use crate::merge::data::{
    AspectComparisonState, AspectConflictEvidence, ConflictClassificationSummary,
    ConflictClassifiedMergePlan, DeletionMergeClass, EndpointContinuityClass,
    IdentityMatchClass, IdentityResolutionReason, IdentityScopedMergePlan,
    MergeConflictClass, MergeConflictClassification, MergePlanningError, MergePlanningRequest,
    RelationConflictEvidence, RelationConflictPropagation, RelationContinuityClass,
    VisibleMergeRecord, VisibleMergeRecordKind,
};
use crate::merge::logic::aspect_plan_lookup::lowered_plan_for_record;
use crate::merge::logic::MergeAccess;
use crate::payloads::data::RecordPayload;
use crate::schema::data::{LoweredAspectBinding, LoweredExecutableAspectBindingKind};
use crate::identity::data::VersionId;
use crate::symbols::data::InternedString;
use crate::storage::data::{EntityReadRecord, RecordLifecycleState, RelationReadRecord, RelationalReadView};
use crate::transactions::data::RecordRef;
use serde_json::Value;

impl<'runtime> MergeAccess<'runtime> {
    pub(crate) fn plan_conflict_scope(
        &self,
        request: MergePlanningRequest,
    ) -> Result<ConflictClassifiedMergePlan, MergePlanningError> {
        let identity_plan = self.plan_identity_scope(request)?;
        self.classify_conflict_scope(identity_plan)
    }

    fn classify_conflict_scope(
        &self,
        identity_plan: IdentityScopedMergePlan,
    ) -> Result<ConflictClassifiedMergePlan, MergePlanningError> {
        let target_view = self
            .runtime
            .visibility_reads()
            .read_version(identity_plan.target_head.version_id);
        let history = self.runtime.history_access();
        let base_envelope = history
            .commit_envelope(identity_plan.merge_base.commit_id)
            .ok_or(MergePlanningError::MissingMergeBaseEnvelope {
                commit_id: identity_plan.merge_base.commit_id,
            })?;
        let base_version_id = base_envelope.commit.version_id;
        let base_view = self.runtime.visibility_reads().read_version(base_version_id);
        let source_records_by_ref = identity_plan
            .source_records
            .iter()
            .map(|record| (record.record_ref.clone(), record))
            .collect::<BTreeMap<_, _>>();
        let validated_by_source = identity_plan
            .validated_schema_correspondences
            .iter()
            .map(|correspondence| (correspondence.source_record.clone(), correspondence))
            .collect::<BTreeMap<_, _>>();

        let classifications = identity_plan
            .candidates
            .iter()
            .map(|candidate| {
                let record = source_records_by_ref.get(&candidate.source_record).ok_or_else(|| {
                    MergePlanningError::MissingConflictSourceRecord {
                        record: candidate.source_record.clone(),
                    }
                })?;
                Ok(classify_candidate(
                    self.runtime,
                    record,
                    candidate.target_record.clone(),
                    validated_by_source.contains_key(&candidate.source_record),
                    base_version_id,
                    &base_view,
                    &target_view,
                    candidate.match_class.clone(),
                    candidate.reason.clone(),
                ))
            })
            .collect::<Result<Vec<_>, MergePlanningError>>()?;
        let conflict_summary = summarize_classifications(Arc::from(classifications.clone()));

        Ok(ConflictClassifiedMergePlan {
            request: identity_plan.request,
            target_head: identity_plan.target_head,
            source_head: identity_plan.source_head,
            merge_base: identity_plan.merge_base,
            ancestry: identity_plan.ancestry,
            target_delta: identity_plan.target_delta,
            source_delta: identity_plan.source_delta,
            effective_identity_declarations: identity_plan.effective_identity_declarations,
            source_records: identity_plan.source_records,
            candidates: identity_plan.candidates,
            validated_schema_correspondences: identity_plan.validated_schema_correspondences,
            identity_summary: identity_plan.identity_summary,
            classifications: Arc::from(classifications),
            conflict_summary,
        })
    }
}

fn summarize_classifications(
    classifications: Arc<[MergeConflictClassification]>,
) -> ConflictClassificationSummary {
    let mut exact_shared_truth_count = 0;
    let mut source_only_addition_count = 0;
    let mut schema_declared_correspondence_count = 0;
    let mut deletion_conflict_count = 0;
    let mut divergent_visible_state_count = 0;
    let mut relation_endpoint_divergence_count = 0;

    for classification in classifications.iter() {
        match classification.class {
            MergeConflictClass::ExactSharedTruth => exact_shared_truth_count += 1,
            MergeConflictClass::SourceOnlyAddition => source_only_addition_count += 1,
            MergeConflictClass::SchemaDeclaredCorrespondence => {
                schema_declared_correspondence_count += 1
            }
            MergeConflictClass::Deletion(_) => deletion_conflict_count += 1,
            MergeConflictClass::DivergentVisibleState => divergent_visible_state_count += 1,
            MergeConflictClass::RelationEndpointDivergence => {
                relation_endpoint_divergence_count += 1
            }
        }
    }

    ConflictClassificationSummary {
        classified_record_count: classifications.len(),
        exact_shared_truth_count,
        source_only_addition_count,
        schema_declared_correspondence_count,
        deletion_conflict_count,
        divergent_visible_state_count,
        relation_endpoint_divergence_count,
        classifications,
    }
}

fn classify_candidate(
    runtime: &crate::logic::runtime::RelationalRuntime,
    record: &VisibleMergeRecord,
    candidate_target_record: Option<RecordRef>,
    has_validated_schema_correspondence: bool,
    base_version_id: VersionId,
    base_view: &RelationalReadView,
    target_view: &RelationalReadView,
    match_class: IdentityMatchClass,
    reason: IdentityResolutionReason,
) -> MergeConflictClassification {
    let base_record_visible = base_record_is_visible(record, base_version_id, base_view);
    let source_record_visible = source_record_is_visible(record);
    let target_record_visible =
        target_record_is_visible(record, candidate_target_record.as_ref(), target_view);
    let target_record = candidate_target_record
        .as_ref()
        .and_then(|target_record| visible_target_record(target_view, target_record));
    let aspect_evidence =
        aspect_conflict_evidence(runtime, record, target_record.as_ref().or(Some(record)));
    let normalized_match_class = normalize_match_class_for_classification(
        match_class,
        reason.clone(),
        has_validated_schema_correspondence,
    );
    let class = if has_validated_schema_correspondence {
        MergeConflictClass::SchemaDeclaredCorrespondence
    } else {
        classify_record_state(
            record,
            base_record_visible,
            source_record_visible,
            target_record_visible,
            normalized_match_class,
            base_view,
        )
    };
    let relation_evidence = relation_conflict_evidence(record, target_record.as_ref(), base_view);

    MergeConflictClassification {
        record: record.record_ref.clone(),
        class,
        identity_reason: reason,
        validated_schema_correspondence: has_validated_schema_correspondence,
        aspect_evidence: Arc::from(aspect_evidence),
        relation_evidence,
        target_record: candidate_target_record,
        base_record_visible,
        source_record_visible,
        target_record_visible,
    }
}

fn normalize_match_class_for_classification(
    match_class: IdentityMatchClass,
    reason: IdentityResolutionReason,
    has_validated_schema_correspondence: bool,
) -> IdentityMatchClass {
    if reason == IdentityResolutionReason::SchemaDeclaredCorrespondence
        && !has_validated_schema_correspondence
    {
        IdentityMatchClass::Ambiguous
    } else {
        match_class
    }
}

fn classify_record_state(
    record: &VisibleMergeRecord,
    base_record_visible: bool,
    source_record_visible: bool,
    target_record_visible: bool,
    match_class: IdentityMatchClass,
    base_view: &RelationalReadView,
) -> MergeConflictClass {
    match (source_record_visible, target_record_visible) {
        (false, true) => MergeConflictClass::Deletion(classify_source_deleted(record, base_view)),
        (true, false) => {
            if base_record_visible {
                MergeConflictClass::Deletion(classify_target_deleted(record, base_view))
            } else {
                MergeConflictClass::SourceOnlyAddition
            }
        }
        (false, false) => MergeConflictClass::Deletion(DeletionMergeClass::DeletedOnBothSides),
        (true, true) => match match_class {
            IdentityMatchClass::Exact => classify_visible_exact_state(record),
            IdentityMatchClass::Reconciliable => MergeConflictClass::SchemaDeclaredCorrespondence,
            IdentityMatchClass::Ambiguous | IdentityMatchClass::MissingTarget => {
                MergeConflictClass::DivergentVisibleState
            }
        },
    }
}

fn classify_source_deleted(
    record: &VisibleMergeRecord,
    base_view: &RelationalReadView,
) -> DeletionMergeClass {
    match record.record_kind {
        VisibleMergeRecordKind::Entity => match (base_entity_record(base_view, record), record.target_entity.as_ref()) {
            (Some(base), Some(target)) if !entity_state_equal(base, target) => {
                DeletionMergeClass::DeletedVsModified
            }
            _ => DeletionMergeClass::SourceDeletedTargetLive,
        },
        VisibleMergeRecordKind::Relation => {
            match (base_relation_record(base_view, record), record.target_relation.as_ref()) {
                (Some(base), Some(target)) if !relation_endpoints_equal(base, target) => {
                    DeletionMergeClass::DeletedVsRewired
                }
                (Some(base), Some(target)) if !relation_state_equal(base, target) => {
                    DeletionMergeClass::DeletedVsModified
                }
                _ => DeletionMergeClass::SourceDeletedTargetLive,
            }
        }
    }
}

fn classify_target_deleted(
    record: &VisibleMergeRecord,
    base_view: &RelationalReadView,
) -> DeletionMergeClass {
    match record.record_kind {
        VisibleMergeRecordKind::Entity => match (base_entity_record(base_view, record), record.source_entity.as_ref()) {
            (Some(base), Some(source)) if !entity_state_equal(base, source) => {
                DeletionMergeClass::DeletedVsModified
            }
            _ => DeletionMergeClass::SourceLiveTargetDeleted,
        },
        VisibleMergeRecordKind::Relation => {
            match (base_relation_record(base_view, record), record.source_relation.as_ref()) {
                (Some(base), Some(source)) if !relation_endpoints_equal(base, source) => {
                    DeletionMergeClass::DeletedVsRewired
                }
                (Some(base), Some(source)) if !relation_state_equal(base, source) => {
                    DeletionMergeClass::DeletedVsModified
                }
                _ => DeletionMergeClass::SourceLiveTargetDeleted,
            }
        }
    }
}

fn classify_visible_exact_state(record: &VisibleMergeRecord) -> MergeConflictClass {
    match record.record_kind {
        VisibleMergeRecordKind::Entity => match (
            record.source_entity.as_ref(),
            record.target_entity.as_ref(),
        ) {
            (Some(source), Some(target)) => {
                if entity_state_equal(source, target) {
                    MergeConflictClass::ExactSharedTruth
                } else {
                    MergeConflictClass::DivergentVisibleState
                }
            }
            _ => MergeConflictClass::DivergentVisibleState,
        },
        VisibleMergeRecordKind::Relation => match (
            record.source_relation.as_ref(),
            record.target_relation.as_ref(),
        ) {
            (Some(source), Some(target)) => {
                if relation_endpoints_equal(source, target) {
                    if relation_state_equal(source, target) {
                        MergeConflictClass::ExactSharedTruth
                    } else {
                        MergeConflictClass::DivergentVisibleState
                    }
                } else {
                    MergeConflictClass::RelationEndpointDivergence
                }
            }
            _ => MergeConflictClass::DivergentVisibleState,
        },
    }
}

fn entity_state_equal(source: &EntityReadRecord, target: &EntityReadRecord) -> bool {
    source.lifecycle == target.lifecycle && source.payload == target.payload
}

fn relation_state_equal(source: &RelationReadRecord, target: &RelationReadRecord) -> bool {
    source.lifecycle == target.lifecycle && source.payload == target.payload
}

fn relation_endpoints_equal(source: &RelationReadRecord, target: &RelationReadRecord) -> bool {
    source.source == target.source && source.target == target.target
}

fn relation_conflict_evidence(
    record: &VisibleMergeRecord,
    target_record: Option<&VisibleMergeRecord>,
    base_view: &RelationalReadView,
) -> Option<RelationConflictEvidence> {
    if record.record_kind != VisibleMergeRecordKind::Relation {
        return None;
    }
    let base = base_relation_record(base_view, record);
    let source = record.source_relation.as_ref();
    let target = target_record
        .and_then(|record| record.target_relation.as_ref())
        .or(record.target_relation.as_ref());

    let endpoint_continuity = match (source, target, base) {
        (Some(source), Some(target), _) => endpoint_continuity_between(source, target),
        (Some(source), None, Some(base)) => endpoint_continuity_between(base, source),
        (None, Some(target), Some(base)) => endpoint_continuity_between(base, target),
        _ => EndpointContinuityClass::EndpointsStable,
    };
    let relation_continuity = match endpoint_continuity {
        EndpointContinuityClass::EndpointsStable => RelationContinuityClass::PreserveRelationIdentity,
        EndpointContinuityClass::SourceEndpointRewired
        | EndpointContinuityClass::TargetEndpointRewired
        | EndpointContinuityClass::BothEndpointsRewired => {
            RelationContinuityClass::RetireAndIntroduceSuccessor
        }
    };
    let propagation = match endpoint_continuity {
        EndpointContinuityClass::EndpointsStable => RelationConflictPropagation::RelationLocalOnly,
        _ => RelationConflictPropagation::EscalatesToTopologyRegionConflict,
    };
    Some(RelationConflictEvidence {
        endpoint_continuity,
        relation_continuity,
        propagation,
    })
}

fn endpoint_continuity_between(
    left: &RelationReadRecord,
    right: &RelationReadRecord,
) -> EndpointContinuityClass {
    match (left.source == right.source, left.target == right.target) {
        (true, true) => EndpointContinuityClass::EndpointsStable,
        (false, true) => EndpointContinuityClass::SourceEndpointRewired,
        (true, false) => EndpointContinuityClass::TargetEndpointRewired,
        (false, false) => EndpointContinuityClass::BothEndpointsRewired,
    }
}

fn base_entity_record<'a>(
    base_view: &'a RelationalReadView,
    record: &VisibleMergeRecord,
) -> Option<&'a EntityReadRecord> {
    match record.record_ref {
        RecordRef::Entity(entity_id) => base_view.get_entity(entity_id),
        RecordRef::Relation(_) => None,
    }
}

fn base_relation_record<'a>(
    base_view: &'a RelationalReadView,
    record: &VisibleMergeRecord,
) -> Option<&'a RelationReadRecord> {
    match record.record_ref {
        RecordRef::Relation(relation_id) => base_view.get_relation(relation_id),
        RecordRef::Entity(_) => None,
    }
}

fn aspect_conflict_evidence(
    runtime: &crate::logic::runtime::RelationalRuntime,
    source_record: &VisibleMergeRecord,
    target_record: Option<&VisibleMergeRecord>,
) -> Vec<AspectConflictEvidence> {
    let Some(plan) = lowered_plan_for_record(runtime, source_record) else {
        return Vec::new();
    };
    plan.executable_bindings
        .iter()
        .map(|binding| AspectConflictEvidence {
            aspect_key: binding.aspect_key.clone(),
            comparison: compare_binding(source_record, target_record, binding),
        })
        .collect()
}

fn visible_target_record(
    target_view: &RelationalReadView,
    target_record: &RecordRef,
) -> Option<VisibleMergeRecord> {
    match target_record {
        RecordRef::Entity(entity_id) => {
            let entity = target_view.get_entity(*entity_id).cloned()?;
            Some(VisibleMergeRecord {
                record_ref: RecordRef::Entity(*entity_id),
                record_kind: VisibleMergeRecordKind::Entity,
                kind_id: Some(entity.kind.kind_id),
                source_kind_id: None,
                target_kind_id: Some(entity.kind.kind_id),
                lineage_id: entity.lineage_id,
                source_lineage_id: None,
                target_lineage_id: entity.lineage_id,
                source_entity: None,
                target_entity: Some(entity),
                source_relation: None,
                target_relation: None,
            })
        }
        RecordRef::Relation(relation_id) => {
            let relation = target_view.get_relation(*relation_id).cloned()?;
            Some(VisibleMergeRecord {
                record_ref: RecordRef::Relation(*relation_id),
                record_kind: VisibleMergeRecordKind::Relation,
                kind_id: Some(relation.kind.kind_id),
                source_kind_id: None,
                target_kind_id: Some(relation.kind.kind_id),
                lineage_id: None,
                source_lineage_id: None,
                target_lineage_id: None,
                source_entity: None,
                target_entity: None,
                source_relation: None,
                target_relation: Some(relation),
            })
        }
    }
}

fn compare_binding(
    source_record: &VisibleMergeRecord,
    target_record: Option<&VisibleMergeRecord>,
    binding: &LoweredAspectBinding,
) -> AspectComparisonState {
    let source = extract_binding_component(source_record, binding, BindingSide::Source);
    let target =
        target_record.and_then(|record| extract_binding_component(record, binding, BindingSide::Target));
    match (source, target) {
        (Some(source), Some(target)) if source == target => AspectComparisonState::Equal,
        (Some(_), Some(_)) => AspectComparisonState::Divergent,
        (Some(_), None) => AspectComparisonState::SourceOnly,
        (None, Some(_)) => AspectComparisonState::TargetOnly,
        (None, None) => AspectComparisonState::Unavailable,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum AspectComponent {
    Json(Value),
    Endpoint(crate::identity::data::EntityId),
    Lifecycle(RecordLifecycleState),
    Opaque(Vec<u8>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BindingSide {
    Source,
    Target,
}

fn extract_binding_component(
    record: &VisibleMergeRecord,
    binding: &LoweredAspectBinding,
    side: BindingSide,
) -> Option<AspectComponent> {
    let entity = match side {
        BindingSide::Source => record.source_entity.as_ref(),
        BindingSide::Target => record.target_entity.as_ref(),
    };
    let relation = match side {
        BindingSide::Source => record.source_relation.as_ref(),
        BindingSide::Target => record.target_relation.as_ref(),
    };

    match (&record.record_kind, &binding.binding_kind) {
        (
            VisibleMergeRecordKind::Entity,
            LoweredExecutableAspectBindingKind::EntityJsonScalarField { field },
        ) => entity.and_then(|entity| {
            interned_field_name(field).and_then(|name| json_component(&entity.payload, name))
        }),
        (
            VisibleMergeRecordKind::Relation,
            LoweredExecutableAspectBindingKind::RelationJsonScalarField { field },
        ) => relation
            .and_then(|relation| relation.payload.as_ref())
            .and_then(|payload| interned_field_name(field).and_then(|name| json_component(payload, name))),
        (
            VisibleMergeRecordKind::Relation,
            LoweredExecutableAspectBindingKind::RelationSourceEndpointIdentity,
        ) => relation.map(|relation| AspectComponent::Endpoint(relation.source)),
        (
            VisibleMergeRecordKind::Relation,
            LoweredExecutableAspectBindingKind::RelationTargetEndpointIdentity,
        ) => relation.map(|relation| AspectComponent::Endpoint(relation.target)),
        (_, LoweredExecutableAspectBindingKind::LifecycleTransitionEquality) => entity
            .map(|entity| AspectComponent::Lifecycle(entity.lifecycle))
            .or_else(|| relation.map(|relation| AspectComponent::Lifecycle(relation.lifecycle))),
        (
            VisibleMergeRecordKind::Entity,
            LoweredExecutableAspectBindingKind::OpaqueWholePayloadBytes,
        ) => opaque_component(entity.map(|entity| &entity.payload)),
        (
            VisibleMergeRecordKind::Relation,
            LoweredExecutableAspectBindingKind::OpaqueWholePayloadBytes,
        ) => opaque_component(relation.and_then(|relation| relation.payload.as_ref())),
        _ => None,
    }
}

fn json_component(payload: &RecordPayload, field_name: &str) -> Option<AspectComponent> {
    payload
        .as_json()?
        .get(field_name)
        .cloned()
        .map(AspectComponent::Json)
}

fn opaque_component(payload: Option<&RecordPayload>) -> Option<AspectComponent> {
    match payload? {
        RecordPayload::StructuredJson(_) => None,
        RecordPayload::OpaqueBytes(bytes) => Some(AspectComponent::Opaque(bytes.clone())),
    }
}

fn interned_field_name(field: &InternedString) -> Option<&str> {
    match field {
        InternedString::Raw(raw) => Some(raw.as_str()),
        InternedString::Symbol(_) => None,
    }
}

fn source_record_is_visible(record: &VisibleMergeRecord) -> bool {
    match record.record_kind {
        VisibleMergeRecordKind::Entity => record
            .source_entity
            .as_ref()
            .is_some_and(is_visible_lifecycle),
        VisibleMergeRecordKind::Relation => record
            .source_relation
            .as_ref()
            .is_some_and(is_visible_lifecycle),
    }
}

fn target_record_is_visible(
    record: &VisibleMergeRecord,
    candidate_target_record: Option<&RecordRef>,
    target_view: &RelationalReadView,
) -> bool {
    if let Some(target_record) = candidate_target_record {
        return record_visible_in_view(target_view, target_record);
    }
    match record.record_kind {
        VisibleMergeRecordKind::Entity => record.target_entity.as_ref().is_some_and(is_visible_lifecycle),
        VisibleMergeRecordKind::Relation => record
            .target_relation
            .as_ref()
            .is_some_and(is_visible_lifecycle),
    }
}

fn record_visible_in_view(view: &RelationalReadView, record_ref: &RecordRef) -> bool {
    match record_ref {
        RecordRef::Entity(entity_id) => view.get_entity(*entity_id).is_some_and(is_visible_lifecycle),
        RecordRef::Relation(relation_id) => view
            .get_relation(*relation_id)
            .is_some_and(is_visible_lifecycle),
    }
}

fn base_record_is_visible(
    record: &VisibleMergeRecord,
    base_version_id: VersionId,
    base_view: &RelationalReadView,
) -> bool {
    match record.record_kind {
        VisibleMergeRecordKind::Entity => record
            .source_entity
            .as_ref()
            .or(record.target_entity.as_ref())
            .map(|entity| record_existed_at_base(entity.created_at_version, entity.retired_at_version, base_version_id))
            .unwrap_or_else(|| record_visible_in_view(base_view, &record.record_ref)),
        VisibleMergeRecordKind::Relation => record
            .source_relation
            .as_ref()
            .or(record.target_relation.as_ref())
            .map(|relation| {
                record_existed_at_base(
                    relation.created_at_version,
                    relation.retired_at_version,
                    base_version_id,
                )
            })
            .unwrap_or_else(|| record_visible_in_view(base_view, &record.record_ref)),
    }
}

fn record_existed_at_base(
    created_at_version: VersionId,
    retired_at_version: Option<VersionId>,
    base_version_id: VersionId,
) -> bool {
    created_at_version <= base_version_id
        && retired_at_version
            .map(|retired_at| retired_at > base_version_id)
            .unwrap_or(true)
}

fn is_visible_lifecycle<T>(record: &T) -> bool
where
    T: MergeLifecycleView,
{
    !matches!(
        record.lifecycle(),
        RecordLifecycleState::DeletedRetained
            | RecordLifecycleState::RetainedDanglingForAudit
            | RecordLifecycleState::Reclaimable
            | RecordLifecycleState::Reusable
    )
}

trait MergeLifecycleView {
    fn lifecycle(&self) -> RecordLifecycleState;
}

impl MergeLifecycleView for EntityReadRecord {
    fn lifecycle(&self) -> RecordLifecycleState {
        self.lifecycle
    }
}

impl MergeLifecycleView for RelationReadRecord {
    fn lifecycle(&self) -> RecordLifecycleState {
        self.lifecycle
    }
}
