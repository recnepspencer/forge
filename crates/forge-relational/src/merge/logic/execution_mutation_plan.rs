mod current_snapshots;
mod materialized_value_resolution;
mod merge_client_keys;
mod source_authoritative_fields;

use std::collections::BTreeMap;
use std::sync::Arc;

use forge_foundational::facade::AspectValue;

use crate::capabilities::AspectPlanSource;
use crate::merge::data::{
    AdoptSourceRecordPlan, BoundExecutableMergeRecordPlan, ExecutableAspectPlan,
    MergeExecutionMutationPlanError, PreparedMergeExecution, ReconcileRecordPlan,
};
use crate::storage::data::EntityReadRecord;
use crate::transactions::data::{AspectFieldPatch, AspectFieldPatchTarget};
use crate::transactions::data::{
    CreateIntent, EntityMutationIntent, EntitySpec, MergeCommitMutationPlan,
    MergeExecutionStructuralSummary, MergeExecutionSummary, MergedCommitPlan, MutationIntent,
    RelationSpec, TransactionId, UpdateEntityFieldsIntent,
};

use current_snapshots::current_entity_snapshot;
use materialized_value_resolution::resolved_entity_field_patch_value;
use merge_client_keys::merge_client_key;
use source_authoritative_fields::{
    entity_create_fields_from_authoritative_state, relation_create_fields_from_authoritative_state,
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
            deleted_on_both_sides_lineage_unchanged_count: 0,
            emitted_mutation_intent_count: 0,
            emitted_entity_create_count: 0,
            emitted_relation_create_count: 0,
            emitted_entity_update_count: 0,
        };
        let source_head_version_id = self
            .runtime
            .history()
            .commit_envelope(
                prepared
                    .bound_executable_plan()
                    .authority_binding
                    .source_head_commit_id,
            )
            .map(|envelope| envelope.commit.version_id)
            .ok_or(MergeExecutionMutationPlanError::MissingSourceHeadEnvelope)?;

        for record_plan in prepared.bound_executable_plan().record_plans.iter() {
            match record_plan {
                BoundExecutableMergeRecordPlan::AdoptSource(plan) => {
                    summary.adopted_source_record_count += 1;
                    let intent =
                        self.derive_source_adoption_intent(plan, source_head_version_id)?;
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
                        if matches!(
                            intent,
                            MutationIntent::Entity(EntityMutationIntent::UpdateFields(_))
                        ) {
                            summary.emitted_entity_update_count += 1;
                        }
                        summary.emitted_mutation_intent_count += 1;
                        merged_intents.push(intent);
                    }
                }
                BoundExecutableMergeRecordPlan::ConvergeDeletedOnBothSides(plan) => {
                    summary.converged_deleted_on_both_sides_count += 1;
                    if plan.lineage_continuity
                        == crate::merge::data::MergeLineageContinuityVerdict::Unchanged
                    {
                        summary.deleted_on_both_sides_lineage_unchanged_count += 1;
                    }
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
            converged_deleted_on_both_sides_count: summary.converged_deleted_on_both_sides_count,
            deleted_on_both_sides_lineage_unchanged_count: summary
                .deleted_on_both_sides_lineage_unchanged_count,
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
                prepared
                    .bound_executable_plan()
                    .parent_order
                    .iter()
                    .copied()
                    .collect(),
            ),
            merge_base_commits: Arc::from([binding.merge_base_commit_id]),
            merged_plan,
            structural_summary: summary,
            merge_execution_summary,
            proof_token: crate::transactions::data::merge_commit_mutation_plan_token(),
        })
    }
}

impl<'runtime> MergeAccess<'runtime> {
    fn derive_source_adoption_intent(
        &self,
        plan: &AdoptSourceRecordPlan,
        _source_head_version_id: crate::identity::data::VersionId,
    ) -> Result<MutationIntent, MergeExecutionMutationPlanError> {
        match &plan.source_visible_snapshot {
            crate::merge::data::VisibleMergeRecordSnapshot::Entity(entity) => {
                Ok(MutationIntent::Create(CreateIntent::Entity(EntitySpec {
                    partition_id: entity.entity_id.partition_id,
                    kind_id: entity.kind.kind_id,
                    client_key: merge_client_key("adopt-source-entity", &plan.source_record),
                    fields: entity_create_fields_from_authoritative_state(
                        self.runtime,
                        plan,
                        entity,
                    )?,
                })))
            }
            crate::merge::data::VisibleMergeRecordSnapshot::Relation(relation) => Ok(
                MutationIntent::Create(CreateIntent::Relation(RelationSpec {
                    partition_id: relation.relation_id.partition_id,
                    kind_id: relation.kind.kind_id,
                    client_key: merge_client_key("adopt-source-relation", &plan.source_record),
                    source: crate::transactions::data::EntityReference::Existing(relation.source),
                    target: crate::transactions::data::EntityReference::Existing(relation.target),
                    fields: relation_create_fields_from_authoritative_state(
                        self.runtime,
                        plan,
                        relation,
                    )?,
                })),
            ),
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
            ) => Err(
                MergeExecutionMutationPlanError::UnsupportedReconcileRecordKind {
                    record: plan.target_record.clone(),
                    detail: "relation reconciliation is not executable in phase D",
                },
            ),
            _ => Err(
                MergeExecutionMutationPlanError::UnsupportedReconcileRecordKind {
                    record: plan.target_record.clone(),
                    detail: "source/target record kinds do not match reconcile executable class",
                },
            ),
        }
    }

    fn derive_entity_reconcile_intent(
        &self,
        plan: &ReconcileRecordPlan,
        source_entity: &EntityReadRecord,
        target_entity_id: crate::identity::data::EntityId,
    ) -> Result<Option<MutationIntent>, MergeExecutionMutationPlanError> {
        let target_entity =
            current_entity_snapshot(self.runtime, target_entity_id).ok_or_else(|| {
                MergeExecutionMutationPlanError::MissingTargetEntitySnapshot {
                    record: plan.target_record.clone(),
                }
            })?;
        let target_binding_plan = self
            .runtime
            .entity_aspect_plan(target_entity.kind.kind_id)
            .ok_or_else(
                || MergeExecutionMutationPlanError::UnsupportedReconcileRecordKind {
                    record: plan.target_record.clone(),
                    detail: "target entity kind has no executable aspect plan",
                },
            )?;
        let mut resolved_fields = BTreeMap::<AspectFieldPatchTarget, AspectValue>::new();

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
                .find(|binding| binding.aspect_key() == aspect_key)
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
            if let Some((field, value)) = resolved_entity_field_patch_value(
                plan,
                source_entity,
                &target_entity,
                binding,
                aspect_key,
                resolved_value,
            )? {
                resolved_fields.insert(field, value);
            }
        }

        if resolved_fields.is_empty() {
            Ok(None)
        } else {
            Ok(Some(MutationIntent::Entity(
                EntityMutationIntent::UpdateFields(UpdateEntityFieldsIntent {
                    entity_id: target_entity_id,
                    fields: AspectFieldPatch::from(resolved_fields),
                }),
            )))
        }
    }
}
