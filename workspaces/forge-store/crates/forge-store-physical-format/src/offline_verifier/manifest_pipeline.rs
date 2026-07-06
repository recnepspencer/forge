use super::codec::{DecodedOfflineManifestSections, OfflineManifestCodec};
use crate::{
    ManifestDiscoveryAuthority, ManifestDiscoveryReport, ManifestTraversalReport,
    MinimalManifestVerifierReport, OfflineVerifierCounterSnapshot, OfflineVerifierDenial,
    OfflineVerifierDenialKind, PersistedPhysicalLayout, PhysicalLayoutReport, PhysicalReference,
    PhysicalReferenceAuthority, PhysicalRootManifest,
};

pub(crate) fn collect_layout_inspection_counters(
    layout: &PersistedPhysicalLayout,
) -> OfflineVerifierCounterSnapshot {
    OfflineVerifierCounterSnapshot::empty()
        .with_root_candidates_inspected(layout.root_manifest_candidates().len() as u32)
}

pub(crate) fn select_single_root_manifest(
    layout: &PersistedPhysicalLayout,
    counters: OfflineVerifierCounterSnapshot,
) -> Result<&[u8], OfflineVerifierDenial> {
    match layout.root_manifest_candidates() {
        [] => Err(OfflineVerifierDenial::new(
            OfflineVerifierDenialKind::MissingRootManifest,
            counters,
        )),
        [root] => Ok(root),
        _ => Err(OfflineVerifierDenial::new(
            OfflineVerifierDenialKind::AmbiguousRootManifest,
            counters,
        )),
    }
}

pub(crate) fn decode_manifest_sections(
    byte_order: crate::PhysicalByteOrder,
    layout: &PersistedPhysicalLayout,
    root_manifest: &[u8],
    counters: OfflineVerifierCounterSnapshot,
) -> Result<DecodedOfflineManifestSections, OfflineVerifierDenial> {
    OfflineManifestCodec::decode(
        byte_order,
        root_manifest,
        layout.segment_manifest(),
        layout.extent_manifest(),
        layout.free_space_map(),
        counters,
    )
}

pub(crate) fn verify_manifest_discovery(
    manifests: ManifestDiscoveryAuthority,
    references: PhysicalReferenceAuthority,
    root: &PhysicalRootManifest,
    root_cell: crate::RootPublicationCell,
    counters: OfflineVerifierCounterSnapshot,
) -> Result<ManifestDiscoveryReport<'_>, OfflineVerifierDenial> {
    manifests
        .reopen_from_root(root, references.admit_root_publication(root_cell))
        .map_err(|denial| {
            OfflineVerifierDenial::new(
                OfflineVerifierDenialKind::ManifestDiscoveryDenied,
                counters,
            )
            .with_manifest_denial(denial)
        })
}

pub(crate) fn reject_backend_residue(
    layout: &PersistedPhysicalLayout,
    report: ManifestDiscoveryReport<'_>,
    manifests: ManifestDiscoveryAuthority,
    counters: OfflineVerifierCounterSnapshot,
) -> Result<(), OfflineVerifierDenial> {
    if let Some(residue) = layout.backend_residue().first().copied() {
        let denial = manifests.reject_backend_residue(report, residue);
        return Err(OfflineVerifierDenial::new(
            OfflineVerifierDenialKind::BackendResidueDiscoverySource,
            counters.with_backend_residue_rejection(),
        )
        .with_manifest_denial(denial));
    }
    Ok(())
}

pub(crate) fn construct_minimal_verifier_report(
    decoded: &DecodedOfflineManifestSections,
    discovered: Vec<PhysicalReference>,
    counters: OfflineVerifierCounterSnapshot,
) -> Result<MinimalManifestVerifierReport, OfflineVerifierDenial> {
    let reference_count = discovered.len() as u32;
    Ok(MinimalManifestVerifierReport::new(
        PhysicalLayoutReport::new(discovered),
        traversal_report(decoded),
        counters.with_parity_ready_references(reference_count),
    ))
}

fn traversal_report(decoded: &DecodedOfflineManifestSections) -> ManifestTraversalReport {
    ManifestTraversalReport::new(
        1,
        decoded.segments.len() as u32,
        decoded.page_slots.len() as u32,
        decoded.extents.len() as u32,
        decoded.allocation_classes.len() as u32,
        decoded.free_space.len() as u32,
    )
}