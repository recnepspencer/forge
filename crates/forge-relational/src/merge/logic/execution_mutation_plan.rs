#![allow(dead_code)]

use std::sync::Arc;

use serde_json::{Map, Value};

use crate::capabilities::AspectPlanSource;
use crate::merge::data::{
    AdoptSourceRecordPlan, BoundExecutableMergeRecordPlan, ExecutableAspectPlan, MaterializedAspectValue,
    MaterializedAspectValuePayload, MergeExecutionMutationPlanError, PreparedMergeExecution,
    ReconcileRecordPlan,
};
use crate::payloads::data::RecordPayload;
use crate::schema::data::{LoweredAspectBinding, LoweredExecutableAspectBindingKind};
use crate::storage::overlay::PartitionAccess;
use crate::storage::data::{EntityReadRecord, RelationReadRecord};
use crate::symbols::data::InternedString;
use crate::transactions::data::{
    CreateIntent, EntityMutationIntent, EntitySpec, MergeCommitMutationPlan,
    MergeExecutionStructuralSummary, MergeExecutionSummary, MergedCommitPlan, MutationIntent,
    RelationSpec, TransactionId,
    UpdateEntityIntent,
};

use super::MergeAccess;

impl<'runtime> MergeAccess<'runtime> {
    pub(crate) fn derive_merge_commit_mutation_plan(
        &self,
        transaction_id: TransactionId,
        prepared: &PreparedMergeExecution,
    ) -> Result<MergeCommitMutationPlan, MergeExecutionMutationPlanError> {
        let mut merged_intents = Vec::new();
        let mut summary = MergeExecutionStructuralSummary {
            executed_record_count: prepared.bound_executable_plan().record_plans.len(),
            adopted_source_record_count: 0,
            preserved_shared_record_count: 0,
            reconciled_record_count: 0,
            converged_deleted_on_both_sides_count: 0,
            emitted_mutation_intent_count: 0,
            emitted_entity_create_count: 0,
            emitted_relation_create_count: 0,
            emitted_entity_update_count: 0,
        };

        for record_plan in prepared.bound_executable_plan().record_plans.iter() {
            match record_plan {
                BoundExecutableMergeRecordPlan::AdoptSource(plan) => {
                    summary.adopted_source_record_count += 1;
                    let intent = derive_source_adoption_intent(plan)?;
                    match &intent {
                        MutationIntent::Create(CreateIntent::Entity(_)) => {
                            summary.emitted_entity_create_count += 1;
                        }
                        MutationIntent::Create(CreateIntent::Relation(_)) => {
                            summary.emitted_relation_create_count += 1;
                        }
                        _ => {}
                    }
                    summary.emitted_mutation_intent_count += 1;
                    merged_intents.push(intent);
                }
                BoundExecutableMergeRecordPlan::PreserveShared(_plan) => {
                    summary.preserved_shared_record_count += 1;
                }
                BoundExecutableMergeRecordPlan::Reconcile(plan) => {
                    summary.reconciled_record_count += 1;
                    if let Some(intent) = self.derive_reconcile_intent(plan)? {
                        if matches!(intent, MutationIntent::Entity(EntityMutationIntent::Update(_)))
                        {
                            summary.emitted_entity_update_count += 1;
                        }
                        summary.emitted_mutation_intent_count += 1;
                        merged_intents.push(intent);
                    }
                }
                BoundExecutableMergeRecordPlan::ConvergeDeletedOnBothSides(_plan) => {
                    summary.converged_deleted_on_both_sides_count += 1;
                }
            }
        }

        let merged_plan = MergedCommitPlan {
            transaction_id,
            merged_intents,
        };
        let binding = &prepared.bound_executable_plan().authority_binding;
        let diagnostics_plan = &prepared.bound_executable_plan().diagnostics_plan;
        let merge_execution_summary = MergeExecutionSummary {
            request: prepared.request().clone(),
            target_head_commit_id: binding.target_head_commit_id,
            source_head_commit_id: binding.source_head_commit_id,
            merge_base_commit_id: binding.merge_base_commit_id,
            executed_record_count: summary.executed_record_count,
            adopted_source_record_count: summary.adopted_source_record_count,
            preserved_shared_record_count: summary.preserved_shared_record_count,
            reconciled_record_count: summary.reconciled_record_count,
            converged_deleted_on_both_sides_count: summary
                .converged_deleted_on_both_sides_count,
            emitted_mutation_intent_count: summary.emitted_mutation_intent_count,
            diagnostics_digest: diagnostics_plan.digest.clone(),
            execution_digest: binding.executable_plan_digest.clone(),
        };

        Ok(MergeCommitMutationPlan {
            transaction_id,
            target_branch: prepared.request().target_branch.clone(),
            source_branch: prepared.request().source_branch.clone(),
            merge_parent_branches: Arc::from([prepared.request().source_branch.clone()]),
            requested_merge_parent_count: 1,
            parent_commits: crate::history::data::OrderedParentList::from_authoritative(
                prepared.bound_executable_plan().parent_order.iter().copied().collect(),
            ),
            merge_base_commits: Arc::from([binding.merge_base_commit_id]),
            merged_plan,
            structural_summary: summary,
            merge_execution_summary,
            proof_token: crate::transactions::data::merge_commit_mutation_plan_token(),
        })
    }
}

