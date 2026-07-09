use std::collections::{BTreeMap, BTreeSet};

use worth_proof::TransitionOutcome;

use crate::aspects::contracts::{AspectContract, AspectShape};
use crate::aspects::masks::{AspectMask, MutationMask};
use crate::aspects::patches::{
    AuthoritativePatchConstructionDenial, AuthoritativeRecordAspectPatch, FieldLevelAspectPatch,
};
use crate::aspects::structs::{FieldKey, FieldRequirement, StructAspectShape};
use crate::values::AspectValue;

impl AuthoritativeRecordAspectPatch {
    pub fn field_level(
        contract: &AspectContract,
        mask: &AspectMask<MutationMask>,
        field_sets: impl IntoIterator<Item = (FieldKey, AspectValue)>,
        field_clears: impl IntoIterator<Item = FieldKey>,
    ) -> TransitionOutcome<Self, AuthoritativePatchConstructionDenial> {
        match FieldLevelAspectPatch::new(contract, mask, field_sets, field_clears) {
            Ok(field_patch) => {
                let mut patch = Self::empty();
                patch
                    .field_patches
                    .insert(field_patch.key().clone(), field_patch);
                TransitionOutcome::success(patch)
            }
            Err(denial) => TransitionOutcome::denied(denial),
        }
    }
}

impl FieldLevelAspectPatch {
    fn new(
        contract: &AspectContract,
        mask: &AspectMask<MutationMask>,
        field_sets: impl IntoIterator<Item = (FieldKey, AspectValue)>,
        field_clears: impl IntoIterator<Item = FieldKey>,
    ) -> Result<Self, AuthoritativePatchConstructionDenial> {
        let AspectShape::Struct(shape) = contract.shape() else {
            return Err(AuthoritativePatchConstructionDenial::FieldPatchRequiresStructAspect);
        };

        contract
            .admits_mutation_mask(mask)
            .map_err(AuthoritativePatchConstructionDenial::MaskNotAdmitted)?;

        if mask.is_whole_aspect() {
            return Err(AuthoritativePatchConstructionDenial::FieldPatchRequiresFieldMask);
        }

        let canonical_sets = canonical_field_sets(shape, mask, field_sets)?;
        let canonical_clears = canonical_field_clears(shape, mask, &canonical_sets, field_clears)?;

        if canonical_sets.is_empty() && canonical_clears.is_empty() {
            return Err(AuthoritativePatchConstructionDenial::EmptyFieldPatch);
        }

        Ok(Self {
            contract: contract.clone(),
            field_sets: canonical_sets,
            field_clears: canonical_clears,
        })
    }
}

fn canonical_field_sets(
    shape: &StructAspectShape,
    mask: &AspectMask<MutationMask>,
    field_sets: impl IntoIterator<Item = (FieldKey, AspectValue)>,
) -> Result<BTreeMap<FieldKey, AspectValue>, AuthoritativePatchConstructionDenial> {
    let mask_fields = selected_mask_fields(mask);
    let mut canonical_sets = BTreeMap::new();
    for (field_key, field_value) in field_sets {
        if !mask_fields.contains(&field_key) {
            return Err(
                AuthoritativePatchConstructionDenial::FieldNotSelectedByMutationMask(field_key),
            );
        }
        let Some(field) = shape.field(&field_key) else {
            return Err(AuthoritativePatchConstructionDenial::UnknownField(
                field_key,
            ));
        };
        let found = field_value.value_family();
        if found != field.value_type() {
            return Err(AuthoritativePatchConstructionDenial::FieldTypeMismatch {
                field: field_key,
                expected: field.value_type(),
                found,
            });
        }
        if canonical_sets
            .insert(field_key.clone(), field_value)
            .is_some()
        {
            return Err(AuthoritativePatchConstructionDenial::DuplicateFieldSet(
                field_key,
            ));
        }
    }
    Ok(canonical_sets)
}

fn canonical_field_clears(
    shape: &StructAspectShape,
    mask: &AspectMask<MutationMask>,
    canonical_sets: &BTreeMap<FieldKey, AspectValue>,
    field_clears: impl IntoIterator<Item = FieldKey>,
) -> Result<BTreeSet<FieldKey>, AuthoritativePatchConstructionDenial> {
    let mask_fields = selected_mask_fields(mask);
    let mut canonical_clears = BTreeSet::new();
    for field_key in field_clears {
        if !mask_fields.contains(&field_key) {
            return Err(
                AuthoritativePatchConstructionDenial::FieldNotSelectedByMutationMask(field_key),
            );
        }
        let Some(field) = shape.field(&field_key) else {
            return Err(AuthoritativePatchConstructionDenial::UnknownField(
                field_key,
            ));
        };
        if matches!(field.requirement(), FieldRequirement::Required) {
            return Err(AuthoritativePatchConstructionDenial::RequiredFieldClearDenied(field_key));
        }
        if !canonical_sets.contains_key(&field_key) {
            canonical_clears.insert(field_key);
        }
    }
    Ok(canonical_clears)
}

fn selected_mask_fields(mask: &AspectMask<MutationMask>) -> BTreeSet<FieldKey> {
    mask.paths()
        .iter()
        .map(|path| path.fields()[0].clone())
        .collect()
}
