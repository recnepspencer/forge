use std::collections::{BTreeMap, BTreeSet};

use worth_proof::TransitionOutcome;

use crate::aspects::patches::{
    AuthoritativePatchConstructionDenial, AuthoritativeRecordAspectPatch,
};
use crate::aspects::validation::ContractValidatedAspectArtifact;
use crate::AspectKey;

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
}
