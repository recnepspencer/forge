use super::WorthUiDensityTokenDescriptor;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct WorthUiDensityTokenKey {
    projection_basis: String,
}

impl WorthUiDensityTokenKey {
    pub(crate) fn from_descriptor(descriptor: &WorthUiDensityTokenDescriptor) -> Self {
        Self {
            projection_basis: format!(
                "{}:{}|{}|{}",
                descriptor.id().as_str().len(),
                descriptor.id().as_str(),
                descriptor.family().digest_basis(),
                descriptor.value().digest_basis()
            ),
        }
    }

    pub(crate) fn projection_basis(&self) -> &str {
        &self.projection_basis
    }
}
