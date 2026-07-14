use super::codec::DecodedOfflineManifestSections;
use crate::{
    ExtentMembership, ManifestDiscoveryAuthority, ManifestDiscoveryReport,
    OfflineVerifierCounterSnapshot, OfflineVerifierDenial, OfflineVerifierDenialKind,
    PersistedExtentBytes, PhysicalExtentRecordAuthority, PhysicalHeaderAuthority,
    PhysicalReference, PhysicalReferenceAuthority,
};

pub(crate) struct ExtentVerificationContext<'a> {
    pub headers: &'a PhysicalHeaderAuthority,
    pub references: PhysicalReferenceAuthority,
    pub manifests: ManifestDiscoveryAuthority,
}

pub(crate) fn verify_all_extents(
    ctx: &ExtentVerificationContext<'_>,
    extents: &[PersistedExtentBytes],
    manifest_report: ManifestDiscoveryReport<'_>,
    decoded: &DecodedOfflineManifestSections,
    counters: OfflineVerifierCounterSnapshot,
    discovered: &mut Vec<PhysicalReference>,
) -> Result<OfflineVerifierCounterSnapshot, OfflineVerifierDenial> {
    let extent_records =
        PhysicalExtentRecordAuthority::for_canonical_physical_format(ctx.headers.clone());
    let mut counters = counters;
    for entry in &decoded.extents {
        let cell = entry.extent();
        let extent = collect_extent_evidence(extents, cell, counters)?;
        let membership = classify_extent_membership(cell, extent.bytes().len());
        counters = counters.with_extent_membership_check();
        let admission = ctx.references.admit_extent(cell);
        let validation =
            verify_extent_manifest_membership(ctx.manifests, manifest_report, admission, counters)?;
        verify_extent_record_located(
            &extent_records,
            extent.bytes(),
            membership,
            validation,
            counters,
        )?;
        counters = counters.with_header_decode();
        discovered.push(admission.reference());
    }
    Ok(counters)
}

fn collect_extent_evidence(
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

fn classify_extent_membership(
    cell: crate::ExtentGenerationCell,
    byte_length: usize,
) -> ExtentMembership {
    ExtentMembership::large_record(cell, byte_length)
}

fn verify_extent_manifest_membership(
    manifests: ManifestDiscoveryAuthority,
    manifest_report: ManifestDiscoveryReport<'_>,
    admission: crate::PhysicalReferenceAdmissionWitness,
    counters: OfflineVerifierCounterSnapshot,
) -> Result<crate::PhysicalReferenceValidationWitness, OfflineVerifierDenial> {
    manifests
        .locate_extent(manifest_report, admission)
        .map_err(|denial| {
            OfflineVerifierDenial::new(OfflineVerifierDenialKind::ManifestDiscoveryDenied, counters)
                .with_manifest_denial(denial)
        })
}

fn verify_extent_record_located(
    extent_records: &PhysicalExtentRecordAuthority,
    bytes: &[u8],
    membership: ExtentMembership,
    validation: crate::PhysicalReferenceValidationWitness,
    counters: OfflineVerifierCounterSnapshot,
) -> Result<(), OfflineVerifierDenial> {
    extent_records
        .locate_extent_record(bytes, membership, validation)
        .map_err(|denial| {
            OfflineVerifierDenial::new(OfflineVerifierDenialKind::ExtentRecordDenied, counters)
                .with_extent_denial(denial)
        })?;
    Ok(())
}
