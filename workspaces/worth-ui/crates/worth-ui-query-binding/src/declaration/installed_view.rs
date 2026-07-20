use super::{
    WorthUiQueryViewDefinition, WorthUiQueryViewIdentity, WorthUiQueryViewIdentityError,
    WorthUiQueryViewLifecycle, WorthUiQueryViewShape,
};
use crate::{WorthUiInstalledQueryDomain, WorthUiQueryExt};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiQueryViewDeclarationDenial {
    InvalidIdentity(WorthUiQueryViewIdentityError),
    QueryDeclarationUnavailable,
}

/// Registration-only envelope shared by snapshot and live view declarations.
/// Execution methods live only on the lifecycle-specific public types.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiInstalledQueryView {
    installed_domain: WorthUiInstalledQueryDomain,
    definition: WorthUiQueryViewDefinition,
}

impl WorthUiInstalledQueryDomain {
    pub fn measurement_view(
        &self,
        identity: impl Into<String>,
    ) -> Result<super::WorthUiInstalledSnapshotQueryView, WorthUiQueryViewDeclarationDenial> {
        self.measurement_view_with_lifecycle(identity, WorthUiQueryViewLifecycle::Snapshot)
            .map(super::WorthUiInstalledSnapshotQueryView::from_registration)
    }

    pub fn live_measurement_view(
        &self,
        identity: impl Into<String>,
    ) -> Result<super::WorthUiInstalledLiveQueryView, WorthUiQueryViewDeclarationDenial> {
        self.measurement_view_with_lifecycle(identity, WorthUiQueryViewLifecycle::Live)
            .map(super::WorthUiInstalledLiveQueryView::from_registration)
    }

    fn measurement_view_with_lifecycle(
        &self,
        identity: impl Into<String>,
        lifecycle: WorthUiQueryViewLifecycle,
    ) -> Result<WorthUiInstalledQueryView, WorthUiQueryViewDeclarationDenial> {
        let identity = WorthUiQueryViewIdentity::new(identity)
            .map_err(WorthUiQueryViewDeclarationDenial::InvalidIdentity)?;
        match lifecycle {
            WorthUiQueryViewLifecycle::Snapshot => {
                self.handle()
                    .measurements()
                    .map_err(|_| WorthUiQueryViewDeclarationDenial::QueryDeclarationUnavailable)?;
            }
            WorthUiQueryViewLifecycle::Live => {
                self.handle()
                    .live_measurements(super::installed_live_view::query_live_resource_name(
                        self, &identity,
                    ))
                    .map_err(|_| WorthUiQueryViewDeclarationDenial::QueryDeclarationUnavailable)?;
            }
        }
        Ok(WorthUiInstalledQueryView {
            installed_domain: self.clone(),
            definition: WorthUiQueryViewDefinition::measurement(
                identity,
                lifecycle,
                WorthUiQueryViewShape::Collection,
            ),
        })
    }
}

impl WorthUiInstalledQueryView {
    pub(crate) fn into_parts(self) -> (WorthUiInstalledQueryDomain, WorthUiQueryViewDefinition) {
        (self.installed_domain, self.definition)
    }

    pub fn definition(&self) -> &WorthUiQueryViewDefinition {
        &self.definition
    }

    pub fn installed_domain(&self) -> &WorthUiInstalledQueryDomain {
        &self.installed_domain
    }
}
