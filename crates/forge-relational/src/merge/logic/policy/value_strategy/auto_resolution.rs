use crate::logic::runtime::RelationalRuntime;
use crate::merge::data::{
    AspectComparisonState, AspectMergePolicyKind, MergeManualResolutionClass,
    MergePolicyDecisionBoundary, MergePolicyRejectClass, MergeResolvedAspectValueStrategy,
    VisibleMergeRecord,
};
use crate::schema::data::LoweredAspectBinding;

use super::binding_values::{binding_aspect_i64, binding_aspect_i64_from_view};
use super::runtime_aspect_value_binding;
use crate::merge::logic::policy::contexts::{
    BindingSide, PolicyReadViewContext, RuntimeAspectValueBinding, ValueLookupFailure,
};

pub(in crate::merge::logic::policy) enum AutoResolutionStrategy {
    NotRequired,
    Resolved(MergeResolvedAspectValueStrategy),
    RequiresManual(MergeManualResolutionClass),
    Reject(MergePolicyRejectClass),
}

pub(in crate::merge::logic::policy) fn resolve_aspect_value_strategy(
    runtime: &RelationalRuntime,
    record: &VisibleMergeRecord,
    classification: &crate::merge::data::MergeConflictClassification,
    binding: Option<&LoweredAspectBinding>,
    comparison: AspectComparisonState,
    applied_policy: Option<&AspectMergePolicyKind>,
    decision_boundary: MergePolicyDecisionBoundary,
    causal_disposition: crate::merge::data::MergeRecordCausalDisposition,
    base_commit_id: crate::history::data::CommitId,
    source_view: &PolicyReadViewContext<'_>,
    target_view: &PolicyReadViewContext<'_>,
    base_view: &PolicyReadViewContext<'_>,
) -> AutoResolutionStrategy {
    if decision_boundary != MergePolicyDecisionBoundary::AutoResolved {
        return AutoResolutionStrategy::NotRequired;
    }
    let value_binding = runtime_aspect_value_binding(binding);
    match applied_policy {
        Some(AspectMergePolicyKind::PreferRicher) => match comparison {
            AspectComparisonState::Equal
            | AspectComparisonState::SourceOnly
            | AspectComparisonState::Divergent => AutoResolutionStrategy::Resolved(
                MergeResolvedAspectValueStrategy::SourceVisibleValue,
            ),
            AspectComparisonState::TargetOnly => AutoResolutionStrategy::Resolved(
                MergeResolvedAspectValueStrategy::TargetVisibleValue,
            ),
            AspectComparisonState::Unavailable => AutoResolutionStrategy::NotRequired,
        },
        Some(AspectMergePolicyKind::LastWriterWins) => match comparison {
            AspectComparisonState::Equal | AspectComparisonState::SourceOnly => {
                AutoResolutionStrategy::Resolved(
                    MergeResolvedAspectValueStrategy::SourceVisibleValue,
                )
            }
            AspectComparisonState::TargetOnly => AutoResolutionStrategy::Resolved(
                MergeResolvedAspectValueStrategy::TargetVisibleValue,
            ),
            AspectComparisonState::Divergent => match causal_disposition {
                crate::merge::data::MergeRecordCausalDisposition::SourceAfterTarget
                | crate::merge::data::MergeRecordCausalDisposition::SourceOnly => {
                    AutoResolutionStrategy::Resolved(
                        MergeResolvedAspectValueStrategy::SourceVisibleValue,
                    )
                }
                crate::merge::data::MergeRecordCausalDisposition::SourceBeforeTarget
                | crate::merge::data::MergeRecordCausalDisposition::TargetOnly => {
                    AutoResolutionStrategy::Resolved(
                        MergeResolvedAspectValueStrategy::TargetVisibleValue,
                    )
                }
                crate::merge::data::MergeRecordCausalDisposition::Concurrent
                | crate::merge::data::MergeRecordCausalDisposition::Equal => {
                    AutoResolutionStrategy::Reject(
                        MergePolicyRejectClass::LastWriterWinsCausalConflict,
                    )
                }
            },
            AspectComparisonState::Unavailable => AutoResolutionStrategy::NotRequired,
        },
        Some(AspectMergePolicyKind::MonotonicCounter) => {
            let Some(binding) = value_binding.as_ref() else {
                return AutoResolutionStrategy::Reject(
                    MergePolicyRejectClass::InvalidBuiltInPolicyValueShape,
                );
            };
            monotonic_counter_strategy(
                runtime,
                record,
                classification,
                binding,
                comparison,
                base_commit_id,
                source_view,
                target_view,
                base_view,
            )
        }
        Some(AspectMergePolicyKind::AdditiveSet) => {
            AutoResolutionStrategy::Reject(MergePolicyRejectClass::InvalidBuiltInPolicyValueShape)
        }
        _ => match comparison {
            AspectComparisonState::Equal | AspectComparisonState::SourceOnly => {
                AutoResolutionStrategy::Resolved(
                    MergeResolvedAspectValueStrategy::SourceVisibleValue,
                )
            }
            AspectComparisonState::TargetOnly | AspectComparisonState::Divergent => {
                AutoResolutionStrategy::Resolved(
                    MergeResolvedAspectValueStrategy::TargetVisibleValue,
                )
            }
            AspectComparisonState::Unavailable => AutoResolutionStrategy::NotRequired,
        },
    }
}

