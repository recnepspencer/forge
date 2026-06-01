use forge_proof::TransitionOutcome;

use crate::aspects::patches::{
    AuthoritativePatchConstructionDenial, AuthoritativeRecordAspectPatch,
};

impl AuthoritativeRecordAspectPatch {
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
