use std::collections::BTreeMap;
use std::sync::Arc;

use crate::identity::data::{EntityId, LineageId, RelationId};
use crate::merge::data::{
    BranchTouchedRecordDelta, HistoryScopedMergePlan, IdentityBasisDeclaration, IdentityBasisKind,
    IdentityBasisScope, IdentityDiscoverySummary, IdentityMatchCandidate, IdentityMatchClass,
    IdentityResolutionReason, IdentityScopedMergePlan, MergeAncestrySummary, MergePlanningError,
    MergePlanningRequest, MergeRecordIdentity, SchemaDeclaredCorrespondenceValidationSummary,
    ValidatedSchemaDeclaredCorrespondence, VisibleMergeRecord, VisibleMergeRecordKind,
};
use crate::merge::logic::aspect_plan_lookup::lowered_plan_for_record;
use crate::merge::logic::naming::resolve_interned_string;
use crate::merge::logic::planning::branch_delta_summary;
use crate::merge::logic::MergeAccess;
use crate::payloads::data::RecordPayload;
use crate::publication::patch::data::AspectKey;
use crate::schema::data::LoweredExecutableAspectBindingKind;
use crate::storage::data::RecordLifecycleState;
use crate::storage::data::{EntityReadRecord, RelationReadRecord, RelationalReadView};
use crate::transactions::data::RecordRef;
use serde_json::Value;

impl<'runtime> MergeAccess<'runtime> {
    pub(crate) fn plan_identity_scope(
        &self,
        request: MergePlanningRequest,
    ) -> Result<IdentityScopedMergePlan, MergePlanningError> {
        let history_plan = self.plan_history_scope(request)?;
        Ok(self.discover_identity_scope(history_plan))
    }

    fn discover_identity_scope(&self, history_plan: HistoryScopedMergePlan) -> IdentityScopedMergePlan {
        let target_view = self
            .runtime
            .visibility_reads()
            .read_version(history_plan.target_head.version_id);
        let source_view = self
            .runtime
            .visibility_reads()
            .read_version(history_plan.source_head.version_id);

        let ancestry = MergeAncestrySummary {
            merge_base_rule: history_plan.merge_base.rule,
            merge_base_commit_id: history_plan.merge_base.commit_id,
            supporting_left_ancestor_count: history_plan.merge_base.supporting_left_ancestors.len(),
            supporting_right_ancestor_count: history_plan.merge_base.supporting_right_ancestors.len(),
            target: branch_delta_summary(&history_plan.target_head, &history_plan.target_delta),
            source: branch_delta_summary(&history_plan.source_head, &history_plan.source_delta),
        };

        let source_records = source_visible_records(
            &source_view,
            &target_view,
            history_plan.source_delta.touched_records.as_ref(),
        );
        let effective_identity_declarations =
            effective_identity_declarations(self.runtime, &source_records);
        let target_index = TargetIdentityIndex::new(
            self.runtime,
            &target_view,
            source_records.as_slice(),
            &effective_identity_declarations,
        );
        let candidates = source_records
            .iter()
            .map(|record| {
                discover_identity_candidate(
                    self.runtime,
                    record,
                    &effective_identity_declarations,
                    &target_index,
                )
            })
            .collect::<Vec<_>>();
        let validated_schema_correspondences =
            validate_schema_declared_correspondences(candidates.as_slice());
        let identity_summary = identity_summary(
            Arc::from(effective_identity_declarations.clone()),
            Arc::from(candidates.clone()),
            Arc::from(validated_schema_correspondences.clone()),
        );

        IdentityScopedMergePlan {
            request: history_plan.request,
            target_head: history_plan.target_head,
            source_head: history_plan.source_head,
            merge_base: history_plan.merge_base,
            ancestry,
            target_delta: history_plan.target_delta,
            source_delta: history_plan.source_delta,
            effective_identity_declarations: Arc::from(effective_identity_declarations),
            source_records: Arc::from(source_records),
            candidates: Arc::from(candidates),
            validated_schema_correspondences: Arc::from(validated_schema_correspondences),
            identity_summary,
        }
    }
}

fn source_visible_records(
    source_view: &RelationalReadView,
    target_view: &RelationalReadView,
    touched_records: &[BranchTouchedRecordDelta],
) -> Vec<VisibleMergeRecord> {
    touched_records
        .iter()
        .map(|delta| visible_record_for_ref(source_view, target_view, delta.target.clone()))
        .collect()
}

