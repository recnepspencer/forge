use smallvec::SmallVec;

use crate::publication::patch::data::{
    ordered_aspect_keys, PublishedAuthoritativeFieldSet, PublishedAuthoritativePatchOperation,
    PublishedAuthoritativePatchValue, RecordStructuralChange,
};
use crate::schema::data::{AspectBinding, LoweredAspectPlan};
use crate::transactions::data::RecordRef;
use forge_foundational::facade::{AspectLocator, AspectValueLocator, LocatorAuthority};

use super::data::{
    CanonicalAspectDeltaEvidence, CanonicalRecordAspectDelta, EvaluatedAspectBinding,
};
use super::lifecycle_transition_evidence::lifecycle_transition;

pub(super) fn evaluate_authoritative_patch_delta(
    target: RecordRef,
    kind_id: crate::identity::data::KindId,
    plan: &LoweredAspectPlan,
    structural_change: RecordStructuralChange,
    patch: &forge_foundational::facade::AuthoritativeRecordAspectPatch,
) -> CanonicalRecordAspectDelta {
    let evaluated_bindings = authoritative_patch_evaluated_bindings(plan, structural_change, patch);
    let changed_aspects = ordered_aspect_keys(
        evaluated_bindings
            .iter()
            .filter(|binding| binding.changed)
            .map(|binding| binding.aspect_key.clone()),
    );
    let contains_opaque_aspect = evaluated_bindings.iter().any(|binding| {
        matches!(
            binding.aspect_shape,
            forge_foundational::AspectShape::Opaque(_)
        )
    });

    CanonicalRecordAspectDelta {
        target,
        kind_id,
        plan_revision: plan.plan_revision,
        structural_change,
        changed_aspects,
        evaluated_bindings,
        contains_opaque_aspect,
    }
}

fn authoritative_patch_evaluated_bindings(
    plan: &LoweredAspectPlan,
    structural_change: RecordStructuralChange,
    patch: &forge_foundational::facade::AuthoritativeRecordAspectPatch,
) -> SmallVec<[EvaluatedAspectBinding; 4]> {
    let mut evaluated = SmallVec::new();
    for binding in &plan.executable_bindings {
        if let Some(evidence) =
            authoritative_patch_binding_evidence(binding, structural_change, patch)
        {
            evaluated.push(EvaluatedAspectBinding {
                aspect_key: binding.aspect_key().clone(),
                contract: binding.contract.clone(),
                changed: true,
                aspect_shape: binding.aspect_shape(),
                evidence,
            });
        }
    }
    evaluated
}

pub(super) fn authoritative_patch_binding_evidence(
    binding: &crate::schema::data::LoweredAspectBinding,
    structural_change: RecordStructuralChange,
    patch: &forge_foundational::facade::AuthoritativeRecordAspectPatch,
) -> Option<CanonicalAspectDeltaEvidence> {
    whole_aspect_set_evidence(binding, patch)
        .or_else(|| whole_aspect_clear_evidence(binding, patch))
        .or_else(|| field_level_patch_evidence(binding, patch))
        .or_else(|| lifecycle_structural_evidence(binding, structural_change))
}

fn lifecycle_structural_evidence(
    binding: &crate::schema::data::LoweredAspectBinding,
    structural_change: RecordStructuralChange,
) -> Option<CanonicalAspectDeltaEvidence> {
    if !matches!(&binding.target, AspectBinding::LifecycleTransition) {
        return None;
    }
    match structural_change {
        RecordStructuralChange::Created | RecordStructuralChange::Deleted => {
            Some(CanonicalAspectDeltaEvidence::Lifecycle {
                locator: authoritative_value_locator(binding),
                transition: lifecycle_transition(structural_change),
            })
        }
        RecordStructuralChange::Updated | RecordStructuralChange::RetainedForAudit => None,
    }
}

fn whole_aspect_set_evidence(
    binding: &crate::schema::data::LoweredAspectBinding,
    patch: &forge_foundational::facade::AuthoritativeRecordAspectPatch,
) -> Option<CanonicalAspectDeltaEvidence> {
    let (_, value) = patch
        .whole_aspect_sets()
        .find(|(key, _)| *key == binding.contract.key())?;
    Some(CanonicalAspectDeltaEvidence::AuthoritativePatchOperation {
        locator: authoritative_value_locator(binding),
        operation: PublishedAuthoritativePatchOperation::WholeAspectSet {
            aspect_key: binding.contract.key().clone(),
            value: match value.view() {
                forge_foundational::facade::ContractValidatedAspectValueView::Scalar(value) => {
                    PublishedAuthoritativePatchValue::Scalar(value.clone())
                }
                forge_foundational::facade::ContractValidatedAspectValueView::Struct(value) => {
                    PublishedAuthoritativePatchValue::Struct(value.clone())
                }
            },
        },
    })
}

fn whole_aspect_clear_evidence(
    binding: &crate::schema::data::LoweredAspectBinding,
    patch: &forge_foundational::facade::AuthoritativeRecordAspectPatch,
) -> Option<CanonicalAspectDeltaEvidence> {
    patch
        .whole_aspect_clears()
        .any(|key| key == binding.contract.key())
        .then(
            || CanonicalAspectDeltaEvidence::AuthoritativePatchOperation {
                locator: authoritative_value_locator(binding),
                operation: PublishedAuthoritativePatchOperation::WholeAspectClear {
                    aspect_key: binding.contract.key().clone(),
                },
            },
        )
}

fn field_level_patch_evidence(
    binding: &crate::schema::data::LoweredAspectBinding,
    patch: &forge_foundational::facade::AuthoritativeRecordAspectPatch,
) -> Option<CanonicalAspectDeltaEvidence> {
    let (_, field_patch) = patch
        .field_patches()
        .find(|(key, _)| *key == binding.contract.key())?;
    Some(CanonicalAspectDeltaEvidence::AuthoritativePatchOperation {
        locator: authoritative_value_locator(binding),
        operation: PublishedAuthoritativePatchOperation::FieldLevelPatch {
            aspect_key: binding.contract.key().clone(),
            field_sets: field_patch
                .field_sets()
                .map(|(field, value)| PublishedAuthoritativeFieldSet {
                    field: field.clone(),
                    value: value.clone(),
                })
                .collect(),
            field_clears: field_patch.field_clears().cloned().collect(),
        },
    })
}

fn authoritative_value_locator(
    binding: &crate::schema::data::LoweredAspectBinding,
) -> AspectValueLocator {
    AspectValueLocator::whole_aspect(AspectLocator::new(
        LocatorAuthority::Authoritative,
        binding.contract.key().clone(),
    ))
}
