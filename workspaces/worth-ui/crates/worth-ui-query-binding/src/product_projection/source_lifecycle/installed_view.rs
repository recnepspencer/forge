use super::{WorthUiScalarProjectionAdvance, WorthUiScalarProjectionInstallation};
use crate::UiScalarProjectionRegistration;

impl WorthUiScalarProjectionInstallation {
    pub fn into_parts_with_live_measurement_view(
        self,
        identity: impl Into<String>,
    ) -> Result<
        (
            UiScalarProjectionRegistration,
            WorthUiScalarProjectionAdvance,
            crate::WorthUiInstalledLiveQueryView,
        ),
        crate::WorthUiQueryViewDeclarationDenial,
    > {
        let view = self.installed_domain.live_measurement_view(identity)?;
        Ok((self.registration, self.initial, view))
    }
}
