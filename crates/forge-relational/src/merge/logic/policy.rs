use std::sync::Arc;

use crate::merge::data::{
    AspectComparisonState, AspectMergePolicyKind, AspectPolicyResolutionRecord,
    CausallyAnnotatedMergePlan, MergeConflictClass, MergePlanningError, MergePlanningRequest,
    MergePolicyResolution, MergePolicyResolutionRecord, MergePolicyResolutionSummary,
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
                let resolution =
                    aggregate_record_resolution(classification.class, aspect_resolutions.as_slice());
                Ok(MergePolicyResolutionRecord {
                    record: classification.record.clone(),
                    target_record: classification.target_record.clone(),
                    classification: classification.class,
                    aspect_resolutions: Arc::from(aspect_resolutions),
                    applied_policies: Arc::from(applied_policies),
                    resolution,
                })
            })
            .collect::<Result<Vec<_>, MergePlanningError>>()?;
        let policy_summary = summarize_policy_records(Arc::from(policy_records.clone()));

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
            policy_records: Arc::from(policy_records),
            policy_summary,
        })
    }
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

fn aggregate_record_resolution(
    classification: MergeConflictClass,
    aspects: &[AspectPolicyResolutionRecord],
) -> MergePolicyResolution {
    if aspects.is_empty() {
        return match classification {
            MergeConflictClass::SourceOnlyAddition | MergeConflictClass::ExactSharedTruth => {
                MergePolicyResolution::AutoResolved
            }
            MergeConflictClass::SchemaDeclaredCorrespondence
            | MergeConflictClass::Deletion(_)
            | MergeConflictClass::DivergentVisibleState
            | MergeConflictClass::RelationEndpointDivergence => {
                MergePolicyResolution::RequiresManualResolution
            }
        };
    }
    if aspects
        .iter()
        .any(|aspect| aspect.resolution == MergePolicyResolution::Reject)
    {
        MergePolicyResolution::Reject
    } else if aspects
        .iter()
        .any(|aspect| aspect.resolution == MergePolicyResolution::RequiresManualResolution)
    {
        MergePolicyResolution::RequiresManualResolution
    } else {
        MergePolicyResolution::AutoResolved
    }
}

fn summarize_policy_records(
    records: Arc<[MergePolicyResolutionRecord]>,
) -> MergePolicyResolutionSummary {
    let mut auto_resolved_count = 0;
    let mut requires_manual_resolution_count = 0;
    let mut reject_count = 0;

    for record in records.iter() {
        match record.resolution {
            MergePolicyResolution::AutoResolved => auto_resolved_count += 1,
            MergePolicyResolution::RequiresManualResolution => {
                requires_manual_resolution_count += 1
            }
            MergePolicyResolution::Reject => reject_count += 1,
        }
    }

    MergePolicyResolutionSummary {
        resolved_record_count: records.len(),
        auto_resolved_count,
        requires_manual_resolution_count,
        reject_count,
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
                resolution_for_aspect(classification, aspect.comparison, applied_policy.as_ref());
            AspectPolicyResolutionRecord {
                aspect_key: aspect.aspect_key.clone(),
                comparison: aspect.comparison,
                applied_policy,
                resolution,
            }
        })
        .collect())
}

fn resolution_for_aspect(
    classification: &crate::merge::data::MergeConflictClassification,
    comparison: AspectComparisonState,
    applied_policy: Option<&AspectMergePolicyKind>,
) -> MergePolicyResolution {
    if classification.identity_reason == crate::merge::data::IdentityResolutionReason::SchemaDeclaredCorrespondence
        && !classification.validated_schema_correspondence
    {
        return MergePolicyResolution::RequiresManualResolution;
    }
    if matches!(applied_policy, Some(AspectMergePolicyKind::FailOnConflict))
        && matches!(
            comparison,
            AspectComparisonState::Divergent | AspectComparisonState::TargetOnly
        )
    {
        return MergePolicyResolution::Reject;
    }

    match comparison {
        AspectComparisonState::Equal | AspectComparisonState::SourceOnly => {
            MergePolicyResolution::AutoResolved
        }
        AspectComparisonState::Unavailable | AspectComparisonState::TargetOnly => {
            MergePolicyResolution::RequiresManualResolution
        }
        AspectComparisonState::Divergent => match (classification.class, applied_policy) {
            (
                MergeConflictClass::SchemaDeclaredCorrespondence,
                Some(AspectMergePolicyKind::PreferRicher),
            ) => MergePolicyResolution::AutoResolved,
            _ => MergePolicyResolution::RequiresManualResolution,
        },
    }
}
