use crate::PhysicalArtifactFamily;
use forge_store_physical_format::{CurrentRootManifestAdmission, PhysicalBootstrapCatalogWitness};

use super::{
    bootstrap_only_path::S8BootstrapOnlyAccessPath, catalog::S8BootstrapLayoutCatalog,
    catalog_read_admission::S8BootstrapCatalogReadAdmission, denial::S8BootstrapOnlyAccessDenied,
    root_discovery::S8MinimalRootDiscoveryLayout,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapCatalogFacade;

impl BootstrapCatalogFacade {
    pub const fn s8() -> Self {
        Self
    }

    pub fn read_catalog(
        &self,
        path: S8BootstrapOnlyAccessPath,
        catalog: PhysicalBootstrapCatalogWitness,
        current_root: CurrentRootManifestAdmission,
    ) -> super::S8BootstrapCatalogReadOutcome {
        if path.physical_format_version() != catalog.identity().physical_format_version() {
            return super::issue_catalog_read(Err(
                S8BootstrapOnlyAccessDenied::BootstrapPathVersionMismatch {
                    expected: catalog.identity().physical_format_version(),
                    actual: path.physical_format_version(),
                },
            ));
        }
        if current_root.root_owner() != catalog.identity().root_owner() {
            return super::issue_catalog_read(Err(
                S8BootstrapOnlyAccessDenied::CurrentRootReadmissionRequired {
                    expected: catalog.identity().root_owner(),
                    actual: current_root.root_owner(),
                },
            ));
        }

        let discovery = S8MinimalRootDiscoveryLayout::new(
            catalog.root_reference(),
            catalog.identity().physical_format_version(),
            catalog.checksum().bytes_checked(),
        );
        let identity = catalog.identity().clone();
        let admission = S8BootstrapCatalogReadAdmission::new(identity.clone());
        let layout_catalog = S8BootstrapLayoutCatalog::new(
            identity,
            discovery,
            catalog.root_entry_count(),
            catalog.segment_count(),
            catalog.page_slot_count(),
            catalog.extent_count(),
            catalog.allocation_class_count(),
            catalog.free_space_count(),
        );
        super::issue_catalog_read(Ok((layout_catalog, admission)))
    }

    pub fn deny_ordinary_family_access(
        &self,
        family: PhysicalArtifactFamily,
    ) -> Result<(), S8BootstrapOnlyAccessDenied> {
        Err(S8BootstrapOnlyAccessDenied::OrdinaryFamilyAccessForbidden { family })
    }
}

pub const fn bootstrap_catalog() -> BootstrapCatalogFacade {
    BootstrapCatalogFacade::s8()
}
