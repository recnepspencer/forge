type CatalogReadSuccess = (
    super::S8BootstrapLayoutCatalog,
    super::S8BootstrapCatalogReadAdmission,
);

#[derive(Debug, PartialEq, Eq)]
enum S8BootstrapCatalogReadCase {
    Success(CatalogReadSuccess),
    Denied(super::S8BootstrapOnlyAccessDenied),
}

#[derive(Debug, PartialEq, Eq)]
pub struct S8BootstrapCatalogReadOutcome {
    case: S8BootstrapCatalogReadCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8BootstrapCatalogReadOutcomeView<'a> {
    Success(&'a CatalogReadSuccess),
    Denied(&'a super::S8BootstrapOnlyAccessDenied),
}

impl S8BootstrapCatalogReadOutcome {
    pub(crate) fn root_admitted(value: CatalogReadSuccess) -> Self {
        Self::from_owner_payload(S8BootstrapCatalogReadCase::Success(value))
    }

    pub(crate) fn denied(value: super::S8BootstrapOnlyAccessDenied) -> Self {
        Self::from_owner_payload(S8BootstrapCatalogReadCase::Denied(value))
    }

    fn from_owner_payload(case: S8BootstrapCatalogReadCase) -> Self {
        Self { case }
    }

    pub fn view(&self) -> S8BootstrapCatalogReadOutcomeView<'_> {
        match &self.case {
            S8BootstrapCatalogReadCase::Success(value) => {
                S8BootstrapCatalogReadOutcomeView::Success(value)
            }
            S8BootstrapCatalogReadCase::Denied(value) => {
                S8BootstrapCatalogReadOutcomeView::Denied(value)
            }
        }
    }

    fn into_owner_payload(self) -> S8BootstrapCatalogReadCase {
        self.case
    }
}

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
