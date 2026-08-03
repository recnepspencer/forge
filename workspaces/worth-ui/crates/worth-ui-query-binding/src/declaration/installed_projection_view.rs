use crate::{WorthUiInstalledQueryDomain, WorthUiQueryViewIdentity};

/// Audience-safe reference to projection meaning installed in one Query world.
///
/// This reference carries no operating world, result, or native-access
/// authority. Admission must still pair it with Query-issued runtime evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiInstalledProjectionView {
    installed_domain: WorthUiInstalledQueryDomain,
    identity: WorthUiQueryViewIdentity,
}

impl WorthUiInstalledQueryDomain {
    pub fn projection_view(
        &self,
        identity: impl Into<String>,
    ) -> Result<UiInstalledProjectionView, super::WorthUiQueryViewIdentityError> {
        Ok(UiInstalledProjectionView {
            installed_domain: self.clone(),
            identity: WorthUiQueryViewIdentity::new(identity)?,
        })
    }
}

impl UiInstalledProjectionView {
    pub fn identity(&self) -> &WorthUiQueryViewIdentity {
        &self.identity
    }

    pub(crate) fn into_parts(self) -> (WorthUiInstalledQueryDomain, WorthUiQueryViewIdentity) {
        (self.installed_domain, self.identity)
    }
}
