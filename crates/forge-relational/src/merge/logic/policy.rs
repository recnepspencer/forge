use std::sync::Arc;

use crate::merge::data::{
    AspectComparisonState, AspectMergePolicyKind, AspectPolicyResolutionRecord,
    CausallyAnnotatedMergePlan, DeletionMergeClass, MergeConflictClass,
    MergeManualResolutionClass, MergePlanningError, MergePlanningRequest,
    MergePolicyDecisionBoundary, MergePolicyOwnershipClass, MergePolicyOwnershipSurface,
    MergePolicyProofBoundary, MergePolicyRejectClass, MergePolicyResolution,
    MergePolicyResolutionRecord, MergePolicyResolutionSummary, TopologyRewireAdmissionPolicy,
    PolicyResolvedMergePlan, ResolvedAspectMergePolicy, VisibleMergeRecordKind,
};
use crate::merge::logic::aspect_plan_lookup::lowered_plan_for_record;
use crate::merge::logic::MergeAccess;

impl<'runtime> MergeAccess<'runtime> {
    pub(crate) fn plan_policy_scope(
        &self,
        request: MergePlanningRequest,
    ) -> Result<PolicyResolvedMergePlan, MergePlanningError> {
        let causal_plan = self.plan_causal_scope(request)?;
        self.resolve_policy_scope(causal_plan)
    }

    fn resolve_policy_scope(
        &self,
        causal_plan: CausallyAnnotatedMergePlan,
    ) -> Result<PolicyResolvedMergePlan, MergePlanningError> {
        let source_records_by_ref = causal_plan
            .source_records
            .iter()
            .map(|record| (record.record_ref.clone(), record))
            .collect::<std::collections::BTreeMap<_, _>>();
        let policy_records = causal_plan
            .classifications
            .iter()
            .map(|classification| {
                let record = source_records_by_ref
                    .get(&classification.record)
                    .ok_or_else(|| MergePlanningError::MissingPolicySourceRecord {
                        record: classification.record.clone(),
                    })?;
                let applied_policies = effective_merge_policies_for_record(self.runtime, record);
                let aspect_resolutions = resolve_aspects_for_record(
                    self.runtime,
                    record,
                    classification,
                    applied_policies.as_slice(),
                )?;
                let ownership_surface =
                    ownership_surface_for_policies(applied_policies.as_slice());
                let decision_boundary = aggregate_record_resolution(
                    classification.class,
                    aspect_resolutions.as_slice(),
                );
                Ok(MergePolicyResolutionRecord {
                    record: classification.record.clone(),
                    target_record: classification.target_record.clone(),
                    classification: classification.class,
                    aspect_resolutions: Arc::from(aspect_resolutions),
                    applied_policies: Arc::from(applied_policies),
                    proof_boundary: MergePolicyProofBoundary {
                        ownership_surface,
                        decision_boundary,
                    },
                })
            })
            .collect::<Result<Vec<_>, MergePlanningError>>()?;
        let policy_records: Arc<[MergePolicyResolutionRecord]> = Arc::from(policy_records);
        let policy_summary = summarize_policy_records(policy_records.clone());

        Ok(PolicyResolvedMergePlan {
            request: causal_plan.request,
            target_head: causal_plan.target_head,
            source_head: causal_plan.source_head,
            merge_base: causal_plan.merge_base,
            ancestry: causal_plan.ancestry,
            target_delta: causal_plan.target_delta,
            source_delta: causal_plan.source_delta,
            effective_identity_declarations: causal_plan.effective_identity_declarations,
            source_records: causal_plan.source_records,
            candidates: causal_plan.candidates,
            validated_schema_correspondences: causal_plan.validated_schema_correspondences,
            identity_summary: causal_plan.identity_summary,
            classifications: causal_plan.classifications,
            conflict_summary: causal_plan.conflict_summary,
            causal_annotations: causal_plan.causal_annotations,
            causal_summary: causal_plan.causal_summary,
            policy_records,
            policy_summary,
        })
    }
}

