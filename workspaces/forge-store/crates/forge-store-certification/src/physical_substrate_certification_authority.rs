use crate::physical_substrate_certification_reports::{
    extent_record_reports, facade_reports, identity_reports, offline_reports, page_record_reports,
};
use crate::physical_substrate_certification_scan::PhysicalSubstrateCertificationScan;
use crate::physical_substrate_complexity_suite::complexity_reports;
use crate::physical_substrate_foundation_suite::foundation_bundle;
use crate::physical_substrate_manifest_suite::manifest_reports;
use crate::physical_substrate_story_suite::story_reports;
use crate::{
    PhysicalPageSegmentExtentSubstrateEvidence, PhysicalPageSegmentExtentSubstrateRun,
    PhysicalSubstrateCertificationDenial,
};
use forge_store_contracts::StableArtifactId;

pub fn certify_physical_page_segment_extent_substrate(
) -> Result<crate::PhysicalPageSegmentExtentSubstrateCloseout, PhysicalSubstrateCertificationDenial>
{
    crate::PhysicalPageSegmentExtentSubstrateCloseout::admit(closeout_run()?)
        .map_err(PhysicalSubstrateCertificationDenial::CloseoutDenied)
}

pub fn certify_s2_physical_substrate_readiness(
) -> Result<crate::S2PhysicalSubstrateReadiness, PhysicalSubstrateCertificationDenial> {
    Ok(certify_physical_page_segment_extent_substrate()?.into_s2_readiness())
}

pub(crate) fn closeout_run(
) -> Result<PhysicalPageSegmentExtentSubstrateRun, PhysicalSubstrateCertificationDenial> {
    let scan = PhysicalSubstrateCertificationScan::with_page_and_extent()?;
    Ok(PhysicalPageSegmentExtentSubstrateRun::new(
        run_id()?,
        PhysicalPageSegmentExtentSubstrateEvidence::new(
            story_reports()?,
            facade_reports(scan.scan(), scan.shortcut_counters())?,
            manifest_reports()?,
            offline_reports(scan.scan())?,
            page_record_reports()?,
            extent_record_reports()?,
            identity_reports()?,
            complexity_reports()?,
            foundation_bundle()?,
            scan.platform_grade_witness(),
            scan.s2_readiness(),
        ),
    ))
}

#[cfg(test)]
pub(crate) fn closeout_run_without_shortcut_row(
) -> Result<PhysicalPageSegmentExtentSubstrateRun, PhysicalSubstrateCertificationDenial> {
    let scan = PhysicalSubstrateCertificationScan::with_page_and_extent()?;
    let mut facade = facade_reports(scan.scan(), scan.shortcut_counters())?;
    facade.retain(|row| row.row() != crate::PlatformPhysicalFacadeEvidenceRow::ShortcutRejections);
    Ok(PhysicalPageSegmentExtentSubstrateRun::new(
        run_id()?,
        PhysicalPageSegmentExtentSubstrateEvidence::new(
            story_reports()?,
            facade,
            manifest_reports()?,
            offline_reports(scan.scan())?,
            page_record_reports()?,
            extent_record_reports()?,
            identity_reports()?,
            complexity_reports()?,
            foundation_bundle()?,
            scan.platform_grade_witness(),
            scan.s2_readiness(),
        ),
    ))
}

#[cfg(test)]
pub(crate) fn closeout_run_without_legacy_overclaim_row(
) -> Result<PhysicalPageSegmentExtentSubstrateRun, PhysicalSubstrateCertificationDenial> {
    let scan = PhysicalSubstrateCertificationScan::with_page_and_extent()?;
    let mut story = story_reports()?;
    story.retain(|row| {
        row.row() != crate::PhysicalSubstrateCloseoutStoryRow::LegacyOverclaimRejected
    });
    Ok(PhysicalPageSegmentExtentSubstrateRun::new(
        run_id()?,
        PhysicalPageSegmentExtentSubstrateEvidence::new(
            story,
            facade_reports(scan.scan(), scan.shortcut_counters())?,
            manifest_reports()?,
            offline_reports(scan.scan())?,
            page_record_reports()?,
            extent_record_reports()?,
            identity_reports()?,
            complexity_reports()?,
            foundation_bundle()?,
            scan.platform_grade_witness(),
            scan.s2_readiness(),
        ),
    ))
}

fn run_id() -> Result<StableArtifactId, PhysicalSubstrateCertificationDenial> {
    StableArtifactId::new("physical_page_segment_extent_substrate_run")
        .map_err(|_| PhysicalSubstrateCertificationDenial::RunIdentityRejected)
}
