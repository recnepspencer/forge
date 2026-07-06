use crate::physical_substrate_closeout_story::{
    PhysicalSubstrateCloseoutStoryReport, PhysicalSubstrateCloseoutStoryRow,
};
use crate::PhysicalFoundationEvidenceBundle;
use crate::{
    PhysicalComplexityEvidenceReport, PhysicalExtentRecordFramingEvidenceReport,
    PhysicalExtentRecordFramingEvidenceRow, PhysicalIdentityEvidenceReport,
    PhysicalIdentityEvidenceRow, PhysicalManifestDiscoveryEvidenceReport,
    PhysicalManifestDiscoveryEvidenceRow, PhysicalOfflineVerifierEvidenceReport,
    PhysicalOfflineVerifierEvidenceRow, PhysicalPageRecordFramingEvidenceReport,
    PhysicalPageRecordFramingEvidenceRow, PlatformPhysicalFacadeEvidenceReport,
    PlatformPhysicalFacadeEvidenceRow,
};
use forge_store_claim_boundaries::PlatformGradeClaimWitness;
use forge_store_contracts::{RoadmapScope, StableArtifactId};
use forge_store_physical_format::PhysicalOperationKind;
use forge_store_readiness::{PhysicalFoundationEvidenceField, S2PhysicalSubstrateReadiness};

#[derive(Debug)]
pub struct PhysicalPageSegmentExtentSubstrateRun {
    run_id: StableArtifactId,
    evidence: PhysicalPageSegmentExtentSubstrateEvidence,
}

impl PhysicalPageSegmentExtentSubstrateRun {
    pub(crate) fn new(
        run_id: StableArtifactId,
        evidence: PhysicalPageSegmentExtentSubstrateEvidence,
    ) -> Self {
        Self { run_id, evidence }
    }

    pub const fn run_id(&self) -> &StableArtifactId {
        &self.run_id
    }

    pub const fn evidence(&self) -> &PhysicalPageSegmentExtentSubstrateEvidence {
        &self.evidence
    }
}

#[derive(Debug)]
pub struct PhysicalPageSegmentExtentSubstrateEvidence {
    story: Vec<PhysicalSubstrateCloseoutStoryReport>,
    facade: Vec<PlatformPhysicalFacadeEvidenceReport>,
    manifest: Vec<PhysicalManifestDiscoveryEvidenceReport>,
    offline_verifier: Vec<PhysicalOfflineVerifierEvidenceReport>,
    page_records: Vec<PhysicalPageRecordFramingEvidenceReport>,
    extent_records: Vec<PhysicalExtentRecordFramingEvidenceReport>,
    identity: Vec<PhysicalIdentityEvidenceReport>,
    complexity: Vec<PhysicalComplexityEvidenceReport>,
    foundation: PhysicalFoundationEvidenceBundle,
    platform_grade_witness: PlatformGradeClaimWitness,
    s2_readiness: S2PhysicalSubstrateReadiness,
}

impl PhysicalPageSegmentExtentSubstrateEvidence {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        story: Vec<PhysicalSubstrateCloseoutStoryReport>,
        facade: Vec<PlatformPhysicalFacadeEvidenceReport>,
        manifest: Vec<PhysicalManifestDiscoveryEvidenceReport>,
        offline_verifier: Vec<PhysicalOfflineVerifierEvidenceReport>,
        page_records: Vec<PhysicalPageRecordFramingEvidenceReport>,
        extent_records: Vec<PhysicalExtentRecordFramingEvidenceReport>,
        identity: Vec<PhysicalIdentityEvidenceReport>,
        complexity: Vec<PhysicalComplexityEvidenceReport>,
        foundation: PhysicalFoundationEvidenceBundle,
        platform_grade_witness: PlatformGradeClaimWitness,
        s2_readiness: S2PhysicalSubstrateReadiness,
    ) -> Self {
        Self {
            story,
            facade,
            manifest,
            offline_verifier,
            page_records,
            extent_records,
            identity,
            complexity,
            foundation,
            platform_grade_witness,
            s2_readiness,
        }
    }

    pub const fn platform_grade_witness(&self) -> PlatformGradeClaimWitness {
        self.platform_grade_witness
    }

    pub fn story(&self) -> &[PhysicalSubstrateCloseoutStoryReport] {
        &self.story
    }

    pub fn facade(&self) -> &[PlatformPhysicalFacadeEvidenceReport] {
        &self.facade
    }

    pub fn manifest(&self) -> &[PhysicalManifestDiscoveryEvidenceReport] {
        &self.manifest
    }

    pub fn offline_verifier(&self) -> &[PhysicalOfflineVerifierEvidenceReport] {
        &self.offline_verifier
    }

    pub fn page_records(&self) -> &[PhysicalPageRecordFramingEvidenceReport] {
        &self.page_records
    }

    pub fn extent_records(&self) -> &[PhysicalExtentRecordFramingEvidenceReport] {
        &self.extent_records
    }

    pub fn identity(&self) -> &[PhysicalIdentityEvidenceReport] {
        &self.identity
    }

    pub fn complexity(&self) -> &[PhysicalComplexityEvidenceReport] {
        &self.complexity
    }

    pub const fn foundation(&self) -> &PhysicalFoundationEvidenceBundle {
        &self.foundation
    }

    pub(crate) const fn s2_readiness(&self) -> S2PhysicalSubstrateReadiness {
        self.s2_readiness
    }
}

