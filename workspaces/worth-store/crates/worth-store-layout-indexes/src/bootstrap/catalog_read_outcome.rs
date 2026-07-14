use worth_store_physical_format::{
    CurrentRootManifestAdmission, PhysicalBootstrapCatalogIdentity,
    PhysicalBootstrapCatalogWitness, PhysicalFormatVersion,
};

use super::{
    BootstrapCatalogAccess, BootstrapLayoutCatalog, BootstrapOnlyAccessDenied,
    BootstrapOnlyAccessPath, MinimalRootDiscoveryLayout,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapCatalogReadAdmission {
    identity: PhysicalBootstrapCatalogIdentity,
}

impl BootstrapCatalogReadAdmission {
    fn new(identity: PhysicalBootstrapCatalogIdentity) -> Self {
        Self { identity }
    }

    pub(crate) fn identity(&self) -> &PhysicalBootstrapCatalogIdentity {
        &self.identity
    }

    pub fn root_owner(&self) -> worth_store_physical_format::PhysicalGenerationOwner {
        self.identity().root_owner()
    }

    pub fn physical_format_version(&self) -> PhysicalFormatVersion {
        self.identity().physical_format_version()
    }
}

type CatalogReadSuccess = (BootstrapLayoutCatalog, BootstrapCatalogReadAdmission);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BootstrapCatalogReadCaseId {
    Admitted,
    CurrentRootReadmissionRequired,
}

impl BootstrapCatalogReadCaseId {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Admitted => "layout.bootstrap.catalog_read.admitted",
            Self::CurrentRootReadmissionRequired => {
                "layout.bootstrap.catalog_read.denied.current_root_readmission"
            }
        }
    }
}

pub fn bootstrap_catalog_read_cases() -> impl Iterator<Item = BootstrapCatalogReadCaseId> {
    [
        BootstrapCatalogReadCaseId::Admitted,
        BootstrapCatalogReadCaseId::CurrentRootReadmissionRequired,
    ]
    .into_iter()
}

#[derive(Debug, PartialEq, Eq)]
enum BootstrapCatalogReadCase {
    Success(CatalogReadSuccess),
    CurrentRootReadmissionRequired(super::BootstrapOnlyAccessDenied),
}

#[derive(Debug, PartialEq, Eq)]
pub struct BootstrapCatalogReadOutcome {
    case: BootstrapCatalogReadCase,
    counters: super::BootstrapCatalogReadCounterSnapshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootstrapCatalogReadOutcomeView<'a> {
    Success(&'a CatalogReadSuccess),
    Denied(&'a super::BootstrapOnlyAccessDenied),
}

impl BootstrapCatalogReadOutcome {
    fn root_admitted(
        value: CatalogReadSuccess,
        counters: super::BootstrapCatalogReadCounterSnapshot,
    ) -> Self {
        Self::from_owner_payload(
            BootstrapCatalogReadCase::Success(value),
            counters.admitted(),
        )
    }

    fn current_root_readmission_required(
        value: super::BootstrapOnlyAccessDenied,
        counters: super::BootstrapCatalogReadCounterSnapshot,
    ) -> Self {
        Self::from_owner_payload(
            BootstrapCatalogReadCase::CurrentRootReadmissionRequired(value),
            counters,
        )
    }

    fn from_owner_payload(
        case: BootstrapCatalogReadCase,
        counters: super::BootstrapCatalogReadCounterSnapshot,
    ) -> Self {
        Self { case, counters }
    }

    pub fn view(&self) -> BootstrapCatalogReadOutcomeView<'_> {
        match &self.case {
            BootstrapCatalogReadCase::Success(value) => {
                BootstrapCatalogReadOutcomeView::Success(value)
            }
            BootstrapCatalogReadCase::CurrentRootReadmissionRequired(value) => {
                BootstrapCatalogReadOutcomeView::Denied(value)
            }
        }
    }

    pub const fn case_id(&self) -> BootstrapCatalogReadCaseId {
        match &self.case {
            BootstrapCatalogReadCase::Success(_) => BootstrapCatalogReadCaseId::Admitted,
            BootstrapCatalogReadCase::CurrentRootReadmissionRequired(_) => {
                BootstrapCatalogReadCaseId::CurrentRootReadmissionRequired
            }
        }
    }

    fn into_owner_payload(self) -> BootstrapCatalogReadCase {
        self.case
    }

    pub const fn counters(&self) -> super::BootstrapCatalogReadCounterSnapshot {
        self.counters
    }
}

impl BootstrapCatalogReadOutcome {
    pub fn is_err(&self) -> bool {
        matches!(self.view(), BootstrapCatalogReadOutcomeView::Denied(_))
    }
    pub fn into_result(self) -> Result<CatalogReadSuccess, super::BootstrapOnlyAccessDenied> {
        match self.into_owner_payload() {
            BootstrapCatalogReadCase::Success(value) => Ok(value),
            BootstrapCatalogReadCase::CurrentRootReadmissionRequired(denial) => Err(denial),
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

impl BootstrapCatalogAccess {
    pub fn read_catalog(
        &self,
        _path: BootstrapOnlyAccessPath,
        catalog: PhysicalBootstrapCatalogWitness,
        current_root: CurrentRootManifestAdmission,
    ) -> BootstrapCatalogReadOutcome {
        let counters = super::BootstrapCatalogReadCounterSnapshot::from_physical_catalog(&catalog);
        if current_root.root_owner() != catalog.identity().root_owner() {
            return BootstrapCatalogReadOutcome::current_root_readmission_required(
                BootstrapOnlyAccessDenied::CurrentRootReadmissionRequired {
                    expected: catalog.identity().root_owner(),
                    actual: current_root.root_owner(),
                },
                counters,
            );
        }

        let discovery = MinimalRootDiscoveryLayout::new(
            catalog.root_reference(),
            catalog.identity().physical_format_version(),
            catalog.checksum().bytes_checked(),
        );
        let identity = catalog.identity().clone();
        let admission = BootstrapCatalogReadAdmission::new(identity.clone());
        let layout_catalog = BootstrapLayoutCatalog::new(
            identity,
            discovery,
            catalog.root_entry_count(),
            catalog.segment_count(),
            catalog.page_slot_count(),
            catalog.extent_count(),
            catalog.allocation_class_count(),
            catalog.free_space_count(),
        );
        BootstrapCatalogReadOutcome::root_admitted((layout_catalog, admission), counters)
    }
}
