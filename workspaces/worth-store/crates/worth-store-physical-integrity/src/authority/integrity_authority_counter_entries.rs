use crate::{
    authority::integrity_authority_basis_entries::push_u32, ContainerIntegrityCounters,
    ManifestIntegrityCounters, WalFrameIntegrityCounters,
};
use worth_foundational::canonicalization_api::lower_lane::basis::CanonicalBasisEntry;

pub(crate) fn push_container_counters(
    entries: &mut Vec<CanonicalBasisEntry>,
    counters: ContainerIntegrityCounters,
) {
    push_u32(
        entries,
        "counters.protected-window-reads",
        counters.protected_window_reads(),
    );
    push_u32(
        entries,
        "counters.header-witness-checks",
        counters.header_witness_checks(),
    );
    push_u32(
        entries,
        "counters.body-boundary-checks",
        counters.body_boundary_checks(),
    );
    push_u32(
        entries,
        "counters.frame-boundary-checks",
        counters.frame_boundary_checks(),
    );
    push_u32(
        entries,
        "counters.extent-boundary-checks",
        counters.extent_boundary_checks(),
    );
    push_u32(
        entries,
        "counters.slot-directory-reads",
        counters.slot_directory_reads(),
    );
    push_u32(
        entries,
        "counters.slot-entries-inspected",
        counters.slot_entries_inspected(),
    );
    push_u32(
        entries,
        "counters.skipped-record-view-constructions",
        counters.skipped_record_view_constructions(),
    );
}

pub(crate) fn push_wal_counters(
    entries: &mut Vec<CanonicalBasisEntry>,
    counters: WalFrameIntegrityCounters,
) {
    push_u32(
        entries,
        "counters.protected-window-reads",
        counters.protected_window_reads(),
    );
    push_u32(
        entries,
        "counters.frame-header-checks",
        counters.frame_header_checks(),
    );
    push_u32(
        entries,
        "counters.payload-boundary-checks",
        counters.payload_boundary_checks(),
    );
    push_u32(
        entries,
        "counters.checksum-posture-checks",
        counters.checksum_posture_checks(),
    );
    push_u32(
        entries,
        "counters.tail-posture-checks",
        counters.tail_posture_checks(),
    );
    push_u32(
        entries,
        "counters.checkpoint-adjacency-checks",
        counters.checkpoint_adjacency_checks(),
    );
    push_u32(
        entries,
        "counters.skipped-replay-attempts",
        counters.skipped_replay_attempts(),
    );
}

pub(crate) fn push_manifest_counters(
    entries: &mut Vec<CanonicalBasisEntry>,
    counters: ManifestIntegrityCounters,
) {
    push_u32(
        entries,
        "counters.root-manifest-reads",
        counters.root_manifest_reads(),
    );
    push_u32(
        entries,
        "counters.segment-manifest-reads",
        counters.segment_manifest_reads(),
    );
    push_u32(
        entries,
        "counters.extent-manifest-reads",
        counters.extent_manifest_reads(),
    );
    push_u32(
        entries,
        "counters.allocation-map-reads",
        counters.allocation_map_reads(),
    );
    push_u32(
        entries,
        "counters.free-space-map-reads",
        counters.free_space_map_reads(),
    );
    push_u32(
        entries,
        "counters.manifest-reference-probes",
        counters.manifest_reference_probes(),
    );
    push_u32(
        entries,
        "counters.backend-residue-rejections",
        counters.backend_residue_rejections(),
    );
    push_u32(
        entries,
        "counters.derived-override-rejections",
        counters.derived_override_rejections(),
    );
}