pub(crate) const fn current_topology_rewire_admission_policy(
) -> TopologyRewireAdmissionPolicy {
    TopologyRewireAdmissionPolicy::AlwaysEscalateToTopologyRegion
}

fn effective_merge_policies_for_record(
    runtime: &crate::logic::runtime::RelationalRuntime,
    record: &crate::merge::data::VisibleMergeRecord,
) -> Vec<ResolvedAspectMergePolicy> {
    let Some(kind_id) = record.source_kind_id.or(record.kind_id) else {
        return Vec::new();
    };
    let registry = &runtime.config().schema.registry;
    let declarations = match record.record_kind {
        VisibleMergeRecordKind::Entity => registry.entity_merge_policy_declarations(kind_id).ok(),
        VisibleMergeRecordKind::Relation => registry.relation_merge_policy_declarations(kind_id).ok(),
    }
    .unwrap_or(&[]);

    declarations
        .iter()
        .map(|declaration| ResolvedAspectMergePolicy {
            aspect_key: declaration.aspect_key.clone(),
            policy: declaration.policy.clone(),
        })
        .collect()
}

fn ownership_surface_for_policies(
    applied_policies: &[ResolvedAspectMergePolicy],
) -> MergePolicyOwnershipSurface {
    if applied_policies
        .iter()
        .any(|policy| policy.policy.ownership_class() == MergePolicyOwnershipClass::CustomPolicy)
    {
        MergePolicyOwnershipSurface::ContainsCustomPolicy
    } else {
        MergePolicyOwnershipSurface::RuntimeOnly
    }
}

fn aggregate_record_resolution(
    classification: MergeConflictClass,
    aspects: &[AspectPolicyResolutionRecord],
) -> MergePolicyDecisionBoundary {
    if aspects.is_empty() {
        return match classification {
            MergeConflictClass::SourceOnlyAddition
            | MergeConflictClass::ExactSharedTruth
            | MergeConflictClass::Deletion(DeletionMergeClass::DeletedOnBothSides) => {
                MergePolicyDecisionBoundary::AutoResolved
            }
            MergeConflictClass::SchemaDeclaredCorrespondence
            | MergeConflictClass::Deletion(DeletionMergeClass::SourceDeletedTargetLive)
            | MergeConflictClass::Deletion(DeletionMergeClass::SourceLiveTargetDeleted)
            | MergeConflictClass::Deletion(DeletionMergeClass::DeletedVsModified)
            | MergeConflictClass::Deletion(DeletionMergeClass::DeletedVsRewired)
            | MergeConflictClass::DivergentVisibleState
            | MergeConflictClass::RelationEndpointDivergence => {
                MergePolicyDecisionBoundary::RequiresManualResolution {
                    class: MergeManualResolutionClass::GenericRuntimeConflict,
                }
            }
        };
    }
    if aspects
        .iter()
        .any(|aspect| matches!(aspect.decision_boundary, MergePolicyDecisionBoundary::Reject { .. }))
    {
        MergePolicyDecisionBoundary::Reject {
            class: MergePolicyRejectClass::BuiltInFailOnConflict,
        }
    } else if aspects
        .iter()
        .any(|aspect| {
            matches!(
                aspect.decision_boundary,
                MergePolicyDecisionBoundary::RequiresManualResolution { .. }
            )
        })
    {
        MergePolicyDecisionBoundary::RequiresManualResolution {
            class: MergeManualResolutionClass::GenericRuntimeConflict,
        }
    } else {
        MergePolicyDecisionBoundary::AutoResolved
    }
}