fn monotonic_counter_strategy(
    runtime: &RelationalRuntime,
    record: &VisibleMergeRecord,
    classification: &crate::merge::data::MergeConflictClassification,
    binding: &RuntimeAspectValueBinding,
    comparison: AspectComparisonState,
    base_commit_id: crate::history::data::CommitId,
    source_view: &PolicyReadViewContext<'_>,
    target_view: &PolicyReadViewContext<'_>,
    base_view: &PolicyReadViewContext<'_>,
) -> AutoResolutionStrategy {
    let resolved_value = match comparison {
        AspectComparisonState::Equal | AspectComparisonState::SourceOnly => {
            match binding_aspect_i64(
                runtime,
                record,
                classification,
                binding,
                BindingSide::Source,
                source_view,
                target_view,
            ) {
                Ok(value) => forge_foundational::facade::AspectValue::Int64(value),
                Err(ValueLookupFailure::InvalidValueShape | ValueLookupFailure::MissingField) => {
                    return AutoResolutionStrategy::Reject(
                        MergePolicyRejectClass::InvalidBuiltInPolicyValueShape,
                    );
                }
                Err(ValueLookupFailure::MissingRecordBasis) => {
                    return AutoResolutionStrategy::RequiresManual(
                        MergeManualResolutionClass::MissingVisibleState,
                    );
                }
            }
        }
        AspectComparisonState::TargetOnly => match binding_aspect_i64(
            runtime,
            record,
            classification,
            binding,
            BindingSide::Target,
            source_view,
            target_view,
        ) {
            Ok(value) => forge_foundational::facade::AspectValue::Int64(value),
            Err(ValueLookupFailure::InvalidValueShape | ValueLookupFailure::MissingField) => {
                return AutoResolutionStrategy::Reject(
                    MergePolicyRejectClass::InvalidBuiltInPolicyValueShape,
                );
            }
            Err(ValueLookupFailure::MissingRecordBasis) => {
                return AutoResolutionStrategy::RequiresManual(
                    MergeManualResolutionClass::MissingVisibleState,
                );
            }
        },
        AspectComparisonState::Divergent => {
            let source = match binding_aspect_i64(
                runtime,
                record,
                classification,
                binding,
                BindingSide::Source,
                source_view,
                target_view,
            ) {
                Ok(value) => value,
                Err(ValueLookupFailure::InvalidValueShape | ValueLookupFailure::MissingField) => {
                    return AutoResolutionStrategy::Reject(
                        MergePolicyRejectClass::InvalidBuiltInPolicyValueShape,
                    );
                }
                Err(ValueLookupFailure::MissingRecordBasis) => {
                    return AutoResolutionStrategy::RequiresManual(
                        MergeManualResolutionClass::MissingVisibleState,
                    );
                }
            };
            let target = match binding_aspect_i64(
                runtime,
                record,
                classification,
                binding,
                BindingSide::Target,
                source_view,
                target_view,
            ) {
                Ok(value) => value,
                Err(ValueLookupFailure::InvalidValueShape | ValueLookupFailure::MissingField) => {
                    return AutoResolutionStrategy::Reject(
                        MergePolicyRejectClass::InvalidBuiltInPolicyValueShape,
                    );
                }
                Err(ValueLookupFailure::MissingRecordBasis) => {
                    return AutoResolutionStrategy::RequiresManual(
                        MergeManualResolutionClass::MissingVisibleState,
                    );
                }
            };
            let base = match binding_aspect_i64_from_view(
                runtime,
                record,
                classification,
                binding,
                base_commit_id,
                base_view,
            ) {
                Ok(value) => value,
                Err(ValueLookupFailure::InvalidValueShape | ValueLookupFailure::MissingField) => {
                    return AutoResolutionStrategy::Reject(
                        MergePolicyRejectClass::InvalidBuiltInPolicyValueShape,
                    );
                }
                Err(ValueLookupFailure::MissingRecordBasis) => {
                    return AutoResolutionStrategy::RequiresManual(
                        MergeManualResolutionClass::MissingAncestorValueBasis,
                    );
                }
            };
            forge_foundational::facade::AspectValue::Int64(source + target - base)
        }
        AspectComparisonState::Unavailable => return AutoResolutionStrategy::NotRequired,
    };
    AutoResolutionStrategy::Resolved(MergeResolvedAspectValueStrategy::InlineAspectValue(
        resolved_value,
    ))
}
