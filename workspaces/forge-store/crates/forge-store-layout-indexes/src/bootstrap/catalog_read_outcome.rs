use crate::production_transition::define_owner_outcome;

type CatalogReadSuccess = (
    super::S8BootstrapLayoutCatalog,
    super::S8BootstrapCatalogReadAdmission,
);

define_owner_outcome!(
    pub S8BootstrapCatalogReadOutcome,
    pub S8BootstrapCatalogReadOutcomeView,
    S8BootstrapCatalogReadCase,
    BootstrapCatalogDiscovery,
    ReadDiscoveredBootstrapCatalog,
    [
        root_admitted => Success(CatalogReadSuccess): CatalogDiscovered => ValidateCurrentRoot => CurrentRootAdmitted,
        denied => Denied(super::S8BootstrapOnlyAccessDenied): CatalogDiscovered => Deny => Denied,
    ]
);

impl S8BootstrapCatalogReadOutcome {
    pub fn is_err(&self) -> bool {
        matches!(self.view(), S8BootstrapCatalogReadOutcomeView::Denied(_))
    }
    pub fn into_result(self) -> Result<CatalogReadSuccess, super::S8BootstrapOnlyAccessDenied> {
        match self.into_owner_payload() {
            S8BootstrapCatalogReadCase::Success(value) => Ok(value),
            S8BootstrapCatalogReadCase::Denied(denial) => Err(denial),
        }
    }

    pub fn unwrap(self) -> CatalogReadSuccess {
        self.into_result().unwrap()
    }
    pub fn expect(self, message: &str) -> CatalogReadSuccess {
        self.into_result().expect(message)
    }
    pub fn unwrap_err(self) -> super::S8BootstrapOnlyAccessDenied {
        self.into_result().unwrap_err()
    }
    pub fn expect_err(self, message: &str) -> super::S8BootstrapOnlyAccessDenied {
        self.into_result().expect_err(message)
    }
}

pub(crate) fn issue_catalog_read(
    result: Result<CatalogReadSuccess, super::S8BootstrapOnlyAccessDenied>,
) -> S8BootstrapCatalogReadOutcome {
    match result {
        Ok(catalog) => S8BootstrapCatalogReadOutcome::root_admitted(catalog),
        Err(denial) => S8BootstrapCatalogReadOutcome::denied(denial),
    }
}
