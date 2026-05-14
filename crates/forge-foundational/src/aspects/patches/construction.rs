use std::collections::{BTreeMap, BTreeSet};

use forge_proof::TransitionOutcome;

use super::{
    AuthoritativePatchConstructionDenial, AuthoritativeRecordAspectPatch, FieldLevelAspectPatch,
};
use crate::aspects::contracts::{AspectContract, AspectShape};
use crate::aspects::keys::AspectKey;
use crate::aspects::masks::{AspectMask, MutationMask};
use crate::aspects::structs::{FieldKey, FieldRequirement};
use crate::aspects::validation::ContractValidatedAspectArtifact;
use crate::values::AspectValue;

impl AuthoritativeRecordAspectPatch {
    pub fn empty() -> Self {
        Self {
            whole_aspect_sets: BTreeMap::new(),
            whole_aspect_clears: BTreeSet::new(),
            field_patches: BTreeMap::new(),
        }
    }

    pub fn whole_aspect(
        sets: impl IntoIterator<Item = ContractValidatedAspectArtifact>,
        clears: impl IntoIterator<Item = AspectKey>,
    ) -> TransitionOutcome<Self, AuthoritativePatchConstructionDenial> {
        let mut patch = Self::empty();
        patch.whole_aspect_clears.extend(clears);

        for artifact in sets {
            let (entry, _proofs, _basis) = artifact.into_parts().into_parts();
            if patch.whole_aspect_sets.contains_key(entry.key()) {
                return TransitionOutcome::denied(
                    AuthoritativePatchConstructionDenial::DuplicateWholeAspectSet(
                        entry.key().clone(),
                    ),
                );
            }
            patch.whole_aspect_clears.remove(entry.key());
            patch.whole_aspect_sets.insert(entry.key().clone(), entry);
        }

        TransitionOutcome::success(patch)
    }

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

    pub fn combine(
        left: Self,
        right: Self,
    ) -> TransitionOutcome<Self, AuthoritativePatchConstructionDenial> {
        match combine_patches(left, right) {
            Ok(patch) => TransitionOutcome::success(patch),
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
                return Err(
                    AuthoritativePatchConstructionDenial::RequiredFieldClearDenied(field_key),
                );
            }
            if !canonical_sets.contains_key(&field_key) {
                canonical_clears.insert(field_key);
            }
        }

        Ok(Self {
            contract: contract.clone(),
            field_sets: canonical_sets,
            field_clears: canonical_clears,
        })
    }
}

fn combine_patches(
    mut left: AuthoritativeRecordAspectPatch,
    right: AuthoritativeRecordAspectPatch,
) -> Result<AuthoritativeRecordAspectPatch, AuthoritativePatchConstructionDenial> {
    for key in right
        .whole_aspect_sets
        .keys()
        .chain(right.whole_aspect_clears.iter())
    {
        if left.field_patches.contains_key(key) {
            return Err(
                AuthoritativePatchConstructionDenial::AmbiguousWholeAndFieldPatch(key.clone()),
            );
        }
    }

    for key in right.field_patches.keys() {
        if left.whole_aspect_sets.contains_key(key) || left.whole_aspect_clears.contains(key) {
            return Err(
                AuthoritativePatchConstructionDenial::AmbiguousWholeAndFieldPatch(key.clone()),
            );
        }
        if left.field_patches.contains_key(key) {
            return Err(AuthoritativePatchConstructionDenial::DuplicateFieldPatch(
                key.clone(),
            ));
        }
    }

    for key in right.whole_aspect_clears {
        if !left.whole_aspect_sets.contains_key(&key) {
            left.whole_aspect_clears.insert(key);
        }
    }

    for (key, value) in right.whole_aspect_sets {
        left.whole_aspect_clears.remove(&key);
        left.whole_aspect_sets.insert(key, value);
    }

    left.field_patches.extend(right.field_patches);
    Ok(left)
}

fn selected_mask_fields(mask: &AspectMask<MutationMask>) -> BTreeSet<FieldKey> {
    mask.paths()
        .iter()
        .map(|path| path.fields()[0].clone())
        .collect()
}