#[derive(Debug)]
pub struct PhysicalPageSegmentExtentSubstrateCloseout {
    scope: RoadmapScope,
    run: PhysicalPageSegmentExtentSubstrateRun,
}

impl PhysicalPageSegmentExtentSubstrateCloseout {
    pub fn admit(
        run: PhysicalPageSegmentExtentSubstrateRun,
    ) -> Result<Self, PhysicalSubstrateCloseoutDenial> {
        let evidence = run.evidence();
        require_story_rows(&evidence.story)?;
        require_facade_rows(&evidence.facade)?;
        require_manifest_rows(&evidence.manifest)?;
        require_offline_verifier_rows(&evidence.offline_verifier)?;
        require_page_record_rows(&evidence.page_records)?;
        require_extent_record_rows(&evidence.extent_records)?;
        require_identity_rows(&evidence.identity)?;
        require_complexity_rows(&evidence.complexity)?;
        require_foundation_rows(&evidence.foundation)?;
        require_platform_witness_scope(
            evidence.platform_grade_witness(),
            evidence.foundation.scope(),
        )?;
        require_s2_readiness_scope(evidence.s2_readiness(), evidence.foundation.scope())?;
        Ok(Self {
            scope: evidence.foundation.scope(),
            run,
        })
    }

    pub const fn scope(&self) -> RoadmapScope {
        self.scope
    }

    pub const fn run(&self) -> &PhysicalPageSegmentExtentSubstrateRun {
        &self.run
    }

    pub const fn evidence(&self) -> &PhysicalPageSegmentExtentSubstrateEvidence {
        self.run.evidence()
    }

