use worth_query::facade::{domain, read};

use crate::{
    WorthUiDomainEntry, WorthUiInstalledQueryDomain, WorthUiInstalledQueryView, WorthUiQueryExt,
    WorthUiQuerySnapshotProjectionOutcome, WorthUiQueryViewDefinition,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryViewProjectionDenial {
    InstalledAuthorityMismatch,
    ViewDefinitionMismatch,
}

/// Installed snapshot view. Only this lifecycle exposes one-shot `read`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiInstalledSnapshotQueryView {
    registration: WorthUiInstalledQueryView,
}

impl WorthUiInstalledSnapshotQueryView {
    pub(super) fn from_registration(registration: WorthUiInstalledQueryView) -> Self {
        debug_assert_eq!(
            registration.definition().lifecycle(),
            crate::WorthUiQueryViewLifecycle::Snapshot
        );
        Self { registration }
    }

    pub fn definition(&self) -> &WorthUiQueryViewDefinition {
        self.registration.definition()
    }

    pub fn installed_domain(&self) -> &WorthUiInstalledQueryDomain {
        self.registration.installed_domain()
    }

    pub fn read(
        &self,
    ) -> Result<
        domain::WorthQueryInstalledDomainReadDeclaration<WorthUiDomainEntry>,
        Box<read::WorthQueryReadDeclarationStop>,
    > {
        self.installed_domain().handle().measurements()
    }

    pub fn project(
        &self,
        completion: &domain::WorthQueryInstalledDomainReadCompletion<WorthUiDomainEntry>,
        declaration: read::WorthQueryProjectionDeclaration,
    ) -> Result<WorthUiQuerySnapshotProjectionOutcome, WorthUiQueryViewProjectionDenial> {
        if completion.receipt().installed_authority()
            != &self.installed_domain().handle().authority_witness()
        {
            return Err(WorthUiQueryViewProjectionDenial::InstalledAuthorityMismatch);
        }
        Ok(WorthUiQuerySnapshotProjectionOutcome::from_installed(
            self.definition().clone(),
            completion.project(declaration),
        ))
    }
}

impl From<WorthUiInstalledSnapshotQueryView> for WorthUiInstalledQueryView {
    fn from(view: WorthUiInstalledSnapshotQueryView) -> Self {
        view.registration
    }
}
