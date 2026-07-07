use super::codec::DecodedOfflineManifestSections;
use crate::{
    ManifestDiscoveryAuthority, ManifestDiscoveryReport, OfflineVerifierCounterSnapshot,
    OfflineVerifierDenial, OfflineVerifierDenialKind, PersistedPageBytes, PhysicalHeaderAuthority,
    PhysicalPageKind, PhysicalPageRecordAuthority, PhysicalReference, PhysicalReferenceAuthority,
    PhysicalSegmentId, SlotDirectory,
};

pub(crate) struct PageVerificationContext<'a> {
    pub headers: &'a PhysicalHeaderAuthority,
    pub references: PhysicalReferenceAuthority,
    pub manifests: ManifestDiscoveryAuthority,
}

pub(crate) fn verify_all_pages(
    ctx: &PageVerificationContext<'_>,
    pages: &[PersistedPageBytes],
    manifest_report: ManifestDiscoveryReport<'_>,
    decoded: &DecodedOfflineManifestSections,
    counters: OfflineVerifierCounterSnapshot,
    discovered: &mut Vec<PhysicalReference>,
) -> Result<OfflineVerifierCounterSnapshot, OfflineVerifierDenial> {
    let page_records = PhysicalPageRecordAuthority::s1(ctx.headers.clone());
    let mut counters = counters;
    for entry in &decoded.page_slots {
        let cell = entry.page_slot();
        let page = collect_page_evidence(pages, cell.segment_id(), cell.page_id(), counters)?;
        let (header, next_counters) = verify_page_header_decode(&page_records, page, counters)?;
        counters = next_counters;
        let (page_payload, next_counters) =
            verify_page_payload_admission(&page_records, page, header, counters)?;
        counters = next_counters;
        counters = verify_page_slot_directory(
            page_payload.bytes(),
            ctx.headers.byte_order(),
            counters,
        )?;
        let admission = ctx.references.admit_page_slot(cell);
        let validation =
            verify_page_manifest_membership(ctx.manifests, manifest_report, admission, counters)?;
        verify_page_record_located(&page_records, page_payload, validation, counters)?;
        discovered.push(admission.reference());
    }
    Ok(counters)
}

fn collect_page_evidence(
    pages: &[PersistedPageBytes],
    segment_id: PhysicalSegmentId,
    page_id: crate::PhysicalPageId,
    counters: OfflineVerifierCounterSnapshot,
) -> Result<&PersistedPageBytes, OfflineVerifierDenial> {
    pages
        .iter()
        .find(|page| page.cell().segment_id() == segment_id && page.cell().page_id() == page_id)
        .ok_or_else(|| OfflineVerifierDenial::new(OfflineVerifierDenialKind::MissingPersistedPage, counters))
}

fn verify_page_header_decode(
    page_records: &PhysicalPageRecordAuthority,
    page: &PersistedPageBytes,
    counters: OfflineVerifierCounterSnapshot,
) -> Result<(crate::PhysicalHeaderDecodeReport, OfflineVerifierCounterSnapshot), OfflineVerifierDenial> {
    let header = page_records
        .decode_record_page_header(page.cell(), page.bytes(), PhysicalPageKind::DataPage)
        .map_err(|denial| {
            OfflineVerifierDenial::new(OfflineVerifierDenialKind::HeaderDecodeDenied, counters.with_header_decode())
                .with_header_denial(denial)
        })?;
    Ok((header, counters.with_header_decode()))
}

fn verify_page_payload_admission<'a>(
    page_records: &PhysicalPageRecordAuthority,
    page: &'a PersistedPageBytes,
    header: crate::PhysicalHeaderDecodeReport,
    counters: OfflineVerifierCounterSnapshot,
) -> Result<(crate::RecordPagePayload<'a>, OfflineVerifierCounterSnapshot), OfflineVerifierDenial> {
    let page_payload = page_records
        .admit_record_page_payload(page.bytes(), header.witness())
        .map_err(|denial| {
            OfflineVerifierDenial::new(OfflineVerifierDenialKind::HeaderDecodeDenied, counters)
                .with_header_denial(denial)
        })?;
    Ok((page_payload, counters))
}

fn verify_page_slot_directory(
    page_bytes: &[u8],
    byte_order: crate::PhysicalByteOrder,
    counters: OfflineVerifierCounterSnapshot,
) -> Result<OfflineVerifierCounterSnapshot, OfflineVerifierDenial> {
    let directory = SlotDirectory::decode(
        page_bytes,
        byte_order,
        crate::PageRecordCounterSnapshot::for_locate_attempt(),
    )
    .map_err(|denial| {
        OfflineVerifierDenial::new(OfflineVerifierDenialKind::PageRecordDenied, counters)
            .with_page_denial(denial)
    })?;
    Ok(counters.with_slot_directory_entries(directory.slot_count() as u32))
}

fn verify_page_manifest_membership(
    manifests: ManifestDiscoveryAuthority,
    manifest_report: ManifestDiscoveryReport<'_>,
    admission: crate::PhysicalReferenceAdmissionWitness,
    counters: OfflineVerifierCounterSnapshot,
) -> Result<crate::PhysicalReferenceValidationWitness, OfflineVerifierDenial> {
    manifests
        .locate_page_slot(manifest_report, admission)
        .map_err(|denial| {
            OfflineVerifierDenial::new(OfflineVerifierDenialKind::ManifestDiscoveryDenied, counters)
                .with_manifest_denial(denial)
        })
}

fn verify_page_record_located<'a>(
    page_records: &PhysicalPageRecordAuthority,
    page_payload: crate::RecordPagePayload<'a>,
    validation: crate::PhysicalReferenceValidationWitness,
    counters: OfflineVerifierCounterSnapshot,
) -> Result<(), OfflineVerifierDenial> {
    page_records
        .locate_record(page_payload, validation)
        .map_err(|denial| {
            OfflineVerifierDenial::new(OfflineVerifierDenialKind::PageRecordDenied, counters)
                .with_page_denial(denial)
        })?;
    Ok(())
}
