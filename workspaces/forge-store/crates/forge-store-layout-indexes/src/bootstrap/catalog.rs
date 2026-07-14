use forge_store_physical_format::{
    PhysicalBootstrapCatalogIdentity, PhysicalFormatVersion, PhysicalRootReference,
};

use super::root_discovery::MinimalRootDiscoveryLayout;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapLayoutCatalog {
    identity: PhysicalBootstrapCatalogIdentity,
    discovery_layout: MinimalRootDiscoveryLayout,
    root_entry_count: u32,
    segment_count: u32,
    page_slot_count: u32,
    extent_count: u32,
    allocation_class_count: u32,
    free_space_count: u32,
}

impl BootstrapLayoutCatalog {
    // Bootstrap counters stay individually named so field order cannot hide a count class.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        identity: PhysicalBootstrapCatalogIdentity,
        discovery_layout: MinimalRootDiscoveryLayout,
        root_entry_count: u32,
        segment_count: u32,
        page_slot_count: u32,
        extent_count: u32,
        allocation_class_count: u32,
        free_space_count: u32,
    ) -> Self {
        Self {
            identity,
            discovery_layout,
            root_entry_count,
            segment_count,
            page_slot_count,
            extent_count,
            allocation_class_count,
            free_space_count,
        }
    }

    pub const fn discovery_layout(&self) -> MinimalRootDiscoveryLayout {
        self.discovery_layout
    }

    pub fn root_reference(&self) -> PhysicalRootReference {
        self.identity.root_reference()
    }

    pub const fn layout_entry_count(&self) -> u32 {
        self.segment_count
            + self.page_slot_count
            + self.extent_count
            + self.allocation_class_count
            + self.free_space_count
    }

    pub const fn root_entry_count(&self) -> u32 {
        self.root_entry_count
    }

    pub const fn segment_count(&self) -> u32 {
        self.segment_count
    }

    pub const fn page_slot_count(&self) -> u32 {
        self.page_slot_count
    }

    pub const fn extent_count(&self) -> u32 {
        self.extent_count
    }

    pub const fn allocation_class_count(&self) -> u32 {
        self.allocation_class_count
    }

    pub const fn free_space_count(&self) -> u32 {
        self.free_space_count
    }

    pub fn physical_format_version(&self) -> PhysicalFormatVersion {
        self.identity.physical_format_version()
    }
}
