#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ManifestDiscoveryCounterSnapshot {
    root_manifest_read_count: u32,
    root_manifest_publish_count: u32,
    root_manifest_entry_count: u32,
    segment_manifest_read_count: u32,
    segment_manifest_entry_count: u32,
    extent_manifest_read_count: u32,
    extent_manifest_entry_count: u32,
    allocation_class_entry_count: u32,
    free_space_map_entry_count: u32,
    manifest_index_probe_count: u32,
    backend_residue_rejection_count: u32,
}

impl ManifestDiscoveryCounterSnapshot {
    pub const fn for_publish() -> Self {
        Self {
            root_manifest_publish_count: 1,
            ..Self::zero()
        }
    }

    pub const fn for_reopen() -> Self {
        Self {
            root_manifest_read_count: 1,
            ..Self::zero()
        }
    }

    pub const fn with_root_entries(mut self, count: u32) -> Self {
        self.root_manifest_entry_count = count;
        self
    }

    pub const fn with_segment_manifest(mut self, entries: u32) -> Self {
        self.segment_manifest_read_count = 1;
        self.segment_manifest_entry_count = entries;
        self
    }

    pub const fn with_extent_manifest(mut self, entries: u32) -> Self {
        self.extent_manifest_read_count = 1;
        self.extent_manifest_entry_count = entries;
        self
    }

    pub const fn with_allocation_entries(mut self, count: u32) -> Self {
        self.allocation_class_entry_count = count;
        self
    }

    pub const fn with_free_space_entries(mut self, count: u32) -> Self {
        self.free_space_map_entry_count = count;
        self
    }

    pub const fn with_manifest_index_probe(mut self) -> Self {
        self.manifest_index_probe_count += 1;
        self
    }

    pub const fn with_backend_residue_rejection(mut self) -> Self {
        self.backend_residue_rejection_count += 1;
        self
    }

    pub const fn root_manifest_read_count(self) -> u32 {
        self.root_manifest_read_count
    }

    pub const fn root_manifest_publish_count(self) -> u32 {
        self.root_manifest_publish_count
    }

    pub const fn root_manifest_entry_count(self) -> u32 {
        self.root_manifest_entry_count
    }

    pub const fn segment_manifest_read_count(self) -> u32 {
        self.segment_manifest_read_count
    }

    pub const fn segment_manifest_entry_count(self) -> u32 {
        self.segment_manifest_entry_count
    }

    pub const fn extent_manifest_read_count(self) -> u32 {
        self.extent_manifest_read_count
    }

    pub const fn extent_manifest_entry_count(self) -> u32 {
        self.extent_manifest_entry_count
    }

    pub const fn allocation_class_entry_count(self) -> u32 {
        self.allocation_class_entry_count
    }

    pub const fn free_space_map_entry_count(self) -> u32 {
        self.free_space_map_entry_count
    }

    pub const fn manifest_index_probe_count(self) -> u32 {
        self.manifest_index_probe_count
    }

    pub const fn backend_residue_rejection_count(self) -> u32 {
        self.backend_residue_rejection_count
    }

    const fn zero() -> Self {
        Self {
            root_manifest_read_count: 0,
            root_manifest_publish_count: 0,
            root_manifest_entry_count: 0,
            segment_manifest_read_count: 0,
            segment_manifest_entry_count: 0,
            extent_manifest_read_count: 0,
            extent_manifest_entry_count: 0,
            allocation_class_entry_count: 0,
            free_space_map_entry_count: 0,
            manifest_index_probe_count: 0,
            backend_residue_rejection_count: 0,
        }
    }
}
