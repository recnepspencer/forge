use super::WorthUiAppearanceTokenDescriptor;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct WorthUiAppearanceTokenKey {
    projection_basis: String,
}

impl WorthUiAppearanceTokenKey {
    pub(crate) fn from_descriptor(descriptor: &WorthUiAppearanceTokenDescriptor) -> Self {
        Self {
            projection_basis: format!(
                "{}:{}|{}|{}|{}",
                descriptor.id().as_str().len(),
                descriptor.id().as_str(),
                descriptor.family().digest_basis(),
                descriptor.source().digest_basis(),
                descriptor.value().digest_basis()
            ),
        }
    }

    pub(crate) fn projection_basis(&self) -> &str {
        &self.projection_basis
    }
}
