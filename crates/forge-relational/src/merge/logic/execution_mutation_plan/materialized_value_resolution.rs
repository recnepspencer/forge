use forge_foundational::facade::{AspectValue, ContractValidatedAspectValueView};

use crate::merge::data::{
    materialized_value_aspect_key, MaterializedAspectValue, MaterializedAspectValueEvidence,
    MergeExecutionMutationPlanError, ReconcileRecordPlan,
};
use crate::schema::data::{LoweredAspectBinding, LoweredExecutableAspectBindingKind};
use crate::storage::data::EntityReadRecord;
use crate::transactions::data::AspectFieldPatchTarget;

pub(super) fn resolved_entity_field_patch_value(
    plan: &ReconcileRecordPlan,
    source_entity: &EntityReadRecord,
    target_entity: &EntityReadRecord,
    binding: &LoweredAspectBinding,
    aspect_key: &crate::publication::patch::data::AspectKey,
    resolved_value: &MaterializedAspectValue,
) -> Result<Option<(AspectFieldPatchTarget, AspectValue)>, MergeExecutionMutationPlanError> {
    match &binding.binding_kind {
        LoweredExecutableAspectBindingKind::EntityFieldScalar { field } => {
            let resolved_value = resolve_materialized_aspect_value(
                plan,
                aspect_key,
                resolved_value,
                source_entity,
                target_entity,
                binding,
            )?;
            if entity_authoritative_binding_aspect_value(target_entity, binding).as_ref()
                == Some(&resolved_value)
            {
                return Ok(None);
            }
            Ok(Some((
                AspectFieldPatchTarget::single(aspect_key.clone(), field.clone()),
                resolved_value,
            )))
        }
        LoweredExecutableAspectBindingKind::EntityFieldStruct { .. } => Err(
            MergeExecutionMutationPlanError::UnsupportedAspectMutationMaterialization {
                record: plan.target_record.clone(),
                aspect_key: aspect_key.clone(),
                detail: "struct entity aspect reconcile is not executable through scalar field update intents",
            },
        ),
        LoweredExecutableAspectBindingKind::LifecycleTransitionEquality => Err(
            MergeExecutionMutationPlanError::UnsupportedAspectMutationMaterialization {
                record: plan.target_record.clone(),
                aspect_key: aspect_key.clone(),
                detail: "lifecycle reconcile is not executable through entity update intents",
            },
        ),
        LoweredExecutableAspectBindingKind::RelationFieldScalar { .. }
        | LoweredExecutableAspectBindingKind::RelationFieldStruct { .. }
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

fn resolve_materialized_aspect_value(
    plan: &ReconcileRecordPlan,
    aspect_key: &crate::publication::patch::data::AspectKey,
    value: &MaterializedAspectValue,
    source_entity: &EntityReadRecord,
    target_entity: &EntityReadRecord,
    binding: &LoweredAspectBinding,
) -> Result<AspectValue, MergeExecutionMutationPlanError> {
    match &value.evidence {
        MaterializedAspectValueEvidence::PinnedVisibleAspect {
            side,
            record,
            locator,
        } => {
            if materialized_value_aspect_key(locator) != aspect_key {
                return Err(
                    MergeExecutionMutationPlanError::InvalidPinnedVisibleAspect {
                        record: plan.target_record.clone(),
                        aspect_key: aspect_key.clone(),
                        detail:
                            "resolved aspect reference key does not match executable aspect key",
                    },
                );
            }
            resolve_pinned_visible_aspect_value(
                plan,
                aspect_key,
                *side,
                record,
                source_entity,
                target_entity,
                binding,
            )
        }
        MaterializedAspectValueEvidence::EqualityWitnessDigest(_) => Err(
            MergeExecutionMutationPlanError::UnsupportedAspectMutationMaterialization {
                record: plan.target_record.clone(),
                aspect_key: aspect_key.clone(),
                detail: "digest-only equality witnesses cannot be lowered into field mutation",
            },
        ),
        MaterializedAspectValueEvidence::InlineAspectValue(value) => Ok(value.clone()),
    }
}

fn resolve_pinned_visible_aspect_value(
    plan: &ReconcileRecordPlan,
    aspect_key: &crate::publication::patch::data::AspectKey,
    side: crate::merge::data::MergeValueSourceSide,
    record: &crate::transactions::data::RecordRef,
    source_entity: &EntityReadRecord,
    target_entity: &EntityReadRecord,
    binding: &LoweredAspectBinding,
) -> Result<AspectValue, MergeExecutionMutationPlanError> {
    match side {
        crate::merge::data::MergeValueSourceSide::Source => {
            ensure_pinned_record_matches(plan, aspect_key, record, &plan.source_record, "source")?;
            entity_authoritative_binding_aspect_value(source_entity, binding).ok_or_else(|| {
                MergeExecutionMutationPlanError::InvalidPinnedVisibleAspect {
                    record: plan.target_record.clone(),
                    aspect_key: aspect_key.clone(),
                    detail: "resolved source aspect reference is missing from source authoritative state",
                }
            })
        }
        crate::merge::data::MergeValueSourceSide::Target => {
            ensure_pinned_record_matches(plan, aspect_key, record, &plan.target_record, "target")?;
            entity_authoritative_binding_aspect_value(target_entity, binding).ok_or_else(|| {
                MergeExecutionMutationPlanError::InvalidPinnedVisibleAspect {
                    record: plan.target_record.clone(),
                    aspect_key: aspect_key.clone(),
                    detail: "resolved target aspect reference is missing from target authoritative state",
                }
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

fn ensure_pinned_record_matches(
    plan: &ReconcileRecordPlan,
    aspect_key: &crate::publication::patch::data::AspectKey,
    actual: &crate::transactions::data::RecordRef,
    expected: &crate::transactions::data::RecordRef,
    side_label: &'static str,
) -> Result<(), MergeExecutionMutationPlanError> {
    if actual == expected {
        return Ok(());
    }
    Err(
        MergeExecutionMutationPlanError::InvalidPinnedVisibleAspect {
            record: plan.target_record.clone(),
            aspect_key: aspect_key.clone(),
            detail: match side_label {
                "source" => "resolved source aspect reference points at a different source record",
                "target" => "resolved target aspect reference points at a different target record",
                _ => "resolved aspect reference points at an unexpected record",
            },
        },
    )
}

fn entity_authoritative_binding_aspect_value(
    entity: &EntityReadRecord,
    binding: &LoweredAspectBinding,
) -> Option<AspectValue> {
    match &binding.binding_kind {
        LoweredExecutableAspectBindingKind::EntityFieldScalar { .. } => {
            let authoritative_state = entity.authoritative_aspect_state.as_ref()?;
            let entry = authoritative_state.get(&binding.aspect_key)?;
            match entry.view() {
                ContractValidatedAspectValueView::Scalar(value) => Some(value.clone()),
                ContractValidatedAspectValueView::Struct(_) => None,
            }
        }
        _ => None,
    }
}
