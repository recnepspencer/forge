use std::collections::BTreeSet;

use sha2::Digest;

use crate::logic::runtime::RelationalRuntime;
use crate::merge::data::{
    LoweredMergePlan, MergeArtifactDigestBasis, MergeBaseDigestBasis, MergeCausalDigestBasis,
    MergeConflictDigestBasis, MergeExecutionAuthorityContract, MergeExecutionAuthorizationRule,
    MergeExecutionConsumptionRule, MergeExecutionDecisionSurface, MergeIdentityDigestBasis,
    MergeLoweredAspectDigestRow, MergeLoweredPlanDigestBasis, MergePlanningArtifactCore,
    MergePlanningSummary, MergePolicyAspectDigestRow, MergePolicyDigestBasis,
    MergeRequestDigestBasis, MergeSchemaKindClass, MergeSchemaKindSemanticSnapshot,
    MergeSchemaSnapshotDigestBasis, VisibleMergeRecordKind,
};
use crate::schema::data::RelationalSchemaRegistry;
use crate::transactions::data::RecordRef;

pub(super) fn materialize_planning_artifact(
    runtime: &RelationalRuntime,
    plan: LoweredMergePlan,
) -> MergePlanningArtifactCore {
    let target_view = runtime
        .visibility_reads()
        .read_version(plan.target_head.version_id);
    let schema_snapshot = merge_schema_snapshot(
        &runtime.config().schema.registry,
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
    let execution_authority_contract = MergeExecutionAuthorityContract {
        decision_surface: MergeExecutionDecisionSurface::LoweredRecordDecisionOnly,
        identity_authority: MergeExecutionConsumptionRule::ConsumeCanonicalLoweredArtifactOnly,
        conflict_authority: MergeExecutionConsumptionRule::ConsumeCanonicalLoweredArtifactOnly,
        policy_authority: MergeExecutionConsumptionRule::ConsumeCanonicalLoweredArtifactOnly,
        value_authorization:
            MergeExecutionAuthorizationRule::MustNotWidenBeyondAuthorizedAspectValueSurface,
    };
    let digest_basis = MergeArtifactDigestBasis {
        request: MergeRequestDigestBasis {
            target_branch: plan.request.target_branch.clone(),
            source_branch: plan.request.source_branch.clone(),
            merge_intent: plan.request.merge_intent,
        },
        schema: schema_snapshot.clone(),
        execution_contract: execution_authority_contract.clone(),
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
            strategy_conflict_classes: std::sync::Arc::from(
                plan.classifications
                    .iter()
                    .map(|classification| {
                        classification
                            .strategy_evidence
                            .as_ref()
                            .map(|evidence| evidence.class)
                    })
                    .collect::<Vec<_>>(),
            ),
            source_strategy_descriptors: std::sync::Arc::from(
                plan.classifications
                    .iter()
                    .map(|classification| {
                        std::sync::Arc::from(
                            classification
                                .strategy_evidence
                                .as_ref()
                                .map(|evidence| evidence.source_descriptors.to_vec())
                                .unwrap_or_default(),
                        )
                    })
                    .collect::<Vec<_>>(),
            ),
            target_strategy_descriptors: std::sync::Arc::from(
                plan.classifications
                    .iter()
                    .map(|classification| {
                        std::sync::Arc::from(
                            classification
                                .strategy_evidence
                                .as_ref()
                                .map(|evidence| evidence.target_descriptors.to_vec())
                                .unwrap_or_default(),
                        )
                    })
                    .collect::<Vec<_>>(),
            ),
            relation_evidence: std::sync::Arc::from(
                plan.classifications
                    .iter()
                    .map(|classification| classification.relation_evidence.clone())
                    .collect::<Vec<_>>(),
            ),
            source_visibility_evidence: std::sync::Arc::from(
                plan.classifications
                    .iter()
                    .map(|classification| classification.source_visibility_evidence.clone())
                    .collect::<Vec<_>>(),
            ),
            target_visibility_evidence: std::sync::Arc::from(
                plan.classifications
                    .iter()
                    .map(|classification| classification.target_visibility_evidence.clone())
                    .collect::<Vec<_>>(),
            ),
            base_visibility_evidence: std::sync::Arc::from(
                plan.classifications
                    .iter()
                    .map(|classification| classification.base_visibility_evidence.clone())
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
            proof_boundaries: std::sync::Arc::from(
                plan.policy_records
                    .iter()
                    .map(|record| record.proof_boundary)
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
                                    policy_ownership: aspect
                                        .applied_policy
                                        .as_ref()
                                        .map(|policy| policy.ownership_class()),
                                    decision_boundary: aspect.decision_boundary,
                                    resolved_value_strategy: aspect.resolved_value_strategy.clone(),
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
            resolution_classes: std::sync::Arc::from(
                plan.lowered_records
                    .iter()
                    .map(|record| record.resolution_class)
                    .collect::<Vec<_>>(),
            ),
            executable_classes: std::sync::Arc::from(
                plan.lowered_records
                    .iter()
                    .map(|record| record.executable_class)
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
                                    resolved_value_strategy: aspect.resolved_value_strategy.clone(),
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

    runtime.performance_access().count_merge_planning_request(
        schema_snapshot.touched_kinds.len(),
        plan.ancestry.target.unique_commit_count,
        plan.ancestry.source.unique_commit_count,
        plan.ancestry.target.touched_record_count,
        plan.ancestry.source.touched_record_count,
    );
    runtime.performance_access().count_merge_identity_discovery(
        plan.identity_summary.candidate_count,
        plan.identity_summary.effective_declarations.len(),
    );
    runtime
        .performance_access()
        .count_merge_conflict_classification(plan.conflict_summary.classified_record_count);
    runtime
        .performance_access()
        .count_merge_causal_annotation(plan.causal_summary.classified_record_count);
    runtime
        .performance_access()
        .count_merge_policy_resolution(plan.policy_summary.resolved_record_count);
    runtime.performance_access().count_merge_lowering(
        plan.lowered_summary.record_count,
        plan.decision_log.decisions.len(),
    );

    MergePlanningArtifactCore {
        request: plan.request,
        schema_snapshot,
        execution_authority_contract,
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
    }
}

pub(crate) fn merge_schema_snapshot_for_execution_ready(
    runtime: &RelationalRuntime,
    target_version_id: crate::identity::data::VersionId,
    source_records: &[crate::merge::data::VisibleMergeRecord],
    target_touched_records: &[crate::merge::data::BranchTouchedRecordDelta],
) -> MergeSchemaSnapshotDigestBasis {
    let target_view = runtime.visibility_reads().read_version(target_version_id);
    merge_schema_snapshot(
        &runtime.config().schema.registry,
        source_records,
        &target_view,
        target_touched_records,
    )
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
                relation_integrity_plan_revision: Some(
                    registration.relation_integrity.plan_revision,
                ),
            });
        }
    }

    touched_kinds.sort_by(|left, right| {
        left.kind_class
            .cmp(&right.kind_class)
            .then(left.kind_id.cmp(&right.kind_id))
    });

    MergeSchemaSnapshotDigestBasis {
        authoritative_schema_id: touched_kinds.first().map(|kind| kind.schema_id.clone()),
        authoritative_schema_version_id: touched_kinds.first().map(|kind| kind.schema_version_id),
        registry_digest: schema_registry_digest(registry),
        touched_kinds: std::sync::Arc::from(touched_kinds),
    }
}

fn schema_registry_digest(registry: &RelationalSchemaRegistry) -> String {
    let bytes = serde_json::to_vec(registry).expect("schema registry serialization");
    let digest = sha2::Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}
