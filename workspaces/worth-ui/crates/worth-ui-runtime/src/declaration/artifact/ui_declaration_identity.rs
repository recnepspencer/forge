use crate::declaration::{
    UiDeclarationAspectDigest, UiDeclarationFamilyDigest, UiDeclarationIdentityDigest,
    UiDeclarationPostureDigest, UiDeclarationStructuralDigest,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiDeclarationEquivalenceContract {
    AuthoredSemanticMeaning,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiDeclarationIdentity {
    contract: UiDeclarationEquivalenceContract,
    digest: UiDeclarationIdentityDigest,
}

impl UiDeclarationIdentity {
    pub(crate) fn new(
        family_digest: UiDeclarationFamilyDigest,
        aspect_digest: UiDeclarationAspectDigest,
        structural_digest: UiDeclarationStructuralDigest,
        posture_digest: UiDeclarationPostureDigest,
        key_basis: &str,
    ) -> Self {
        let digest = UiDeclarationIdentityDigest::new(
            stable_text_digest(key_basis)
                ^ family_digest.raw().rotate_left(7)
                ^ aspect_digest.raw().rotate_left(19)
                ^ structural_digest.raw().rotate_left(31)
                ^ posture_digest.raw().rotate_left(43),
        );

        Self {
            contract: UiDeclarationEquivalenceContract::AuthoredSemanticMeaning,
            digest,
        }
    }

    pub fn equivalence_contract(&self) -> UiDeclarationEquivalenceContract {
        self.contract
    }

    pub fn digest(&self) -> UiDeclarationIdentityDigest {
        self.digest
    }

    pub fn inspection_identity(&self) -> worth_ui_inspection::UiInspectionDeclarationIdentity {
        worth_ui_inspection::UiInspectionDeclarationIdentity::new(self.digest.raw())
    }
}

pub(crate) fn stable_text_digest(text: &str) -> u64 {
    text.as_bytes()
        .iter()
        .fold(0xCBF2_9CE4_8422_2325, |digest, byte| {
            digest.wrapping_mul(0x0000_0100_0000_01B3) ^ u64::from(*byte)
        })
}
