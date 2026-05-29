use std::collections::BTreeMap;
use std::sync::Arc;

use crate::capabilities::AspectPlanSource;
use crate::merge::data::{
    AdoptSourceRecordPlan, BoundExecutableMergePlan, BoundExecutableMergeRecordPlan,
    ConvergeDeletedOnBothSidesRecordPlan, ExecutableAspectPlan, ExecutionReadyLoweredMergePlan,
    LoweredAspectExecutionIntent, LoweredRecordDecision, MergeExecutableClass,
    MergeExecutableRecordProvenance, MergeExecutionAuthorityBinding,
    MergeExecutionCompilationError, MergeExecutionError, MergeExecutionPreparationError,
    MergeExecutionRequest, MergeValueMaterialization, MergeValueSourceSide, PreparedMergeExecution,
    PreserveSharedRecordPlan, ReconcileRecordPlan, ReconciledIdentityBasis, RuntimeInstanceId,
};
use crate::merge::data::{MaterializedAspectValue, MaterializedAspectValueEvidence};
use crate::merge::logic::aspect_components::{
    binding_component_from_visible_record, VisibleRecordSide,
};
use crate::merge::logic::aspect_witness_digest::canonical_aspect_witness_digest;
use crate::schema::data::LoweredAspectBinding;
use crate::transactions::data::RecordRef;

use super::planning_artifact::{
    materialize_planning_artifact, merge_schema_snapshot_for_execution_ready,
};
use super::MergeAccess;

impl<'runtime> MergeAccess<'runtime> {
    pub fn prepare_merge_execution(
        &self,
        request: MergeExecutionRequest,
    ) -> Result<PreparedMergeExecution, MergeExecutionPreparationError> {
        let planning_request = crate::merge::data::MergePlanningRequest::from(request.clone());
        let lowered_plan = self
            .lower_planning_scope(planning_request)
            .map_err(MergeExecutionPreparationError::Planning)?;
        let artifact = materialize_planning_artifact(self.runtime, lowered_plan.clone());
        let execution_ready_plan = ExecutionReadyLoweredMergePlan::try_from_lowered(
            lowered_plan,
            artifact.schema_snapshot.clone(),
        )
        .map_err(MergeExecutionPreparationError::NotExecutionReady)?;
        let bound_executable_plan =
            compile_bound_executable_plan(self.runtime, &request, &execution_ready_plan)
                .map_err(MergeExecutionPreparationError::Compilation)?;

        Ok(PreparedMergeExecution::new(
            request,
            artifact,
            execution_ready_plan,
            bound_executable_plan,
        ))
    }

