use crate::application::WorthQueryDomainEntryMarker;
use crate::domain_installation::{
    admit_domain_package_with_artifact_support, WorthQueryArtifactInstallationSupport,
    WorthQueryDomainPackage, WorthQueryPendingDomainInstallations,
};

use super::error::{WorthQueryTestBackendError, WorthQueryTestBackendErrorKind};

pub(super) type TestDomainInstaller = Box<
    dyn FnOnce(&mut WorthQueryPendingDomainInstallations) -> Result<(), WorthQueryTestBackendError>,
>;

pub(super) fn domain_package_installer<D: WorthQueryDomainEntryMarker + 'static>(
    package: WorthQueryDomainPackage<D>,
    artifact_support: WorthQueryArtifactInstallationSupport,
) -> TestDomainInstaller {
    Box::new(move |installations| {
        let validated = package.validate().map_err(|error| {
            WorthQueryTestBackendError::new(
                WorthQueryTestBackendErrorKind::DomainInstallationFailed,
                format!("failed to validate in-memory test domain: {error}"),
            )
        })?;
        let admitted = admit_domain_package_with_artifact_support(validated, &artifact_support)
            .map_err(|error| {
                WorthQueryTestBackendError::new(
                    WorthQueryTestBackendErrorKind::DomainInstallationFailed,
                    format!("failed to admit in-memory test domain: {error}"),
                )
            })?;
        installations.install(admitted).map_err(|error| {
            WorthQueryTestBackendError::new(
                WorthQueryTestBackendErrorKind::DomainInstallationFailed,
                format!("failed to compile in-memory test domain: {error}"),
            )
        })
    })
}
