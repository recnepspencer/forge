use crate::offline_manifest_codec::{DecodedOfflineManifestSections, OfflineManifestCodec};
use crate::{
    ExtentMembership, ManifestDiscoveryAuthority, ManifestTraversalReport,
    MinimalManifestVerifierReport, OfflineVerifierCounterSnapshot, OfflineVerifierDenial,
    OfflineVerifierDenialKind, PersistedExtentBytes, PersistedPageBytes, PersistedPhysicalLayout,
    PhysicalHeaderAuthority, PhysicalLayoutReport, PhysicalManifestUniverseBuilder,
    PhysicalPageKind, PhysicalPageRecordAuthority, PhysicalReference, PhysicalReferenceAuthority,
    PhysicalRootManifest, PhysicalSegmentId, SlotDirectory,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfflinePhysicalVerifier {
    headers: PhysicalHeaderAuthority,
    references: PhysicalReferenceAuthority,
    manifests: ManifestDiscoveryAuthority,
}

impl OfflinePhysicalVerifier {
    pub const fn s1(headers: PhysicalHeaderAuthority) -> Self {
        Self {
            headers,
            references: PhysicalReferenceAuthority::s1(),
            manifests: ManifestDiscoveryAuthority::s1(),
        }
    }

    pub fn verify(
        &self,
        layout: &PersistedPhysicalLayout,
    ) -> Result<MinimalManifestVerifierReport, OfflineVerifierDenial> {
        let counters = OfflineVerifierCounterSnapshot::empty()
            .with_root_candidates_inspected(layout.root_manifest_candidates().len() as u32);
        let root_manifest = select_single_root_manifest(layout, counters)?;
        let decoded = OfflineManifestCodec::decode(
            self.headers.byte_order(),
            root_manifest,
            layout.segment_manifest(),
            layout.extent_manifest(),
            layout.free_space_map(),
            counters,
        )?;
        let counters = counters.with_manifest_rows_decoded(decoded.decoded_rows);
        reject_malformed_memberships(&decoded, counters)?;
        let root = build_root_manifest(&decoded);
        let manifest_report = self
            .manifests
            .reopen_from_root(&root, self.references.admit_root_publication(decoded.root))
            .map_err(|denial| {
                OfflineVerifierDenial::new(
                    OfflineVerifierDenialKind::ManifestDiscoveryDenied,
                    counters,
                )
                .with_manifest_denial(denial)
            })?;
        reject_backend_residue(layout, manifest_report, self.manifests, counters)?;
        let mut discovered = vec![self
            .references
            .admit_root_publication(decoded.root)
            .reference()];
        let counters =
            self.verify_pages(layout, manifest_report, &decoded, counters, &mut discovered)?;
        let counters =
            self.verify_extents(layout, manifest_report, &decoded, counters, &mut discovered)?;
        let counters =
            self.verify_free_space(manifest_report, &decoded, counters, &mut discovered)?;
        let counters = counters.with_parity_ready_references(discovered.len() as u32);
        Ok(MinimalManifestVerifierReport::new(
            PhysicalLayoutReport::new(discovered),
            traversal_report(&decoded),
            counters,
        ))
    }

    fn verify_pages(
        &self,
        layout: &PersistedPhysicalLayout,
        manifest_report: crate::ManifestDiscoveryReport<'_>,
        decoded: &DecodedOfflineManifestSections,
        counters: OfflineVerifierCounterSnapshot,
        discovered: &mut Vec<PhysicalReference>,
    ) -> Result<OfflineVerifierCounterSnapshot, OfflineVerifierDenial> {
        let page_records = PhysicalPageRecordAuthority::s1(self.headers.clone());
        let mut counters = counters;
        for entry in &decoded.page_slots {
            let cell = entry.page_slot();
            let page = find_page(layout.pages(), cell.segment_id(), cell.page_id(), counters)?;
            let header = page_records
                .decode_record_page_header(page.cell(), page.bytes(), PhysicalPageKind::DataPage)
                .map_err(|denial| {
                    OfflineVerifierDenial::new(
                        OfflineVerifierDenialKind::HeaderDecodeDenied,
                        counters.with_header_decode(),
                    )
                    .with_header_denial(denial)
                })?;
            counters = counters.with_header_decode();
            let page_payload = page_records
                .admit_record_page_payload(page.bytes(), header.witness())
                .map_err(|denial| {
                    OfflineVerifierDenial::new(
                        OfflineVerifierDenialKind::HeaderDecodeDenied,
                        counters,
                    )
                    .with_header_denial(denial)
                })?;
            let directory = SlotDirectory::decode(
                page_payload.bytes(),
                self.headers.byte_order(),
                crate::PageRecordCounterSnapshot::for_locate_attempt(),
            )
            .map_err(|denial| {
                OfflineVerifierDenial::new(OfflineVerifierDenialKind::PageRecordDenied, counters)
                    .with_page_denial(denial)
            })?;
            counters = counters.with_slot_directory_entries(directory.slot_count() as u32);
            let admission = self.references.admit_page_slot(cell);
            let validation = self
                .manifests
                .locate_page_slot(manifest_report, admission)
                .map_err(|denial| {
                    OfflineVerifierDenial::new(
                        OfflineVerifierDenialKind::ManifestDiscoveryDenied,
                        counters,
                    )
                    .with_manifest_denial(denial)
                })?;
            page_records
                .locate_record(page_payload, validation)
                .map_err(|denial| {
                    OfflineVerifierDenial::new(
                        OfflineVerifierDenialKind::PageRecordDenied,
                        counters,
                    )
                    .with_page_denial(denial)
                })?;
            discovered.push(admission.reference());
        }
        Ok(counters)
    }

    fn verify_extents(
        &self,
        layout: &PersistedPhysicalLayout,
        manifest_report: crate::ManifestDiscoveryReport<'_>,
        decoded: &DecodedOfflineManifestSections,
        counters: OfflineVerifierCounterSnapshot,
        discovered: &mut Vec<PhysicalReference>,
    ) -> Result<OfflineVerifierCounterSnapshot, OfflineVerifierDenial> {
        let extent_records = crate::PhysicalExtentRecordAuthority::s1(self.headers.clone());
        let mut counters = counters;
        for entry in &decoded.extents {
            let cell = entry.extent();
            let extent = find_extent(layout.extents(), cell, counters)?;
            let membership = ExtentMembership::large_record(cell, extent.bytes().len());
            counters = counters.with_extent_membership_check();
            let admission = self.references.admit_extent(cell);
            let validation = self
                .manifests
                .locate_extent(manifest_report, admission)
                .map_err(|denial| {
                    OfflineVerifierDenial::new(
                        OfflineVerifierDenialKind::ManifestDiscoveryDenied,
                        counters,
                    )
                    .with_manifest_denial(denial)
                })?;
            extent_records
                .locate_extent_record(extent.bytes(), membership, validation)
                .map_err(|denial| {
                    OfflineVerifierDenial::new(
                        OfflineVerifierDenialKind::ExtentRecordDenied,
                        counters,
                    )
                    .with_extent_denial(denial)
                })?;
            counters = counters.with_header_decode();
            discovered.push(admission.reference());
        }
        Ok(counters)
    }

    fn verify_free_space(
        &self,
        manifest_report: crate::ManifestDiscoveryReport<'_>,
        decoded: &DecodedOfflineManifestSections,
        counters: OfflineVerifierCounterSnapshot,
        discovered: &mut Vec<PhysicalReference>,
    ) -> Result<OfflineVerifierCounterSnapshot, OfflineVerifierDenial> {
        let mut counters = counters;
        for entry in &decoded.free_space {
            counters = counters.with_free_space_entry_checked();
            let admission = self.references.admit_free_space_reuse(entry.reuse_cell());
            self.manifests
                .validate_free_space_reuse(manifest_report, admission)
                .map_err(|denial| {
                    OfflineVerifierDenial::new(
                        OfflineVerifierDenialKind::ManifestDiscoveryDenied,
                        counters,
                    )
                    .with_manifest_denial(denial)
                })?;
            discovered.push(admission.reference());
        }
        Ok(counters)
    }
}

fn select_single_root_manifest(
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

fn reject_backend_residue(
    layout: &PersistedPhysicalLayout,
    report: crate::ManifestDiscoveryReport<'_>,
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

fn reject_malformed_memberships(
    decoded: &DecodedOfflineManifestSections,
    counters: OfflineVerifierCounterSnapshot,
) -> Result<(), OfflineVerifierDenial> {
    for page_slot in &decoded.page_slots {
        if !contains_segment(decoded, page_slot.page_slot().segment_id()) {
            return malformed_membership(counters);
        }
    }
    for extent in &decoded.extents {
        if !contains_segment(decoded, extent.extent().segment_id()) {
            return malformed_membership(counters);
        }
    }
    for free_space in &decoded.free_space {
        if !free_space_segment_is_manifested(decoded, free_space.reuse_cell()) {
            return malformed_membership(counters);
        }
    }
    Ok(())
}

fn malformed_membership<T>(
    counters: OfflineVerifierCounterSnapshot,
) -> Result<T, OfflineVerifierDenial> {
    Err(OfflineVerifierDenial::new(
        OfflineVerifierDenialKind::MalformedManifestMembership,
        counters,
    ))
}

fn contains_segment(
    decoded: &DecodedOfflineManifestSections,
    segment_id: PhysicalSegmentId,
) -> bool {
    decoded
        .segments
        .iter()
        .any(|entry| entry.segment().segment_id() == segment_id)
}

fn free_space_segment_is_manifested(
    decoded: &DecodedOfflineManifestSections,
    cell: crate::FreeSpaceReuseCell,
) -> bool {
    match cell.address() {
        crate::FreeSpaceReuseAddress::PageSlot { segment_id, .. }
        | crate::FreeSpaceReuseAddress::Extent { segment_id, .. } => {
            contains_segment(decoded, segment_id)
        }
    }
}

fn build_root_manifest(decoded: &DecodedOfflineManifestSections) -> PhysicalRootManifest {
    let mut builder = PhysicalManifestUniverseBuilder::s1(decoded.root);
    for segment in &decoded.segments {
        builder = builder.segment(segment.segment());
    }
    for page_slot in &decoded.page_slots {
        builder = builder.ordinary_page(page_slot.page_slot());
    }
    for extent in &decoded.extents {
        builder = builder.extent(extent.extent());
    }
    for allocation in &decoded.allocation_classes {
        builder = builder.allocation_class(allocation.allocation_class());
    }
    for free_space in &decoded.free_space {
        builder = builder.free_space_reuse(free_space.reuse_cell());
    }
    builder.publish()
}

fn find_page(
    pages: &[PersistedPageBytes],
    segment_id: PhysicalSegmentId,
    page_id: crate::PhysicalPageId,
    counters: OfflineVerifierCounterSnapshot,
) -> Result<&PersistedPageBytes, OfflineVerifierDenial> {
    pages
        .iter()
        .find(|page| page.cell().segment_id() == segment_id && page.cell().page_id() == page_id)
        .ok_or_else(|| {
            OfflineVerifierDenial::new(OfflineVerifierDenialKind::MissingPersistedPage, counters)
        })
}

fn find_extent(
    extents: &[PersistedExtentBytes],
    cell: crate::ExtentGenerationCell,
    counters: OfflineVerifierCounterSnapshot,
) -> Result<&PersistedExtentBytes, OfflineVerifierDenial> {
    extents
        .iter()
        .find(|extent| extent.cell() == cell)
        .ok_or_else(|| {
            OfflineVerifierDenial::new(OfflineVerifierDenialKind::MissingPersistedExtent, counters)
        })
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
