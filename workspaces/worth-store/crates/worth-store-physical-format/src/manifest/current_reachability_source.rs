use crate::{
    ExtentGenerationCell, PageGenerationCell, PhysicalRootManifest, PhysicalStoreIdentity,
};

/// Runtime-issued inventory of the exact current physical allocations.
///
/// This is neither a rebuild witness nor a backup manifest. It is the narrow
/// owner source from which consumers may prove current-root reachability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalCurrentReachabilitySource {
    manifest: PhysicalRootManifest,
    store_identity: PhysicalStoreIdentity,
    page_cells: Vec<PageGenerationCell>,
    extent_cells: Vec<ExtentGenerationCell>,
}

impl PhysicalCurrentReachabilitySource {
    pub(crate) fn issue(
        manifest: PhysicalRootManifest,
        store_identity: PhysicalStoreIdentity,
        mut page_cells: Vec<PageGenerationCell>,
        mut extent_cells: Vec<ExtentGenerationCell>,
    ) -> Self {
        page_cells.sort_by_key(|cell| {
            (
                cell.segment_id().get(),
                cell.page_id().get(),
                cell.generation().get(),
            )
        });
        extent_cells.sort_by_key(|cell| {
            (
                cell.segment_id().get(),
                cell.extent_id().get(),
                cell.generation().get(),
            )
        });
        Self {
            manifest,
            store_identity,
            page_cells,
            extent_cells,
        }
    }

    pub const fn manifest(&self) -> &PhysicalRootManifest {
        &self.manifest
    }

    pub const fn store_identity(&self) -> &PhysicalStoreIdentity {
        &self.store_identity
    }

    pub fn store_authority_identity(&self) -> worth_store_authority::StoreCurrentAuthorityIdentity {
        self.store_identity.authority_identity()
    }

    pub fn page_cells(&self) -> &[PageGenerationCell] {
        &self.page_cells
    }

    pub fn extent_cells(&self) -> &[ExtentGenerationCell] {
        &self.extent_cells
    }
}