fn derive_source_adoption_intent(
    plan: &AdoptSourceRecordPlan,
) -> Result<MutationIntent, MergeExecutionMutationPlanError> {
    match &plan.source_visible_snapshot {
        crate::merge::data::VisibleMergeRecordSnapshot::Entity(entity) => {
            Ok(MutationIntent::Create(CreateIntent::Entity(EntitySpec {
                partition_id: entity.entity_id.partition_id,
                kind_id: entity.kind.kind_id,
                client_key: merge_client_key("adopt-source-entity", &plan.source_record),
                payload: entity.payload.clone(),
            })))
        }
        crate::merge::data::VisibleMergeRecordSnapshot::Relation(relation) => {
            Ok(MutationIntent::Create(CreateIntent::Relation(RelationSpec {
                partition_id: relation.relation_id.partition_id,
                kind_id: relation.kind.kind_id,
                client_key: merge_client_key("adopt-source-relation", &plan.source_record),
                source: relation.source,
                target: relation.target,
                payload: relation.payload.clone(),
            })))
        }
    }
}

impl<'runtime> MergeAccess<'runtime> {
    fn derive_reconcile_intent(
        &self,
        plan: &ReconcileRecordPlan,
    ) -> Result<Option<MutationIntent>, MergeExecutionMutationPlanError> {
        match (&plan.source_visible_snapshot, &plan.target_record) {
            (
                crate::merge::data::VisibleMergeRecordSnapshot::Entity(source_entity),
                crate::transactions::data::RecordRef::Entity(target_entity_id),
            ) => self.derive_entity_reconcile_intent(plan, source_entity, *target_entity_id),
            (
                crate::merge::data::VisibleMergeRecordSnapshot::Relation(_),
                crate::transactions::data::RecordRef::Relation(_),
            ) => Err(MergeExecutionMutationPlanError::UnsupportedReconcileRecordKind {
                record: plan.target_record.clone(),
                detail: "relation reconciliation is not executable in phase D",
            }),
            _ => Err(MergeExecutionMutationPlanError::UnsupportedReconcileRecordKind {
                record: plan.target_record.clone(),
                detail: "source/target record kinds do not match reconcile executable class",
            }),
        }
    }

    fn derive_entity_reconcile_intent(
        &self,
        plan: &ReconcileRecordPlan,
        source_entity: &EntityReadRecord,
        target_entity_id: crate::identity::data::EntityId,
    ) -> Result<Option<MutationIntent>, MergeExecutionMutationPlanError> {
        let target_entity = current_entity_snapshot(self.runtime, target_entity_id).ok_or_else(|| {
            MergeExecutionMutationPlanError::MissingTargetEntitySnapshot {
                record: plan.target_record.clone(),
            }
        })?;
        let target_binding_plan = self
            .runtime
            .entity_aspect_plan(target_entity.kind.kind_id)
            .ok_or_else(|| MergeExecutionMutationPlanError::UnsupportedReconcileRecordKind {
                record: plan.target_record.clone(),
                detail: "target entity kind has no executable aspect plan",
            })?;
        let mut payload = target_entity.payload.as_json().cloned().ok_or_else(|| {
            MergeExecutionMutationPlanError::UnsupportedReconcileRecordKind {
                record: plan.target_record.clone(),
                detail: "entity reconcile requires structured json payload",
            }
        })?;
        let object = payload.as_object_mut().ok_or_else(|| {
            MergeExecutionMutationPlanError::UnsupportedReconcileRecordKind {
                record: plan.target_record.clone(),
                detail: "entity reconcile requires top-level object payload",
            }
        })?;

        for aspect_plan in plan.aspect_plan.iter() {
            let ExecutableAspectPlan::ReconcileValue {
                aspect_key,
                resolved_value,
                ..
            } = aspect_plan
            else {
                continue;
            };
            let binding = target_binding_plan
                .executable_bindings
                .iter()
                .find(|binding| binding.aspect_key == *aspect_key)
                .ok_or_else(|| MergeExecutionMutationPlanError::UnsupportedAspectMutationMaterialization {
                    record: plan.target_record.clone(),
                    aspect_key: aspect_key.clone(),
                    detail: "target entity executable binding missing during reconcile mutation derivation",
                })?;
            let resolved_value = resolved_value.as_ref().ok_or_else(|| {
                MergeExecutionMutationPlanError::MissingResolvedAspectValue {
                    record: plan.target_record.clone(),
                    aspect_key: aspect_key.clone(),
                }
            })?;
            apply_entity_aspect_resolution(
                self.runtime,
                object,
                plan,
                source_entity,
                &target_entity,
                binding,
                aspect_key,
                resolved_value,
            )?;
        }

        let next_payload = RecordPayload::from(payload);
        if next_payload == target_entity.payload {
            Ok(None)
        } else {
            Ok(Some(MutationIntent::Entity(EntityMutationIntent::Update(
                UpdateEntityIntent {
                    entity_id: target_entity_id,
                    payload: next_payload,
                },
            ))))
        }
    }
}

