use crate::declaration::{
    UiDeclarationArtifactDigest, UiDeclarationAspectDigest, UiDeclarationFamilyDigest,
    UiDeclarationIdentityDigest, UiDeclarationPostureDigest, UiDeclarationStructuralDigest,
    UiDeclarationSupportDigest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiDeclarationDigestProjection {
    artifact: UiDeclarationArtifactDigest,
    identity: UiDeclarationIdentityDigest,
    family: UiDeclarationFamilyDigest,
    aspect: UiDeclarationAspectDigest,
    structural: UiDeclarationStructuralDigest,
    posture: UiDeclarationPostureDigest,
    support: UiDeclarationSupportDigest,
}

impl UiDeclarationDigestProjection {
    pub(crate) fn new(
        artifact: UiDeclarationArtifactDigest,
        identity: UiDeclarationIdentityDigest,
        family: UiDeclarationFamilyDigest,
        aspect: UiDeclarationAspectDigest,
        structural: UiDeclarationStructuralDigest,
        posture: UiDeclarationPostureDigest,
        support: UiDeclarationSupportDigest,
    ) -> Self {
        Self {
            artifact,
            identity,
            family,
            aspect,
            structural,
            posture,
            support,
        }
    }

    pub fn artifact(&self) -> UiDeclarationArtifactDigest {
        self.artifact
    }

    pub fn identity(&self) -> UiDeclarationIdentityDigest {
        self.identity
    }

    pub fn family(&self) -> UiDeclarationFamilyDigest {
        self.family
    }

    pub fn aspect(&self) -> UiDeclarationAspectDigest {
        self.aspect
    }

    pub fn structural(&self) -> UiDeclarationStructuralDigest {
        self.structural
    }

    pub fn posture(&self) -> UiDeclarationPostureDigest {
        self.posture
    }

    pub fn support(&self) -> UiDeclarationSupportDigest {
        self.support
    }
}
