macro_rules! declaration_digest_type {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct $name(u64);

        impl $name {
            pub(crate) fn new(raw: u64) -> Self {
                Self(raw)
            }

            pub fn raw(self) -> u64 {
                self.0
            }
        }
    };
}

declaration_digest_type!(UiDeclarationArtifactDigest);
declaration_digest_type!(UiDeclarationIdentityDigest);
declaration_digest_type!(UiDeclarationFamilyDigest);
declaration_digest_type!(UiDeclarationAspectDigest);
declaration_digest_type!(UiDeclarationStructuralDigest);
declaration_digest_type!(UiDeclarationPostureDigest);
declaration_digest_type!(UiDeclarationSupportDigest);
