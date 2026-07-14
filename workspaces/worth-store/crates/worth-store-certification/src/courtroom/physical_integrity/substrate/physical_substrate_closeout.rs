use crate::scenario::physical_integrity::physical_substrate_closeout_story::{
    PhysicalSubstrateCloseoutStoryReport, PhysicalSubstrateCloseoutStoryRow,
};
use crate::PhysicalFoundationEvidenceBundle;
use crate::{
    PhysicalComplexityEvidenceReport, PhysicalExtentRecordFramingEvidenceReport,
    PhysicalExtentRecordFramingEvidenceRow, PhysicalIdentityEvidenceReport,
    PhysicalIdentityEvidenceRow, PhysicalManifestDiscoveryEvidenceReport,
    PhysicalManifestDiscoveryEvidenceRow, PhysicalOfflineVerifierEvidenceReport,
    PhysicalOfflineVerifierEvidenceRow, PhysicalPageRecordFramingEvidenceReport,
    PhysicalPageRecordFramingEvidenceRow, PhysicalStoreRuntimeEvidenceReport,
    PhysicalStoreRuntimeEvidenceRow,
};
use worth_store_claim_boundaries::PlatformGradeClaimWitness;
use worth_store_contracts::{RoadmapScope, StableArtifactId};
use worth_store_physical_format::PhysicalOperationKind;
use worth_store_readiness::{PhysicalFoundationEvidenceField, PhysicalSubstrateReadiness};

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
    facade: Vec<PhysicalStoreRuntimeEvidenceReport>,
    manifest: Vec<PhysicalManifestDiscoveryEvidenceReport>,
    offline_verifier: Vec<PhysicalOfflineVerifierEvidenceReport>,
    page_records: Vec<PhysicalPageRecordFramingEvidenceReport>,
    extent_records: Vec<PhysicalExtentRecordFramingEvidenceReport>,
    identity: Vec<PhysicalIdentityEvidenceReport>,
    complexity: Vec<PhysicalComplexityEvidenceReport>,
    foundation: PhysicalFoundationEvidenceBundle,
    platform_grade_witness: PlatformGradeClaimWitness,
    physical_substrate_readiness: PhysicalSubstrateReadiness,
}

impl PhysicalPageSegmentExtentSubstrateEvidence {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        story: Vec<PhysicalSubstrateCloseoutStoryReport>,
        facade: Vec<PhysicalStoreRuntimeEvidenceReport>,
        manifest: Vec<PhysicalManifestDiscoveryEvidenceReport>,
        offline_verifier: Vec<PhysicalOfflineVerifierEvidenceReport>,
        page_records: Vec<PhysicalPageRecordFramingEvidenceReport>,
        extent_records: Vec<PhysicalExtentRecordFramingEvidenceReport>,
        identity: Vec<PhysicalIdentityEvidenceReport>,
        complexity: Vec<PhysicalComplexityEvidenceReport>,
        foundation: PhysicalFoundationEvidenceBundle,
        platform_grade_witness: PlatformGradeClaimWitness,
        physical_substrate_readiness: PhysicalSubstrateReadiness,
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
            physical_substrate_readiness,
        }
    }

    pub const fn platform_grade_witness(&self) -> PlatformGradeClaimWitness {
        self.platform_grade_witness
    }

    pub fn story(&self) -> &[PhysicalSubstrateCloseoutStoryReport] {
        &self.story
    }

    pub fn facade(&self) -> &[PhysicalStoreRuntimeEvidenceReport] {
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

    pub(crate) const fn physical_substrate_readiness(&self) -> PhysicalSubstrateReadiness {
        self.physical_substrate_readiness
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
        require_physical_substrate_readiness_scope(
            evidence.physical_substrate_readiness(),
            evidence.foundation.scope(),
        )?;
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

    #[cfg(test)]
    pub(crate) fn into_physical_substrate_readiness(self) -> PhysicalSubstrateReadiness {
        self.evidence().physical_substrate_readiness()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalSubstrateCloseoutDenial {
    MissingStoryRow(PhysicalSubstrateCloseoutStoryRow),
    MissingFacadeRow(PhysicalStoreRuntimeEvidenceRow),
    MissingManifestRow(PhysicalManifestDiscoveryEvidenceRow),
    MissingOfflineVerifierRow(PhysicalOfflineVerifierEvidenceRow),
    MissingPageRecordRow(PhysicalPageRecordFramingEvidenceRow),
    MissingExtentRecordRow(PhysicalExtentRecordFramingEvidenceRow),
    MissingIdentityRow(PhysicalIdentityEvidenceRow),
    MissingComplexityOperation(PhysicalOperationKind),
    UnverifiedComplexityOperation(PhysicalOperationKind),
    MissingFoundationEvidence(PhysicalFoundationEvidenceField),
    PlatformWitnessScopeMismatch,
    ReadinessScopeMismatch,
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
    reports: &[PhysicalStoreRuntimeEvidenceReport],
) -> Result<(), PhysicalSubstrateCloseoutDenial> {
    for row in PhysicalStoreRuntimeEvidenceRow::physical_format_required() {
        if !reports.iter().any(|report| report.row() == row) {
            return Err(PhysicalSubstrateCloseoutDenial::MissingFacadeRow(row));
        }
    }
    Ok(())
}

fn require_manifest_rows(
    reports: &[PhysicalManifestDiscoveryEvidenceReport],
) -> Result<(), PhysicalSubstrateCloseoutDenial> {
    for row in PhysicalManifestDiscoveryEvidenceRow::physical_format_required() {
        if !reports.iter().any(|report| report.row() == row) {
            return Err(PhysicalSubstrateCloseoutDenial::MissingManifestRow(row));
        }
    }
    Ok(())
}

fn require_offline_verifier_rows(
    reports: &[PhysicalOfflineVerifierEvidenceReport],
) -> Result<(), PhysicalSubstrateCloseoutDenial> {
    for row in PhysicalOfflineVerifierEvidenceRow::physical_format_required() {
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
    for operation in PhysicalOperationKind::required_physical_operations() {
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
    for field in PhysicalFoundationEvidenceField::required_for_physical_format() {
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

fn require_physical_substrate_readiness_scope(
    readiness: PhysicalSubstrateReadiness,
    scope: RoadmapScope,
) -> Result<(), PhysicalSubstrateCloseoutDenial> {
    if readiness.scope() == scope {
        Ok(())
    } else {
        Err(PhysicalSubstrateCloseoutDenial::ReadinessScopeMismatch)
    }
}
