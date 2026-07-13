type CatalogReadSuccess = (
    super::BootstrapLayoutCatalog,
    super::BootstrapCatalogReadAdmission,
);

#[derive(Debug, PartialEq, Eq)]
enum BootstrapCatalogReadCase {
    Success(CatalogReadSuccess),
    Denied(super::BootstrapOnlyAccessDenied),
}

#[derive(Debug, PartialEq, Eq)]
pub struct BootstrapCatalogReadOutcome {
    case: BootstrapCatalogReadCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootstrapCatalogReadOutcomeView<'a> {
    Success(&'a CatalogReadSuccess),
    Denied(&'a super::BootstrapOnlyAccessDenied),
}

impl BootstrapCatalogReadOutcome {
    pub(crate) fn root_admitted(value: CatalogReadSuccess) -> Self {
        Self::from_owner_payload(BootstrapCatalogReadCase::Success(value))
    }

    pub(crate) fn denied(value: super::BootstrapOnlyAccessDenied) -> Self {
        Self::from_owner_payload(BootstrapCatalogReadCase::Denied(value))
    }

    fn from_owner_payload(case: BootstrapCatalogReadCase) -> Self {
        Self { case }
    }

    pub fn view(&self) -> BootstrapCatalogReadOutcomeView<'_> {
        match &self.case {
            BootstrapCatalogReadCase::Success(value) => {
                BootstrapCatalogReadOutcomeView::Success(value)
            }
            BootstrapCatalogReadCase::Denied(value) => {
                BootstrapCatalogReadOutcomeView::Denied(value)
            }
        }
    }

    fn into_owner_payload(self) -> BootstrapCatalogReadCase {
        self.case
    }
}

impl BootstrapCatalogReadOutcome {
    pub fn is_err(&self) -> bool {
        matches!(self.view(), BootstrapCatalogReadOutcomeView::Denied(_))
    }
    pub fn into_result(self) -> Result<CatalogReadSuccess, super::BootstrapOnlyAccessDenied> {
        match self.into_owner_payload() {
            BootstrapCatalogReadCase::Success(value) => Ok(value),
            BootstrapCatalogReadCase::Denied(denial) => Err(denial),
        }
    }

    pub fn unwrap(self) -> CatalogReadSuccess {
        self.into_result().unwrap()
    }
    pub fn expect(self, message: &str) -> CatalogReadSuccess {
        self.into_result().expect(message)
    }
    pub fn unwrap_err(self) -> super::BootstrapOnlyAccessDenied {
        self.into_result().unwrap_err()
    }
    pub fn expect_err(self, message: &str) -> super::BootstrapOnlyAccessDenied {
        self.into_result().expect_err(message)
    }
}

pub(crate) fn issue_catalog_read(
    result: Result<CatalogReadSuccess, super::BootstrapOnlyAccessDenied>,
) -> BootstrapCatalogReadOutcome {
    match result {
        Ok(catalog) => BootstrapCatalogReadOutcome::root_admitted(catalog),
        Err(denial) => BootstrapCatalogReadOutcome::denied(denial),
    }
}
