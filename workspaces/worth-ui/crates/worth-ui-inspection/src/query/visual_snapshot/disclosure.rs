#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiVisualInspectionAudience {
    LocalDevelopment,
    ApplicationOperator,
    DiagnosticAgent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiVisualPixelRedaction {
    UnredactedSyntheticContent,
    OpaqueBlack,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiVisualInspectionDisclosure {
    audience: UiVisualInspectionAudience,
    pixel_redaction: UiVisualPixelRedaction,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiVisualInspectionPolicy {
    disclosure: UiVisualInspectionDisclosure,
    maximum_snapshot_count: u8,
    maximum_capture_bytes: u64,
    maximum_retained_pixel_bytes: u64,
    maximum_retained_structural_bytes_per_receipt: u64,
    maximum_retained_structural_bytes_per_session: u64,
    maximum_visible_region_records: u32,
    maximum_hit_test_region_records: u32,
    maximum_query_results: u16,
    maximum_query_candidates: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiVisualInspectionCapacity {
    snapshot_count: u8,
    query_results: u16,
    query_candidates: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiVisualInspectionByteBudget {
    capture: u64,
    retained_pixels: u64,
    retained_structure_per_receipt: u64,
    retained_structure_per_session: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiVisualInspectionRegionCapacity {
    visible_records: u32,
    hit_test_records: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiVisualInspectionPolicyDenial {
    ZeroSnapshotCapacity,
    ZeroQueryResultCapacity,
    ResultCapacityExceedsCandidateCapacity,
}

impl UiVisualInspectionPolicy {
    pub const fn production_default(
        disclosure: UiVisualInspectionDisclosure,
    ) -> Result<Self, UiVisualInspectionPolicyDenial> {
        Self::bounded(
            disclosure,
            UiVisualInspectionCapacity::bounded(8, 32, 4_096),
            UiVisualInspectionRegionCapacity::bounded(65_536, 65_536),
            UiVisualInspectionByteBudget::bounded(64 << 20, 256 << 20, 64 << 20, 256 << 20),
        )
    }

    pub const fn bounded(
        disclosure: UiVisualInspectionDisclosure,
        capacity: UiVisualInspectionCapacity,
        regions: UiVisualInspectionRegionCapacity,
        bytes: UiVisualInspectionByteBudget,
    ) -> Result<Self, UiVisualInspectionPolicyDenial> {
        if capacity.snapshot_count == 0 {
            return Err(UiVisualInspectionPolicyDenial::ZeroSnapshotCapacity);
        }
        if capacity.query_results == 0 {
            return Err(UiVisualInspectionPolicyDenial::ZeroQueryResultCapacity);
        }
        if capacity.query_results > capacity.query_candidates {
            return Err(UiVisualInspectionPolicyDenial::ResultCapacityExceedsCandidateCapacity);
        }
        Ok(Self {
            disclosure,
            maximum_snapshot_count: capacity.snapshot_count,
            maximum_capture_bytes: bytes.capture,
            maximum_retained_pixel_bytes: bytes.retained_pixels,
            maximum_retained_structural_bytes_per_receipt: bytes.retained_structure_per_receipt,
            maximum_retained_structural_bytes_per_session: bytes.retained_structure_per_session,
            maximum_visible_region_records: regions.visible_records,
            maximum_hit_test_region_records: regions.hit_test_records,
            maximum_query_results: capacity.query_results,
            maximum_query_candidates: capacity.query_candidates,
        })
    }

    pub const fn audience(self) -> UiVisualInspectionAudience {
        self.disclosure.audience()
    }

    pub const fn disclosure(self) -> UiVisualInspectionDisclosure {
        self.disclosure
    }

    pub const fn maximum_snapshot_count(self) -> u8 {
        self.maximum_snapshot_count
    }

    pub const fn maximum_capture_bytes(self) -> u64 {
        self.maximum_capture_bytes
    }

    pub const fn maximum_retained_pixel_bytes(self) -> u64 {
        self.maximum_retained_pixel_bytes
    }

    pub const fn maximum_retained_structural_bytes_per_receipt(self) -> u64 {
        self.maximum_retained_structural_bytes_per_receipt
    }

    pub const fn maximum_retained_structural_bytes_per_session(self) -> u64 {
        self.maximum_retained_structural_bytes_per_session
    }

    pub const fn maximum_visible_region_records(self) -> u32 {
        self.maximum_visible_region_records
    }

    pub const fn maximum_hit_test_region_records(self) -> u32 {
        self.maximum_hit_test_region_records
    }

    pub const fn maximum_query_results(self) -> u16 {
        self.maximum_query_results
    }

    pub const fn maximum_query_candidates(self) -> u16 {
        self.maximum_query_candidates
    }
}

impl UiVisualInspectionDisclosure {
    pub const fn local_development_unredacted() -> Self {
        Self {
            audience: UiVisualInspectionAudience::LocalDevelopment,
            pixel_redaction: UiVisualPixelRedaction::UnredactedSyntheticContent,
        }
    }

    pub const fn redacted(audience: UiVisualInspectionAudience) -> Self {
        Self {
            audience,
            pixel_redaction: UiVisualPixelRedaction::OpaqueBlack,
        }
    }

    pub const fn audience(self) -> UiVisualInspectionAudience {
        self.audience
    }

    pub const fn pixel_redaction(self) -> UiVisualPixelRedaction {
        self.pixel_redaction
    }
}

impl UiVisualInspectionCapacity {
    pub const fn bounded(snapshot_count: u8, query_results: u16, query_candidates: u16) -> Self {
        Self {
            snapshot_count,
            query_results,
            query_candidates,
        }
    }
}

impl UiVisualInspectionByteBudget {
    pub const fn bounded(
        capture: u64,
        retained_pixels: u64,
        retained_structure_per_receipt: u64,
        retained_structure_per_session: u64,
    ) -> Self {
        Self {
            capture,
            retained_pixels,
            retained_structure_per_receipt,
            retained_structure_per_session,
        }
    }
}

impl UiVisualInspectionRegionCapacity {
    pub const fn bounded(visible_records: u32, hit_test_records: u32) -> Self {
        Self {
            visible_records,
            hit_test_records,
        }
    }
}
