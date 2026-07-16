use worth_query::facade::{domain, read};

use super::{
    WorthUiQueryViewDefinition, WorthUiQueryViewIdentity,
    WorthUiQueryViewIdentityError, WorthUiQueryViewLifecycle, WorthUiQueryViewShape,
};
use crate::{
    WorthUiDomainEntry, WorthUiInstalledQueryDomain, WorthUiQueryExt,
    WorthUiQueryProjectionOutcome,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiQueryViewDeclarationDenial {
    InvalidIdentity(WorthUiQueryViewIdentityError),
    QueryDeclarationUnavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryViewProjectionDenial {
    InstalledAuthorityMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiInstalledQueryView {
    installed_domain: WorthUiInstalledQueryDomain,
    definition: WorthUiQueryViewDefinition,
}

impl WorthUiInstalledQueryDomain {
    pub fn measurement_view(
        &self,
        identity: impl Into<String>,
    ) -> Result<WorthUiInstalledQueryView, WorthUiQueryViewDeclarationDenial> {
        self.measurement_view_with_lifecycle(identity, WorthUiQueryViewLifecycle::Snapshot)
    }

    pub fn live_measurement_view(
        &self,
        identity: impl Into<String>,
    ) -> Result<WorthUiInstalledQueryView, WorthUiQueryViewDeclarationDenial> {
        self.measurement_view_with_lifecycle(identity, WorthUiQueryViewLifecycle::Live)
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
                    .live_measurements()
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
    pub(crate) fn into_parts(
        self,
    ) -> (WorthUiInstalledQueryDomain, WorthUiQueryViewDefinition) {
        (self.installed_domain, self.definition)
    }

    pub fn definition(&self) -> &WorthUiQueryViewDefinition {
        &self.definition
    }

    pub fn installed_domain(&self) -> &WorthUiInstalledQueryDomain {
        &self.installed_domain
    }

    pub fn read(
        &self,
    ) -> Result<
        domain::WorthQueryInstalledDomainReadDeclaration<WorthUiDomainEntry>,
        read::WorthQueryReadDeclarationStop,
    > {
        self.installed_domain.handle().measurements()
    }

    pub fn project(
        &self,
        completion: &domain::WorthQueryInstalledDomainReadCompletion<WorthUiDomainEntry>,
        declaration: read::WorthQueryProjectionDeclaration,
    ) -> Result<WorthUiQueryProjectionOutcome, WorthUiQueryViewProjectionDenial> {
        if completion.receipt().installed_authority()
            != &self.installed_domain.handle().authority_witness()
        {
            return Err(WorthUiQueryViewProjectionDenial::InstalledAuthorityMismatch);
        }
        Ok(WorthUiQueryProjectionOutcome::from_installed(
            self.definition.clone(),
            completion.project(declaration),
        ))
    }
}