fn summarize_policy_records(
    records: Arc<[MergePolicyResolutionRecord]>,
) -> MergePolicyResolutionSummary {
    let mut auto_resolved_count = 0;
    let mut requires_manual_resolution_count = 0;
    let mut reject_count = 0;
    let mut runtime_only_record_count = 0;
    let mut custom_policy_record_count = 0;

    for record in records.iter() {
        match record.proof_boundary.decision_boundary.resolution() {
            MergePolicyResolution::AutoResolved => auto_resolved_count += 1,
            MergePolicyResolution::RequiresManualResolution => {
                requires_manual_resolution_count += 1
            }
            MergePolicyResolution::Reject => reject_count += 1,
        }
        match record.proof_boundary.ownership_surface {
            MergePolicyOwnershipSurface::RuntimeOnly => runtime_only_record_count += 1,
            MergePolicyOwnershipSurface::ContainsCustomPolicy => custom_policy_record_count += 1,
        }
    }

    MergePolicyResolutionSummary {
        resolved_record_count: records.len(),
        auto_resolved_count,
        requires_manual_resolution_count,
        reject_count,
        runtime_only_record_count,
        custom_policy_record_count,
        records,
    }
}

fn resolve_aspects_for_record(
    runtime: &crate::logic::runtime::RelationalRuntime,
    record: &crate::merge::data::VisibleMergeRecord,
    classification: &crate::merge::data::MergeConflictClassification,
    applied_policies: &[ResolvedAspectMergePolicy],
) -> Result<Vec<AspectPolicyResolutionRecord>, MergePlanningError> {
    if lowered_plan_for_record(runtime, record).is_none() {
        return Ok(Vec::new());
    }
    Ok(classification
        .aspect_evidence
        .iter()
        .map(|aspect| {
            let applied_policy = applied_policies
                .iter()
                .find(|policy| policy.aspect_key == aspect.aspect_key)
                .map(|policy| policy.policy.clone());
            let resolution =
                decision_boundary_for_aspect(
                    classification,
                    aspect.comparison,
                    applied_policy.as_ref(),
                );
            AspectPolicyResolutionRecord {
                aspect_key: aspect.aspect_key.clone(),
                comparison: aspect.comparison,
                applied_policy,
                decision_boundary: resolution,
            }
        })
        .collect())
}