fn apply_entity_aspect_resolution(
    runtime: &crate::logic::runtime::RelationalRuntime,
    payload: &mut Map<String, Value>,
    plan: &ReconcileRecordPlan,
    source_entity: &EntityReadRecord,
    target_entity: &EntityReadRecord,
    binding: &LoweredAspectBinding,
    aspect_key: &crate::publication::patch::data::AspectKey,
    resolved_value: &MaterializedAspectValue,
) -> Result<(), MergeExecutionMutationPlanError> {
    match &binding.binding_kind {
        LoweredExecutableAspectBindingKind::EntityJsonScalarField { field } => {
            let field_name = interned_field_name(runtime, field).ok_or_else(|| {
                MergeExecutionMutationPlanError::UnsupportedAspectMutationMaterialization {
                    record: plan.target_record.clone(),
                    aspect_key: aspect_key.clone(),
                    detail: "entity json field binding could not be resolved",
                }
            })?;
            let resolved_json = resolve_materialized_json_value(
                plan,
                aspect_key,
                resolved_value,
                source_entity,
                target_entity,
            )?;
            payload.insert(field_name.to_string(), resolved_json);
            Ok(())
        }
        LoweredExecutableAspectBindingKind::OpaqueWholePayloadBytes => Err(
            MergeExecutionMutationPlanError::UnsupportedAspectMutationMaterialization {
                record: plan.target_record.clone(),
                aspect_key: aspect_key.clone(),
                detail: "opaque entity payload reconcile is not executable through update intents",
            },
        ),
        LoweredExecutableAspectBindingKind::LifecycleTransitionEquality => Err(
            MergeExecutionMutationPlanError::UnsupportedAspectMutationMaterialization {
                record: plan.target_record.clone(),
                aspect_key: aspect_key.clone(),
                detail: "lifecycle reconcile is not executable through entity update intents",
            },
        ),
        LoweredExecutableAspectBindingKind::RelationJsonScalarField { .. }
        | LoweredExecutableAspectBindingKind::RelationSourceEndpointIdentity
        | LoweredExecutableAspectBindingKind::RelationTargetEndpointIdentity => Err(
            MergeExecutionMutationPlanError::UnsupportedAspectMutationMaterialization {
                record: plan.target_record.clone(),
                aspect_key: aspect_key.clone(),
                detail: "relation-scoped aspect binding is not executable for entity reconcile",
            },
        ),
    }
}

