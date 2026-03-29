use std::borrow::Cow;
use std::collections::BTreeMap;
use std::sync::Arc;

use crate::merge::data::{
    AspectComparisonState, AspectMergePolicyKind, AspectPolicyResolutionRecord,
    CausallyAnnotatedMergePlan, DeletionMergeClass, MergeConflictClass,
    MergeManualResolutionClass, MergePlanningError, MergePlanningRequest,
    MergePolicyDecisionBoundary, MergePolicyOwnershipClass, MergePolicyOwnershipSurface,
    MergePolicyProofBoundary, MergePolicyRejectClass, MergePolicyResolution,
    MergeResolvedAspectValueStrategy,
    MergePolicyResolutionRecord, MergePolicyResolutionSummary, TopologyRewireAdmissionPolicy,
    PolicyResolvedMergePlan, ResolvedAspectMergePolicy, VisibleMergeRecordKind,
};
use crate::merge::logic::aspect_plan_lookup::lowered_plan_for_record;
use crate::merge::logic::MergeAccess;
use crate::payloads::data::RecordPayload;
use crate::schema::data::{LoweredAspectBinding, LoweredExecutableAspectBindingKind};
use crate::storage::data::RelationalReadView;
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BindingSide {
    Source,
    Target,
}

#[derive(Debug, Clone)]
enum RuntimeAspectValueBinding {
    EntityField(crate::symbols::data::InternedString),
    RelationField(crate::symbols::data::InternedString),
}

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
        let history = self.runtime.history_access();
        let base_envelope = history
            .commit_envelope(causal_plan.merge_base.commit_id)
            .ok_or(MergePlanningError::MissingMergeBaseEnvelope {
                commit_id: causal_plan.merge_base.commit_id,
            })?;
        let source_view = self
            .runtime
            .visibility_reads()
            .read_version(causal_plan.source_head.version_id);
        let target_view = self
            .runtime
            .visibility_reads()
            .read_version(causal_plan.target_head.version_id);
        let base_view = self
            .runtime
            .visibility_reads()
            .read_version(base_envelope.commit.version_id);
        let source_records_by_ref = causal_plan
            .source_records
            .iter()
            .map(|record| (record.record_ref.clone(), record))
            .collect::<std::collections::BTreeMap<_, _>>();
        let causal_dispositions_by_record = causal_plan
            .causal_annotations
            .iter()
            .map(|annotation| (annotation.record.clone(), annotation.disposition))
            .collect::<BTreeMap<_, _>>();
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
                    *causal_dispositions_by_record
                        .get(&classification.record)
                        .ok_or_else(|| MergePlanningError::MissingCausalAnnotation {
                            record: classification.record.clone(),
                        })?,
                    &source_view,
                    &target_view,
                    &base_view,
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
    } else if let Some(class) = manual_class {
        MergePolicyDecisionBoundary::RequiresManualResolution { class }
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
    causal_disposition: crate::merge::data::MergeRecordCausalDisposition,
    source_view: &RelationalReadView,
    target_view: &RelationalReadView,
    base_view: &RelationalReadView,
) -> Result<Vec<AspectPolicyResolutionRecord>, MergePlanningError> {
    let Some(lowered_plan) = lowered_plan_for_record(runtime, record) else {
        return Ok(Vec::new());
    };
    Ok(classification
        .aspect_evidence
        .iter()
        .map(|aspect| {
            let applied_policy = applied_policies
                .iter()
                .find(|policy| policy.aspect_key == aspect.aspect_key)
                .map(|policy| policy.policy.clone());
            let binding = lowered_plan
                .executable_bindings
                .iter()
                .find(|binding| {
                    binding_matches_aspect(runtime, binding, &aspect.aspect_key)
                });
            let decision_boundary = decision_boundary_for_aspect(
                classification,
                aspect.comparison,
                applied_policy.as_ref(),
                causal_disposition,
            );
            let resolved_value_strategy = resolve_aspect_value_strategy(
                runtime,
                record,
                classification,
                binding,
                aspect.aspect_key.clone(),
                aspect.comparison,
                applied_policy.as_ref(),
                decision_boundary,
                causal_disposition,
                source_view,
                target_view,
                base_view,
            );
            AspectPolicyResolutionRecord {
                aspect_key: aspect.aspect_key.clone(),
                comparison: aspect.comparison,
                applied_policy,
                decision_boundary,
                resolved_value_strategy,
            }
        })
        .collect())
}

