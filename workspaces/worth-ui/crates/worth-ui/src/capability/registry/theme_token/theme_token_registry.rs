use super::{
    FrozenThemeTokenCapabilities, ThemeTokenAcceptedRegistrationProof, ThemeTokenDescriptor,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ThemeTokenRegistry {
    descriptors: Vec<ThemeTokenDescriptor>,
}

impl ThemeTokenRegistry {
    pub(crate) fn empty() -> Self {
        Self {
            descriptors: Vec::new(),
        }
    }

    pub(crate) fn push(&mut self, descriptor: ThemeTokenDescriptor) {
        self.descriptors.push(descriptor);
    }

    pub(crate) fn freeze(
        self,
        accepted_theme_tokens: &ThemeTokenAcceptedRegistrationProof,
    ) -> FrozenThemeTokenCapabilities {
        FrozenThemeTokenCapabilities::from_accepted_descriptors(
            self.descriptors,
            accepted_theme_tokens,
        )
    }
}