fn resolve_materialized_json_value(
    plan: &ReconcileRecordPlan,
    aspect_key: &crate::publication::patch::data::AspectKey,
    value: &MaterializedAspectValue,
    source_entity: &EntityReadRecord,
    target_entity: &EntityReadRecord,
) -> Result<Value, MergeExecutionMutationPlanError> {
    match &value.payload {
        MaterializedAspectValuePayload::VisibleAspectReference {
            side,
            record,
            aspect_key: reference_key,
        } => {
            if reference_key != aspect_key {
                return Err(MergeExecutionMutationPlanError::InvalidVisibleAspectReference {
                    record: plan.target_record.clone(),
                    aspect_key: aspect_key.clone(),
                    detail: "resolved aspect reference key does not match executable aspect key",
                });
            }
            match side {
                crate::merge::data::MergeValueSourceSide::Source => {
                    if *record != plan.source_record {
                        return Err(MergeExecutionMutationPlanError::InvalidVisibleAspectReference {
                            record: plan.target_record.clone(),
                            aspect_key: aspect_key.clone(),
                            detail: "resolved source aspect reference points at a different source record",
                        });
                    }
                    source_entity
                        .payload
                        .as_json()
                        .and_then(|json| json.get(aspect_key_name(aspect_key)?))
                        .cloned()
                        .ok_or_else(|| MergeExecutionMutationPlanError::InvalidVisibleAspectReference {
                            record: plan.target_record.clone(),
                            aspect_key: aspect_key.clone(),
                            detail: "resolved source aspect reference is missing from source payload",
                        })
                }
                crate::merge::data::MergeValueSourceSide::Target => {
                    if *record != plan.target_record {
                        return Err(MergeExecutionMutationPlanError::InvalidVisibleAspectReference {
                            record: plan.target_record.clone(),
                            aspect_key: aspect_key.clone(),
                            detail: "resolved target aspect reference points at a different target record",
                        });
                    }
                    target_entity
                        .payload
                        .as_json()
                        .and_then(|json| json.get(aspect_key_name(aspect_key)?))
                        .cloned()
                        .ok_or_else(|| MergeExecutionMutationPlanError::InvalidVisibleAspectReference {
                            record: plan.target_record.clone(),
                            aspect_key: aspect_key.clone(),
                            detail: "resolved target aspect reference is missing from target payload",
                        })
                }
                crate::merge::data::MergeValueSourceSide::Base => Err(
                    MergeExecutionMutationPlanError::UnsupportedAspectMutationMaterialization {
                        record: plan.target_record.clone(),
                        aspect_key: aspect_key.clone(),
                        detail: "base-bound resolved values are not executable in phase D",
                    },
                ),
            }
        }
        MaterializedAspectValuePayload::EqualityWitnessDigest(_) => Err(
            MergeExecutionMutationPlanError::UnsupportedAspectMutationMaterialization {
                record: plan.target_record.clone(),
                aspect_key: aspect_key.clone(),
                detail: "digest-only equality witnesses cannot be lowered into payload mutation",
            },
        ),
    }
}

fn current_entity_snapshot(
    runtime: &crate::logic::runtime::RelationalRuntime,
    entity_id: crate::identity::data::EntityId,
) -> Option<EntityReadRecord> {
    let current_state = runtime.storage_access().current_state();
    let partition = current_state.get_partition(entity_id.partition_id)?;
    let slot = partition.entity_arena.get(&entity_id)?;
    let kind_id = slot.kind_id()?;
    let kind = runtime.config.schema.registry.resolve_entity(kind_id).ok()?;
    Some(EntityReadRecord {
        entity_id,
        lineage_id: None,
        kind,
        lifecycle: slot.lifecycle(),
        created_at_version: partition.entity_arena.created_at[entity_id.local_slot.0 as usize],
        retired_at_version: slot.retired_at(),
        payload: slot.payload()?.clone(),
    })
}

#[allow(dead_code)]
fn current_relation_snapshot(
    runtime: &crate::logic::runtime::RelationalRuntime,
    relation_id: crate::identity::data::RelationId,
) -> Option<RelationReadRecord> {
    let current_state = runtime.storage_access().current_state();
    let partition = current_state.get_partition(relation_id.partition_id)?;
    let slot = partition.relation_arena.get(&relation_id)?;
    let kind_id = slot.kind_id()?;
    let endpoints = slot.extra().as_ref()?;
    let kind = runtime.config.schema.registry.resolve_relation(kind_id).ok()?;
    Some(RelationReadRecord {
        relation_id,
        kind,
        lifecycle: slot.lifecycle(),
        created_at_version: partition.relation_arena.created_at[relation_id.local_slot.0 as usize],
        retired_at_version: slot.retired_at(),
        source: endpoints.source,
        target: endpoints.target,
        payload: slot.payload().cloned(),
    })
}

fn merge_client_key(prefix: &str, record: &crate::transactions::data::RecordRef) -> InternedString {
    let suffix = match record {
        crate::transactions::data::RecordRef::Entity(entity_id) => format!(
            "entity-{}-{}-{}",
            entity_id.partition_id.0, entity_id.local_slot.0, entity_id.generation.0
        ),
        crate::transactions::data::RecordRef::Relation(relation_id) => format!(
            "relation-{}-{}-{}",
            relation_id.partition_id.0, relation_id.local_slot.0, relation_id.generation.0
        ),
    };
    InternedString::Raw(format!("{prefix}-{suffix}"))
}

fn aspect_key_name(
    aspect_key: &crate::publication::patch::data::AspectKey,
) -> Option<&str> {
    match &aspect_key.0 {
        InternedString::Raw(raw) => Some(raw.as_str()),
        InternedString::Symbol(_) => None,
    }
}

fn interned_field_name<'a>(
    runtime: &'a crate::logic::runtime::RelationalRuntime,
    field: &'a InternedString,
) -> Option<&'a str> {
    match field {
        InternedString::Raw(raw) => Some(raw.as_str()),
        InternedString::Symbol(symbol) => runtime.resolve_symbol(*symbol),
    }
}
