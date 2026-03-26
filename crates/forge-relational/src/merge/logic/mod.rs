mod aspect_plan_lookup;
mod causal;
mod conflicts;
mod identity;
mod lowering;
mod planning;
mod policy;

use std::collections::BTreeSet;
use std::time::Instant;

use crate::logic::runtime::RelationalRuntime;
use crate::merge::data::{
    MergeArtifactDigestBasis, MergeBaseDigestBasis, MergeCausalDigestBasis,
    MergeConflictDigestBasis, MergeExecutionAuthorityContract,
    MergeExecutionAuthorizationRule, MergeExecutionConsumptionRule,
    MergeExecutionDecisionSurface, MergeIdentityDigestBasis, MergeLoweredAspectDigestRow,
    MergeLoweredPlanDigestBasis, MergePlanningArtifactCore, MergePlanningError,
    MergePlanningRequest, MergePlanningSummary, MergePolicyAspectDigestRow,
    MergePolicyDigestBasis, MergeRequestDigestBasis, MergeSchemaKindClass,
    MergeSchemaKindSemanticSnapshot, MergeSchemaSnapshotDigestBasis, VisibleMergeRecordKind,
};
use crate::schema::data::RelationalSchemaRegistry;
use crate::transactions::data::RecordRef;

pub struct MergeAccess<'runtime> {
    runtime: &'runtime RelationalRuntime,
}

