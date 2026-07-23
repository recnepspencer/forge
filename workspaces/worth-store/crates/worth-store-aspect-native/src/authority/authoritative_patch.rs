use worth_foundational::{AspectKey, AspectMask, AuthoritativeRecordAspectPatch, MutationMask};

use crate::{
    StoreAspectContractStamp, StoreAspectIdentity, StoreAspectNativeDenial,
    StorePhysicalBoundaryWitness,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreAspectPatchAuthorityInput {
    patch: AuthoritativeRecordAspectPatch,
    physical_witness: StorePhysicalBoundaryWitness,
}

impl StoreAspectPatchAuthorityInput {
    pub const fn new(
        patch: AuthoritativeRecordAspectPatch,
        physical_witness: StorePhysicalBoundaryWitness,
    ) -> Self {
        Self {
            patch,
            physical_witness,
        }
    }

    pub const fn patch(&self) -> &AuthoritativeRecordAspectPatch {
        &self.patch
    }

    pub const fn physical_witness(&self) -> StorePhysicalBoundaryWitness {
        self.physical_witness
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreAspectPatchBoundaryFact {
    identity: StoreAspectIdentity,
    patch_input: StoreAspectPatchAuthorityInput,
}

impl StoreAspectPatchBoundaryFact {
    pub fn from_authoritative_patch(
        identity: StoreAspectIdentity,
        patch_input: StoreAspectPatchAuthorityInput,
    ) -> Result<Self, StoreAspectNativeDenial> {
        if !patch_targets_only_identity(patch_input.patch(), identity.aspect_key()) {
            return Err(StoreAspectNativeDenial::IdentityMismatch);
        }

        Ok(Self {
            identity,
            patch_input,
        })
    }

    pub const fn identity(&self) -> &StoreAspectIdentity {
        &self.identity
    }

    pub const fn patch_input(&self) -> &StoreAspectPatchAuthorityInput {
        &self.patch_input
    }

    pub fn contract_stamp(&self) -> Option<StoreAspectContractStamp> {
        let patch = self.patch_input.patch();
        let mut stamps =
            patch
                .whole_aspect_sets()
                .map(|(_, value)| StoreAspectContractStamp::from_validated_value(value))
                .chain(
                    patch
                        .whole_aspect_clear_contracts()
                        .map(|(_, contract)| StoreAspectContractStamp::from_contract(contract)),
                )
                .chain(patch.field_patches().map(|(_, field_patch)| {
                    StoreAspectContractStamp::from_field_patch(field_patch)
                }));
        let first = stamps.next()?;
        stamps.all(|stamp| stamp == first).then_some(first)
    }

    pub fn semantic_byte_width(&self) -> usize {
        self.identity
            .aspect_key()
            .as_str()
            .len()
            .saturating_add(self.patch_input.patch().semantic_byte_width())
    }

    pub fn is_within_mutation_mask(&self, admitted: &AspectMask<MutationMask>) -> bool {
        if admitted.is_whole_aspect() {
            return true;
        }
        let patch = self.patch_input.patch();
        if patch.whole_aspect_sets().next().is_some()
            || patch.whole_aspect_clears().next().is_some()
        {
            return false;
        }
        patch.field_patches().all(|(_, field_patch)| {
            field_patch
                .mask()
                .paths()
                .iter()
                .all(|path| admitted.paths().binary_search(path).is_ok())
        })
    }
}

fn patch_targets_only_identity(
    patch: &AuthoritativeRecordAspectPatch,
    identity: &AspectKey,
) -> bool {
    let mut targeted = false;

    for (key, _) in patch.whole_aspect_sets() {
        if key != identity {
            return false;
        }
        targeted = true;
    }

    for key in patch.whole_aspect_clears() {
        if key != identity {
            return false;
        }
        targeted = true;
    }

    for (key, _) in patch.field_patches() {
        if key != identity {
            return false;
        }
        targeted = true;
    }

    targeted
}