fn binding_matches_aspect(
    runtime: &crate::logic::runtime::RelationalRuntime,
    binding: &LoweredAspectBinding,
    aspect_key: &crate::publication::patch::data::AspectKey,
) -> bool {
    if aspect_key_equivalent(runtime, &binding.aspect_key, aspect_key) {
        return true;
    }
    let Some(aspect_name) = interned_string_value(runtime, &aspect_key.0) else {
        return false;
    };
    match &binding.binding_kind {
        LoweredExecutableAspectBindingKind::EntityJsonScalarField { field }
        | LoweredExecutableAspectBindingKind::RelationJsonScalarField { field } => {
            interned_string_value(runtime, field)
                .map(|field_name| field_name == aspect_name)
                .unwrap_or(false)
        }
        _ => false,
    }
}

fn aspect_key_equivalent(
    runtime: &crate::logic::runtime::RelationalRuntime,
    left: &crate::publication::patch::data::AspectKey,
    right: &crate::publication::patch::data::AspectKey,
) -> bool {
    if left == right {
        return true;
    }
    match (
        interned_string_value(runtime, &left.0),
        interned_string_value(runtime, &right.0),
    ) {
        (Some(left), Some(right)) => left == right,
        _ => false,
    }
}

fn interned_string_value<'a>(
    runtime: &'a crate::logic::runtime::RelationalRuntime,
    value: &'a crate::symbols::data::InternedString,
) -> Option<Cow<'a, str>> {
    match value {
        crate::symbols::data::InternedString::Raw(raw) => Some(Cow::Borrowed(raw.as_str())),
        crate::symbols::data::InternedString::Symbol(symbol) => {
            runtime.resolve_symbol(*symbol).map(Cow::Borrowed)
        }
    }
}