impl<'runtime> MergeAccess<'runtime> {
    pub(crate) fn new(runtime: &'runtime RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn runtime(&self) -> &'runtime RelationalRuntime {
        self.runtime
    }

    pub fn inspect_history_scope(
        &self,
        request: MergePlanningRequest,
    ) -> Result<MergePlanningArtifactCore, MergePlanningError> {
        self.inspect_planning_scope(request)
    }

    pub fn inspect_planning_scope(
        &self,
        request: MergePlanningRequest,
    ) -> Result<MergePlanningArtifactCore, MergePlanningError> {
        let started_at = Instant::now();
        let plan = self.lower_planning_scope(request)?;
        let target_view = self
            .runtime
            .visibility_reads()
            .read_version(plan.target_head.version_id);
        let schema_snapshot = merge_schema_snapshot(
            &self.runtime.config().schema.registry,
            plan.source_records.as_ref(),
            &target_view,
            plan.target_delta.touched_records.as_ref(),
        );
        let request_summary = format!(
            "{}:{}:{:?}",
            plan.request.target_branch.0, plan.request.source_branch.0, plan.request.merge_intent
        );
        let ancestry_summary = format!(
            "base:{};target_commits:{};source_commits:{};target_records:{};source_records:{};identity_candidates:{};exact:{};missing:{};validated_schema_correspondences:{};classified_records:{};concurrent:{};source_only:{};policy_auto:{};policy_reject:{};lowered_admitted:{};lowered_blocked:{};lowered_rejected:{}",
            plan.ancestry.merge_base_commit_id.0,
            plan.ancestry.target.unique_commit_count,
            plan.ancestry.source.unique_commit_count,
            plan.ancestry.target.touched_record_count,
            plan.ancestry.source.touched_record_count,
            plan.identity_summary.candidate_count,
            plan.identity_summary.exact_match_count,
            plan.identity_summary.missing_target_count,
            plan.identity_summary
                .schema_declared_correspondence
                .validated_count,
            plan.conflict_summary.classified_record_count,
            plan.causal_summary.concurrent_count,
            plan.causal_summary.source_only_count,
            plan.policy_summary.auto_resolved_count,
            plan.policy_summary.reject_count,
            plan.lowered_summary.admitted_count,
            plan.lowered_summary.blocked_count,
            plan.lowered_summary.rejected_count
        );
        let digest_basis = MergeArtifactDigestBasis {
            request: MergeRequestDigestBasis {
                target_branch: plan.request.target_branch.clone(),
                source_branch: plan.request.source_branch.clone(),
                merge_intent: plan.request.merge_intent,
            },
            schema: schema_snapshot.clone(),
            execution_contract: MergeExecutionAuthorityContract {
                decision_surface: MergeExecutionDecisionSurface::LoweredRecordDecisionOnly,
                identity_authority: MergeExecutionConsumptionRule::ConsumeCanonicalLoweredArtifactOnly,
                conflict_authority: MergeExecutionConsumptionRule::ConsumeCanonicalLoweredArtifactOnly,
                policy_authority: MergeExecutionConsumptionRule::ConsumeCanonicalLoweredArtifactOnly,
                value_authorization: MergeExecutionAuthorizationRule::MustNotWidenBeyondAuthorizedAspectValueSurface,
            },
            merge_base: MergeBaseDigestBasis {
                rule: plan.merge_base.rule,
                commit_id: plan.merge_base.commit_id,
                supporting_left_ancestors: plan.merge_base.supporting_left_ancestors.clone(),
                supporting_right_ancestors: plan.merge_base.supporting_right_ancestors.clone(),
            },
            identity: MergeIdentityDigestBasis {
                effective_declarations: plan.effective_identity_declarations.clone(),
                candidate_scopes: std::sync::Arc::from(
                    plan.candidates
                        .iter()
                        .map(|candidate| candidate.scope.clone())
                        .collect::<Vec<_>>(),
                ),
                candidate_sources: std::sync::Arc::from(
                    plan.candidates
                        .iter()
                        .map(|candidate| candidate.source_record.clone())
                        .collect::<Vec<_>>(),
                ),
                candidate_targets: std::sync::Arc::from(
                    plan.candidates
                        .iter()
                        .map(|candidate| candidate.target_record.clone())
                        .collect::<Vec<_>>(),
                ),
                candidate_bases: std::sync::Arc::from(
                    plan.candidates
                        .iter()
                        .map(|candidate| candidate.basis.clone())
                        .collect::<Vec<_>>(),
                ),
                candidate_match_classes: std::sync::Arc::from(
                    plan.candidates
                        .iter()
                        .map(|candidate| candidate.match_class.clone())
                        .collect::<Vec<_>>(),
                ),
                candidate_reasons: std::sync::Arc::from(
                    plan.candidates
                        .iter()
                        .map(|candidate| candidate.reason.clone())
                        .collect::<Vec<_>>(),
                ),
            },
            causal: MergeCausalDigestBasis {
                records: std::sync::Arc::from(
                    plan.causal_annotations
                        .iter()
                        .map(|annotation| annotation.record.clone())
                        .collect::<Vec<_>>(),
                ),
                dispositions: std::sync::Arc::from(
                    plan.causal_annotations
                        .iter()
                        .map(|annotation| annotation.disposition)
                        .collect::<Vec<_>>(),
                ),
            },
            conflict: MergeConflictDigestBasis {
                records: std::sync::Arc::from(
                    plan.classifications
                        .iter()
                        .map(|classification| classification.record.clone())
                        .collect::<Vec<_>>(),
                ),
                classes: std::sync::Arc::from(
                    plan.classifications
                        .iter()
                        .map(|classification| classification.class)
                        .collect::<Vec<_>>(),
                ),
                validated_schema_correspondence: std::sync::Arc::from(
                    plan.classifications
                        .iter()
                        .map(|classification| classification.validated_schema_correspondence)
                        .collect::<Vec<_>>(),
                ),
                relation_evidence: std::sync::Arc::from(
                    plan.classifications
                        .iter()
                        .map(|classification| classification.relation_evidence)
                        .collect::<Vec<_>>(),
                ),
                aspect_evidence_keys: std::sync::Arc::from(
                    plan.classifications
                        .iter()
                        .map(|classification| {
                            std::sync::Arc::from(
                                classification
                                    .aspect_evidence
                                    .iter()
                                    .map(|evidence| evidence.aspect_key.clone())
                                    .collect::<Vec<_>>(),
                            )
                        })
                        .collect::<Vec<_>>(),
                ),
                aspect_evidence_comparisons: std::sync::Arc::from(
                    plan.classifications
                        .iter()
                        .map(|classification| {
                            std::sync::Arc::from(
                                classification
                                    .aspect_evidence
                                    .iter()
                                    .map(|evidence| evidence.comparison)
                                    .collect::<Vec<_>>(),
                            )
                        })
                        .collect::<Vec<_>>(),
                ),
            },
            policy: MergePolicyDigestBasis {
                records: std::sync::Arc::from(
                    plan.policy_records
                        .iter()
                        .map(|record| record.record.clone())
                        .collect::<Vec<_>>(),
                ),
                resolutions: std::sync::Arc::from(
                    plan.policy_records
                        .iter()
                        .map(|record| record.resolution)
                        .collect::<Vec<_>>(),
                ),
                applied_policies: std::sync::Arc::from(
                    plan.policy_records
                        .iter()
                        .map(|record| record.applied_policies.clone())
                        .collect::<Vec<_>>(),
                ),
                aspect_rows: std::sync::Arc::from(
                    plan.policy_records
                        .iter()
                        .map(|record| {
                            std::sync::Arc::from(
                                record
                                    .aspect_resolutions
                                    .iter()
                                    .map(|aspect| MergePolicyAspectDigestRow {
                                        aspect_key: aspect.aspect_key.clone(),
                                        comparison: aspect.comparison,
                                        applied_policy: aspect.applied_policy.clone(),
                                        resolution: aspect.resolution,
                                    })
                                    .collect::<Vec<_>>(),
                            )
                        })
                        .collect::<Vec<_>>(),
                ),
            },
            lowered_plan: MergeLoweredPlanDigestBasis {
                records: std::sync::Arc::from(
                    plan.lowered_records
                        .iter()
                        .map(|record| record.record.clone())
                        .collect::<Vec<_>>(),
                ),
                readiness: std::sync::Arc::from(
                    plan.lowered_records
                        .iter()
                        .map(|record| record.readiness)
                        .collect::<Vec<_>>(),
                ),
                record_decisions: std::sync::Arc::from(
                    plan.lowered_records
                        .iter()
                        .map(record_decision_kind)
                        .collect::<Vec<_>>(),
                ),
                lowered_actions: std::sync::Arc::from(
                    plan.lowered_records
                        .iter()
                        .map(|record| record.lowered_action)
                        .collect::<Vec<_>>(),
                ),
                blocked_reasons: std::sync::Arc::from(
                    plan.lowered_records
                        .iter()
                        .map(|record| record.blocked_reason)
                        .collect::<Vec<_>>(),
                ),
                rejected_reasons: std::sync::Arc::from(
                    plan.lowered_records
                        .iter()
                        .map(|record| record.rejected_reason)
                        .collect::<Vec<_>>(),
                ),
                execution_bundle_kinds: std::sync::Arc::from(
                    plan.lowered_records
                        .iter()
                        .map(|record| match &record.record_decision {
                            crate::merge::data::LoweredRecordDecision::Execute(bundle) => {
                                Some(bundle.kind)
                            }
                            crate::merge::data::LoweredRecordDecision::Block(_)
                            | crate::merge::data::LoweredRecordDecision::Reject(_) => None,
                        })
                        .collect::<Vec<_>>(),
                ),
                denial_bundle_kinds: std::sync::Arc::from(
                    plan.lowered_records
                        .iter()
                        .map(|record| match &record.record_decision {
                            crate::merge::data::LoweredRecordDecision::Block(bundle)
                            | crate::merge::data::LoweredRecordDecision::Reject(bundle) => {
                                Some(bundle.kind)
                            }
                            crate::merge::data::LoweredRecordDecision::Execute(_) => None,
                        })
                        .collect::<Vec<_>>(),
                ),
                aspect_rows: std::sync::Arc::from(
                    plan.lowered_records
                        .iter()
                        .map(|record| {
                            std::sync::Arc::from(
                                record
                                    .aspect_outcomes
                                    .iter()
                                    .map(|aspect| MergeLoweredAspectDigestRow {
                                        aspect_key: aspect.aspect_key.clone(),
                                        readiness: aspect.readiness,
                                        lowered_action: aspect.lowered_action,
                                        authorized_values: aspect.authorized_values,
                                        execution_intent: aspect.execution_intent,
                                        denial_intent: aspect.denial_intent,
                                        blocked_reason: aspect.blocked_reason,
                                        rejected_reason: aspect.rejected_reason,
                                    })
                                    .collect::<Vec<_>>(),
                            )
                        })
                        .collect::<Vec<_>>(),
                ),
            },
            decision_log: plan.decision_log_digest_basis.clone(),
        };
        self.runtime.performance_access().count_merge_planning_request(
            schema_snapshot.touched_kinds.len(),
            plan.ancestry.target.unique_commit_count,
            plan.ancestry.source.unique_commit_count,
            plan.ancestry.target.touched_record_count,
            plan.ancestry.source.touched_record_count,
        );
        self.runtime.performance_access().count_merge_identity_discovery(
            plan.identity_summary.candidate_count,
            plan.identity_summary.effective_declarations.len(),
        );
        self.runtime.performance_access().count_merge_conflict_classification(
            plan.conflict_summary.classified_record_count,
        );
        self.runtime.performance_access().count_merge_causal_annotation(
            plan.causal_summary.classified_record_count,
        );
        self.runtime.performance_access().count_merge_policy_resolution(
            plan.policy_summary.resolved_record_count,
        );
        self.runtime.performance_access().count_merge_lowering(
            plan.lowered_summary.record_count,
            plan.decision_log.decisions.len(),
        );
        self.runtime.performance_access().count_merge_planning_elapsed(
            started_at.elapsed().as_nanos(),
        );
        Ok(MergePlanningArtifactCore {
            request: plan.request,
            schema_snapshot,
            execution_authority_contract: MergeExecutionAuthorityContract {
                decision_surface: MergeExecutionDecisionSurface::LoweredRecordDecisionOnly,
                identity_authority: MergeExecutionConsumptionRule::ConsumeCanonicalLoweredArtifactOnly,
                conflict_authority: MergeExecutionConsumptionRule::ConsumeCanonicalLoweredArtifactOnly,
                policy_authority: MergeExecutionConsumptionRule::ConsumeCanonicalLoweredArtifactOnly,
                value_authorization: MergeExecutionAuthorizationRule::MustNotWidenBeyondAuthorizedAspectValueSurface,
            },
            merge_base: plan.merge_base,
            ancestry: plan.ancestry,
            identity_discovery: plan.identity_summary,
            conflict_classification: plan.conflict_summary,
            causal_annotation: plan.causal_summary,
            policy_resolution: plan.policy_summary,
            lowered_plan: plan.lowered_summary,
            decision_log: plan.decision_log,
            digest_basis,
            decision_log_digest_basis: plan.decision_log_digest_basis,
            summary: MergePlanningSummary {
                request_summary,
                ancestry_summary,
            },
        })
    }
}

fn record_decision_kind(
    record: &crate::merge::data::LoweredMergePlanRecord,
) -> crate::merge::data::LoweredRecordDecisionKind {
    match record.record_decision {
        crate::merge::data::LoweredRecordDecision::Execute(_) => {
            crate::merge::data::LoweredRecordDecisionKind::Execute
        }
        crate::merge::data::LoweredRecordDecision::Block(_) => {
            crate::merge::data::LoweredRecordDecisionKind::Block
        }
        crate::merge::data::LoweredRecordDecision::Reject(_) => {
            crate::merge::data::LoweredRecordDecisionKind::Reject
        }
    }
}

fn merge_schema_snapshot(
    registry: &RelationalSchemaRegistry,
    source_records: &[crate::merge::data::VisibleMergeRecord],
    target_view: &crate::storage::data::RelationalReadView,
    target_touched_records: &[crate::merge::data::BranchTouchedRecordDelta],
) -> MergeSchemaSnapshotDigestBasis {
    let mut touched_entity_kinds = BTreeSet::new();
    let mut touched_relation_kinds = BTreeSet::new();
    for record in source_records {
        for kind_id in [record.source_kind_id, record.target_kind_id]
            .into_iter()
            .flatten()
        {
            match record.record_kind {
                VisibleMergeRecordKind::Entity => {
                    touched_entity_kinds.insert(kind_id);
                }
                VisibleMergeRecordKind::Relation => {
                    touched_relation_kinds.insert(kind_id);
                }
            }
        }
    }
    for delta in target_touched_records {
        match &delta.target {
            RecordRef::Entity(entity_id) => {
                if let Some(entity) = target_view.get_entity(*entity_id) {
                    touched_entity_kinds.insert(entity.kind.kind_id);
                }
            }
            RecordRef::Relation(relation_id) => {
                if let Some(relation) = target_view.get_relation(*relation_id) {
                    touched_relation_kinds.insert(relation.kind.kind_id);
                }
            }
        }
    }

    let mut touched_kinds = Vec::new();
    for kind_id in touched_entity_kinds {
        if let Ok(registration) = registry.entity_registration(kind_id) {
            touched_kinds.push(MergeSchemaKindSemanticSnapshot {
                kind_class: MergeSchemaKindClass::Entity,
                kind_id,
                kind_name: registration.kind_name.clone(),
                schema_id: registration.schema_id.clone(),
                schema_version_id: registration.schema_version_id,
                aspect_plan_revision: registration.aspect_declarations.plan_revision,
                identity_declarations: registration
                    .aspect_declarations
                    .identity_declarations
                    .clone(),
                merge_policy_declarations: registration
                    .aspect_declarations
                    .merge_policy_declarations
                    .clone(),
                relation_payload_class: None,
                relation_integrity_plan_revision: None,
            });
        }
    }
    for kind_id in touched_relation_kinds {
        if let Ok(registration) = registry.relation_registration(kind_id) {
            touched_kinds.push(MergeSchemaKindSemanticSnapshot {
                kind_class: MergeSchemaKindClass::Relation,
                kind_id,
                kind_name: registration.kind_name.clone(),
                schema_id: registration.schema_id.clone(),
                schema_version_id: registration.schema_version_id,
                aspect_plan_revision: registration.aspect_declarations.plan_revision,
                identity_declarations: registration
                    .aspect_declarations
                    .identity_declarations
                    .clone(),
                merge_policy_declarations: registration
                    .aspect_declarations
                    .merge_policy_declarations
                    .clone(),
                relation_payload_class: Some(registration.payload_class),
                relation_integrity_plan_revision: Some(registration.relation_integrity.plan_revision),
            });
        }
    }

    let (authoritative_schema_id, authoritative_schema_version_id) =
        match registry.authoritative_schema_basis() {
            Ok(Some((schema_id, schema_version_id))) => (Some(schema_id), Some(schema_version_id)),
            _ => (None, None),
        };
    MergeSchemaSnapshotDigestBasis {
        authoritative_schema_id,
        authoritative_schema_version_id,
        touched_kinds: std::sync::Arc::from(touched_kinds),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::facade::history::BranchId;
    use crate::facade::merge::{
        AspectMergePolicyDeclaration, AspectMergePolicyKind, IdentityBasisDeclaration,
    };
    use crate::facade::runtime::RelationalRuntimeApi;
    use crate::facade::schema::{
        EntityKindRegistration, KindAspectDeclarations, RelationKindRegistration,
        RelationalSchemaRegistry, SchemaId, SchemaVersionId,
    };
    use crate::facade::transactions::{
        DeleteRelationIntent, MutationIntent, RelationMutationIntent, TransactionOptions,
        WorkerIntentBatch,
    };
    use crate::merge::data::{IdentityBasisKind, IdentityBasisScope, MergeIntent};
    use crate::tests::support::{
        changed_entities, create_branch_from_main, create_entity, create_entity_outcome,
        create_entity_outcome_on_branch, create_relation, entity_payload_aspect,
        merge_commit_from_branches, persisted_runtime_with_test_schema, update_entity,
        update_entity_on_branch,
    };
    use crate::{config::data::{CascadeDeletePolicy, CrossContextPolicy}, schema::data::RelationPayloadClass, symbols::data::InternedString};

    #[test]
    fn inspect_history_scope_uses_current_merge_base_rule_and_branch_delta_shape() {
        let mut runtime = persisted_runtime_with_test_schema();
        let root = create_entity_outcome(&mut runtime, "root");
        let linear = create_entity_outcome(&mut runtime, "linear");
        create_branch_from_main(&mut runtime, "feature");
        let feature = create_entity_outcome_on_branch(
            &mut runtime,
            "feature",
            BranchId("feature".to_string()),
        );

        let artifact = runtime
            .merge_access()
            .inspect_history_scope(crate::merge::data::MergePlanningRequest::new(
                BranchId("main".to_string()),
                BranchId("feature".to_string()),
                MergeIntent::ReconcileIntoTarget,
            ))
            .unwrap();

        assert_eq!(artifact.request.target_branch, BranchId("main".to_string()));
        assert_eq!(artifact.request.source_branch, BranchId("feature".to_string()));
        assert_eq!(
            artifact.digest_basis.request.target_branch,
            BranchId("main".to_string())
        );
        assert_eq!(
            artifact.digest_basis.request.source_branch,
            BranchId("feature".to_string())
        );
        let merge_base = artifact.merge_base;
        assert_eq!(merge_base.commit_id, linear.commit.commit_id);
        assert_eq!(
            artifact.digest_basis.merge_base.commit_id,
            linear.commit.commit_id
        );
        assert_eq!(artifact.ancestry.target.unique_commit_count, 0);
        assert_eq!(artifact.ancestry.source.unique_commit_count, 1);
        assert!(merge_base
            .supporting_left_ancestors
            .iter()
            .any(|commit_id| *commit_id == root.commit.commit_id));
        assert!(merge_base
            .supporting_right_ancestors
            .iter()
            .any(|commit_id| *commit_id == feature.commit.commit_id));
        assert!(artifact.summary.request_summary.contains("main:feature"));
        assert!(artifact
            .summary
            .ancestry_summary
            .contains(&format!("base:{}", linear.commit.commit_id.0)));
        assert_eq!(artifact.identity_discovery.candidate_count, 1);
        assert_eq!(artifact.identity_discovery.missing_target_count, 1);
        assert_eq!(artifact.conflict_classification.classified_record_count, 1);
        assert_eq!(artifact.causal_annotation.classified_record_count, 1);
        assert_eq!(artifact.causal_annotation.source_only_count, 1);
        assert_eq!(artifact.policy_resolution.auto_resolved_count, 1);
        assert_eq!(artifact.lowered_plan.admitted_count, 1);
        assert!(artifact.lowered_plan.fully_execution_ready);
        assert_eq!(artifact.decision_log.decisions.len(), 1);
        assert_eq!(
            artifact.execution_authority_contract.decision_surface,
            crate::merge::data::MergeExecutionDecisionSurface::LoweredRecordDecisionOnly
        );
        assert_eq!(
            artifact.execution_authority_contract.value_authorization,
            crate::merge::data::MergeExecutionAuthorizationRule::MustNotWidenBeyondAuthorizedAspectValueSurface
        );
        assert_eq!(
            artifact.decision_log.decisions[0].decision,
            crate::merge::data::MergePlanningDecisionKind::Admitted
        );
        assert_eq!(
            artifact.lowered_plan.records[0].lowered_action,
            Some(crate::merge::data::LoweredMergeAction::KeepSourceAddition)
        );
        assert_eq!(
            artifact.lowered_plan.records[0].record_decision,
            crate::merge::data::LoweredRecordDecision::Execute(crate::merge::data::LoweredRecordExecutionBundle {
                kind: crate::merge::data::LoweredRecordExecutionIntentKind::AdoptSourceRecord,
                aspects: std::sync::Arc::from(Vec::<crate::merge::data::LoweredRecordExecutionAspectIntent>::new()),
            })
        );
        assert!(
            artifact.lowered_plan.records[0].aspect_outcomes.is_empty()
                || artifact.lowered_plan.records[0]
                    .aspect_outcomes
                    .iter()
                    .all(|outcome| {
                        outcome.lowered_action
                            == Some(crate::merge::data::LoweredAspectAction::AdoptSourceAspect)
                            && outcome.authorized_values
                                == Some(crate::merge::data::AuthorizedAspectValueSurface {
                                    source: crate::merge::data::AuthorizedAspectValueUsage::ConsumeVisibleValue,
                                    target: crate::merge::data::AuthorizedAspectValueUsage::NotAuthorized,
                                    base: crate::merge::data::AuthorizedAspectValueUsage::NotAuthorized,
                                })
                            && outcome.execution_intent
                                == Some(crate::merge::data::LoweredAspectExecutionIntent::AdoptSourceValue {
                                    authorized_values: crate::merge::data::AuthorizedAspectValueSurface {
                                        source: crate::merge::data::AuthorizedAspectValueUsage::ConsumeVisibleValue,
                                        target: crate::merge::data::AuthorizedAspectValueUsage::NotAuthorized,
                                        base: crate::merge::data::AuthorizedAspectValueUsage::NotAuthorized,
                                    },
                                })
                    })
        );
    }

    #[test]
    fn history_scope_delta_excludes_foreign_branch_commits_from_prior_merge_ancestry() {
        let mut runtime = persisted_runtime_with_test_schema();
        let root = create_entity_outcome(&mut runtime, "root");
        create_branch_from_main(&mut runtime, "feature");
        create_branch_from_main(&mut runtime, "other");

        let other = create_entity_outcome_on_branch(
            &mut runtime,
            "other-branch-change",
            BranchId("other".to_string()),
        );
        let main_merge = merge_commit_from_branches(
            &mut runtime,
            BranchId("main".to_string()),
            vec![BranchId("other".to_string())],
        );
        let main_linear = create_entity_outcome(&mut runtime, "main-linear");
        let feature = create_entity_outcome_on_branch(
            &mut runtime,
            "feature-branch-change",
            BranchId("feature".to_string()),
        );

        let plan = runtime
            .merge_access()
            .plan_history_scope(crate::merge::data::MergePlanningRequest::new(
                BranchId("main".to_string()),
                BranchId("feature".to_string()),
                MergeIntent::ReconcileIntoTarget,
            ))
            .unwrap();

        assert_eq!(plan.merge_base.commit_id, root.commit.commit_id);
        assert_eq!(
            plan.target_delta.commits.as_ref(),
            &[main_merge.commit.commit_id, main_linear.commit.commit_id]
        );
        assert!(
            !plan.target_delta.commits.iter().any(|commit_id| *commit_id == other.commit.commit_id),
            "target delta must stay branch-local even when main ancestry contains merged parents"
        );
        assert!(
            !plan
                .target_delta
                .touched_records
                .iter()
                .any(|delta| delta.commit_ids.iter().any(|commit_id| *commit_id == other.commit.commit_id)),
            "touched-record evidence must not cite foreign-branch commits"
        );
        assert_eq!(plan.source_delta.commits.as_ref(), &[feature.commit.commit_id]);
    }

    #[test]
    fn inspect_planning_scope_discovers_exact_storage_and_missing_target_identity_candidates() {
        let mut runtime = persisted_runtime_with_test_schema();
        let shared = create_entity(&mut runtime, "shared");
        create_branch_from_main(&mut runtime, "feature");
        update_entity(&mut runtime, shared, "shared-main");
        update_entity_on_branch(
            &mut runtime,
            shared,
            "shared-feature",
            BranchId("feature".to_string()),
        );
        let feature_only = create_entity_outcome_on_branch(
            &mut runtime,
            "feature-only",
            BranchId("feature".to_string()),
        );
        let feature_only_entity = changed_entities(&feature_only)[0];

        let artifact = runtime
            .merge_access()
            .inspect_planning_scope(crate::merge::data::MergePlanningRequest::new(
                BranchId("main".to_string()),
                BranchId("feature".to_string()),
                MergeIntent::ReconcileIntoTarget,
            ))
            .unwrap();

        assert_eq!(artifact.ancestry.target.unique_commit_count, 1);
        assert_eq!(artifact.ancestry.source.unique_commit_count, 2);
        assert!(artifact
            .identity_discovery
            .effective_declarations
            .iter()
            .any(|declaration| declaration.scope == IdentityBasisScope::EntityKind(crate::facade::identity::KindId(1))
                && declaration.basis == IdentityBasisKind::StorageIdentity));
        assert!(artifact
            .identity_discovery
            .effective_declarations
            .iter()
            .any(|declaration| declaration.scope == IdentityBasisScope::EntityKind(crate::facade::identity::KindId(1))
                && declaration.basis == IdentityBasisKind::LineageIdentity));

        let exact_shared = artifact
            .identity_discovery
            .candidates
            .iter()
            .find(|candidate| candidate.source_record == crate::transactions::data::RecordRef::Entity(shared))
            .expect("shared candidate");
        assert_eq!(exact_shared.match_class, crate::merge::data::IdentityMatchClass::Exact);
        assert_eq!(exact_shared.basis, IdentityBasisKind::StorageIdentity);
        assert_eq!(
            exact_shared.target_record,
            Some(crate::transactions::data::RecordRef::Entity(shared))
        );

        let missing_feature_only = artifact
            .identity_discovery
            .candidates
            .iter()
            .find(|candidate| candidate.source_record == crate::transactions::data::RecordRef::Entity(feature_only_entity))
            .expect("feature-only candidate");
        assert_eq!(
            missing_feature_only.match_class,
            crate::merge::data::IdentityMatchClass::MissingTarget
        );
        assert_eq!(artifact.identity_discovery.exact_match_count, 1);
        assert_eq!(artifact.identity_discovery.missing_target_count, 1);
        assert_eq!(artifact.conflict_classification.classified_record_count, 2);
        assert_eq!(artifact.conflict_classification.divergent_visible_state_count, 1);
        assert_eq!(artifact.conflict_classification.source_only_addition_count, 1);
        assert_eq!(artifact.causal_annotation.classified_record_count, 2);
        assert_eq!(artifact.causal_annotation.concurrent_count, 1);
        assert_eq!(artifact.causal_annotation.source_only_count, 1);
        assert_eq!(artifact.policy_resolution.auto_resolved_count, 1);
        assert_eq!(artifact.policy_resolution.requires_manual_resolution_count, 1);
        assert_eq!(artifact.lowered_plan.admitted_count, 1);
        assert_eq!(artifact.lowered_plan.blocked_count, 1);
        assert!(!artifact.lowered_plan.fully_execution_ready);
        assert_eq!(artifact.decision_log.decisions.len(), 2);
        assert!(artifact
            .lowered_plan
            .records
            .iter()
            .any(|record| record.lowered_action == Some(crate::merge::data::LoweredMergeAction::KeepSourceAddition)));
        assert!(artifact
            .lowered_plan
            .records
            .iter()
            .any(|record| record.blocked_reason
                == Some(crate::merge::data::LoweredMergeBlockedReason::ManualConflictResolutionRequired)));
        assert!(artifact
            .lowered_plan
            .records
            .iter()
            .any(|record| {
                record.blocked_reason
                    == Some(crate::merge::data::LoweredMergeBlockedReason::ManualConflictResolutionRequired)
                    || record.aspect_outcomes.iter().any(|outcome| {
                        outcome.blocked_reason
                            == Some(crate::merge::data::LoweredMergeBlockedReason::ManualConflictResolutionRequired)
                    })
            }));
    }

    #[test]
    fn inspect_planning_scope_discovers_relation_storage_identity_for_deleted_source_relation() {
        let mut runtime = persisted_runtime_with_test_schema();
        let left = create_entity(&mut runtime, "left");
        let right = create_entity(&mut runtime, "right");
        let relation = create_relation(&mut runtime, left, right, "edge");
        create_branch_from_main(&mut runtime, "feature");

        let mut feature_txn = runtime.begin_transaction(TransactionOptions {
            target_branch: Some(BranchId("feature".to_string())),
            ..TransactionOptions::default()
        });
        feature_txn.push_batch(
            WorkerIntentBatch::new("delete-feature-relation").push(MutationIntent::Relation(
                RelationMutationIntent::Delete(DeleteRelationIntent { relation_id: relation }),
            )),
        );
        feature_txn.commit().expect("delete relation on feature");

        let artifact = runtime
            .merge_access()
            .inspect_planning_scope(crate::merge::data::MergePlanningRequest::new(
                BranchId("main".to_string()),
                BranchId("feature".to_string()),
                MergeIntent::ReconcileIntoTarget,
            ))
            .unwrap();

        let relation_candidate = artifact
            .identity_discovery
            .candidates
            .iter()
            .find(|candidate| {
                candidate.source_record == crate::transactions::data::RecordRef::Relation(relation)
            })
            .expect("relation candidate");
        assert_eq!(relation_candidate.match_class, crate::merge::data::IdentityMatchClass::Exact);
        assert_eq!(relation_candidate.basis, IdentityBasisKind::StorageIdentity);
        assert_eq!(
            relation_candidate.scope,
            Some(IdentityBasisScope::RelationKind(crate::facade::identity::KindId(2)))
        );
        assert_eq!(
            relation_candidate.target_record,
            Some(crate::transactions::data::RecordRef::Relation(relation))
        );
        assert_eq!(artifact.conflict_classification.deletion_conflict_count, 1);
        assert_eq!(
            artifact.conflict_classification.classifications[0].relation_evidence,
            Some(crate::merge::data::RelationConflictEvidence {
                endpoint_continuity: crate::merge::data::EndpointContinuityClass::EndpointsStable,
                relation_continuity: crate::merge::data::RelationContinuityClass::PreserveRelationIdentity,
                propagation: crate::merge::data::RelationConflictPropagation::RelationLocalOnly,
            })
        );
        assert_eq!(artifact.causal_annotation.source_only_count, 1);
        assert_eq!(artifact.policy_resolution.requires_manual_resolution_count, 1);
        assert_eq!(artifact.lowered_plan.blocked_count, 1);
        assert_eq!(
            artifact.lowered_plan.records[0].blocked_reason,
            Some(crate::merge::data::LoweredMergeBlockedReason::DeletionSemanticsRequireExplicitResolution)
        );
        assert_eq!(
            artifact.lowered_plan.records[0].record_decision,
            crate::merge::data::LoweredRecordDecision::Block(crate::merge::data::LoweredRecordDenialBundle {
                kind: crate::merge::data::LoweredRecordDenialKind::BlockedDeletion,
                aspects: std::sync::Arc::from(Vec::<crate::merge::data::LoweredRecordDenialAspectIntent>::new()),
            })
        );
        assert!(artifact.lowered_plan.records[0]
            .aspect_outcomes
            .iter()
            .all(|outcome| outcome.blocked_reason
                == Some(crate::merge::data::LoweredMergeBlockedReason::DeletionSemanticsRequireExplicitResolution)));
    }

    #[test]
    fn inspect_planning_scope_uses_aspect_key_declared_key_set_for_storage_distinct_correspondence() {
        let name_key = crate::publication::patch::data::AspectKey(InternedString::Raw("name".to_string()));
        let registry = RelationalSchemaRegistry::new()
            .register_entity_kind(EntityKindRegistration {
                kind_id: crate::facade::identity::KindId(1),
                kind_name: "test.entity".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(1),
                aspect_declarations: KindAspectDeclarations::new(vec![entity_payload_aspect("name", "name")])
                    .with_identity_declarations(vec![IdentityBasisDeclaration {
                        scope: IdentityBasisScope::AspectKey(name_key.clone()),
                        basis: IdentityBasisKind::DeclaredKeySet(Arc::from([name_key.clone()])),
                    }])
                    .with_merge_policy_declarations(vec![AspectMergePolicyDeclaration {
                        aspect_key: name_key.clone(),
                        policy: AspectMergePolicyKind::PreferRicher,
                    }]),
            })
            .and_then(|registry| {
                registry.register_relation_kind(RelationKindRegistration {
                    kind_id: crate::facade::identity::KindId(2),
                    kind_name: "test.relation".to_string(),
                    schema_id: SchemaId("test".to_string()),
                    schema_version_id: SchemaVersionId(1),
                    payload_class: RelationPayloadClass::PayloadBearingRelation,
                    cross_context_policy: CrossContextPolicy::AllowExplicit,
                    cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
                    aspect_declarations: KindAspectDeclarations::default(),
                    relation_integrity: crate::schema::data::RelationIntegrityDeclarations::default(),
                })
            })
            .unwrap();
        let mut runtime = RelationalRuntimeApi::builder().schema_registry(registry).build();
        create_entity(&mut runtime, "root");
        create_branch_from_main(&mut runtime, "feature");
        let main_entity = create_entity(&mut runtime, "shared-name");
        let feature_entity = create_entity_outcome_on_branch(
            &mut runtime,
            "shared-name",
            BranchId("feature".to_string()),
        );
        let feature_entity_id = changed_entities(&feature_entity)[0];

        let artifact = runtime
            .merge_access()
            .inspect_planning_scope(crate::merge::data::MergePlanningRequest::new(
                BranchId("main".to_string()),
                BranchId("feature".to_string()),
                MergeIntent::ReconcileIntoTarget,
            ))
            .unwrap();

        let candidate = artifact
            .identity_discovery
            .candidates
            .iter()
            .find(|candidate| {
                candidate.source_record
                    == crate::transactions::data::RecordRef::Entity(feature_entity_id)
            })
            .expect("feature entity candidate");
        assert_eq!(candidate.match_class, crate::merge::data::IdentityMatchClass::Reconciliable);
        assert_eq!(candidate.reason, crate::merge::data::IdentityResolutionReason::SchemaDeclaredCorrespondence);
        assert_eq!(
            candidate.scope,
            Some(IdentityBasisScope::AspectKey(name_key.clone()))
        );
        assert_eq!(
            candidate.basis,
            IdentityBasisKind::DeclaredKeySet(Arc::from([crate::publication::patch::data::AspectKey(
                InternedString::Raw("name".to_string())
            )]))
        );
        assert_eq!(
            candidate.target_record,
            Some(crate::transactions::data::RecordRef::Entity(main_entity))
        );
        assert_eq!(artifact.identity_discovery.reconciliable_match_count, 1);
        assert_eq!(
            artifact
                .identity_discovery
                .schema_declared_correspondence
                .validated_count,
            1
        );
        assert_eq!(
            artifact
                .conflict_classification
                .schema_declared_correspondence_count,
            1
        );
        assert_eq!(artifact.causal_annotation.source_only_count, 0);
        assert_eq!(artifact.causal_annotation.concurrent_count, 1);
        assert_eq!(artifact.policy_resolution.auto_resolved_count, 1);
        assert_eq!(artifact.lowered_plan.admitted_count, 1);
        assert!(artifact.lowered_plan.fully_execution_ready);
        assert_eq!(
            artifact.decision_log_digest_basis.canonical_decisions.as_ref(),
            &[crate::merge::data::MergePlanningDecisionKind::Admitted]
        );
        assert_eq!(
            artifact.digest_basis.lowered_plan.record_decisions.as_ref(),
            &[crate::merge::data::LoweredRecordDecisionKind::Execute]
        );
        assert_eq!(
            artifact.lowered_plan.records[0].lowered_action,
            Some(crate::merge::data::LoweredMergeAction::ReconcileSchemaCorrespondence)
        );
        assert_eq!(
            artifact.lowered_plan.records[0].record_decision,
            crate::merge::data::LoweredRecordDecision::Execute(crate::merge::data::LoweredRecordExecutionBundle {
                kind: crate::merge::data::LoweredRecordExecutionIntentKind::ReconcileRecord,
                aspects: std::sync::Arc::from(vec![
                    crate::merge::data::LoweredRecordExecutionAspectIntent {
                        aspect_key: name_key.clone(),
                        intent: crate::merge::data::LoweredAspectExecutionIntent::ReconcileVisibleValues {
                            authorized_values: crate::merge::data::AuthorizedAspectValueSurface {
                                source: crate::merge::data::AuthorizedAspectValueUsage::ConsumeVisibleValue,
                                target: crate::merge::data::AuthorizedAspectValueUsage::ConsumeVisibleValue,
                                base: crate::merge::data::AuthorizedAspectValueUsage::ConsumeBaseValue,
                            },
                        },
                    }
                ]),
            })
        );
        assert_eq!(artifact.lowered_plan.records[0].aspect_outcomes.len(), 1);
        assert_eq!(
            artifact.lowered_plan.records[0].aspect_outcomes[0].applied_policy,
            Some(crate::merge::data::AspectMergePolicyKind::PreferRicher)
        );
        assert_eq!(
            artifact.lowered_plan.records[0].aspect_outcomes[0].lowered_action,
            Some(crate::merge::data::LoweredAspectAction::ReconcileCorrespondedAspect)
        );
        assert_eq!(
            artifact.lowered_plan.records[0].aspect_outcomes[0].authorized_values,
            Some(crate::merge::data::AuthorizedAspectValueSurface {
                source: crate::merge::data::AuthorizedAspectValueUsage::ConsumeVisibleValue,
                target: crate::merge::data::AuthorizedAspectValueUsage::ConsumeVisibleValue,
                base: crate::merge::data::AuthorizedAspectValueUsage::ConsumeBaseValue,
            })
        );
        assert_eq!(
            artifact.lowered_plan.records[0].aspect_outcomes[0].execution_intent,
            Some(crate::merge::data::LoweredAspectExecutionIntent::ReconcileVisibleValues {
                authorized_values: crate::merge::data::AuthorizedAspectValueSurface {
                    source: crate::merge::data::AuthorizedAspectValueUsage::ConsumeVisibleValue,
                    target: crate::merge::data::AuthorizedAspectValueUsage::ConsumeVisibleValue,
                    base: crate::merge::data::AuthorizedAspectValueUsage::ConsumeBaseValue,
                },
            })
        );

        let plan = runtime
            .merge_access()
            .plan_identity_scope(crate::merge::data::MergePlanningRequest::new(
                BranchId("main".to_string()),
                BranchId("feature".to_string()),
                MergeIntent::ReconcileIntoTarget,
            ))
            .unwrap();
        assert_eq!(plan.validated_schema_correspondences.len(), 1);
        let correspondence = &plan.validated_schema_correspondences[0];
        assert_eq!(
            correspondence.source_record,
            crate::transactions::data::RecordRef::Entity(feature_entity_id)
        );
        assert_eq!(
            correspondence.target_record,
            crate::transactions::data::RecordRef::Entity(main_entity)
        );
        assert_eq!(correspondence.candidate_count_for_source, 1);
        assert_eq!(correspondence.candidate_count_for_target, 1);
    }

    #[test]
    fn identity_scope_rejects_schema_declared_correspondence_when_target_is_non_unique_across_request() {
        let name_key = crate::publication::patch::data::AspectKey(InternedString::Raw("name".to_string()));
        let registry = RelationalSchemaRegistry::new()
            .register_entity_kind(EntityKindRegistration {
                kind_id: crate::facade::identity::KindId(1),
                kind_name: "test.entity".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(1),
                aspect_declarations: KindAspectDeclarations::new(vec![entity_payload_aspect("name", "name")])
                    .with_identity_declarations(vec![IdentityBasisDeclaration {
                        scope: IdentityBasisScope::AspectKey(name_key.clone()),
                        basis: IdentityBasisKind::DeclaredKeySet(Arc::from([name_key.clone()])),
                    }])
                    .with_merge_policy_declarations(vec![AspectMergePolicyDeclaration {
                        aspect_key: name_key.clone(),
                        policy: AspectMergePolicyKind::PreferRicher,
                    }]),
            })
            .and_then(|registry| {
                registry.register_relation_kind(RelationKindRegistration {
                    kind_id: crate::facade::identity::KindId(2),
                    kind_name: "test.relation".to_string(),
                    schema_id: SchemaId("test".to_string()),
                    schema_version_id: SchemaVersionId(1),
                    payload_class: RelationPayloadClass::PayloadBearingRelation,
                    cross_context_policy: CrossContextPolicy::AllowExplicit,
                    cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
                    aspect_declarations: KindAspectDeclarations::default(),
                    relation_integrity: crate::schema::data::RelationIntegrityDeclarations::default(),
                })
            })
            .unwrap();
        let mut runtime = RelationalRuntimeApi::builder().schema_registry(registry).build();
        create_entity(&mut runtime, "root");
        create_branch_from_main(&mut runtime, "feature");
        let main_entity = create_entity(&mut runtime, "shared-name");
        let feature_first = create_entity_outcome_on_branch(
            &mut runtime,
            "shared-name",
            BranchId("feature".to_string()),
        );
        let feature_second = create_entity_outcome_on_branch(
            &mut runtime,
            "shared-name",
            BranchId("feature".to_string()),
        );
        let feature_ids = [
            changed_entities(&feature_first)[0],
            changed_entities(&feature_second)[0],
        ];

        let artifact = runtime
            .merge_access()
            .inspect_planning_scope(crate::merge::data::MergePlanningRequest::new(
                BranchId("main".to_string()),
                BranchId("feature".to_string()),
                MergeIntent::ReconcileIntoTarget,
            ))
            .unwrap();
        assert_eq!(
            artifact
                .identity_discovery
                .schema_declared_correspondence
                .candidate_count,
            2
        );
        assert_eq!(
            artifact
                .identity_discovery
                .schema_declared_correspondence
                .validated_count,
            0
        );
        assert_eq!(
            artifact
                .identity_discovery
                .schema_declared_correspondence
                .rejected_non_unique_target_count,
            1
        );
        assert_eq!(
            artifact.conflict_classification.schema_declared_correspondence_count,
            0
        );
        assert_eq!(artifact.conflict_classification.divergent_visible_state_count, 2);
        assert_eq!(artifact.causal_annotation.concurrent_count, 2);
        assert_eq!(artifact.causal_annotation.source_only_count, 0);
        assert_eq!(artifact.policy_resolution.auto_resolved_count, 0);
        assert_eq!(artifact.policy_resolution.requires_manual_resolution_count, 2);
        assert_eq!(artifact.lowered_plan.admitted_count, 0);
        assert_eq!(artifact.lowered_plan.blocked_count, 2);
        assert!(!artifact.lowered_plan.fully_execution_ready);
        assert!(artifact
            .lowered_plan
            .records
            .iter()
            .all(|record| record.blocked_reason
                == Some(crate::merge::data::LoweredMergeBlockedReason::ManualConflictResolutionRequired)));

        let plan = runtime
            .merge_access()
            .plan_identity_scope(crate::merge::data::MergePlanningRequest::new(
                BranchId("main".to_string()),
                BranchId("feature".to_string()),
                MergeIntent::ReconcileIntoTarget,
            ))
            .unwrap();
        assert!(plan.validated_schema_correspondences.is_empty());
        for feature_id in feature_ids {
            let candidate = plan
                .candidates
                .iter()
                .find(|candidate| {
                    candidate.source_record
                        == crate::transactions::data::RecordRef::Entity(feature_id)
                })
                .expect("feature candidate");
            assert_eq!(
                candidate.target_record,
                Some(crate::transactions::data::RecordRef::Entity(main_entity))
            );
            assert_eq!(
                candidate.reason,
                crate::merge::data::IdentityResolutionReason::SchemaDeclaredCorrespondence
            );
        }
    }

    #[test]
    fn inspect_planning_scope_classifies_exact_shared_entity_divergence() {
        let mut runtime = persisted_runtime_with_test_schema();
        let shared = create_entity(&mut runtime, "shared");
        create_branch_from_main(&mut runtime, "feature");
        update_entity(&mut runtime, shared, "shared-main");
        update_entity_on_branch(
            &mut runtime,
            shared,
            "shared-feature",
            BranchId("feature".to_string()),
        );

        let artifact = runtime
            .merge_access()
            .inspect_planning_scope(crate::merge::data::MergePlanningRequest::new(
                BranchId("main".to_string()),
                BranchId("feature".to_string()),
                MergeIntent::ReconcileIntoTarget,
            ))
            .unwrap();

        assert_eq!(artifact.conflict_classification.classified_record_count, 1);
        let classification = artifact
            .conflict_classification
            .classifications
            .iter()
            .find(|classification| {
                classification.record
                    == crate::transactions::data::RecordRef::Entity(shared)
            })
            .expect("shared classification");
        assert_eq!(
            classification.class,
            crate::merge::data::MergeConflictClass::DivergentVisibleState
        );
        assert!(classification.source_record_visible);
        assert!(classification.target_record_visible);
        let annotation = artifact
            .causal_annotation
            .annotations
            .iter()
            .find(|annotation| {
                annotation.record
                    == crate::transactions::data::RecordRef::Entity(shared)
            })
            .expect("shared causal annotation");
        assert_eq!(
            annotation.disposition,
            crate::merge::data::MergeRecordCausalDisposition::Concurrent
        );
        assert!(annotation.source_latest_touch.is_some());
        assert!(annotation.target_latest_touch.is_some());
        assert_eq!(artifact.policy_resolution.requires_manual_resolution_count, 1);
        assert_eq!(artifact.lowered_plan.blocked_count, 1);
        assert_eq!(
            artifact.lowered_plan.records[0].blocked_reason,
            Some(crate::merge::data::LoweredMergeBlockedReason::ManualConflictResolutionRequired)
        );
        assert_eq!(
            artifact.lowered_plan.records[0].record_decision,
            crate::merge::data::LoweredRecordDecision::Block(crate::merge::data::LoweredRecordDenialBundle {
                kind: crate::merge::data::LoweredRecordDenialKind::BlockedManualResolution,
                aspects: std::sync::Arc::from(
                    artifact.lowered_plan.records[0]
                        .aspect_outcomes
                        .iter()
                        .filter_map(|outcome| {
                            Some(crate::merge::data::LoweredRecordDenialAspectIntent {
                                aspect_key: outcome.aspect_key.clone(),
                                intent: outcome.denial_intent?,
                            })
                        })
                        .collect::<Vec<_>>()
                ),
            })
        );
        assert!(artifact.lowered_plan.records[0]
            .aspect_outcomes
            .iter()
            .all(|outcome| outcome.blocked_reason
                == Some(crate::merge::data::LoweredMergeBlockedReason::ManualConflictResolutionRequired)));
    }

    #[test]
    fn inspect_planning_scope_rejects_fail_on_conflict_policy_for_divergent_state() {
        let registry = RelationalSchemaRegistry::new()
            .register_entity_kind(EntityKindRegistration {
                kind_id: crate::facade::identity::KindId(1),
                kind_name: "test.entity".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(1),
                aspect_declarations: KindAspectDeclarations::new(vec![entity_payload_aspect("name", "name")])
                    .with_merge_policy_declarations(vec![AspectMergePolicyDeclaration {
                        aspect_key: crate::publication::patch::data::AspectKey(
                            InternedString::Raw("name".to_string())
                        ),
                        policy: AspectMergePolicyKind::FailOnConflict,
                    }]),
            })
            .and_then(|registry| {
                registry.register_relation_kind(RelationKindRegistration {
                    kind_id: crate::facade::identity::KindId(2),
                    kind_name: "test.relation".to_string(),
                    schema_id: SchemaId("test".to_string()),
                    schema_version_id: SchemaVersionId(1),
                    payload_class: RelationPayloadClass::PayloadBearingRelation,
                    cross_context_policy: CrossContextPolicy::AllowExplicit,
                    cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
                    aspect_declarations: KindAspectDeclarations::default(),
                    relation_integrity: crate::schema::data::RelationIntegrityDeclarations::default(),
                })
            })
            .unwrap();
        let mut runtime = RelationalRuntimeApi::builder().schema_registry(registry).build();
        let shared = create_entity(&mut runtime, "shared");
        create_branch_from_main(&mut runtime, "feature");
        update_entity(&mut runtime, shared, "shared-main");
        update_entity_on_branch(
            &mut runtime,
            shared,
            "shared-feature",
            BranchId("feature".to_string()),
        );

        let artifact = runtime
            .merge_access()
            .inspect_planning_scope(crate::merge::data::MergePlanningRequest::new(
                BranchId("main".to_string()),
                BranchId("feature".to_string()),
                MergeIntent::ReconcileIntoTarget,
            ))
            .unwrap();

        assert_eq!(artifact.policy_resolution.reject_count, 1);
        assert_eq!(artifact.policy_resolution.auto_resolved_count, 0);
        assert_eq!(artifact.lowered_plan.rejected_count, 1);
        assert!(!artifact.lowered_plan.fully_execution_ready);
        assert_eq!(
            artifact.decision_log.decisions[0].decision,
            crate::merge::data::MergePlanningDecisionKind::Rejected
        );
        assert_eq!(
            artifact.lowered_plan.records[0].rejected_reason,
            Some(crate::merge::data::LoweredMergeRejectedReason::FailOnConflictPolicy)
        );
        assert_eq!(
            artifact.lowered_plan.records[0].record_decision,
            crate::merge::data::LoweredRecordDecision::Reject(crate::merge::data::LoweredRecordDenialBundle {
                kind: crate::merge::data::LoweredRecordDenialKind::RejectedPolicy,
                aspects: std::sync::Arc::from(
                    artifact.lowered_plan.records[0]
                        .aspect_outcomes
                        .iter()
                        .filter_map(|outcome| {
                            Some(crate::merge::data::LoweredRecordDenialAspectIntent {
                                aspect_key: outcome.aspect_key.clone(),
                                intent: outcome.denial_intent?,
                            })
                        })
                        .collect::<Vec<_>>()
                ),
            })
        );
        assert!(artifact.lowered_plan.records[0]
            .aspect_outcomes
            .iter()
            .all(|outcome| outcome.rejected_reason
                == Some(crate::merge::data::LoweredMergeRejectedReason::FailOnConflictPolicy)));
    }
}
