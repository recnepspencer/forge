use forge_store_physical_format::{MinimalManifestVerifierReport, PersistedPhysicalLayout};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalArtifactFixtureCatalog {
    root_manifest_candidates: u32,
    segment_manifest_bytes: u64,
    extent_manifest_bytes: u64,
    free_space_map_bytes: u64,
    persisted_pages: u32,
    persisted_extents: u32,
    discovered_references: u32,
    page_slots: u32,
    extents: u32,
    free_space_entries: u32,
}

impl PhysicalArtifactFixtureCatalog {
    pub(crate) fn from_reopened_layout(
        layout: &PersistedPhysicalLayout,
        report: &MinimalManifestVerifierReport,
    ) -> Self {
        Self {
            root_manifest_candidates: layout.root_manifest_candidates().len() as u32,
            segment_manifest_bytes: layout.segment_manifest().len() as u64,
            extent_manifest_bytes: layout.extent_manifest().len() as u64,
            free_space_map_bytes: layout.free_space_map().len() as u64,
            persisted_pages: layout.pages().len() as u32,
            persisted_extents: layout.extents().len() as u32,
            discovered_references: report.layout().discovered_references().len() as u32,
            page_slots: report.traversal().page_slot_count(),
            extents: report.traversal().extent_count(),
            free_space_entries: report.traversal().free_space_count(),
        }
    }

    pub const fn root_manifest_candidates(&self) -> u32 {
        self.root_manifest_candidates
    }

    pub const fn persisted_pages(&self) -> u32 {
        self.persisted_pages
    }

    pub const fn persisted_extents(&self) -> u32 {
        self.persisted_extents
    }

    pub const fn discovered_references(&self) -> u32 {
        self.discovered_references
    }

    pub const fn page_slots(&self) -> u32 {
        self.page_slots
    }

    pub const fn extents(&self) -> u32 {
        self.extents
    }

    pub const fn free_space_entries(&self) -> u32 {
        self.free_space_entries
    }

    pub const fn segment_manifest_bytes(&self) -> u64 {
        self.segment_manifest_bytes
    }

    pub const fn extent_manifest_bytes(&self) -> u64 {
        self.extent_manifest_bytes
    }

    pub const fn free_space_map_bytes(&self) -> u64 {
        self.free_space_map_bytes
    }
}
