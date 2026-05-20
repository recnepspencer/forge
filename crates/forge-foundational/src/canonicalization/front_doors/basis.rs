use forge_proof::TransitionOutcome;

use crate::aspects::{
    AspectContract, AspectKey, AspectMask, AuthoritativeRecordAspectPatch,
    AuthoritativeRecordAspectStateArtifact,
};

use super::super::{
    prepare_aspect_contract_for_canonical_basis, prepare_aspect_mask_for_canonical_basis,
    prepare_aspect_patch_for_canonical_basis, prepare_aspect_state_for_canonical_basis,
    prepare_canonical_basis_bundle, prepare_identity_for_canonical_basis,
    prepare_locator_for_canonical_basis, CanonicalBasisConstructionDenial,
    CanonicalBasisReadyArtifact, CanonicalBundleReadyArtifact, CanonicalIdentityInput,
    CanonicalLocatorInput, CanonicalizationRuleVersion, DigestPreparationMaskMode,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CanonicalBasisFrontDoor;

impl CanonicalBasisFrontDoor {
    pub fn at(self, version: CanonicalizationRuleVersion) -> CanonicalBasisVersionStep {
        CanonicalBasisVersionStep { version }
    }
}

pub struct CanonicalBasisVersionStep {
    version: CanonicalizationRuleVersion,
}

impl CanonicalBasisVersionStep {
    pub fn bundle(
        self,
        sequences: impl IntoIterator<Item = CanonicalBasisReadyArtifact>,
    ) -> TransitionOutcome<CanonicalBundleReadyArtifact, CanonicalBasisConstructionDenial> {
        prepare_canonical_basis_bundle(self.version, sequences)
    }

    pub fn from_contract(
        self,
        contract: AspectContract,
    ) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial> {
        prepare_aspect_contract_for_canonical_basis(self.version, contract)
    }

    pub fn from_mask<Mode>(
        self,
        aspect_key: AspectKey,
        mask: AspectMask<Mode>,
    ) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial>
    where
        Mode: DigestPreparationMaskMode,
    {
        prepare_aspect_mask_for_canonical_basis(self.version, aspect_key, mask)
    }

    pub fn from_patch(
        self,
        patch: &AuthoritativeRecordAspectPatch,
    ) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial> {
        prepare_aspect_patch_for_canonical_basis(self.version, patch)
    }

    pub fn from_state(
        self,
        state: AuthoritativeRecordAspectStateArtifact,
    ) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial> {
        prepare_aspect_state_for_canonical_basis(self.version, state)
    }

    pub fn from_identity(
        self,
        identity: CanonicalIdentityInput,
    ) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial> {
        prepare_identity_for_canonical_basis(self.version, identity)
    }

    pub fn from_locator(
        self,
        locator: CanonicalLocatorInput,
    ) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial> {
        prepare_locator_for_canonical_basis(self.version, locator)
    }
}
