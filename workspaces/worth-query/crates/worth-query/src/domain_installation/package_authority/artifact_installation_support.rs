use std::sync::Arc;

type WorthQueryArtifactSupportInstaller = Arc<
    dyn Fn(
            worth_query_installation::facade::WorthQueryInstallationAdmissionProfile,
        ) -> worth_query_installation::facade::WorthQueryInstallationAdmissionProfile
        + Send
        + Sync,
>;

#[derive(Clone, Default)]
pub struct WorthQueryArtifactInstallationSupport {
    installers: Vec<WorthQueryArtifactSupportInstaller>,
}

impl WorthQueryArtifactInstallationSupport {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn artifact_version<F>(
        mut self,
        schema_version: worth_query_installation::facade::WorthQueryArtifactSchemaVersion,
        protocol_version: worth_query_installation::facade::WorthQueryArtifactProtocolVersion,
        status: worth_query_installation::facade::WorthQueryArtifactVersionSupport,
    ) -> Self
    where
        F: worth_query_installation::facade::WorthQueryArtifactFamily,
    {
        self.installers.push(Arc::new(move |profile| {
            profile.artifact_version::<F>(schema_version, protocol_version, status.clone())
        }));
        self
    }

    #[must_use]
    pub fn artifact_comparator<F>(
        mut self,
        status: worth_query_installation::facade::WorthQueryInstallationSupportStatus,
    ) -> Self
    where
        F: worth_query_installation::facade::WorthQueryArtifactComparatorFamily,
    {
        self.installers.push(Arc::new(move |profile| {
            profile.artifact_comparator::<F>(status)
        }));
        self
    }

    pub(crate) fn apply(
        &self,
        mut profile: worth_query_installation::facade::WorthQueryInstallationAdmissionProfile,
    ) -> worth_query_installation::facade::WorthQueryInstallationAdmissionProfile {
        for install in &self.installers {
            profile = install(profile);
        }
        profile
    }
}
