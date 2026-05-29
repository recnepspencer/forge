use std::sync::Arc;

use crate::merge::data::{
    AspectComparisonState, AspectMergePolicyKind, AspectPolicyResolutionRecord, DeletionMergeClass,
    MergeConflictClass, MergeConflictClassification, MergeManualResolutionClass,
    MergePolicyDecisionBoundary, MergePolicyOwnershipClass, MergePolicyOwnershipSurface,
    MergePolicyRejectClass, MergePolicyResolution, MergePolicyResolutionRecord,
    MergePolicyResolutionSummary, ResolvedAspectMergePolicy, TopologyRewireAdmissionPolicy,
};

pub(crate) const fn current_topology_rewire_admission_policy() -> TopologyRewireAdmissionPolicy {
    TopologyRewireAdmissionPolicy::AlwaysEscalateToTopologyRegion
}

pub(super) fn effective_merge_policies_for_record(
    runtime: &crate::logic::runtime::RelationalRuntime,
    record: &crate::merge::data::VisibleMergeRecord,
) -> Vec<ResolvedAspectMergePolicy> {
    let Some(kind_id) = record.source_kind_id.or(record.kind_id) else {
        return Vec::new();
    };
    let registry = &runtime.config().schema.registry;
    let declarations = match record.record_kind {
        crate::merge::data::VisibleMergeRecordKind::Entity => {
            registry.entity_merge_policy_declarations(kind_id).ok()
        }
        crate::merge::data::VisibleMergeRecordKind::Relation => {
            registry.relation_merge_policy_declarations(kind_id).ok()
        }
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

pub(super) fn ownership_surface_for_policies(
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

pub(super) fn aggregate_record_resolution(
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
            MergeConflictClass::StrategyIntentConflict => {
                MergePolicyDecisionBoundary::RequiresManualResolution {
                    class: MergeManualResolutionClass::StrategyIntentConflict,
                }
            }
        };
    }
    let mut reject_class: Option<MergePolicyRejectClass> = None;
    let mut manual_class: Option<MergeManualResolutionClass> = None;

    for aspect in aspects {
        match aspect.decision_boundary {
            MergePolicyDecisionBoundary::AutoResolved => {}
            MergePolicyDecisionBoundary::RequiresManualResolution { class } => {
                manual_class = Some(match manual_class {
                    None => class,
                    Some(existing) if existing == class => existing,
                    Some(_) => MergeManualResolutionClass::MixedAspectManualResolution,
                });
            }
            MergePolicyDecisionBoundary::Reject { class } => {
                reject_class = Some(match reject_class {
                    None => class,
                    Some(existing) if existing == class => existing,
                    Some(_) => MergePolicyRejectClass::MixedAspectRejectClasses,
                });
            }
        }
    }

    if let Some(class) = reject_class {
        MergePolicyDecisionBoundary::Reject { class }
    } else if classification == MergeConflictClass::StrategyIntentConflict {
        MergePolicyDecisionBoundary::RequiresManualResolution {
            class: MergeManualResolutionClass::StrategyIntentConflict,
        }
    } else if let Some(class) = manual_class {
        MergePolicyDecisionBoundary::RequiresManualResolution { class }
    } else {
        MergePolicyDecisionBoundary::AutoResolved
    }
}

pub(super) fn summarize_policy_records(
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

pub(super) fn decision_boundary_for_aspect(
    classification: &MergeConflictClassification,
    comparison: AspectComparisonState,
    applied_policy: Option<&AspectMergePolicyKind>,
    causal_disposition: crate::merge::data::MergeRecordCausalDisposition,
) -> MergePolicyDecisionBoundary {
    if classification.identity_reason
        == crate::merge::data::IdentityResolutionReason::SchemaDeclaredCorrespondence
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
        AspectComparisonState::Unavailable => {
            MergePolicyDecisionBoundary::RequiresManualResolution {
                class: MergeManualResolutionClass::MissingVisibleState,
            }
        }
        AspectComparisonState::TargetOnly => match (classification.class, applied_policy) {
            (
                MergeConflictClass::SchemaDeclaredCorrespondence
                | MergeConflictClass::DivergentVisibleState,
                Some(
                    AspectMergePolicyKind::LastWriterWins
                    | AspectMergePolicyKind::MonotonicCounter
                    | AspectMergePolicyKind::AdditiveSet,
                ),
            ) => MergePolicyDecisionBoundary::AutoResolved,
            _ => MergePolicyDecisionBoundary::RequiresManualResolution {
                class: MergeManualResolutionClass::GenericRuntimeConflict,
            },
        },
        AspectComparisonState::Divergent => match (classification.class, applied_policy) {
            (
                MergeConflictClass::SchemaDeclaredCorrespondence
                | MergeConflictClass::DivergentVisibleState,
                Some(AspectMergePolicyKind::PreferRicher),
            )
            | (
                MergeConflictClass::SchemaDeclaredCorrespondence
                | MergeConflictClass::DivergentVisibleState,
                Some(AspectMergePolicyKind::MonotonicCounter | AspectMergePolicyKind::AdditiveSet),
            ) => MergePolicyDecisionBoundary::AutoResolved,
            (
                MergeConflictClass::SchemaDeclaredCorrespondence
                | MergeConflictClass::DivergentVisibleState,
                Some(AspectMergePolicyKind::LastWriterWins),
            ) => match causal_disposition {
                crate::merge::data::MergeRecordCausalDisposition::SourceAfterTarget
                | crate::merge::data::MergeRecordCausalDisposition::SourceOnly => {
                    MergePolicyDecisionBoundary::AutoResolved
                }
                crate::merge::data::MergeRecordCausalDisposition::SourceBeforeTarget
                | crate::merge::data::MergeRecordCausalDisposition::TargetOnly => {
                    MergePolicyDecisionBoundary::AutoResolved
                }
                crate::merge::data::MergeRecordCausalDisposition::Concurrent
                | crate::merge::data::MergeRecordCausalDisposition::Equal => {
                    MergePolicyDecisionBoundary::Reject {
                        class: MergePolicyRejectClass::LastWriterWinsCausalConflict,
                    }
                }
            },
            _ => MergePolicyDecisionBoundary::RequiresManualResolution {
                class: MergeManualResolutionClass::GenericRuntimeConflict,
            },
        },
    }
}
