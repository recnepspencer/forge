use crate::physical_substrate_certification_scan::PhysicalSubstrateCertificationScan;
use crate::{
    PhysicalExtentRecordFramingEvidenceReport, PhysicalExtentRecordFramingEvidenceRow,
    PhysicalIdentityEvidenceReport, PhysicalIdentityEvidenceRow,
    PhysicalOfflineVerifierEvidenceReport, PhysicalOfflineVerifierEvidenceRow,
    PhysicalPageRecordFramingEvidenceReport, PhysicalPageRecordFramingEvidenceRow,
    PhysicalRuntimeVerifierComparison, PhysicalSubstrateCertificationDenial,
    PlatformPhysicalFacadeEvidenceReport, PlatformPhysicalFacadeEvidenceRow,
};
use worth_store_physical_format::{
    ExtentRecordCounterSnapshot, OfflineVerifierLayoutObservation, PageRecordCounterSnapshot,
    PhysicalReferenceValidationCounterSnapshot, PlatformPhysicalFacadeCounterSnapshot,
    PlatformPhysicalScanReport, RuntimeLayoutObservation,
};

pub(crate) fn facade_reports(
    scan: &PlatformPhysicalScanReport,
    counters: PlatformPhysicalFacadeCounterSnapshot,
) -> Result<Vec<PlatformPhysicalFacadeEvidenceReport>, PhysicalSubstrateCertificationDenial> {
    Ok(vec![
        PlatformPhysicalFacadeEvidenceReport::from_facade_evidence(
            PlatformPhysicalFacadeEvidenceRow::OperationSurface,
            &scan.platform_evidence(),
        )
        .map_err(|_| PhysicalSubstrateCertificationDenial::FacadeEvidenceRejected)?,
        PlatformPhysicalFacadeEvidenceReport::from_facade_evidence(
            PlatformPhysicalFacadeEvidenceRow::RuntimeVerifierParity,
            &scan.platform_evidence(),
        )
        .map_err(|_| PhysicalSubstrateCertificationDenial::FacadeEvidenceRejected)?,
        PlatformPhysicalFacadeEvidenceReport::from_shortcut_counters(counters)
            .map_err(|_| PhysicalSubstrateCertificationDenial::FacadeEvidenceRejected)?,
    ])
}

pub(crate) fn offline_reports(
    scan: &PlatformPhysicalScanReport,
) -> Result<Vec<PhysicalOfflineVerifierEvidenceReport>, PhysicalSubstrateCertificationDenial> {
    let runtime = RuntimeLayoutObservation::from_facade_scan(scan);
    let offline = OfflineVerifierLayoutObservation::from_verifier_report(scan.verifier_report());
    let comparison = PhysicalRuntimeVerifierComparison::compare(&runtime, &offline)
        .map_err(|_| PhysicalSubstrateCertificationDenial::RuntimeVerifierComparisonDenied)?;
    let mismatch_scan = PhysicalSubstrateCertificationScan::with_page_only()?;
    let mismatch_runtime = RuntimeLayoutObservation::from_facade_scan(&mismatch_scan);
    let mismatch = PhysicalRuntimeVerifierComparison::compare(&mismatch_runtime, &offline)
        .err()
        .ok_or(PhysicalSubstrateCertificationDenial::RuntimeVerifierMismatchNotDetected)?;
    Ok(vec![
        PhysicalOfflineVerifierEvidenceReport::from_verifier_report(
            PhysicalOfflineVerifierEvidenceRow::MinimalManifestSmoke,
            scan.verifier_report(),
        )
        .map_err(|_| PhysicalSubstrateCertificationDenial::OfflineVerifierEvidenceRejected)?,
        PhysicalOfflineVerifierEvidenceReport::from_runtime_verifier_comparison(
            PhysicalOfflineVerifierEvidenceRow::RuntimeLayoutMatch,
            &comparison,
        )
        .map_err(|_| PhysicalSubstrateCertificationDenial::OfflineVerifierEvidenceRejected)?,
        PhysicalOfflineVerifierEvidenceReport::from_runtime_verifier_mismatch(
            PhysicalOfflineVerifierEvidenceRow::RuntimeDisagreementReported,
            &mismatch,
        )
        .map_err(|_| PhysicalSubstrateCertificationDenial::OfflineVerifierEvidenceRejected)?,
    ])
}

