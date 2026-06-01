use forge_foundational::facade::{
    AspectMask, AuthoritativeRecordAspectPatch, CanonicalFieldPath,
    ContractValidatedAspectArtifact, MutationMask,
};
use forge_proof::TransitionOutcome;

use super::data::{CanonicalDeltaError, CanonicalRecordAspectDelta, EvaluatedAspectBinding};
use super::patch_fragments::{validate_patch_value, validate_struct_patch_value};
use crate::transactions::data::AspectDeltaPatchConstructionDenial;

pub(super) fn authoritative_patch_filtered_to_changed_bindings(
    delta: &CanonicalRecordAspectDelta,
    authoritative_patch: &AuthoritativeRecordAspectPatch,
) -> Result<AuthoritativeRecordAspectPatch, CanonicalDeltaError> {
    let changed_bindings: Vec<_> = delta
        .evaluated_bindings
        .iter()
        .filter(|binding| binding.changed)
        .collect();
    if changed_bindings.is_empty() {
        return Ok(AuthoritativeRecordAspectPatch::empty());
    }

    let whole_patch = changed_whole_aspect_patch(delta, authoritative_patch, &changed_bindings)?;
    let field_patch = changed_field_level_patch(delta, authoritative_patch, &changed_bindings)?;
    combine_patch_fragments(delta, whole_patch, field_patch)
}

fn changed_whole_aspect_patch(
    delta: &CanonicalRecordAspectDelta,
    authoritative_patch: &AuthoritativeRecordAspectPatch,
    changed_bindings: &[&EvaluatedAspectBinding],
) -> Result<AuthoritativeRecordAspectPatch, CanonicalDeltaError> {
    let mut sets = Vec::new();
    let mut clears = Vec::new();

    for binding in changed_bindings {
        if authoritative_patch
            .whole_aspect_clears()
            .any(|aspect_key| aspect_key == binding.contract.key())
        {
            clears.push(binding.contract.key().clone());
        }

        let Some((_, value)) = authoritative_patch
            .whole_aspect_sets()
            .find(|(aspect_key, _)| *aspect_key == binding.contract.key())
        else {
            continue;
        };
        match value.view() {
            forge_foundational::facade::ContractValidatedAspectValueView::Scalar(value) => {
                sets.push(validate_patch_value(&delta.target, binding, value.clone())?);
            }
            forge_foundational::facade::ContractValidatedAspectValueView::Struct(value) => {
                sets.push(validate_struct_patch_value(
                    &delta.target,
                    binding,
                    value.clone(),
                )?);
            }
        }
    }

    construct_whole_patch(delta, sets, clears)
}

fn changed_field_level_patch(
    delta: &CanonicalRecordAspectDelta,
    authoritative_patch: &AuthoritativeRecordAspectPatch,
    changed_bindings: &[&EvaluatedAspectBinding],
) -> Result<AuthoritativeRecordAspectPatch, CanonicalDeltaError> {
    let mut filtered_patch = AuthoritativeRecordAspectPatch::empty();

    for binding in changed_bindings {
        let Some((_, field_patch)) = authoritative_patch
            .field_patches()
            .find(|(aspect_key, _)| *aspect_key == binding.contract.key())
        else {
            continue;
        };
        let field_sets: Vec<_> = field_patch
            .field_sets()
            .map(|(field, value)| (field.clone(), value.clone()))
            .collect();
        let field_clears: Vec<_> = field_patch.field_clears().cloned().collect();
        let mutation_mask = mutation_mask_for_field_patch(&field_sets, &field_clears);
        let binding_patch =
            construct_field_patch(delta, binding, &mutation_mask, field_sets, field_clears)?;
        filtered_patch = combine_patch_fragments(delta, filtered_patch, binding_patch)?;
    }

    Ok(filtered_patch)
}

fn construct_whole_patch(
    delta: &CanonicalRecordAspectDelta,
    sets: Vec<ContractValidatedAspectArtifact>,
    clears: Vec<forge_foundational::facade::AspectKey>,
) -> Result<AuthoritativeRecordAspectPatch, CanonicalDeltaError> {
    if sets.is_empty() && clears.is_empty() {
        return Ok(AuthoritativeRecordAspectPatch::empty());
    }
    match AuthoritativeRecordAspectPatch::whole_aspect(sets, clears) {
        TransitionOutcome::Success(patch) => Ok(patch),
        TransitionOutcome::Denied(denial) => patch_construction_error(delta, denial),
    }
}

fn mutation_mask_for_field_patch(
    field_sets: &[(
        forge_foundational::facade::FieldKey,
        forge_foundational::facade::AspectValue,
    )],
    field_clears: &[forge_foundational::facade::FieldKey],
) -> AspectMask<MutationMask> {
    AspectMask::<MutationMask>::new(
        field_sets
            .iter()
            .map(|(field, _)| field)
            .chain(field_clears.iter())
            .cloned()
            .map(CanonicalFieldPath::single),
    )
}

fn construct_field_patch(
    delta: &CanonicalRecordAspectDelta,
    binding: &EvaluatedAspectBinding,
    mutation_mask: &AspectMask<MutationMask>,
    field_sets: Vec<(
        forge_foundational::facade::FieldKey,
        forge_foundational::facade::AspectValue,
    )>,
    field_clears: Vec<forge_foundational::facade::FieldKey>,
) -> Result<AuthoritativeRecordAspectPatch, CanonicalDeltaError> {
    match AuthoritativeRecordAspectPatch::field_level(
        &binding.contract,
        mutation_mask,
        field_sets,
        field_clears,
    ) {
        TransitionOutcome::Success(patch) => Ok(patch),
        TransitionOutcome::Denied(denial) => patch_construction_error(delta, denial),
    }
}

fn combine_patch_fragments(
    delta: &CanonicalRecordAspectDelta,
    left: AuthoritativeRecordAspectPatch,
    right: AuthoritativeRecordAspectPatch,
) -> Result<AuthoritativeRecordAspectPatch, CanonicalDeltaError> {
    match AuthoritativeRecordAspectPatch::combine(left, right) {
        TransitionOutcome::Success(patch) => Ok(patch),
        TransitionOutcome::Denied(denial) => patch_construction_error(delta, denial),
    }
}

fn patch_construction_error(
    delta: &CanonicalRecordAspectDelta,
    denial: forge_foundational::facade::AuthoritativePatchConstructionDenial,
) -> Result<AuthoritativeRecordAspectPatch, CanonicalDeltaError> {
    Err(CanonicalDeltaError::FoundationalPatchConstruction {
        target: delta.target.clone(),
        denial: AspectDeltaPatchConstructionDenial::FoundationalPatchConstructionDenied(denial),
    })
}