    pub fn verify_prepared_merge_execution(
        &self,
        prepared: &PreparedMergeExecution,
    ) -> Result<(), MergeExecutionError> {
        self.runtime
            .performance_access()
            .count_merge_execution_verification_request();
        let binding = &prepared.bound_executable_plan().authority_binding;
        if binding.target_branch != prepared.request().target_branch {
            return Err(MergeExecutionError::Compilation(
                MergeExecutionCompilationError::PreparedAuthorityBindingMismatch {
                    detail: "binding target branch does not match prepared request",
                },
            ));
        }
        if binding.source_branch != prepared.request().source_branch {
            return Err(MergeExecutionError::Compilation(
                MergeExecutionCompilationError::PreparedAuthorityBindingMismatch {
                    detail: "binding source branch does not match prepared request",
                },
            ));
        }
        if binding.merge_intent != prepared.request().merge_intent {
            return Err(MergeExecutionError::Compilation(
                MergeExecutionCompilationError::PreparedAuthorityBindingMismatch {
                    detail: "binding merge intent does not match prepared request",
                },
            ));
        }
        let current_runtime_instance_id = RuntimeInstanceId(self.runtime.runtime_instance_id());
        if binding.runtime_instance_id != current_runtime_instance_id {
            return Err(MergeExecutionError::RuntimeInstanceMismatch {
                planned: binding.runtime_instance_id,
                current: current_runtime_instance_id,
            });
        }

        let target_head = self
            .runtime
            .history()
            .branch_head(&binding.target_branch)
            .cloned();
        self.runtime
            .performance_access()
            .count_merge_execution_branch_head_checks(1);
        if target_head.as_ref().map(|head| head.commit_id) != Some(binding.target_head_commit_id) {
            return Err(MergeExecutionError::StaleBranchHead {
                branch: binding.target_branch.clone(),
                planned: binding.target_head_commit_id,
                current: target_head.map(|head| head.commit_id),
            });
        }

        let source_head = self
            .runtime
            .history()
            .branch_head(&binding.source_branch)
            .cloned();
        self.runtime
            .performance_access()
            .count_merge_execution_branch_head_checks(1);
        if source_head.as_ref().map(|head| head.commit_id) != Some(binding.source_head_commit_id) {
            return Err(MergeExecutionError::StaleBranchHead {
                branch: binding.source_branch.clone(),
                planned: binding.source_head_commit_id,
                current: source_head.map(|head| head.commit_id),
            });
        }

        let merge_base = self
            .runtime
            .history()
            .latest_common_ancestor_between_branches(
                &binding.target_branch,
                &binding.source_branch,
            );
        self.runtime
            .performance_access()
            .count_merge_execution_merge_base_checks(1);
        if merge_base != Some(binding.merge_base_commit_id) {
            return Err(MergeExecutionError::MergeBaseDrift {
                planned: binding.merge_base_commit_id,
                current: merge_base,
            });
        }

        let execution_ready = prepared.execution_ready_plan();
        if binding.target_head_commit_id != execution_ready.target_head.commit_id {
            return Err(MergeExecutionError::Compilation(
                MergeExecutionCompilationError::PreparedAuthorityBindingMismatch {
                    detail: "binding target head does not match execution-ready proof",
                },
            ));
        }
        if binding.source_head_commit_id != execution_ready.source_head.commit_id {
            return Err(MergeExecutionError::Compilation(
                MergeExecutionCompilationError::PreparedAuthorityBindingMismatch {
                    detail: "binding source head does not match execution-ready proof",
                },
            ));
        }
        if binding.merge_base_commit_id != execution_ready.merge_base.commit_id {
            return Err(MergeExecutionError::Compilation(
                MergeExecutionCompilationError::PreparedAuthorityBindingMismatch {
                    detail: "binding merge base does not match execution-ready proof",
                },
            ));
        }
        let prepared_schema_digest =
            crate::merge::data::schema_snapshot_digest(&execution_ready.schema_snapshot);
        if binding.schema_snapshot_digest != prepared_schema_digest {
            return Err(MergeExecutionError::Compilation(
                MergeExecutionCompilationError::PreparedAuthorityBindingMismatch {
                    detail: "binding schema digest does not match execution-ready proof",
                },
            ));
        }
        let current_schema_snapshot = merge_schema_snapshot_for_execution_ready(
            self.runtime,
            execution_ready.target_head.version_id,
            execution_ready.source_records.as_ref(),
            execution_ready.target_touched_records.as_ref(),
        );
        self.runtime
            .performance_access()
            .count_merge_execution_schema_snapshot_kinds(
                current_schema_snapshot.touched_kinds.len(),
            );
        let current_digest = crate::merge::data::schema_snapshot_digest(&current_schema_snapshot);
        if current_digest != binding.schema_snapshot_digest {
            return Err(MergeExecutionError::SchemaSemanticDrift {
                planned_digest: binding.schema_snapshot_digest.clone(),
                current_digest,
            });
        }

        let compiled = prepared.bound_executable_plan();
        let current_compiled_digest = crate::merge::data::compiled_executable_plan_digest(
            &binding.target_branch,
            &binding.source_branch,
            binding.merge_intent,
            compiled.parent_order.as_ref(),
            compiled.record_plans.as_ref(),
        );
        self.runtime
            .performance_access()
            .count_merge_execution_compiled_plan_digest_checks(1);
        if current_compiled_digest != binding.executable_plan_digest {
            return Err(MergeExecutionError::Compilation(
                MergeExecutionCompilationError::PreparedAuthorityBindingMismatch {
                    detail: "compiled executable plan digest does not match binding certification",
                },
            ));
        }

        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn compile_execution_ready_merge_plan_for_test(
        &self,
        execution_ready: &ExecutionReadyLoweredMergePlan,
    ) -> Result<BoundExecutableMergePlan, MergeExecutionCompilationError> {
        compile_bound_executable_plan(
            self.runtime,
            &MergeExecutionRequest {
                target_branch: execution_ready.target_head.branch_id.clone(),
                source_branch: execution_ready.source_head.branch_id.clone(),
                merge_intent: crate::merge::data::MergeIntent::ReconcileIntoTarget,
            },
            execution_ready,
        )
    }
}

fn compile_bound_executable_plan(
    runtime: &crate::logic::runtime::RelationalRuntime,
    request: &MergeExecutionRequest,
    execution_ready: &ExecutionReadyLoweredMergePlan,
) -> Result<BoundExecutableMergePlan, MergeExecutionCompilationError> {
    let parent_order = crate::merge::data::bound_parent_order(execution_ready);
    let source_records_by_ref = execution_ready
        .source_records
        .iter()
        .map(|record| (record.record_ref.clone(), record))
        .collect::<BTreeMap<_, _>>();

    let record_plans = execution_ready
        .lowered_records
        .iter()
        .map(|lowered_record| compile_record_plan(runtime, &source_records_by_ref, lowered_record))
        .collect::<Result<Vec<_>, _>>()?;
    let record_plans: Arc<[BoundExecutableMergeRecordPlan]> = Arc::from(record_plans);
    let diagnostics_plan =
        crate::merge::data::diagnostics_plan_from_record_plans(record_plans.as_ref());
    let executable_plan_digest = crate::merge::data::compiled_executable_plan_digest(
        &request.target_branch,
        &request.source_branch,
        request.merge_intent,
        parent_order.as_ref(),
        record_plans.as_ref(),
    );

    let binding = MergeExecutionAuthorityBinding {
        target_branch: request.target_branch.clone(),
        source_branch: request.source_branch.clone(),
        merge_intent: request.merge_intent,
        runtime_instance_id: RuntimeInstanceId(runtime.runtime_instance_id()),
        target_head_commit_id: execution_ready.target_head.commit_id,
        source_head_commit_id: execution_ready.source_head.commit_id,
        merge_base_commit_id: execution_ready.merge_base.commit_id,
        schema_snapshot_digest: crate::merge::data::schema_snapshot_digest(
            &execution_ready.schema_snapshot,
        ),
        freshness_policy: execution_ready.freshness_policy,
        executable_plan_digest,
    };

    Ok(BoundExecutableMergePlan {
        authority_binding: binding,
        parent_order,
        record_plans,
        diagnostics_plan,
    })
}

fn compile_record_plan(
    runtime: &crate::logic::runtime::RelationalRuntime,
    source_records_by_ref: &BTreeMap<RecordRef, &crate::merge::data::VisibleMergeRecord>,
    lowered_record: &crate::merge::data::LoweredMergePlanRecord,
) -> Result<BoundExecutableMergeRecordPlan, MergeExecutionCompilationError> {
    let source_record = source_records_by_ref
        .get(&lowered_record.record)
        .copied()
        .ok_or_else(|| MergeExecutionCompilationError::MissingSourceRecord {
            record: lowered_record.record.clone(),
        })?;
    let provenance = MergeExecutableRecordProvenance {
        classification: lowered_record.classification,
        resolution_class: lowered_record.resolution_class,
        executable_class: lowered_record.executable_class.ok_or_else(|| {
            MergeExecutionCompilationError::MissingExecutableClass {
                record: lowered_record.record.clone(),
                resolution_class: lowered_record.resolution_class,
            }
        })?,
        causal_disposition: lowered_record.causal_disposition,
        policy_proof_boundary: lowered_record.policy_proof_boundary,
        applied_policies: lowered_record.applied_policies.clone(),
    };
    let aspect_plan = compile_executable_aspect_plans(runtime, source_record, lowered_record)?;

    match &lowered_record.record_decision {
        LoweredRecordDecision::Execute(bundle) => {
            let executable_class = provenance.executable_class;
            if !matches!(
                (executable_class, bundle.kind),
                (
                    MergeExecutableClass::AdoptSourceRecord,
                    crate::merge::data::LoweredRecordExecutionIntentKind::AdoptSourceRecord
                ) | (
                    MergeExecutableClass::PreserveSharedRecord,
                    crate::merge::data::LoweredRecordExecutionIntentKind::PreserveSharedRecord
                ) | (
                    MergeExecutableClass::ReconcileRecord,
                    crate::merge::data::LoweredRecordExecutionIntentKind::ReconcileRecord
                ) | (
                    MergeExecutableClass::ConvergeDeletedOnBothSides,
                    crate::merge::data::LoweredRecordExecutionIntentKind::ConvergeDeletedOnBothSides
                )
            ) {
                return Err(
                    MergeExecutionCompilationError::ExecutableClassDecisionMismatch {
                        record: lowered_record.record.clone(),
                        executable_class,
                        decision: crate::merge::data::LoweredRecordDecisionKind::Execute,
                    },
                );
            }
            match executable_class {
            MergeExecutableClass::AdoptSourceRecord => {
                let source_visible_snapshot = crate::merge::data::visible_record_snapshot(source_record)
                    .ok_or_else(|| MergeExecutionCompilationError::MissingSourceSnapshot {
                        record: lowered_record.record.clone(),
                        record_kind: match source_record.record_kind {
                            crate::merge::data::VisibleMergeRecordKind::Entity => "entity",
                            crate::merge::data::VisibleMergeRecordKind::Relation => "relation",
                        },
                    })?;
                Ok(BoundExecutableMergeRecordPlan::AdoptSource(
                    AdoptSourceRecordPlan {
                        source_record: lowered_record.record.clone(),
                        record_kind: source_record.record_kind.clone(),
                        source_visible_snapshot,
                        provenance,
                        aspect_plan,
                    },
                ))
            }
            MergeExecutableClass::PreserveSharedRecord => {
                Ok(BoundExecutableMergeRecordPlan::PreserveShared(
                    PreserveSharedRecordPlan {
                        record: lowered_record.record.clone(),
                        target_record: lowered_record.target_record.clone(),
                        equality_witness: crate::merge::data::SharedTruthWitness {
                            witness_digest: crate::merge::data::equality_witness_digest(
                                source_record,
                            ),
                        },
                        provenance,
                        aspect_plan,
                    },
                ))
            }
            MergeExecutableClass::ReconcileRecord => {
                if source_record.record_kind == crate::merge::data::VisibleMergeRecordKind::Relation {
                    return Err(MergeExecutionCompilationError::PreparedAuthorityBindingMismatch {
                        detail: "relation reconcile records are not executable in phase D",
                    });
                }
                let target_record = lowered_record.target_record.clone().ok_or_else(|| {
                    MergeExecutionCompilationError::MissingTargetRecord {
                        record: lowered_record.record.clone(),
                    }
                })?;
                let source_visible_snapshot = crate::merge::data::visible_record_snapshot(source_record)
                    .ok_or_else(|| MergeExecutionCompilationError::MissingSourceSnapshot {
                        record: lowered_record.record.clone(),
                        record_kind: match source_record.record_kind {
                            crate::merge::data::VisibleMergeRecordKind::Entity => "entity",
                            crate::merge::data::VisibleMergeRecordKind::Relation => "relation",
                        },
                    })?;
                Ok(BoundExecutableMergeRecordPlan::Reconcile(
                    ReconcileRecordPlan {
                        source_record: lowered_record.record.clone(),
                        target_record: target_record.clone(),
                        source_visible_snapshot,
                        identity_basis: ReconciledIdentityBasis {
                            source_record: lowered_record.record.clone(),
                            target_record,
                        },
                        causal_disposition: lowered_record.causal_disposition,
                        provenance,
                        aspect_plan,
                    },
                ))
            }
            MergeExecutableClass::ConvergeDeletedOnBothSides => Ok(
                BoundExecutableMergeRecordPlan::ConvergeDeletedOnBothSides(
                    ConvergeDeletedOnBothSidesRecordPlan {
                        source_record: lowered_record.record.clone(),
                        target_record: lowered_record.target_record.clone(),
                        equality_witness: crate::merge::data::SharedTruthWitness {
                            witness_digest: crate::merge::data::equality_witness_digest(
                                source_record,
                            ),
                        },
                        semantics: crate::merge::data::DeletedOnBothSidesSemantics::AuthoritativeMutualDeletionConvergence,
                        lineage_continuity: derive_deleted_on_both_sides_lineage_continuity(
                            lowered_record,
                            source_record,
                        ),
                        provenance,
                    },
                ),
            ),
        }
        }
        LoweredRecordDecision::Block(_) => {
            Err(MergeExecutionCompilationError::UnsupportedRecordDecision {
                record: lowered_record.record.clone(),
                decision: crate::merge::data::LoweredRecordDecisionKind::Block,
            })
        }
        LoweredRecordDecision::Reject(_) => {
            Err(MergeExecutionCompilationError::UnsupportedRecordDecision {
                record: lowered_record.record.clone(),
                decision: crate::merge::data::LoweredRecordDecisionKind::Reject,
            })
        }
    }
}

fn compile_executable_aspect_plans(
    runtime: &crate::logic::runtime::RelationalRuntime,
    source_record: &crate::merge::data::VisibleMergeRecord,
    lowered_record: &crate::merge::data::LoweredMergePlanRecord,
) -> Result<Arc<[ExecutableAspectPlan]>, MergeExecutionCompilationError> {
    let mut plans = Vec::with_capacity(lowered_record.aspect_outcomes.len());
    for aspect in lowered_record.aspect_outcomes.iter() {
        let execution_intent = aspect.execution_intent.ok_or_else(|| {
            MergeExecutionCompilationError::MissingAspectExecutionIntent {
                record: lowered_record.record.clone(),
                aspect_key: aspect.aspect_key.clone(),
            }
        })?;
        let authorized_values = aspect.authorized_values.ok_or_else(|| {
            MergeExecutionCompilationError::MissingAuthorizedAspectValues {
                record: lowered_record.record.clone(),
                aspect_key: aspect.aspect_key.clone(),
            }
        })?;
        let plan = match execution_intent {
            LoweredAspectExecutionIntent::AdoptSourceValue { .. } => {
                ExecutableAspectPlan::AdoptSourceValue {
                    aspect_key: aspect.aspect_key.clone(),
                    source_value: crate::merge::data::aspect_reference(
                        MergeValueSourceSide::Source,
                        source_record.record_ref.clone(),
                        aspect.aspect_key.clone(),
                    ),
                }
            }
            LoweredAspectExecutionIntent::PreserveSharedValue { .. } => {
                let witness_digest = aspect_shared_witness_digest(
                    runtime,
                    source_record,
                    &aspect.aspect_key,
                    lowered_record.record.clone(),
                )?;
                ExecutableAspectPlan::PreserveSharedValue {
                    aspect_key: aspect.aspect_key.clone(),
                    shared_value: MaterializedAspectValue {
                        policy: MergeValueMaterialization::EqualityWitnessDigest,
                        evidence: MaterializedAspectValueEvidence::EqualityWitnessDigest(
                            witness_digest,
                        ),
                    },
                }
            }
            LoweredAspectExecutionIntent::ReconcileVisibleValues { .. } => {
                let source_value = aspect_materialized_reference(
                    authorized_values.source,
                    MergeValueSourceSide::Source,
                    lowered_record.record.clone(),
                    aspect.aspect_key.clone(),
                );
                let target_value = aspect_materialized_reference(
                    authorized_values.target,
                    MergeValueSourceSide::Target,
                    lowered_record
                        .target_record
                        .clone()
                        .unwrap_or_else(|| lowered_record.record.clone()),
                    aspect.aspect_key.clone(),
                );
                let base_value = aspect_materialized_reference(
                    authorized_values.base,
                    MergeValueSourceSide::Base,
                    lowered_record.record.clone(),
                    aspect.aspect_key.clone(),
                );
                let resolved_value = resolved_materialized_value(lowered_record, aspect);
                ExecutableAspectPlan::ReconcileValue {
                    aspect_key: aspect.aspect_key.clone(),
                    resolved_value,
                    source_value,
                    target_value,
                    base_value,
                }
            }
        };
        plans.push(plan);
    }
    Ok(Arc::from(plans))
}

fn resolved_materialized_value(
    lowered_record: &crate::merge::data::LoweredMergePlanRecord,
    aspect: &crate::merge::data::LoweredAspectOutcome,
) -> Option<MaterializedAspectValue> {
    match aspect.resolved_value_strategy.as_ref()? {
        crate::merge::data::MergeResolvedAspectValueStrategy::SourceVisibleValue => {
            Some(crate::merge::data::aspect_reference(
                MergeValueSourceSide::Source,
                lowered_record.record.clone(),
                aspect.aspect_key.clone(),
            ))
        }
        crate::merge::data::MergeResolvedAspectValueStrategy::TargetVisibleValue => {
            Some(crate::merge::data::aspect_reference(
                MergeValueSourceSide::Target,
                lowered_record
                    .target_record
                    .clone()
                    .unwrap_or_else(|| lowered_record.record.clone()),
                aspect.aspect_key.clone(),
            ))
        }
        crate::merge::data::MergeResolvedAspectValueStrategy::BaseVisibleValue => {
            Some(crate::merge::data::aspect_reference(
                MergeValueSourceSide::Base,
                lowered_record.record.clone(),
                aspect.aspect_key.clone(),
            ))
        }
        crate::merge::data::MergeResolvedAspectValueStrategy::InlineAspectValue(value) => {
            Some(MaterializedAspectValue {
                policy: MergeValueMaterialization::EagerInlineAspectValue,
                evidence: MaterializedAspectValueEvidence::InlineAspectValue(value.clone()),
            })
        }
    }
}

fn aspect_materialized_reference(
    usage: crate::merge::data::AuthorizedAspectValueUsage,
    side: MergeValueSourceSide,
    record: RecordRef,
    aspect_key: forge_foundational::facade::AspectKey,
) -> Option<MaterializedAspectValue> {
    match usage {
        crate::merge::data::AuthorizedAspectValueUsage::NotAuthorized => None,
        crate::merge::data::AuthorizedAspectValueUsage::ConsumeVisibleValue
        | crate::merge::data::AuthorizedAspectValueUsage::ConsumeBaseValue => Some(
            crate::merge::data::aspect_reference(side, record, aspect_key),
        ),
        crate::merge::data::AuthorizedAspectValueUsage::EqualityWitnessOnly => None,
    }
}

fn aspect_shared_witness_digest(
    runtime: &crate::logic::runtime::RelationalRuntime,
    source_record: &crate::merge::data::VisibleMergeRecord,
    aspect_key: &forge_foundational::facade::AspectKey,
    record: RecordRef,
) -> Result<String, MergeExecutionCompilationError> {
    let binding =
        aspect_binding_for_record(runtime, source_record, aspect_key).ok_or_else(|| {
            MergeExecutionCompilationError::MissingAspectBinding {
                record: record.clone(),
                aspect_key: aspect_key.clone(),
            }
        })?;
    let source_component =
        binding_component_from_visible_record(source_record, binding, VisibleRecordSide::Source)
            .ok_or_else(
                || MergeExecutionCompilationError::MissingAspectValueWitness {
                    record: record.clone(),
                    aspect_key: aspect_key.clone(),
                },
            )?;
    let target_component =
        binding_component_from_visible_record(source_record, binding, VisibleRecordSide::Target)
            .ok_or_else(
                || MergeExecutionCompilationError::MissingAspectValueWitness {
                    record: record.clone(),
                    aspect_key: aspect_key.clone(),
                },
            )?;
    canonical_aspect_witness_digest(aspect_key, &source_component, &target_component).map_err(
        |error| MergeExecutionCompilationError::UnsupportedAspectValueWitness {
            record,
            aspect_key: aspect_key.clone(),
            detail: error.detail().to_owned(),
        },
    )
}

fn aspect_binding_for_record<'a>(
    runtime: &'a crate::logic::runtime::RelationalRuntime,
    source_record: &crate::merge::data::VisibleMergeRecord,
    aspect_key: &forge_foundational::facade::AspectKey,
) -> Option<&'a LoweredAspectBinding> {
    let kind_id = source_record.source_kind_id.or(source_record.kind_id)?;
    let plan = match source_record.record_kind {
        crate::merge::data::VisibleMergeRecordKind::Entity => runtime.entity_aspect_plan(kind_id),
        crate::merge::data::VisibleMergeRecordKind::Relation => {
            runtime.relation_aspect_plan(kind_id)
        }
    }?;
    plan.executable_bindings
        .iter()
        .find(|binding| binding.aspect_key() == aspect_key)
}

fn derive_deleted_on_both_sides_lineage_continuity(
    lowered_record: &crate::merge::data::LoweredMergePlanRecord,
    source_record: &crate::merge::data::VisibleMergeRecord,
) -> crate::merge::data::MergeLineageContinuityVerdict {
    let source_lineage_id = source_record.source_lineage_id.or(source_record.lineage_id);
    let target_lineage_id = source_record.target_lineage_id.or(source_record.lineage_id);

    if lowered_record.target_record.is_none() {
        return crate::merge::data::MergeLineageContinuityVerdict::Unchanged;
    }

    match (source_lineage_id, target_lineage_id) {
        (Some(source_lineage_id), Some(target_lineage_id))
            if source_lineage_id == target_lineage_id =>
        {
            crate::merge::data::MergeLineageContinuityVerdict::Unchanged
        }
        _ => crate::merge::data::MergeLineageContinuityVerdict::Preserved,
    }
}
