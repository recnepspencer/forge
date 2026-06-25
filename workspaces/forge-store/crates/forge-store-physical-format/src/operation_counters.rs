use crate::{
    ExtentRecordCounterSnapshot, ManifestDiscoveryCounterSnapshot, OfflineVerifierCounterSnapshot,
    PageRecordCounterSnapshot, PhysicalHeaderDecodeCounterSnapshot, PhysicalOperationKind,
    PhysicalReferenceValidationCounterSnapshot, PlatformPhysicalFacadeCounterSnapshot,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalOperationCounterSnapshot {
    operation: PhysicalOperationKind,
    rows: Vec<PhysicalOperationCounterRow>,
}

impl PhysicalOperationCounterSnapshot {
    pub fn from_header_decode(counters: PhysicalHeaderDecodeCounterSnapshot) -> Self {
        Self::new(
            PhysicalOperationKind::HeaderDecode,
            vec![
                row(
                    "physical.header_decode_attempt",
                    counters.header_decode_attempt_count(),
                ),
                row(
                    "physical.page_header_decode",
                    counters.page_header_decode_count(),
                ),
                row(
                    "physical.frame_header_decode",
                    counters.frame_header_decode_count(),
                ),
                row(
                    "physical.unknown_kind_denial",
                    counters.unknown_kind_denial_count(),
                ),
                row(
                    "physical.unsupported_version_denial",
                    counters.unsupported_version_denial_count(),
                ),
                row(
                    "physical.length_mismatch_denial",
                    counters.length_mismatch_denial_count(),
                ),
                row(
                    "physical.reserved_field_denial",
                    counters.reserved_field_denial_count(),
                ),
                row(
                    "physical.logical_decode_after_invalid_header",
                    counters.logical_decode_after_invalid_header_count(),
                ),
            ],
        )
    }

    pub fn from_reference_validation(counters: PhysicalReferenceValidationCounterSnapshot) -> Self {
        Self::new(
            PhysicalOperationKind::PhysicalReferenceValidation,
            vec![
                row(
                    "physical.reference_validation",
                    counters.validation_attempt_count(),
                ),
                row(
                    "physical.page_slot_validation",
                    counters.page_slot_validation_count(),
                ),
                row(
                    "physical.extent_validation",
                    counters.extent_validation_count(),
                ),
                row(
                    "physical.free_space_reuse_validation",
                    counters.free_space_reuse_validation_count(),
                ),
                row(
                    "physical.root_publication_validation",
                    counters.root_publication_validation_count(),
                ),
                row(
                    "physical.segment_id_check",
                    counters.segment_id_check_count(),
                ),
                row("physical.page_id_check", counters.page_id_check_count()),
                row("physical.extent_id_check", counters.extent_id_check_count()),
                row("physical.slot_check", counters.slot_check_count()),
                row(
                    "physical.root_reference_check",
                    counters.root_reference_check_count(),
                ),
                row(
                    "physical.allocation_class_check",
                    counters.allocation_class_check_count(),
                ),
                row(
                    "physical.generation_check",
                    counters.generation_check_count(),
                ),
            ],
        )
    }

    pub fn from_page_record_locate(counters: PageRecordCounterSnapshot) -> Self {
        Self::new(
            PhysicalOperationKind::LocateByReference,
            page_rows(counters),
        )
    }

    pub fn from_page_record_append(counters: PageRecordCounterSnapshot) -> Self {
        Self::new(
            PhysicalOperationKind::AppendRecordPlacement,
            page_rows(counters),
        )
    }

    pub fn from_extent_record_locate(counters: ExtentRecordCounterSnapshot) -> Self {
        Self::new(
            PhysicalOperationKind::LocateByReference,
            vec![
                row("physical.extent_read", counters.extent_read_count()),
                row("physical.extent_write", counters.extent_write_count()),
                row(
                    "physical.extent_header_decode",
                    counters.extent_header_decode_count(),
                ),
                row(
                    "physical.extent_membership_check",
                    counters.extent_membership_check_count(),
                ),
                row(
                    "physical.extent_length_check",
                    counters.extent_length_check_count(),
                ),
                row("physical.extent_locate", counters.extent_locate_count()),
                row(
                    "physical.extent_payload_view",
                    counters.extent_payload_view_count(),
                ),
            ],
        )
    }

    pub fn from_manifest_lookup(counters: ManifestDiscoveryCounterSnapshot) -> Self {
        Self::new(
            PhysicalOperationKind::ManifestLookup,
            manifest_rows(counters),
        )
    }

    pub fn from_manifest_traversal(counters: ManifestDiscoveryCounterSnapshot) -> Self {
        Self::new(
            PhysicalOperationKind::ManifestTraversal,
            manifest_rows(counters),
        )
    }

    pub fn from_root_open(counters: PlatformPhysicalFacadeCounterSnapshot) -> Self {
        Self::new(
            PhysicalOperationKind::RootManifestOpen,
            vec![
                row("physical.open", counters.opens()),
                row("physical.reopen", counters.reopens()),
                row("physical.root_publication", counters.root_publications()),
            ],
        )
    }

    pub fn from_offline_verifier_walk(counters: OfflineVerifierCounterSnapshot) -> Self {
        Self::new(
            PhysicalOperationKind::OfflineVerifierWalk,
            vec![
                row(
                    "physical.verifier_root_candidates",
                    counters.root_candidates_inspected(),
                ),
                row(
                    "physical.verifier_manifest_rows",
                    counters.manifest_rows_decoded(),
                ),
                row("physical.verifier_header_decode", counters.header_decodes()),
                row(
                    "physical.verifier_slot_directory_entries",
                    counters.slot_directory_entries(),
                ),
                row(
                    "physical.verifier_extent_membership",
                    counters.extent_membership_checks(),
                ),
                row(
                    "physical.verifier_free_space_entries",
                    counters.free_space_entries_checked(),
                ),
            ],
        )
    }

    pub const fn operation(&self) -> PhysicalOperationKind {
        self.operation
    }

    pub fn rows(&self) -> &[PhysicalOperationCounterRow] {
        &self.rows
    }

    pub fn observed(&self, name: &str) -> Option<u64> {
        self.rows
            .iter()
            .find(|row| row.name() == name)
            .map(PhysicalOperationCounterRow::count)
    }

    fn new(operation: PhysicalOperationKind, rows: Vec<PhysicalOperationCounterRow>) -> Self {
        Self { operation, rows }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalOperationCounterRow {
    name: &'static str,
    count: u64,
}

impl PhysicalOperationCounterRow {
    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn count(&self) -> u64 {
        self.count
    }
}

fn page_rows(counters: PageRecordCounterSnapshot) -> Vec<PhysicalOperationCounterRow> {
    vec![
        row("physical.page_read", counters.page_read_count()),
        row("physical.page_write", counters.page_write_count()),
        row("physical.frame_decode", counters.frame_decode_count()),
        row("physical.record_locate", counters.record_locate_count()),
        row("physical.slot_lookup", counters.slot_lookup_count()),
        row("physical.page_local_scan", counters.page_local_scan_count()),
        row(
            "physical.record_payload_view",
            counters.record_payload_view_count(),
        ),
    ]
}

fn manifest_rows(counters: ManifestDiscoveryCounterSnapshot) -> Vec<PhysicalOperationCounterRow> {
    vec![
        row(
            "physical.root_manifest_read",
            counters.root_manifest_read_count(),
        ),
        row(
            "physical.root_manifest_publish",
            counters.root_manifest_publish_count(),
        ),
        row(
            "physical.root_manifest_entries",
            counters.root_manifest_entry_count(),
        ),
        row(
            "physical.segment_manifest_read",
            counters.segment_manifest_read_count(),
        ),
        row(
            "physical.segment_manifest_entries",
            counters.segment_manifest_entry_count(),
        ),
        row(
            "physical.extent_manifest_read",
            counters.extent_manifest_read_count(),
        ),
        row(
            "physical.extent_manifest_entries",
            counters.extent_manifest_entry_count(),
        ),
        row(
            "physical.allocation_class_entries",
            counters.allocation_class_entry_count(),
        ),
        row(
            "physical.free_space_map_entries",
            counters.free_space_map_entry_count(),
        ),
        row(
            "physical.manifest_index_probe",
            counters.manifest_index_probe_count(),
        ),
    ]
}

fn row(name: &'static str, count: u32) -> PhysicalOperationCounterRow {
    PhysicalOperationCounterRow {
        name,
        count: count as u64,
    }
}
