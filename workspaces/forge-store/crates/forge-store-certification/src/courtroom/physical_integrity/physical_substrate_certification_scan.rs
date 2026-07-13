use crate::PhysicalSubstrateCertificationDenial;
use forge_store_claim_boundaries::PlatformGradeClaimWitness;
use forge_store_contracts::{
    AcceptedHandoffReadiness, HandoffEvidenceDigestSet, StableDigest, ROADMAP_2_S1_SCOPE,
};
use forge_store_physical_format::{
    PhysicalExtentId, PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId,
    PhysicalRecordSlot, PhysicalReferenceAuthority, PhysicalSegmentId,
    PlatformPhysicalAppendRequest, PlatformPhysicalFacade, PlatformPhysicalFacadeCounterSnapshot,
    PlatformPhysicalOpenRequest, PlatformPhysicalScanReport,
};
use forge_store_readiness::{
    close_physical_substrate_readiness, prove_physical_substrate_readiness,
    PhysicalSubstrateReadiness,
};

pub(crate) struct PhysicalSubstrateCertificationScan {
    scan: PlatformPhysicalScanReport,
    shortcut_counters: PlatformPhysicalFacadeCounterSnapshot,
    platform_grade_witness: PlatformGradeClaimWitness,
    physical_substrate_readiness: PhysicalSubstrateReadiness,
}

impl PhysicalSubstrateCertificationScan {
    pub(crate) fn with_page_and_extent() -> Result<Self, PhysicalSubstrateCertificationDenial> {
        let mut facade = open_facade()?;
        let page_append = facade
            .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
                slot_cell(1)?,
                b"authoritative",
            ))
            .map_err(|_| PhysicalSubstrateCertificationDenial::FacadeOperationDenied)?;
        let extent_append = facade
            .append_physical_record(PlatformPhysicalAppendRequest::extent(
                extent_cell(1)?,
                b"derived-large",
            ))
            .map_err(|_| PhysicalSubstrateCertificationDenial::FacadeOperationDenied)?;
        facade
            .page_access()
            .locate_record(page_append.reference())
            .map_err(|_| PhysicalSubstrateCertificationDenial::FacadeOperationDenied)?;
        facade
            .extent_access()
            .read_record(extent_append.reference())
            .map_err(|_| PhysicalSubstrateCertificationDenial::FacadeOperationDenied)?;
        let published = facade
            .publish_physical_root()
            .map_err(|_| PhysicalSubstrateCertificationDenial::FacadeOperationDenied)?;
        let scan = facade
            .scan_physical_layout()
            .map_err(|_| PhysicalSubstrateCertificationDenial::FacadeOperationDenied)?;
        let mut reopened = PlatformPhysicalFacade::reopen(
            readiness()?,
            PlatformPhysicalOpenRequest::physical_format_canonical(),
            published.replay_artifact(),
        )
        .map_err(|_| PhysicalSubstrateCertificationDenial::FacadeOperationDenied)?;
        reopened
            .page_access()
            .locate_record(page_append.reference())
            .map_err(|_| PhysicalSubstrateCertificationDenial::FacadeOperationDenied)?;
        if facade.reject_full_store_heap_materialization().is_ok()
            || facade.reject_backend_residue_guess().is_ok()
        {
            return Err(PhysicalSubstrateCertificationDenial::FacadeOperationDenied);
        }
        let platform_grade_witness =
            PlatformGradeClaimWitness::from_facade_evidence(&scan.platform_evidence())
                .map_err(|_| PhysicalSubstrateCertificationDenial::PlatformWitnessRejected)?;
        let s1_closeout = close_physical_substrate_readiness(readiness()?)
            .map_err(|_| PhysicalSubstrateCertificationDenial::S2HandoffEvidenceRejected)?;
        let physical_substrate_readiness = prove_physical_substrate_readiness(s1_closeout)
            .map_err(|_| PhysicalSubstrateCertificationDenial::S2HandoffEvidenceRejected)?;
        Ok(Self {
            scan,
            shortcut_counters: facade.counters(),
            platform_grade_witness,
            physical_substrate_readiness,
        })
    }

    pub(crate) fn with_page_only(
    ) -> Result<PlatformPhysicalScanReport, PhysicalSubstrateCertificationDenial> {
        let mut facade = open_facade()?;
        facade
            .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
                slot_cell(9)?,
                b"page-only",
            ))
            .map_err(|_| PhysicalSubstrateCertificationDenial::FacadeOperationDenied)?;
        facade
            .page_access()
            .locate_record(
                PhysicalReferenceAuthority::for_canonical_physical_format()
                    .admit_page_slot(slot_cell(9)?)
                    .reference(),
            )
            .map_err(|_| PhysicalSubstrateCertificationDenial::FacadeOperationDenied)?;
        facade
            .publish_physical_root()
            .map_err(|_| PhysicalSubstrateCertificationDenial::FacadeOperationDenied)?;
        facade
            .scan_physical_layout()
            .map_err(|_| PhysicalSubstrateCertificationDenial::FacadeOperationDenied)
    }

    pub(crate) const fn scan(&self) -> &PlatformPhysicalScanReport {
        &self.scan
    }

    pub(crate) const fn shortcut_counters(&self) -> PlatformPhysicalFacadeCounterSnapshot {
        self.shortcut_counters
    }

    pub(crate) const fn platform_grade_witness(&self) -> PlatformGradeClaimWitness {
        self.platform_grade_witness
    }

    pub(crate) const fn physical_substrate_readiness(&self) -> PhysicalSubstrateReadiness {
        self.physical_substrate_readiness
    }
}

