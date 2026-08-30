use crate::facade::entry::WorthUiApp;

pub enum WorthUiPresentationAsyncInstallationCertificationDenial {
    AlreadyInstalled(Box<worth_ui_query_binding::WorthUiPresentationAsyncInstallation>),
}

impl std::fmt::Debug for WorthUiPresentationAsyncInstallationCertificationDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyInstalled(_) => formatter.write_str("AlreadyInstalled(..)"),
        }
    }
}

pub trait WorthUiPresentationAsyncInstallationCertificationExt {
    fn install_presentation_async_for_certification(
        &mut self,
        installation: worth_ui_query_binding::WorthUiPresentationAsyncInstallation,
    ) -> Result<(), WorthUiPresentationAsyncInstallationCertificationDenial>;
}

impl WorthUiPresentationAsyncInstallationCertificationExt for WorthUiApp {
    fn install_presentation_async_for_certification(
        &mut self,
        installation: worth_ui_query_binding::WorthUiPresentationAsyncInstallation,
    ) -> Result<(), WorthUiPresentationAsyncInstallationCertificationDenial> {
        self.install_presentation_async(installation)
            .map_err(|denial| {
                WorthUiPresentationAsyncInstallationCertificationDenial::AlreadyInstalled(Box::new(
                    denial.into_installation(),
                ))
            })
    }
}