fn visible_record_for_ref(
    source_view: &RelationalReadView,
    target_view: &RelationalReadView,
    record_ref: RecordRef,
) -> VisibleMergeRecord {
    match record_ref {
        RecordRef::Entity(entity_id) => {
            let source = source_view.get_entity(entity_id).cloned();
            let target = target_view.get_entity(entity_id).cloned();
            VisibleMergeRecord {
                record_ref: RecordRef::Entity(entity_id),
                record_kind: VisibleMergeRecordKind::Entity,
                kind_id: source
                    .as_ref()
                    .map(|record| record.kind.kind_id)
                    .or_else(|| target.as_ref().map(|record| record.kind.kind_id)),
                source_kind_id: source.as_ref().map(|record| record.kind.kind_id),
                target_kind_id: target.as_ref().map(|record| record.kind.kind_id),
                lineage_id: source
                    .as_ref()
                    .and_then(|record| record.lineage_id)
                    .or_else(|| target.as_ref().and_then(|record| record.lineage_id)),
                source_lineage_id: source.as_ref().and_then(|record| record.lineage_id),
                target_lineage_id: target.as_ref().and_then(|record| record.lineage_id),
                source_entity: source,
                target_entity: target,
                source_relation: None,
                target_relation: None,
            }
        }
        RecordRef::Relation(relation_id) => {
            let source = source_view.get_relation(relation_id).cloned();
            let target = target_view.get_relation(relation_id).cloned();
            VisibleMergeRecord {
                record_ref: RecordRef::Relation(relation_id),
                record_kind: VisibleMergeRecordKind::Relation,
                kind_id: source
                    .as_ref()
                    .map(|record| record.kind.kind_id)
                    .or_else(|| target.as_ref().map(|record| record.kind.kind_id)),
                source_kind_id: source.as_ref().map(|record| record.kind.kind_id),
                target_kind_id: target.as_ref().map(|record| record.kind.kind_id),
                lineage_id: None,
                source_lineage_id: None,
                target_lineage_id: None,
                source_entity: None,
                target_entity: None,
                source_relation: source,
                target_relation: target,
            }
        }
    }
}

fn effective_identity_declarations(
    runtime: &crate::logic::runtime::RelationalRuntime,
    source_records: &[VisibleMergeRecord],
) -> Vec<IdentityBasisDeclaration> {
    let mut declarations = Vec::<IdentityBasisDeclaration>::new();
    let registry = &runtime.config().schema.registry;
    for record in source_records {
        match record.record_kind {
            VisibleMergeRecordKind::Entity => {
                let Some(kind_id) = record.kind_id else {
                    continue;
                };
                if let Ok(schema_declarations) = registry.entity_identity_declarations(kind_id) {
                    for declaration in schema_declarations {
                        push_unique_declaration(&mut declarations, declaration.clone());
                    }
                }
            }
            VisibleMergeRecordKind::Relation => {
                let Some(kind_id) = record.kind_id else {
                    continue;
                };
                if let Ok(schema_declarations) = registry.relation_identity_declarations(kind_id) {
                    for declaration in schema_declarations {
                        push_unique_declaration(&mut declarations, declaration.clone());
                    }
                }
            }
        }
    }
    declarations
}

fn push_unique_declaration(
    declarations: &mut Vec<IdentityBasisDeclaration>,
    candidate: IdentityBasisDeclaration,
) {
    if !declarations.iter().any(|existing| existing == &candidate) {
        declarations.push(candidate);
    }
}

fn identity_summary(
    effective_declarations: Arc<[IdentityBasisDeclaration]>,
    candidates: Arc<[IdentityMatchCandidate]>,
    validated_schema_correspondences: Arc<[ValidatedSchemaDeclaredCorrespondence]>,
) -> IdentityDiscoverySummary {
    let mut exact_match_count = 0;
    let mut reconciliable_match_count = 0;
    let mut ambiguous_match_count = 0;
    let mut missing_target_count = 0;
    let mut storage_basis_candidate_count = 0;
    let mut lineage_basis_candidate_count = 0;
    let mut structural_basis_candidate_count = 0;
    let mut custom_basis_candidate_count = 0;
    let schema_declared_candidates = candidates
        .iter()
        .filter(|candidate| candidate.reason == IdentityResolutionReason::SchemaDeclaredCorrespondence)
        .collect::<Vec<_>>();
    let mut schema_source_counts = BTreeMap::<RecordRef, usize>::new();
    let mut schema_target_counts = BTreeMap::<RecordRef, usize>::new();
    for candidate in &schema_declared_candidates {
        *schema_source_counts
            .entry(candidate.source_record.clone())
            .or_insert(0) += 1;
        if let Some(target_record) = &candidate.target_record {
            *schema_target_counts.entry(target_record.clone()).or_insert(0) += 1;
        }
    }
    let rejected_non_unique_source_count = schema_source_counts
        .values()
        .filter(|count| **count > 1)
        .count();
    let rejected_non_unique_target_count = schema_target_counts
        .values()
        .filter(|count| **count > 1)
        .count();

    for candidate in candidates.iter() {
        match candidate.match_class {
            IdentityMatchClass::Exact => exact_match_count += 1,
            IdentityMatchClass::Reconciliable => reconciliable_match_count += 1,
            IdentityMatchClass::Ambiguous => ambiguous_match_count += 1,
            IdentityMatchClass::MissingTarget => missing_target_count += 1,
        }
        match &candidate.basis {
            IdentityBasisKind::StorageIdentity => storage_basis_candidate_count += 1,
            IdentityBasisKind::LineageIdentity => lineage_basis_candidate_count += 1,
            IdentityBasisKind::StructuralFingerprint => structural_basis_candidate_count += 1,
            IdentityBasisKind::DeclaredKeySet(_) | IdentityBasisKind::Custom(_) => {
                custom_basis_candidate_count += 1
            }
        }
    }

    IdentityDiscoverySummary {
        effective_declarations,
        candidate_count: candidates.len(),
        exact_match_count,
        reconciliable_match_count,
        schema_declared_correspondence: SchemaDeclaredCorrespondenceValidationSummary {
            candidate_count: schema_declared_candidates.len(),
            validated_count: validated_schema_correspondences.len(),
            rejected_non_unique_source_count,
            rejected_non_unique_target_count,
        },
        ambiguous_match_count,
        missing_target_count,
        storage_basis_candidate_count,
        lineage_basis_candidate_count,
        structural_basis_candidate_count,
        custom_basis_candidate_count,
        candidates,
    }
}