fn open_facade() -> Result<PlatformPhysicalFacade, PhysicalSubstrateCertificationDenial> {
    PlatformPhysicalFacade::open_physical_format(
        readiness()?,
        PlatformPhysicalOpenRequest::physical_format_canonical(),
    )
    .map_err(|_| PhysicalSubstrateCertificationDenial::FacadeOperationDenied)
}

fn readiness() -> Result<AcceptedHandoffReadiness, PhysicalSubstrateCertificationDenial> {
    AcceptedHandoffReadiness::from_foundational_handoff_artifacts(ROADMAP_2_S1_SCOPE, digest_set()?)
        .map_err(|_| PhysicalSubstrateCertificationDenial::ReadinessRejected)
}

fn digest_set() -> Result<HandoffEvidenceDigestSet, PhysicalSubstrateCertificationDenial> {
    Ok(HandoffEvidenceDigestSet::new(
        digest("backend")?,
        digest("deferred")?,
        digest("harness")?,
        digest("terms")?,
        digest("audit")?,
        digest("complexity")?,
        digest("provenance")?,
    ))
}

fn digest(name: &str) -> Result<StableDigest, PhysicalSubstrateCertificationDenial> {
    StableDigest::new(format!("sha256:{name}"))
        .map_err(|_| PhysicalSubstrateCertificationDenial::RunIdentityRejected)
}

fn slot_cell(
    value: u16,
) -> Result<forge_store_physical_format::SlotGenerationCell, PhysicalSubstrateCertificationDenial> {
    Ok(PhysicalGenerationAuthority::for_canonical_physical_format()
        .slot_cell(segment(1)?, page(1)?, slot(value)?)
        .with_slot_generation(generation(5)?))
}

fn extent_cell(
    value: u64,
) -> Result<forge_store_physical_format::ExtentGenerationCell, PhysicalSubstrateCertificationDenial>
{
    Ok(PhysicalGenerationAuthority::for_canonical_physical_format()
        .extent_cell(segment(1)?, extent(value)?)
        .with_extent_generation(generation(7)?))
}

fn segment(value: u64) -> Result<PhysicalSegmentId, PhysicalSubstrateCertificationDenial> {
    PhysicalSegmentId::from_raw(value)
        .map_err(|_| PhysicalSubstrateCertificationDenial::PhysicalIdentifierRejected)
}

fn page(value: u64) -> Result<PhysicalPageId, PhysicalSubstrateCertificationDenial> {
    PhysicalPageId::from_raw(value)
        .map_err(|_| PhysicalSubstrateCertificationDenial::PhysicalIdentifierRejected)
}

fn extent(value: u64) -> Result<PhysicalExtentId, PhysicalSubstrateCertificationDenial> {
    PhysicalExtentId::from_raw(value)
        .map_err(|_| PhysicalSubstrateCertificationDenial::PhysicalIdentifierRejected)
}

fn slot(value: u16) -> Result<PhysicalRecordSlot, PhysicalSubstrateCertificationDenial> {
    PhysicalRecordSlot::from_raw(value)
        .map_err(|_| PhysicalSubstrateCertificationDenial::PhysicalIdentifierRejected)
}

fn generation(value: u64) -> Result<PhysicalGeneration, PhysicalSubstrateCertificationDenial> {
    PhysicalGeneration::from_raw(value)
        .map_err(|_| PhysicalSubstrateCertificationDenial::PhysicalIdentifierRejected)
}
