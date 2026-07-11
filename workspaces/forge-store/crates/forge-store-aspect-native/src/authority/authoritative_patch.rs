use forge_foundational::{AspectKey, AuthoritativeRecordAspectPatch};

use crate::{StoreAspectIdentity, StoreAspectNativeDenial, StorePhysicalBoundaryWitness};

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