fn validate_schema_declared_correspondences(
    candidates: &[IdentityMatchCandidate],
) -> Vec<ValidatedSchemaDeclaredCorrespondence> {
    let raw = candidates
        .iter()
        .filter_map(|candidate| {
            if candidate.reason != IdentityResolutionReason::SchemaDeclaredCorrespondence {
                return None;
            }
            let scope = candidate.scope.clone()?;
            let target_record = candidate.target_record.clone()?;
            Some(ValidatedSchemaDeclaredCorrespondence {
                scope,
                basis: candidate.basis.clone(),
                source_record: candidate.source_record.clone(),
                target_record,
                candidate_count_for_source: 0,
                candidate_count_for_target: 0,
            })
        })
        .collect::<Vec<_>>();

    let mut source_counts = BTreeMap::<RecordRef, usize>::new();
    let mut target_counts = BTreeMap::<RecordRef, usize>::new();
    for correspondence in &raw {
        *source_counts
            .entry(correspondence.source_record.clone())
            .or_insert(0) += 1;
        *target_counts
            .entry(correspondence.target_record.clone())
            .or_insert(0) += 1;
    }

    raw.into_iter()
        .filter_map(|correspondence| {
            let source_count = source_counts
                .get(&correspondence.source_record)
                .copied()
                .unwrap_or(0);
            let target_count = target_counts
                .get(&correspondence.target_record)
                .copied()
                .unwrap_or(0);
            (source_count == 1 && target_count == 1).then_some(ValidatedSchemaDeclaredCorrespondence {
                candidate_count_for_source: source_count,
                candidate_count_for_target: target_count,
                ..correspondence
            })
        })
        .collect()
}

