use super::{FrozenSettingCapabilities, SettingAcceptedRegistrationProof, SettingDescriptor};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SettingsRegistry {
    descriptors: Vec<SettingDescriptor>,
}

impl SettingsRegistry {
    pub(crate) fn empty() -> Self {
        Self {
            descriptors: Vec::new(),
        }
    }

    pub(crate) fn push(&mut self, descriptor: SettingDescriptor) {
        self.descriptors.push(descriptor);
    }

    pub(crate) fn freeze(
        self,
        accepted_settings: &SettingAcceptedRegistrationProof,
    ) -> FrozenSettingCapabilities {
        FrozenSettingCapabilities::from_accepted_descriptors(self.descriptors, accepted_settings)
    }
}