pub(crate) fn page_record_reports(
) -> Result<Vec<PhysicalPageRecordFramingEvidenceReport>, PhysicalSubstrateCertificationDenial> {
    let counters = successful_page_locate_counters();
    Ok(vec![
        PhysicalPageRecordFramingEvidenceReport::from_counters(
            PhysicalPageRecordFramingEvidenceRow::ReopenLocateStableFramedRecord,
            counters,
        )
        .map_err(|_| PhysicalSubstrateCertificationDenial::PageRecordEvidenceRejected)?,
        PhysicalPageRecordFramingEvidenceReport::from_counters(
            PhysicalPageRecordFramingEvidenceRow::SlotLookupCountersExact,
            counters,
        )
        .map_err(|_| PhysicalSubstrateCertificationDenial::PageRecordEvidenceRejected)?,
    ])
}

pub(crate) fn extent_record_reports(
) -> Result<Vec<PhysicalExtentRecordFramingEvidenceReport>, PhysicalSubstrateCertificationDenial> {
    Ok(vec![
        PhysicalExtentRecordFramingEvidenceReport::from_counters(
            PhysicalExtentRecordFramingEvidenceRow::ExtentBackedLargeRecord,
            successful_extent_locate_counters(),
        )
        .map_err(|_| PhysicalSubstrateCertificationDenial::ExtentRecordEvidenceRejected)?,
    ])
}

pub(crate) fn identity_reports(
) -> Result<Vec<PhysicalIdentityEvidenceReport>, PhysicalSubstrateCertificationDenial> {
    Ok(vec![
        stale_identity(
            PhysicalIdentityEvidenceRow::StaleSlotReferenceDeniedBeforeDecode,
            PhysicalReferenceValidationCounterSnapshot::for_page_slot_attempt(),
        )?,
        stale_identity(
            PhysicalIdentityEvidenceRow::StaleExtentReferenceDeniedBeforeDecode,
            PhysicalReferenceValidationCounterSnapshot::for_extent_attempt(),
        )?,
        stale_identity(
            PhysicalIdentityEvidenceRow::StaleFreeSpaceReferenceDeniedBeforeDecode,
            PhysicalReferenceValidationCounterSnapshot::for_free_space_slot_attempt(),
        )?,
        stale_identity(
            PhysicalIdentityEvidenceRow::StaleRootPublicationReferenceDeniedBeforeDecode,
            PhysicalReferenceValidationCounterSnapshot::for_root_publication_attempt(),
        )?,
    ])
}

fn stale_identity(
    row: PhysicalIdentityEvidenceRow,
    counters: PhysicalReferenceValidationCounterSnapshot,
) -> Result<PhysicalIdentityEvidenceReport, PhysicalSubstrateCertificationDenial> {
    PhysicalIdentityEvidenceReport::from_reference_validation(
        row,
        counters
            .with_generation_check()
            .with_stale_generation_rejection(),
    )
    .map_err(|_| PhysicalSubstrateCertificationDenial::IdentityEvidenceRejected)
}

fn successful_page_locate_counters() -> PageRecordCounterSnapshot {
    PageRecordCounterSnapshot::for_locate_attempt()
        .with_slot_lookup()
        .with_frame_decode()
        .with_record_payload_view()
}

fn successful_extent_locate_counters() -> ExtentRecordCounterSnapshot {
    ExtentRecordCounterSnapshot::for_locate_attempt()
        .with_membership_check()
        .with_length_check()
        .with_header_decode()
        .with_payload_view()
}