fn decision_boundary_for_aspect(
    classification: &crate::merge::data::MergeConflictClassification,
    comparison: AspectComparisonState,
    applied_policy: Option<&AspectMergePolicyKind>,
) -> MergePolicyDecisionBoundary {
    if classification.identity_reason == crate::merge::data::IdentityResolutionReason::SchemaDeclaredCorrespondence
        && !classification.validated_schema_correspondence
    {
        return MergePolicyDecisionBoundary::RequiresManualResolution {
            class: MergeManualResolutionClass::UnvalidatedSchemaCorrespondence,
        };
    }
    if matches!(applied_policy, Some(AspectMergePolicyKind::FailOnConflict))
        && matches!(
            comparison,
            AspectComparisonState::Divergent | AspectComparisonState::TargetOnly
        )
    {
        return MergePolicyDecisionBoundary::Reject {
            class: MergePolicyRejectClass::BuiltInFailOnConflict,
        };
    }

    match comparison {
        AspectComparisonState::Equal | AspectComparisonState::SourceOnly => {
            MergePolicyDecisionBoundary::AutoResolved
        }
        AspectComparisonState::Unavailable => MergePolicyDecisionBoundary::RequiresManualResolution {
            class: MergeManualResolutionClass::MissingVisibleState,
        },
        AspectComparisonState::TargetOnly => {
            MergePolicyDecisionBoundary::RequiresManualResolution {
                class: MergeManualResolutionClass::GenericRuntimeConflict,
            }
        }
        AspectComparisonState::Divergent => match (classification.class, applied_policy) {
            (
                MergeConflictClass::SchemaDeclaredCorrespondence,
                Some(AspectMergePolicyKind::PreferRicher),
            ) => MergePolicyDecisionBoundary::AutoResolved,
            _ => MergePolicyDecisionBoundary::RequiresManualResolution {
                class: MergeManualResolutionClass::GenericRuntimeConflict,
            },
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{
        aggregate_record_resolution, current_topology_rewire_admission_policy,
        ownership_surface_for_policies, summarize_policy_records,
    };
    use crate::merge::data::{
        AspectMergePolicyKind, CustomMergePolicyIdentity, DeletionMergeClass,
        MergeManualResolutionClass, MergePolicyDecisionBoundary,
        MergeConflictClass, MergePolicyOwnershipClass, MergePolicyOwnershipSurface,
        MergePolicyProofBoundary, MergePolicyResolutionRecord, ResolvedAspectMergePolicy,
        TopologyRewireAdmissionPolicy,
    };
    use crate::identity::data::{EntityId, PartitionId};
    use crate::publication::patch::data::AspectKey;
    use crate::symbols::data::InternedString;
    use crate::transactions::data::RecordRef;
    use std::sync::Arc;

    #[test]
    fn deleted_on_both_sides_without_aspect_rows_is_auto_resolved() {
        assert_eq!(
            aggregate_record_resolution(
                MergeConflictClass::Deletion(DeletionMergeClass::DeletedOnBothSides),
                &[],
            ),
            MergePolicyDecisionBoundary::AutoResolved
        );
    }

    #[test]
    fn topology_rewire_policy_is_explicitly_fail_closed_in_7d() {
        assert_eq!(
            current_topology_rewire_admission_policy(),
            TopologyRewireAdmissionPolicy::AlwaysEscalateToTopologyRegion
        );
    }

    #[test]
    fn ownership_class_distinguishes_runtime_and_custom_policies() {
        assert_eq!(
            AspectMergePolicyKind::PreferRicher.ownership_class(),
            MergePolicyOwnershipClass::RuntimeBuiltIn
        );
        assert_eq!(
            AspectMergePolicyKind::Custom(CustomMergePolicyIdentity {
                name: Arc::from("domain"),
                semantic_version: 1,
            })
            .ownership_class(),
            MergePolicyOwnershipClass::CustomPolicy
        );
    }

    #[test]
    fn ownership_surface_reports_custom_policy_participation() {
        let runtime_only = [ResolvedAspectMergePolicy {
            aspect_key: AspectKey(InternedString::from("name")),
            policy: AspectMergePolicyKind::PreferRicher,
        }];
        let custom = [ResolvedAspectMergePolicy {
            aspect_key: AspectKey(InternedString::from("name")),
            policy: AspectMergePolicyKind::Custom(CustomMergePolicyIdentity {
                name: Arc::from("domain"),
                semantic_version: 1,
            }),
        }];

        assert_eq!(
            ownership_surface_for_policies(&runtime_only),
            MergePolicyOwnershipSurface::RuntimeOnly
        );
        assert_eq!(
            ownership_surface_for_policies(&custom),
            MergePolicyOwnershipSurface::ContainsCustomPolicy
        );
    }

    #[test]
    fn policy_summary_reports_runtime_only_vs_custom_record_counts() {
        let records = Arc::from(vec![
            MergePolicyResolutionRecord {
                record: RecordRef::Entity(EntityId::new(PartitionId::main(), 1, 1)),
                target_record: None,
                classification: MergeConflictClass::ExactSharedTruth,
                aspect_resolutions: Arc::from(Vec::new()),
                applied_policies: Arc::from(Vec::new()),
                proof_boundary: MergePolicyProofBoundary {
                    ownership_surface: MergePolicyOwnershipSurface::RuntimeOnly,
                    decision_boundary: MergePolicyDecisionBoundary::AutoResolved,
                },
            },
            MergePolicyResolutionRecord {
                record: RecordRef::Entity(EntityId::new(PartitionId::main(), 2, 1)),
                target_record: None,
                classification: MergeConflictClass::SchemaDeclaredCorrespondence,
                aspect_resolutions: Arc::from(Vec::new()),
                applied_policies: Arc::from(Vec::new()),
                proof_boundary: MergePolicyProofBoundary {
                    ownership_surface: MergePolicyOwnershipSurface::ContainsCustomPolicy,
                    decision_boundary: MergePolicyDecisionBoundary::RequiresManualResolution {
                        class: MergeManualResolutionClass::GenericRuntimeConflict,
                    },
                },
            },
        ]);

        let summary = summarize_policy_records(records);
        assert_eq!(summary.runtime_only_record_count, 1);
        assert_eq!(summary.custom_policy_record_count, 1);
    }
}
