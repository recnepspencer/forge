use crate::courtroom::physical_integrity::physical_substrate_certification_reports::{
    extent_record_reports, facade_reports, identity_reports, offline_reports, page_record_reports,
};
use crate::courtroom::physical_integrity::physical_substrate_certification_scan::PhysicalSubstrateCertificationScan;
use crate::courtroom::physical_integrity::physical_substrate_complexity_suite::complexity_reports;
use crate::courtroom::physical_integrity::physical_substrate_foundation_suite::foundation_bundle;
use crate::courtroom::physical_integrity::physical_substrate_manifest_suite::manifest_reports;
use crate::scenario::physical_integrity::physical_substrate_story_suite::story_reports;
use crate::{
    PhysicalPageSegmentExtentSubstrateEvidence, PhysicalPageSegmentExtentSubstrateRun,
    PhysicalSubstrateCertificationDenial,
};
use worth_store_contracts::StableArtifactId;

struct PhysicalSubstrateCloseoutEvidence {
    story: Vec<crate::PhysicalSubstrateCloseoutStoryReport>,
    facade: Vec<crate::PhysicalStoreRuntimeEvidenceReport>,
    manifest: Vec<crate::PhysicalManifestDiscoveryEvidenceReport>,
    offline: Vec<crate::PhysicalOfflineVerifierEvidenceReport>,
    page_record: Vec<crate::PhysicalPageRecordFramingEvidenceReport>,
    extent_record: Vec<crate::PhysicalExtentRecordFramingEvidenceReport>,
    identity: Vec<crate::PhysicalIdentityEvidenceReport>,
    complexity: Vec<crate::PhysicalComplexityEvidenceReport>,
    foundation: crate::PhysicalFoundationEvidenceBundle,
}

pub fn certify_physical_page_segment_extent_substrate(
) -> Result<crate::PhysicalPageSegmentExtentSubstrateCloseout, PhysicalSubstrateCertificationDenial>
{
    crate::PhysicalPageSegmentExtentSubstrateCloseout::admit(closeout_run()?)
        .map_err(PhysicalSubstrateCertificationDenial::CloseoutDenied)
}

pub(crate) fn closeout_run(
) -> Result<PhysicalPageSegmentExtentSubstrateRun, PhysicalSubstrateCertificationDenial> {
    let scan = scan_closeout_substrate()?;
    Ok(construct_closeout_run(
        run_id()?,
        &scan,
        collect_closeout_evidence(&scan)?,
    ))
}

fn collect_closeout_evidence(
    scan: &PhysicalSubstrateCertificationScan,
) -> Result<PhysicalSubstrateCloseoutEvidence, PhysicalSubstrateCertificationDenial> {
    Ok(PhysicalSubstrateCloseoutEvidence {
        story: collect_story_reports()?,
        facade: collect_facade_reports(scan)?,
        manifest: collect_manifest_reports()?,
        offline: collect_offline_reports(scan)?,
        page_record: collect_page_record_reports()?,
        extent_record: collect_extent_record_reports()?,
        identity: collect_identity_reports()?,
        complexity: collect_complexity_reports()?,
        foundation: collect_foundation_bundle()?,
    })
}

fn scan_closeout_substrate(
) -> Result<PhysicalSubstrateCertificationScan, PhysicalSubstrateCertificationDenial> {
    PhysicalSubstrateCertificationScan::with_page_and_extent()
}

fn collect_story_reports(
) -> Result<Vec<crate::PhysicalSubstrateCloseoutStoryReport>, PhysicalSubstrateCertificationDenial>
{
    story_reports()
}

fn collect_facade_reports(
    scan: &PhysicalSubstrateCertificationScan,
) -> Result<Vec<crate::PhysicalStoreRuntimeEvidenceReport>, PhysicalSubstrateCertificationDenial> {
    facade_reports(scan.scan(), scan.shortcut_counters())
}

fn collect_manifest_reports(
) -> Result<Vec<crate::PhysicalManifestDiscoveryEvidenceReport>, PhysicalSubstrateCertificationDenial>
{
    manifest_reports()
}

fn collect_offline_reports(
    scan: &PhysicalSubstrateCertificationScan,
) -> Result<Vec<crate::PhysicalOfflineVerifierEvidenceReport>, PhysicalSubstrateCertificationDenial>
{
    offline_reports(scan.scan())
}

fn collect_page_record_reports(
) -> Result<Vec<crate::PhysicalPageRecordFramingEvidenceReport>, PhysicalSubstrateCertificationDenial>
{
    page_record_reports()
}

fn collect_extent_record_reports() -> Result<
    Vec<crate::PhysicalExtentRecordFramingEvidenceReport>,
    PhysicalSubstrateCertificationDenial,
> {
    extent_record_reports()
}

fn collect_identity_reports(
) -> Result<Vec<crate::PhysicalIdentityEvidenceReport>, PhysicalSubstrateCertificationDenial> {
    identity_reports()
}

fn collect_complexity_reports(
) -> Result<Vec<crate::PhysicalComplexityEvidenceReport>, PhysicalSubstrateCertificationDenial> {
    complexity_reports()
}

fn collect_foundation_bundle(
) -> Result<crate::PhysicalFoundationEvidenceBundle, PhysicalSubstrateCertificationDenial> {
    foundation_bundle()
}

fn construct_closeout_run(
    run: StableArtifactId,
    scan: &PhysicalSubstrateCertificationScan,
    evidence: PhysicalSubstrateCloseoutEvidence,
) -> PhysicalPageSegmentExtentSubstrateRun {
    PhysicalPageSegmentExtentSubstrateRun::new(
        run,
        PhysicalPageSegmentExtentSubstrateEvidence::new(
            evidence.story,
            evidence.facade,
            evidence.manifest,
            evidence.offline,
            evidence.page_record,
            evidence.extent_record,
            evidence.identity,
            evidence.complexity,
            evidence.foundation,
            scan.platform_grade_witness(),
            scan.physical_substrate_readiness(),
        ),
    )
}

#[cfg(test)]
pub(crate) fn closeout_run_without_shortcut_row(
) -> Result<PhysicalPageSegmentExtentSubstrateRun, PhysicalSubstrateCertificationDenial> {
    let scan = scan_closeout_substrate()?;
    let mut evidence = collect_closeout_evidence(&scan)?;
    evidence
        .facade
        .retain(|row| row.row() != crate::PhysicalStoreRuntimeEvidenceRow::ShortcutRejections);
    Ok(construct_closeout_run(run_id()?, &scan, evidence))
}

#[cfg(test)]
pub(crate) fn closeout_run_without_legacy_overclaim_row(
) -> Result<PhysicalPageSegmentExtentSubstrateRun, PhysicalSubstrateCertificationDenial> {
    let scan = scan_closeout_substrate()?;
    let mut evidence = collect_closeout_evidence(&scan)?;
    evidence.story.retain(|row| {
        row.row() != crate::PhysicalSubstrateCloseoutStoryRow::LegacyOverclaimRejected
    });
    Ok(construct_closeout_run(run_id()?, &scan, evidence))
}

fn run_id() -> Result<StableArtifactId, PhysicalSubstrateCertificationDenial> {
    StableArtifactId::new("physical_page_segment_extent_substrate_run")
        .map_err(|_| PhysicalSubstrateCertificationDenial::RunIdentityRejected)
}
