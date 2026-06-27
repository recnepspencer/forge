#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ManifestIntegrityCounters {
    root_manifest_reads: u32,
    segment_manifest_reads: u32,
    extent_manifest_reads: u32,
    allocation_map_reads: u32,
    free_space_map_reads: u32,
    manifest_reference_probes: u32,
    backend_residue_rejections: u32,
    derived_override_rejections: u32,
}

impl ManifestIntegrityCounters {
    pub const fn start() -> Self {
        Self {
            root_manifest_reads: 1,
            segment_manifest_reads: 0,
            extent_manifest_reads: 0,
            allocation_map_reads: 0,
            free_space_map_reads: 0,
            manifest_reference_probes: 0,
            backend_residue_rejections: 0,
            derived_override_rejections: 0,
        }
    }

    pub const fn with_manifest_sections(mut self) -> Self {
        self.segment_manifest_reads += 1;
        self.extent_manifest_reads += 1;
        self.allocation_map_reads += 1;
        self.free_space_map_reads += 1;
        self
    }

    pub const fn with_reference_probe(mut self) -> Self {
        self.manifest_reference_probes += 1;
        self
    }

    pub const fn with_backend_residue_rejection(mut self) -> Self {
        self.backend_residue_rejections += 1;
        self
    }

    pub const fn with_derived_override_rejection(mut self) -> Self {
        self.derived_override_rejections += 1;
        self
    }

    pub const fn root_manifest_reads(self) -> u32 {
        self.root_manifest_reads
    }

    pub const fn segment_manifest_reads(self) -> u32 {
        self.segment_manifest_reads
    }

    pub const fn extent_manifest_reads(self) -> u32 {
        self.extent_manifest_reads
    }

    pub const fn allocation_map_reads(self) -> u32 {
        self.allocation_map_reads
    }

    pub const fn free_space_map_reads(self) -> u32 {
        self.free_space_map_reads
    }

    pub const fn manifest_reference_probes(self) -> u32 {
        self.manifest_reference_probes
    }

    pub const fn backend_residue_rejections(self) -> u32 {
        self.backend_residue_rejections
    }

    pub const fn derived_override_rejections(self) -> u32 {
        self.derived_override_rejections
    }
}