fn decision_boundary_for_aspect(
    classification: &crate::merge::data::MergeConflictClassification,
    comparison: AspectComparisonState,
    applied_policy: Option<&AspectMergePolicyKind>,
    causal_disposition: crate::merge::data::MergeRecordCausalDisposition,
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

fn resolve_aspect_value_strategy(
    runtime: &crate::logic::runtime::RelationalRuntime,
    record: &crate::merge::data::VisibleMergeRecord,
    classification: &crate::merge::data::MergeConflictClassification,
    binding: Option<&LoweredAspectBinding>,
    aspect_key: crate::publication::patch::data::AspectKey,
    comparison: AspectComparisonState,
    applied_policy: Option<&AspectMergePolicyKind>,
    decision_boundary: MergePolicyDecisionBoundary,
    causal_disposition: crate::merge::data::MergeRecordCausalDisposition,
    source_view: &RelationalReadView,
    target_view: &RelationalReadView,
    base_view: &RelationalReadView,
) -> Option<MergeResolvedAspectValueStrategy> {
    if decision_boundary != MergePolicyDecisionBoundary::AutoResolved {
        return None;
    }
    let value_binding = runtime_aspect_value_binding(runtime, record, binding, &aspect_key);
    match applied_policy {
        Some(AspectMergePolicyKind::PreferRicher) => match comparison {
            AspectComparisonState::Equal | AspectComparisonState::SourceOnly => {
                Some(MergeResolvedAspectValueStrategy::SourceVisibleValue)
            }
            AspectComparisonState::TargetOnly => {
                Some(MergeResolvedAspectValueStrategy::TargetVisibleValue)
            }
            AspectComparisonState::Divergent => {
                Some(MergeResolvedAspectValueStrategy::SourceVisibleValue)
            }
            AspectComparisonState::Unavailable => None,
        },
        Some(AspectMergePolicyKind::LastWriterWins) => match comparison {
            AspectComparisonState::Equal | AspectComparisonState::SourceOnly => {
                Some(MergeResolvedAspectValueStrategy::SourceVisibleValue)
            }
            AspectComparisonState::TargetOnly => {
                Some(MergeResolvedAspectValueStrategy::TargetVisibleValue)
            }
            AspectComparisonState::Divergent => match causal_disposition {
                crate::merge::data::MergeRecordCausalDisposition::SourceAfterTarget
                | crate::merge::data::MergeRecordCausalDisposition::SourceOnly => {
                    Some(MergeResolvedAspectValueStrategy::SourceVisibleValue)
                }
                crate::merge::data::MergeRecordCausalDisposition::SourceBeforeTarget
                | crate::merge::data::MergeRecordCausalDisposition::TargetOnly => {
                    Some(MergeResolvedAspectValueStrategy::TargetVisibleValue)
                }
                crate::merge::data::MergeRecordCausalDisposition::Concurrent
                | crate::merge::data::MergeRecordCausalDisposition::Equal => None,
            },
            AspectComparisonState::Unavailable => None,
        },
        Some(AspectMergePolicyKind::MonotonicCounter) => value_binding.as_ref().and_then(|binding| {
            monotonic_counter_strategy(
                runtime,
                record,
                classification,
                binding,
                &aspect_key,
                comparison,
                source_view,
                target_view,
                base_view,
            )
        }),
        Some(AspectMergePolicyKind::AdditiveSet) => value_binding.as_ref().and_then(|binding| {
            additive_set_strategy(
                runtime,
                record,
                classification,
                binding,
                &aspect_key,
                comparison,
                source_view,
                target_view,
                base_view,
            )
        }),
        _ => match comparison {
            AspectComparisonState::Equal | AspectComparisonState::SourceOnly => {
                Some(MergeResolvedAspectValueStrategy::SourceVisibleValue)
            }
            _ => None,
        },
    }
}

fn runtime_aspect_value_binding(
    runtime: &crate::logic::runtime::RelationalRuntime,
    record: &crate::merge::data::VisibleMergeRecord,
    binding: Option<&LoweredAspectBinding>,
    aspect_key: &crate::publication::patch::data::AspectKey,
) -> Option<RuntimeAspectValueBinding> {
    if let Some(binding) = binding {
        match &binding.binding_kind {
            LoweredExecutableAspectBindingKind::EntityJsonScalarField { field } => {
                return Some(RuntimeAspectValueBinding::EntityField(field.clone()));
            }
            LoweredExecutableAspectBindingKind::RelationJsonScalarField { field } => {
                return Some(RuntimeAspectValueBinding::RelationField(field.clone()));
            }
            _ => {}
        }
    }

    if let Some(kind_id) = record.source_kind_id.or(record.kind_id) {
        let declarations = match record.record_kind {
            VisibleMergeRecordKind::Entity => &runtime
                .config()
                .schema
                .registry
                .entity_registration(kind_id)
                .ok()?
                .aspect_declarations
                .aspects,
            VisibleMergeRecordKind::Relation => &runtime
                .config()
                .schema
                .registry
                .relation_registration(kind_id)
                .ok()?
                .aspect_declarations
                .aspects,
        };

        if let Some(binding) = declarations.iter().find_map(|declared| {
            if !aspect_key_equivalent(runtime, &declared.key, aspect_key) {
                return None;
            }
            match &declared.binding {
                crate::schema::data::AspectBinding::EntityPayloadField { field } => {
                    Some(RuntimeAspectValueBinding::EntityField(field.clone()))
                }
                crate::schema::data::AspectBinding::RelationPayloadField { field } => {
                    Some(RuntimeAspectValueBinding::RelationField(field.clone()))
                }
                _ => None,
            }
        }) {
            return Some(binding);
        }
    }

    let aspect_name = interned_string_value(runtime, &aspect_key.0)?;
    let field = crate::symbols::data::InternedString::Raw(aspect_name.into_owned());
    match record.record_kind {
        VisibleMergeRecordKind::Entity => Some(RuntimeAspectValueBinding::EntityField(field)),
        VisibleMergeRecordKind::Relation => Some(RuntimeAspectValueBinding::RelationField(field)),
    }
}

fn monotonic_counter_strategy(
    runtime: &crate::logic::runtime::RelationalRuntime,
    record: &crate::merge::data::VisibleMergeRecord,
    classification: &crate::merge::data::MergeConflictClassification,
    binding: &RuntimeAspectValueBinding,
    aspect_key: &crate::publication::patch::data::AspectKey,
    comparison: AspectComparisonState,
    source_view: &RelationalReadView,
    target_view: &RelationalReadView,
    base_view: &RelationalReadView,
) -> Option<MergeResolvedAspectValueStrategy> {
    let base = binding_json_number_from_view(runtime, record, classification, binding, base_view);
    let resolved = match comparison {
        AspectComparisonState::Equal => {
            Value::from(binding_json_number(
                runtime,
                record,
                classification,
                binding,
                BindingSide::Source,
                source_view,
                target_view,
            )?)
        }
        AspectComparisonState::SourceOnly => {
            Value::from(binding_json_number(
                runtime,
                record,
                classification,
                binding,
                BindingSide::Source,
                source_view,
                target_view,
            )?)
        }
        AspectComparisonState::TargetOnly => {
            Value::from(binding_json_number(
                runtime,
                record,
                classification,
                binding,
                BindingSide::Target,
                source_view,
                target_view,
            )?)
        }
        AspectComparisonState::Divergent => {
            let source = binding_json_number(
                runtime,
                record,
                classification,
                binding,
                BindingSide::Source,
                source_view,
                target_view,
            );
            let target = binding_json_number(
                runtime,
                record,
                classification,
                binding,
                BindingSide::Target,
                source_view,
                target_view,
            );
            #[cfg(test)]
            if source.is_none() || target.is_none() || base.is_none() {
                eprintln!(
                    "monotonic-counter-missing-value record={:?} class={:?} source={:?} target={:?} base={:?}",
                    record.record_ref, classification.class, source, target, base
                );
            }
            let source = source?;
            let target = target?;
            let base = base?;
            Value::from(source + target - base)
        }
        AspectComparisonState::Unavailable => return None,
    };
    let _ = aspect_key;
    Some(MergeResolvedAspectValueStrategy::InlineCanonicalJson(resolved))
}

fn additive_set_strategy(
    runtime: &crate::logic::runtime::RelationalRuntime,
    record: &crate::merge::data::VisibleMergeRecord,
    classification: &crate::merge::data::MergeConflictClassification,
    binding: &RuntimeAspectValueBinding,
    aspect_key: &crate::publication::patch::data::AspectKey,
    comparison: AspectComparisonState,
    source_view: &RelationalReadView,
    target_view: &RelationalReadView,
    base_view: &RelationalReadView,
) -> Option<MergeResolvedAspectValueStrategy> {
    let resolved = match comparison {
        AspectComparisonState::Equal => {
            binding_json_set(
                runtime,
                record,
                classification,
                binding,
                BindingSide::Source,
                source_view,
                target_view,
            )?
        }
        AspectComparisonState::SourceOnly => {
            binding_json_set(
                runtime,
                record,
                classification,
                binding,
                BindingSide::Source,
                source_view,
                target_view,
            )?
        }
        AspectComparisonState::TargetOnly => {
            binding_json_set(
                runtime,
                record,
                classification,
                binding,
                BindingSide::Target,
                source_view,
                target_view,
            )?
        }
        AspectComparisonState::Divergent => {
            let source = binding_json_set(
                runtime,
                record,
                classification,
                binding,
                BindingSide::Source,
                source_view,
                target_view,
            );
            let target = binding_json_set(
                runtime,
                record,
                classification,
                binding,
                BindingSide::Target,
                source_view,
                target_view,
            );
            let base =
                binding_json_set_from_view(runtime, record, classification, binding, base_view);
            #[cfg(test)]
            if source.is_none() || target.is_none() || base.is_none() {
                eprintln!(
                    "additive-set-missing-value record={:?} class={:?} source={:?} target={:?} base={:?}",
                    record.record_ref, classification.class, source, target, base
                );
            }
            let source = source?;
            let target = target?;
            let base = base?;
            merge_additive_sets(&base, &source, &target)
        }
        AspectComparisonState::Unavailable => return None,
    };
    let _ = aspect_key;
    Some(MergeResolvedAspectValueStrategy::InlineCanonicalJson(Value::Array(
        resolved,
    )))
}

fn binding_json_number(
    runtime: &crate::logic::runtime::RelationalRuntime,
    record: &crate::merge::data::VisibleMergeRecord,
    classification: &crate::merge::data::MergeConflictClassification,
    binding: &RuntimeAspectValueBinding,
    side: BindingSide,
    source_view: &RelationalReadView,
    target_view: &RelationalReadView,
) -> Option<i64> {
    binding_json_value(
        runtime,
        record,
        classification,
        binding,
        side,
        source_view,
        target_view,
    )?
    .as_i64()
}

fn binding_json_number_from_view(
    runtime: &crate::logic::runtime::RelationalRuntime,
    record: &crate::merge::data::VisibleMergeRecord,
    classification: &crate::merge::data::MergeConflictClassification,
    binding: &RuntimeAspectValueBinding,
    base_view: &RelationalReadView,
) -> Option<i64> {
    binding_json_value_from_view(runtime, record, classification, binding, base_view)?.as_i64()
}

fn binding_json_set(
    runtime: &crate::logic::runtime::RelationalRuntime,
    record: &crate::merge::data::VisibleMergeRecord,
    classification: &crate::merge::data::MergeConflictClassification,
    binding: &RuntimeAspectValueBinding,
    side: BindingSide,
    source_view: &RelationalReadView,
    target_view: &RelationalReadView,
) -> Option<Vec<Value>> {
    json_array_set(binding_json_value(
        runtime,
        record,
        classification,
        binding,
        side,
        source_view,
        target_view,
    )?)
}

fn binding_json_set_from_view(
    runtime: &crate::logic::runtime::RelationalRuntime,
    record: &crate::merge::data::VisibleMergeRecord,
    classification: &crate::merge::data::MergeConflictClassification,
    binding: &RuntimeAspectValueBinding,
    base_view: &RelationalReadView,
) -> Option<Vec<Value>> {
    json_array_set(binding_json_value_from_view(
        runtime,
        record,
        classification,
        binding,
        base_view,
    )?)
}

fn binding_json_value(
    runtime: &crate::logic::runtime::RelationalRuntime,
    record: &crate::merge::data::VisibleMergeRecord,
    classification: &crate::merge::data::MergeConflictClassification,
    binding: &RuntimeAspectValueBinding,
    side: BindingSide,
    source_view: &RelationalReadView,
    target_view: &RelationalReadView,
) -> Option<Value> {
    let source_record_ref = &record.record_ref;
    let target_record_ref = classification.target_record.as_ref().unwrap_or(&record.record_ref);
    match (&record.record_kind, binding, side) {
        (
            VisibleMergeRecordKind::Entity,
            RuntimeAspectValueBinding::EntityField(field),
            BindingSide::Source,
        ) => match source_record_ref {
            crate::transactions::data::RecordRef::Entity(entity_id) => source_view
                .get_entity(*entity_id)
                .and_then(|entity| json_field_value(runtime, &entity.payload, field)),
            _ => None,
        },
        (
            VisibleMergeRecordKind::Entity,
            RuntimeAspectValueBinding::EntityField(field),
            BindingSide::Target,
        ) => match target_record_ref {
            crate::transactions::data::RecordRef::Entity(entity_id) => target_view
                .get_entity(*entity_id)
                .and_then(|entity| json_field_value(runtime, &entity.payload, field)),
            _ => None,
        },
        (
            VisibleMergeRecordKind::Relation,
            RuntimeAspectValueBinding::RelationField(field),
            BindingSide::Source,
        ) => match source_record_ref {
            crate::transactions::data::RecordRef::Relation(relation_id) => source_view
                .get_relation(*relation_id)
                .and_then(|relation| relation.payload.as_ref())
                .and_then(|payload| json_field_value(runtime, payload, field)),
            _ => None,
        },
        (
            VisibleMergeRecordKind::Relation,
            RuntimeAspectValueBinding::RelationField(field),
            BindingSide::Target,
        ) => match target_record_ref {
            crate::transactions::data::RecordRef::Relation(relation_id) => target_view
                .get_relation(*relation_id)
                .and_then(|relation| relation.payload.as_ref())
                .and_then(|payload| json_field_value(runtime, payload, field)),
            _ => None,
        },
        _ => None,
    }
}

fn binding_json_value_from_view(
    runtime: &crate::logic::runtime::RelationalRuntime,
    record: &crate::merge::data::VisibleMergeRecord,
    classification: &crate::merge::data::MergeConflictClassification,
    binding: &RuntimeAspectValueBinding,
    base_view: &RelationalReadView,
) -> Option<Value> {
    let base_record_ref = classification
        .target_record
        .as_ref()
        .filter(|_| classification.base_record_visible)
        .unwrap_or(&record.record_ref);
    match (&record.record_kind, binding) {
        (
            VisibleMergeRecordKind::Entity,
            RuntimeAspectValueBinding::EntityField(field),
        ) => match base_record_ref {
            crate::transactions::data::RecordRef::Entity(entity_id) => {
                base_view
                    .get_entity(*entity_id)
                    .and_then(|entity| json_field_value(runtime, &entity.payload, field))
            }
            _ => None,
        },
        (
            VisibleMergeRecordKind::Relation,
            RuntimeAspectValueBinding::RelationField(field),
        ) => match base_record_ref {
            crate::transactions::data::RecordRef::Relation(relation_id) => base_view
                .get_relation(*relation_id)
                .and_then(|relation| relation.payload.as_ref())
                .and_then(|payload| json_field_value(runtime, payload, field)),
            _ => None,
        },
        _ => None,
    }
}

fn json_field_value(
    runtime: &crate::logic::runtime::RelationalRuntime,
    payload: &RecordPayload,
    field: &crate::symbols::data::InternedString,
) -> Option<Value> {
    let field_name = match field {
        crate::symbols::data::InternedString::Raw(raw) => raw.as_str(),
        crate::symbols::data::InternedString::Symbol(symbol) => runtime.resolve_symbol(*symbol)?,
    };
    payload.as_json()?.get(field_name).cloned()
}

fn json_array_set(value: Value) -> Option<Vec<Value>> {
    let mut values = match value {
        Value::Array(values) => values,
        _ => return None,
    };
    values.sort_by_key(|value| serde_json::to_string(value).unwrap_or_default());
    values.dedup();
    Some(values)
}

fn merge_additive_sets(base: &[Value], source: &[Value], target: &[Value]) -> Vec<Value> {
    let base_keys = base
        .iter()
        .map(value_fingerprint)
        .collect::<BTreeMap<_, _>>();
    let source_keys = source
        .iter()
        .map(value_fingerprint)
        .collect::<BTreeMap<_, _>>();
    let target_keys = target
        .iter()
        .map(value_fingerprint)
        .collect::<BTreeMap<_, _>>();

    let mut merged = BTreeMap::<String, Value>::new();

    for (key, value) in &base_keys {
        let removed_by_source = !source_keys.contains_key(key);
        let removed_by_target = !target_keys.contains_key(key);
        if !(removed_by_source && removed_by_target) {
            merged.insert(key.clone(), value.clone());
        }
    }

    for (key, value) in &source_keys {
        merged.insert(key.clone(), value.clone());
    }
    for (key, value) in &target_keys {
        merged.insert(key.clone(), value.clone());
    }

    merged.into_values().collect()
}

fn value_fingerprint(value: &Value) -> (String, Value) {
    (
        serde_json::to_string(value).unwrap_or_else(|_| "<invalid-json>".to_string()),
        value.clone(),
    )
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
        MergePolicyRejectClass,
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

    #[test]
    fn aggregate_record_resolution_preserves_specific_manual_resolution_class() {
        let aspects = [crate::merge::data::AspectPolicyResolutionRecord {
            aspect_key: AspectKey(InternedString::from("name")),
            comparison: crate::merge::data::AspectComparisonState::Unavailable,
            applied_policy: None,
            decision_boundary: MergePolicyDecisionBoundary::RequiresManualResolution {
                class: MergeManualResolutionClass::MissingVisibleState,
            },
            resolved_value_strategy: None,
        }];

        assert_eq!(
            aggregate_record_resolution(MergeConflictClass::DivergentVisibleState, &aspects),
            MergePolicyDecisionBoundary::RequiresManualResolution {
                class: MergeManualResolutionClass::MissingVisibleState,
            }
        );
    }

    #[test]
    fn aggregate_record_resolution_marks_mixed_manual_resolution_classes_explicitly() {
        let aspects = [
            crate::merge::data::AspectPolicyResolutionRecord {
                aspect_key: AspectKey(InternedString::from("name")),
                comparison: crate::merge::data::AspectComparisonState::Unavailable,
                applied_policy: None,
                decision_boundary: MergePolicyDecisionBoundary::RequiresManualResolution {
                    class: MergeManualResolutionClass::MissingVisibleState,
                },
                resolved_value_strategy: None,
            },
            crate::merge::data::AspectPolicyResolutionRecord {
                aspect_key: AspectKey(InternedString::from("other")),
                comparison: crate::merge::data::AspectComparisonState::TargetOnly,
                applied_policy: None,
                decision_boundary: MergePolicyDecisionBoundary::RequiresManualResolution {
                    class: MergeManualResolutionClass::UnvalidatedSchemaCorrespondence,
                },
                resolved_value_strategy: None,
            },
        ];

        assert_eq!(
            aggregate_record_resolution(MergeConflictClass::SchemaDeclaredCorrespondence, &aspects),
            MergePolicyDecisionBoundary::RequiresManualResolution {
                class: MergeManualResolutionClass::MixedAspectManualResolution,
            }
        );
    }

    #[test]
    fn aggregate_record_resolution_preserves_specific_reject_class() {
        let aspects = [crate::merge::data::AspectPolicyResolutionRecord {
            aspect_key: AspectKey(InternedString::from("name")),
            comparison: crate::merge::data::AspectComparisonState::Divergent,
            applied_policy: Some(AspectMergePolicyKind::FailOnConflict),
            decision_boundary: MergePolicyDecisionBoundary::Reject {
                class: MergePolicyRejectClass::BuiltInFailOnConflict,
            },
            resolved_value_strategy: None,
        }];

        assert_eq!(
            aggregate_record_resolution(
                MergeConflictClass::SchemaDeclaredCorrespondence,
                &aspects
            ),
            MergePolicyDecisionBoundary::Reject {
                class: MergePolicyRejectClass::BuiltInFailOnConflict,
            }
        );
    }
}
