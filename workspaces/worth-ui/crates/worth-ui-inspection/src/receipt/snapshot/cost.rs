#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiVisualInspectionCostLane {
    OrdinaryPresentation,
    ExplicitSnapshot,
    ExplicitQuery,
    Overlay,
    ExecutableWorld,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiVisualInspectionCostReceipt {
    region_records_examined: u64,
    spatial_index_probes: u64,
    candidates_considered: u64,
    trace_index_probes: u64,
    pixel_bytes_requested: u64,
    pixel_bytes_transferred: u64,
    pixel_bytes_retained: u64,
    coordinate_transforms: u64,
    overlay_regions_emitted: u64,
    lease_count: u64,
    retained_structural_bytes: u64,
}

impl UiVisualInspectionCostReceipt {
    #[doc(hidden)]
    pub const fn from_runtime_projection(counters: [u64; 11]) -> Self {
        Self {
            region_records_examined: counters[0],
            spatial_index_probes: counters[1],
            candidates_considered: counters[2],
            trace_index_probes: counters[3],
            pixel_bytes_requested: counters[4],
            pixel_bytes_transferred: counters[5],
            pixel_bytes_retained: counters[6],
            coordinate_transforms: counters[7],
            overlay_regions_emitted: counters[8],
            lease_count: counters[9],
            retained_structural_bytes: counters[10],
        }
    }

    pub const fn counters(self) -> [u64; 11] {
        [
            self.region_records_examined,
            self.spatial_index_probes,
            self.candidates_considered,
            self.trace_index_probes,
            self.pixel_bytes_requested,
            self.pixel_bytes_transferred,
            self.pixel_bytes_retained,
            self.coordinate_transforms,
            self.overlay_regions_emitted,
            self.lease_count,
            self.retained_structural_bytes,
        ]
    }

    pub const fn pixel_bytes_requested(self) -> u64 {
        self.pixel_bytes_requested
    }

    pub const fn spatial_index_probes(self) -> u64 {
        self.spatial_index_probes
    }

    pub const fn candidates_considered(self) -> u64 {
        self.candidates_considered
    }

    pub const fn trace_index_probes(self) -> u64 {
        self.trace_index_probes
    }

    pub const fn pixel_bytes_transferred(self) -> u64 {
        self.pixel_bytes_transferred
    }

    pub const fn pixel_bytes_retained(self) -> u64 {
        self.pixel_bytes_retained
    }

    pub const fn coordinate_transforms(self) -> u64 {
        self.coordinate_transforms
    }

    pub const fn overlay_regions_emitted(self) -> u64 {
        self.overlay_regions_emitted
    }

    pub const fn lease_count(self) -> u64 {
        self.lease_count
    }

    pub const fn retained_structural_bytes(self) -> u64 {
        self.retained_structural_bytes
    }
}