    pub(crate) fn into_s2_readiness(self) -> S2PhysicalSubstrateReadiness {
        self.evidence().s2_readiness()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalSubstrateCloseoutDenial {
    MissingStoryRow(PhysicalSubstrateCloseoutStoryRow),
    MissingFacadeRow(PlatformPhysicalFacadeEvidenceRow),
    MissingManifestRow(PhysicalManifestDiscoveryEvidenceRow),
    MissingOfflineVerifierRow(PhysicalOfflineVerifierEvidenceRow),
    MissingPageRecordRow(PhysicalPageRecordFramingEvidenceRow),
    MissingExtentRecordRow(PhysicalExtentRecordFramingEvidenceRow),
    MissingIdentityRow(PhysicalIdentityEvidenceRow),
    MissingComplexityOperation(PhysicalOperationKind),
    UnverifiedComplexityOperation(PhysicalOperationKind),
    MissingFoundationEvidence(PhysicalFoundationEvidenceField),
    PlatformWitnessScopeMismatch,
    S2ReadinessScopeMismatch,
}

fn require_story_rows(
    reports: &[PhysicalSubstrateCloseoutStoryReport],
) -> Result<(), PhysicalSubstrateCloseoutDenial> {
    for row in [
        PhysicalSubstrateCloseoutStoryRow::PhysicalSubstrateStoryTranscript,
        PhysicalSubstrateCloseoutStoryRow::LegacyOverclaimRejected,
    ] {
        if !reports.iter().any(|report| report.row() == row) {
            return Err(PhysicalSubstrateCloseoutDenial::MissingStoryRow(row));
        }
    }
    Ok(())
}

fn require_facade_rows(
    reports: &[PlatformPhysicalFacadeEvidenceReport],
) -> Result<(), PhysicalSubstrateCloseoutDenial> {
    for row in PlatformPhysicalFacadeEvidenceRow::s1_required() {
        if !reports.iter().any(|report| report.row() == row) {
            return Err(PhysicalSubstrateCloseoutDenial::MissingFacadeRow(row));
        }
    }
    Ok(())
}

fn require_manifest_rows(
    reports: &[PhysicalManifestDiscoveryEvidenceReport],
) -> Result<(), PhysicalSubstrateCloseoutDenial> {
    for row in PhysicalManifestDiscoveryEvidenceRow::s1_required() {
        if !reports.iter().any(|report| report.row() == row) {
            return Err(PhysicalSubstrateCloseoutDenial::MissingManifestRow(row));
        }
    }
    Ok(())
}

fn require_offline_verifier_rows(
    reports: &[PhysicalOfflineVerifierEvidenceReport],
) -> Result<(), PhysicalSubstrateCloseoutDenial> {
    for row in PhysicalOfflineVerifierEvidenceRow::s1_required() {
        if !reports.iter().any(|report| report.row() == row) {
            return Err(PhysicalSubstrateCloseoutDenial::MissingOfflineVerifierRow(
                row,
            ));
        }
    }
    Ok(())
}

fn require_page_record_rows(
    reports: &[PhysicalPageRecordFramingEvidenceReport],
) -> Result<(), PhysicalSubstrateCloseoutDenial> {
    for row in [
        PhysicalPageRecordFramingEvidenceRow::ReopenLocateStableFramedRecord,
        PhysicalPageRecordFramingEvidenceRow::SlotLookupCountersExact,
    ] {
        if !reports.iter().any(|report| report.row() == row) {
            return Err(PhysicalSubstrateCloseoutDenial::MissingPageRecordRow(row));
        }
    }
    Ok(())
}

fn require_extent_record_rows(
    reports: &[PhysicalExtentRecordFramingEvidenceReport],
) -> Result<(), PhysicalSubstrateCloseoutDenial> {
    let row = PhysicalExtentRecordFramingEvidenceRow::ExtentBackedLargeRecord;
    if !reports.iter().any(|report| report.row() == row) {
        return Err(PhysicalSubstrateCloseoutDenial::MissingExtentRecordRow(row));
    }
    Ok(())
}

fn require_identity_rows(
    reports: &[PhysicalIdentityEvidenceReport],
) -> Result<(), PhysicalSubstrateCloseoutDenial> {
    for row in [
        PhysicalIdentityEvidenceRow::StaleSlotReferenceDeniedBeforeDecode,
        PhysicalIdentityEvidenceRow::StaleExtentReferenceDeniedBeforeDecode,
        PhysicalIdentityEvidenceRow::StaleFreeSpaceReferenceDeniedBeforeDecode,
        PhysicalIdentityEvidenceRow::StaleRootPublicationReferenceDeniedBeforeDecode,
    ] {
        if !reports.iter().any(|report| report.row() == row) {
            return Err(PhysicalSubstrateCloseoutDenial::MissingIdentityRow(row));
        }
    }
    Ok(())
}

fn require_complexity_rows(
    reports: &[PhysicalComplexityEvidenceReport],
) -> Result<(), PhysicalSubstrateCloseoutDenial> {
    for operation in PhysicalOperationKind::s1_required() {
        let Some(report) = reports
            .iter()
            .find(|report| report.contract().operation() == operation)
        else {
            return Err(PhysicalSubstrateCloseoutDenial::MissingComplexityOperation(
                operation,
            ));
        };
        if !report.is_platform_grade_verified() {
            return Err(PhysicalSubstrateCloseoutDenial::UnverifiedComplexityOperation(operation));
        }
    }
    Ok(())
}

fn require_foundation_rows(
    foundation: &PhysicalFoundationEvidenceBundle,
) -> Result<(), PhysicalSubstrateCloseoutDenial> {
    for field in PhysicalFoundationEvidenceField::required_for_s1() {
        if !foundation
            .entries()
            .iter()
            .any(|entry| entry.field() == field)
        {
            return Err(PhysicalSubstrateCloseoutDenial::MissingFoundationEvidence(
                field,
            ));
        }
    }
    Ok(())
}

fn require_platform_witness_scope(
    platform_grade_witness: PlatformGradeClaimWitness,
    scope: RoadmapScope,
) -> Result<(), PhysicalSubstrateCloseoutDenial> {
    if platform_grade_witness.scope() == scope {
        Ok(())
    } else {
        Err(PhysicalSubstrateCloseoutDenial::PlatformWitnessScopeMismatch)
    }
}

fn require_s2_readiness_scope(
    readiness: S2PhysicalSubstrateReadiness,
    scope: RoadmapScope,
) -> Result<(), PhysicalSubstrateCloseoutDenial> {
    if readiness.scope() == scope {
        Ok(())
    } else {
        Err(PhysicalSubstrateCloseoutDenial::S2ReadinessScopeMismatch)
    }
}
