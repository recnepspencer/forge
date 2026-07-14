use forge_store_physical_format::PhysicalBootstrapCatalogWitness;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapCatalogReadCounterSnapshot {
    catalog_candidates_read: u64,
    checksum_bytes_verified: u64,
    root_entries_read: u64,
    layout_entries_read: u64,
    admitted_catalogs: u64,
}

impl BootstrapCatalogReadCounterSnapshot {
    pub(super) fn from_physical_catalog(catalog: &PhysicalBootstrapCatalogWitness) -> Self {
        Self {
            catalog_candidates_read: 1,
            checksum_bytes_verified: catalog.checksum().bytes_checked(),
            root_entries_read: catalog.root_entry_count() as u64,
            layout_entries_read: catalog.segment_count() as u64
                + catalog.page_slot_count() as u64
                + catalog.extent_count() as u64
                + catalog.allocation_class_count() as u64
                + catalog.free_space_count() as u64,
            admitted_catalogs: 0,
        }
    }

    pub(super) const fn admitted(mut self) -> Self {
        self.admitted_catalogs = 1;
        self
    }

    pub const fn catalog_candidates_read(self) -> u64 {
        self.catalog_candidates_read
    }

    pub const fn checksum_bytes_verified(self) -> u64 {
        self.checksum_bytes_verified
    }

    pub const fn root_entries_read(self) -> u64 {
        self.root_entries_read
    }

    pub const fn layout_entries_read(self) -> u64 {
        self.layout_entries_read
    }

    pub const fn admitted_catalogs(self) -> u64 {
        self.admitted_catalogs
    }
}