fn discover_identity_candidate(
    runtime: &crate::logic::runtime::RelationalRuntime,
    source: &VisibleMergeRecord,
    effective_identity_declarations: &[IdentityBasisDeclaration],
    target_index: &TargetIdentityIndex,
) -> IdentityMatchCandidate {
    let kind_scope = source.kind_id.map(|kind_id| match source.record_kind {
        VisibleMergeRecordKind::Entity => IdentityBasisScope::EntityKind(kind_id),
        VisibleMergeRecordKind::Relation => IdentityBasisScope::RelationKind(kind_id),
    });
    let declared_bases = kind_scope
        .as_ref()
        .map(|scope| {
            effective_identity_declarations
                .iter()
                .filter(|declaration| {
                    identity_declaration_applies_to_record(runtime, source, declaration, scope)
                })
                .cloned()
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| {
            kind_scope
                .clone()
                .map(|scope| {
                    vec![IdentityBasisDeclaration {
                        scope,
                        basis: IdentityBasisKind::StorageIdentity,
                    }]
                })
                .unwrap_or_default()
        });
    let source_identity = source_identity_for_basis(source, declared_bases.first().map(|declaration| &declaration.basis));

    for declaration in &declared_bases {
        match &declaration.basis {
            IdentityBasisKind::StorageIdentity => {
                if let Some(target_record) = target_storage_match(target_index, &source.record_ref) {
                    return IdentityMatchCandidate {
                        scope: Some(declaration.scope.clone()),
                        source_record: source.record_ref.clone(),
                        target_record: Some(target_record.clone()),
                        source: source_identity_for_basis(source, Some(&declaration.basis)),
                        target: Some(MergeRecordIdentity::StorageRecord(target_record)),
                        match_class: IdentityMatchClass::Exact,
                        reason: IdentityResolutionReason::ExactStorageIdentity,
                        basis: declaration.basis.clone(),
                    };
                }
            }
            IdentityBasisKind::LineageIdentity => {
                if let Some(lineage_id) = source.source_lineage_id {
                    match target_index.entities_by_lineage.get(&lineage_id) {
                        Some(matches) if matches.len() == 1 => {
                            let target_record = RecordRef::Entity(matches[0]);
                            return IdentityMatchCandidate {
                                scope: Some(declaration.scope.clone()),
                                source_record: source.record_ref.clone(),
                                target_record: Some(target_record),
                                source: MergeRecordIdentity::Lineage(lineage_id),
                                target: Some(MergeRecordIdentity::Lineage(lineage_id)),
                                match_class: IdentityMatchClass::Exact,
                                reason: IdentityResolutionReason::ExactLineageIdentity,
                                basis: declaration.basis.clone(),
                            };
                        }
                        Some(matches) if matches.len() > 1 => {
                            return IdentityMatchCandidate {
                                scope: Some(declaration.scope.clone()),
                                source_record: source.record_ref.clone(),
                                target_record: None,
                                source: MergeRecordIdentity::Lineage(lineage_id),
                                target: None,
                                match_class: IdentityMatchClass::Ambiguous,
                                reason: IdentityResolutionReason::DeclaredBasisAmbiguousVisibleTargetMatch,
                                basis: declaration.basis.clone(),
                            };
                        }
                        _ => {}
                    }
                } else {
                    return IdentityMatchCandidate {
                        scope: Some(declaration.scope.clone()),
                        source_record: source.record_ref.clone(),
                        target_record: None,
                        source: source_identity.clone(),
                        target: None,
                        match_class: IdentityMatchClass::MissingTarget,
                        reason: IdentityResolutionReason::DeclaredBasisUnavailableOnSource,
                        basis: declaration.basis.clone(),
                    };
                }
            }
            IdentityBasisKind::DeclaredKeySet(keys) => match declared_key_set_match(
                runtime,
                source,
                keys.as_ref(),
                target_index,
            ) {
                DeclaredKeySetMatch::ExactTarget(target_record) => {
                    return IdentityMatchCandidate {
                        scope: Some(declaration.scope.clone()),
                        source_record: source.record_ref.clone(),
                        target_record: Some(target_record.clone()),
                        source: MergeRecordIdentity::StorageRecord(source.record_ref.clone()),
                        target: Some(MergeRecordIdentity::StorageRecord(target_record)),
                        match_class: IdentityMatchClass::Reconciliable,
                        reason: IdentityResolutionReason::SchemaDeclaredCorrespondence,
                        basis: declaration.basis.clone(),
                    };
                }
                DeclaredKeySetMatch::MissingSourceEvidence => {
                    return IdentityMatchCandidate {
                        scope: Some(declaration.scope.clone()),
                        source_record: source.record_ref.clone(),
                        target_record: None,
                        source: source_identity.clone(),
                        target: None,
                        match_class: IdentityMatchClass::MissingTarget,
                        reason: IdentityResolutionReason::DeclaredBasisUnavailableOnSource,
                        basis: declaration.basis.clone(),
                    };
                }
                DeclaredKeySetMatch::AmbiguousTarget => {
                    return IdentityMatchCandidate {
                        scope: Some(declaration.scope.clone()),
                        source_record: source.record_ref.clone(),
                        target_record: None,
                        source: source_identity.clone(),
                        target: None,
                        match_class: IdentityMatchClass::Ambiguous,
                        reason: IdentityResolutionReason::DeclaredBasisAmbiguousVisibleTargetMatch,
                        basis: declaration.basis.clone(),
                    };
                }
                DeclaredKeySetMatch::NoTargetMatch => {}
            },
            _ => {}
        }
    }

    let fallback_basis = declared_bases
        .first()
        .map(|declaration| declaration.basis.clone())
        .unwrap_or(IdentityBasisKind::StorageIdentity);
    IdentityMatchCandidate {
        scope: kind_scope,
        source_record: source.record_ref.clone(),
        target_record: None,
        source: source_identity,
        target: None,
        match_class: IdentityMatchClass::MissingTarget,
        reason: IdentityResolutionReason::DeclaredBasisNoVisibleTargetMatch,
        basis: fallback_basis,
    }
}

fn source_identity_for_basis(
    source: &VisibleMergeRecord,
    basis: Option<&IdentityBasisKind>,
) -> MergeRecordIdentity {
    match basis {
        Some(IdentityBasisKind::LineageIdentity) => source
            .source_lineage_id
            .map(MergeRecordIdentity::Lineage)
            .unwrap_or_else(|| MergeRecordIdentity::StorageRecord(source.record_ref.clone())),
        _ => MergeRecordIdentity::StorageRecord(source.record_ref.clone()),
    }
}

fn declared_key_set_match(
    runtime: &crate::logic::runtime::RelationalRuntime,
    source: &VisibleMergeRecord,
    aspect_keys: &[AspectKey],
    target_index: &TargetIdentityIndex,
) -> DeclaredKeySetMatch {
    let Some(source_signature) = extract_declared_key_signature(runtime, source, aspect_keys) else {
        return DeclaredKeySetMatch::MissingSourceEvidence;
    };
    let matching_targets = target_index
        .declared_key_matches(source.record_kind.clone(), source.kind_id, aspect_keys)
        .get(&source_signature)
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .filter(|target_record| *target_record != source.record_ref)
        .collect::<Vec<_>>();

    match matching_targets.as_slice() {
        [] => DeclaredKeySetMatch::NoTargetMatch,
        [target_record] => DeclaredKeySetMatch::ExactTarget(target_record.clone()),
        _ => DeclaredKeySetMatch::AmbiguousTarget,
    }
}

fn extract_declared_key_signature(
    runtime: &crate::logic::runtime::RelationalRuntime,
    record: &VisibleMergeRecord,
    aspect_keys: &[AspectKey],
) -> Option<DeclaredIdentityDigest> {
    let plan = lowered_plan_for_record(runtime, record)?;
    let bindings = aspect_keys
        .iter()
        .map(|aspect_key| {
            plan.executable_bindings
                .iter()
                .find(|binding| binding.aspect_key == *aspect_key)
        })
        .collect::<Option<Vec<_>>>()?;
    let components = bindings
        .into_iter()
        .map(|binding| extract_identity_component(runtime, record, binding))
        .collect::<Option<Vec<_>>>()?;
    Some(declared_identity_signature(&components))
}

fn extract_identity_component(
    runtime: &crate::logic::runtime::RelationalRuntime,
    record: &VisibleMergeRecord,
    binding: &crate::schema::data::LoweredAspectBinding,
) -> Option<DeclaredIdentityComponent> {
    let entity = record.source_entity.as_ref();
    let relation = record.source_relation.as_ref();
    match (&record.record_kind, entity, relation, &binding.binding_kind) {
        (
            VisibleMergeRecordKind::Entity,
            Some(entity),
            _,
            LoweredExecutableAspectBindingKind::EntityJsonScalarField { field },
        ) => resolve_interned_string(runtime, field)
            .and_then(|field_name| extract_json_component(&entity.payload, field_name.as_ref())),
        (
            VisibleMergeRecordKind::Relation,
            _,
            Some(relation),
            LoweredExecutableAspectBindingKind::RelationJsonScalarField { field },
        ) => relation
            .payload
            .as_ref()
            .and_then(|payload| {
                resolve_interned_string(runtime, field)
                    .and_then(|field_name| extract_json_component(payload, field_name.as_ref()))
            }),
        (
            VisibleMergeRecordKind::Relation,
            _,
            Some(relation),
            LoweredExecutableAspectBindingKind::RelationSourceEndpointIdentity,
        ) => Some(DeclaredIdentityComponent::EntityEndpoint(relation.source)),
        (
            VisibleMergeRecordKind::Relation,
            _,
            Some(relation),
            LoweredExecutableAspectBindingKind::RelationTargetEndpointIdentity,
        ) => Some(DeclaredIdentityComponent::EntityEndpoint(relation.target)),
        (
            _,
            _,
            _,
            LoweredExecutableAspectBindingKind::LifecycleTransitionEquality,
        ) => lifecycle_of_record(record).map(DeclaredIdentityComponent::Lifecycle),
        (
            VisibleMergeRecordKind::Entity,
            Some(entity),
            _,
            LoweredExecutableAspectBindingKind::OpaqueWholePayloadBytes,
        ) => extract_opaque_component(&entity.payload),
        (
            VisibleMergeRecordKind::Relation,
            _,
            Some(relation),
            LoweredExecutableAspectBindingKind::OpaqueWholePayloadBytes,
        ) => relation
            .payload
            .as_ref()
            .and_then(extract_opaque_component),
        _ => None,
    }
}

fn extract_json_component(payload: &RecordPayload, field_name: &str) -> Option<DeclaredIdentityComponent> {
    payload
        .as_json()?
        .get(field_name)
        .cloned()
        .map(DeclaredIdentityComponent::JsonScalar)
}

fn extract_opaque_component(payload: &RecordPayload) -> Option<DeclaredIdentityComponent> {
    match payload {
        RecordPayload::StructuredJson(_) => None,
        RecordPayload::OpaqueBytes(bytes) => Some(DeclaredIdentityComponent::OpaqueBytes(bytes.clone())),
    }
}

fn lifecycle_of_record(record: &VisibleMergeRecord) -> Option<RecordLifecycleState> {
    match (record.source_entity.as_ref(), record.source_relation.as_ref()) {
        (Some(entity), _) => Some(entity.lifecycle),
        (_, Some(relation)) => Some(relation.lifecycle),
        _ => None,
    }
}

fn target_storage_match(target_index: &TargetIdentityIndex, record_ref: &RecordRef) -> Option<RecordRef> {
    match record_ref {
        RecordRef::Entity(entity_id) => target_index
            .entities_by_storage
            .contains_key(entity_id)
            .then_some(RecordRef::Entity(*entity_id)),
        RecordRef::Relation(relation_id) => target_index
            .relations_by_storage
            .contains_key(relation_id)
            .then_some(RecordRef::Relation(*relation_id)),
    }
}

struct TargetIdentityIndex {
    entities_by_storage: BTreeMap<EntityId, EntityReadRecord>,
    entities_by_lineage: BTreeMap<LineageId, Vec<EntityId>>,
    relations_by_storage: BTreeMap<RelationId, RelationReadRecord>,
    declared_key_indexes: BTreeMap<DeclaredKeyIndexKey, BTreeMap<DeclaredIdentityDigest, Vec<RecordRef>>>,
}

impl TargetIdentityIndex {
    fn new(
        runtime: &crate::logic::runtime::RelationalRuntime,
        target_view: &RelationalReadView,
        source_records: &[VisibleMergeRecord],
        effective_identity_declarations: &[IdentityBasisDeclaration],
    ) -> Self {
        let relevant_entity_kinds = source_records
            .iter()
            .filter(|record| record.record_kind == VisibleMergeRecordKind::Entity)
            .flat_map(|record| [record.source_kind_id, record.target_kind_id, record.kind_id])
            .flatten()
            .collect::<std::collections::BTreeSet<_>>();
        let relevant_relation_kinds = source_records
            .iter()
            .filter(|record| record.record_kind == VisibleMergeRecordKind::Relation)
            .flat_map(|record| [record.source_kind_id, record.target_kind_id, record.kind_id])
            .flatten()
            .collect::<std::collections::BTreeSet<_>>();
        let target_entities = target_view
            .entities()
            .iter()
            .filter(|record| relevant_entity_kinds.contains(&record.kind.kind_id))
            .cloned()
            .collect::<Vec<_>>();
        let target_relations = target_view
            .relations()
            .iter()
            .filter(|record| relevant_relation_kinds.contains(&record.kind.kind_id))
            .cloned()
            .collect::<Vec<_>>();
        let entities_by_storage = target_view
            .entities()
            .iter()
            .filter(|record| relevant_entity_kinds.contains(&record.kind.kind_id))
            .cloned()
            .map(|record| (record.entity_id, record))
            .collect::<BTreeMap<_, _>>();
        let relations_by_storage = target_view
            .relations()
            .iter()
            .filter(|record| relevant_relation_kinds.contains(&record.kind.kind_id))
            .cloned()
            .map(|record| (record.relation_id, record))
            .collect::<BTreeMap<_, _>>();
        let mut entities_by_lineage = BTreeMap::<LineageId, Vec<EntityId>>::new();
        for entity in &target_entities {
            if let Some(lineage_id) = entity.lineage_id {
                entities_by_lineage
                    .entry(lineage_id)
                    .or_default()
                    .push(entity.entity_id);
            }
        }
        let visible_records = target_entities
            .into_iter()
            .map(|entity| VisibleMergeRecord {
                record_ref: RecordRef::Entity(entity.entity_id),
                record_kind: VisibleMergeRecordKind::Entity,
                kind_id: Some(entity.kind.kind_id),
                source_kind_id: Some(entity.kind.kind_id),
                target_kind_id: None,
                lineage_id: entity.lineage_id,
                source_lineage_id: entity.lineage_id,
                target_lineage_id: None,
                source_entity: Some(entity),
                target_entity: None,
                source_relation: None,
                target_relation: None,
            })
            .chain(target_relations.into_iter().map(|relation| VisibleMergeRecord {
                record_ref: RecordRef::Relation(relation.relation_id),
                record_kind: VisibleMergeRecordKind::Relation,
                kind_id: Some(relation.kind.kind_id),
                source_kind_id: Some(relation.kind.kind_id),
                target_kind_id: None,
                lineage_id: None,
                source_lineage_id: None,
                target_lineage_id: None,
                source_entity: None,
                target_entity: None,
                source_relation: Some(relation),
                target_relation: None,
            }))
            .collect::<Vec<_>>();
        let declared_key_indexes =
            build_declared_key_indexes(runtime, &visible_records, effective_identity_declarations);
        runtime.performance_access().count_merge_identity_target_indexing(
            target_view.entities().len() + target_view.relations().len(),
            visible_records.len(),
        );
        Self {
            entities_by_storage,
            entities_by_lineage,
            relations_by_storage,
            declared_key_indexes,
        }
    }

    fn declared_key_matches(
        &self,
        record_kind: VisibleMergeRecordKind,
        kind_id: Option<crate::identity::data::KindId>,
        aspect_keys: &[AspectKey],
    ) -> &BTreeMap<DeclaredIdentityDigest, Vec<RecordRef>> {
        let key = DeclaredKeyIndexKey {
            record_kind,
            kind_id,
            aspect_keys: aspect_keys.to_vec(),
        };
        self.declared_key_indexes.get(&key).unwrap_or(&EMPTY_DECLARED_KEY_INDEX)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum DeclaredIdentityComponent {
    JsonScalar(Value),
    EntityEndpoint(EntityId),
    Lifecycle(RecordLifecycleState),
    OpaqueBytes(Vec<u8>),
}

enum DeclaredKeySetMatch {
    ExactTarget(RecordRef),
    MissingSourceEvidence,
    AmbiguousTarget,
    NoTargetMatch,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct DeclaredKeyIndexKey {
    record_kind: VisibleMergeRecordKind,
    kind_id: Option<crate::identity::data::KindId>,
    aspect_keys: Vec<AspectKey>,
}

#[derive(Debug, Default)]
struct DeclaredKeyDeclarationCatalog {
    kind_scoped: BTreeMap<IdentityBasisScope, Vec<Vec<AspectKey>>>,
    aspect_scoped: BTreeMap<AspectKey, Vec<Vec<AspectKey>>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct DeclaredIdentityDigest(u128);

static EMPTY_DECLARED_KEY_INDEX: std::sync::LazyLock<BTreeMap<DeclaredIdentityDigest, Vec<RecordRef>>> =
    std::sync::LazyLock::new(BTreeMap::new);

fn identity_declaration_applies_to_record(
    runtime: &crate::logic::runtime::RelationalRuntime,
    source: &VisibleMergeRecord,
    declaration: &IdentityBasisDeclaration,
    kind_scope: &IdentityBasisScope,
) -> bool {
    match &declaration.scope {
        IdentityBasisScope::EntityKind(_) | IdentityBasisScope::RelationKind(_) => {
            &declaration.scope == kind_scope
        }
        IdentityBasisScope::AspectKey(aspect_key) => lowered_plan_for_record(runtime, source)
            .map(|plan| {
                plan.executable_bindings
                    .iter()
                    .any(|binding| binding.aspect_key == *aspect_key)
            })
            .unwrap_or(false),
    }
}

fn build_declared_key_indexes(
    runtime: &crate::logic::runtime::RelationalRuntime,
    visible_records: &[VisibleMergeRecord],
    effective_identity_declarations: &[IdentityBasisDeclaration],
) -> BTreeMap<DeclaredKeyIndexKey, BTreeMap<DeclaredIdentityDigest, Vec<RecordRef>>> {
    let catalog = build_declared_key_declaration_catalog(effective_identity_declarations);
    let mut indexes = BTreeMap::<DeclaredKeyIndexKey, BTreeMap<DeclaredIdentityDigest, Vec<RecordRef>>>::new();
    for record in visible_records {
        let Some(kind_id) = record.kind_id else {
            continue;
        };
        let kind_scope = match record.record_kind {
            VisibleMergeRecordKind::Entity => IdentityBasisScope::EntityKind(kind_id),
            VisibleMergeRecordKind::Relation => IdentityBasisScope::RelationKind(kind_id),
        };
        for keys in applicable_declared_key_sets(runtime, record, &catalog, &kind_scope) {
            let Some(signature) = extract_declared_key_signature(runtime, record, keys.as_slice()) else {
                continue;
            };
            let index_key = DeclaredKeyIndexKey {
                record_kind: record.record_kind.clone(),
                kind_id: record.kind_id,
                aspect_keys: keys.clone(),
            };
            indexes
                .entry(index_key)
                .or_default()
                .entry(signature)
                .or_default()
                .push(record.record_ref.clone());
        }
    }
    indexes
}

fn build_declared_key_declaration_catalog(
    effective_identity_declarations: &[IdentityBasisDeclaration],
) -> DeclaredKeyDeclarationCatalog {
    let mut catalog = DeclaredKeyDeclarationCatalog::default();
    for declaration in effective_identity_declarations {
        let IdentityBasisKind::DeclaredKeySet(keys) = &declaration.basis else {
            continue;
        };
        let key_vec = keys.to_vec();
        match &declaration.scope {
            IdentityBasisScope::EntityKind(_) | IdentityBasisScope::RelationKind(_) => {
                push_unique_key_set(catalog.kind_scoped.entry(declaration.scope.clone()).or_default(), &key_vec);
            }
            IdentityBasisScope::AspectKey(aspect_key) => {
                push_unique_key_set(catalog.aspect_scoped.entry(aspect_key.clone()).or_default(), &key_vec);
            }
        }
    }
    catalog
}

fn applicable_declared_key_sets(
    runtime: &crate::logic::runtime::RelationalRuntime,
    record: &VisibleMergeRecord,
    catalog: &DeclaredKeyDeclarationCatalog,
    kind_scope: &IdentityBasisScope,
) -> Vec<Vec<AspectKey>> {
    let mut applicable = catalog
        .kind_scoped
        .get(kind_scope)
        .cloned()
        .unwrap_or_default();
    let Some(plan) = lowered_plan_for_record(runtime, record) else {
        return applicable;
    };
    for binding in &plan.executable_bindings {
        if let Some(key_sets) = catalog.aspect_scoped.get(&binding.aspect_key) {
            for key_set in key_sets {
                push_unique_key_set(&mut applicable, key_set);
            }
        }
    }
    applicable
}

fn push_unique_key_set(target: &mut Vec<Vec<AspectKey>>, candidate: &[AspectKey]) {
    if !target.iter().any(|existing| existing.as_slice() == candidate) {
        target.push(candidate.to_vec());
    }
}

fn declared_identity_signature(components: &[DeclaredIdentityComponent]) -> DeclaredIdentityDigest {
    const FNV_OFFSET: u128 = 0x6c62272e07bb014262b821756295c58d;
    const FNV_PRIME: u128 = 0x0000000001000000000000000000013B;
    fn mix_bytes(hash: &mut u128, bytes: &[u8]) {
        for byte in bytes {
            *hash ^= *byte as u128;
            *hash = hash.wrapping_mul(FNV_PRIME);
        }
        *hash ^= 0xff_u128;
        *hash = hash.wrapping_mul(FNV_PRIME);
    }
    let mut hash = FNV_OFFSET;
    for component in components {
        match component {
            DeclaredIdentityComponent::JsonScalar(value) => {
                mix_bytes(&mut hash, b"json");
                mix_json_value(&mut hash, value);
            }
            DeclaredIdentityComponent::EntityEndpoint(entity_id) => {
                mix_bytes(&mut hash, b"endpoint");
                mix_bytes(&mut hash, &entity_id.partition_id.0.to_le_bytes());
                mix_bytes(&mut hash, &entity_id.local_slot.0.to_le_bytes());
                mix_bytes(&mut hash, &entity_id.generation.0.to_le_bytes());
            }
            DeclaredIdentityComponent::Lifecycle(state) => {
                mix_bytes(&mut hash, b"lifecycle");
                mix_bytes(&mut hash, format!("{state:?}").as_bytes());
            }
            DeclaredIdentityComponent::OpaqueBytes(bytes) => {
                mix_bytes(&mut hash, b"opaque");
                mix_bytes(&mut hash, bytes);
            }
        }
    }
    DeclaredIdentityDigest(hash)
}

fn mix_json_value(hash: &mut u128, value: &Value) {
    match value {
        Value::Null => mix_identity_bytes(hash, b"null"),
        Value::Bool(boolean) => {
            mix_identity_bytes(hash, b"bool");
            mix_identity_bytes(hash, &[*boolean as u8]);
        }
        Value::Number(number) => {
            mix_identity_bytes(hash, b"number");
            mix_identity_bytes(hash, number.to_string().as_bytes());
        }
        Value::String(string) => {
            mix_identity_bytes(hash, b"string");
            mix_identity_bytes(hash, string.as_bytes());
        }
        Value::Array(values) => {
            mix_identity_bytes(hash, b"array");
            for value in values {
                mix_json_value(hash, value);
            }
        }
        Value::Object(map) => {
            mix_identity_bytes(hash, b"object");
            for (key, value) in map {
                mix_identity_bytes(hash, key.as_bytes());
                mix_json_value(hash, value);
            }
        }
    }
}

fn mix_identity_bytes(hash: &mut u128, bytes: &[u8]) {
    const FNV_PRIME: u128 = 0x0000000001000000000000000000013B;
    for byte in bytes {
        *hash ^= *byte as u128;
        *hash = hash.wrapping_mul(FNV_PRIME);
    }
    *hash ^= 0xff_u128;
    *hash = hash.wrapping_mul(FNV_PRIME);
}
