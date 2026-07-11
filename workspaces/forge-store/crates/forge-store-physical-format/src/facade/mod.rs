mod append;
mod counters;
mod denials;
mod evidence;
mod reopen;
mod replay_artifact;
mod reports;
mod requests;
mod root_publication;
mod runtime_receipt;
mod scan;
mod shortcut_rejection;
pub(crate) mod storage;
mod storage_reference_index;
mod storage_segment_occupancy;
mod storage_support;
#[cfg(test)]
mod tests;

use crate::{
    layout_access::{
        allocation_family::{AdmittedAllocationLayoutFamily, AllocationLayoutFamilyHome},
        extent_family::{AdmittedExtentLayoutFamily, ExtentLayoutFamilyHome},
        fragmentation_family::{AdmittedFragmentationLayoutFamily, FragmentationLayoutFamilyHome},
        frame_family::{AdmittedFrameLayoutFamily, FrameLayoutFamilyHome},
        free_space_family::{AdmittedFreeSpaceLayoutFamily, FreeSpaceLayoutFamilyHome},
        manifest_family::{AdmittedManifestLayoutFamily, ManifestLayoutFamilyHome},
        page_family::{AdmittedPageLayoutFamily, PageLayoutFamilyHome},
        root_discovery_family::{AdmittedRootDiscoveryLayoutFamily, RootDiscoveryLayoutFamilyHome},
        segment_family::{AdmittedSegmentLayoutFamily, SegmentLayoutFamilyHome},
        AdmittedAllocationLayoutRule, AdmittedExtentLayoutRule, AdmittedFragmentationLayoutRule,
        AdmittedFrameLayoutRule, AdmittedFreeSpaceLayoutRule, AdmittedManifestIndexLayoutRule,
        AdmittedPageLayoutRule, AdmittedRootManifestLayoutRule, AdmittedSegmentLayoutRule,
    },
    PhysicalHeaderAuthority, PhysicalPageRecordAuthority, PhysicalReference,
    PhysicalReferenceAuthority,
};
use forge_store_contracts::{AcceptedHandoffReadiness, RoadmapScope};
use storage::PlatformPhysicalFacadeStorage;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PlatformPhysicalFacadeOperation {
    AppendPhysicalRecord,
    ReadPhysicalRecord,
    ScanPhysicalManifest,
    LocatePhysicalReference,
    PublishPhysicalRoot,
    ReopenPhysicalStore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformPhysicalFacadeVocabulary {
    operation: PlatformPhysicalFacadeOperation,
}

impl PlatformPhysicalFacadeVocabulary {
    pub const fn new(operation: PlatformPhysicalFacadeOperation) -> Self {
        Self { operation }
    }

    pub const fn operation(&self) -> PlatformPhysicalFacadeOperation {
        self.operation
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformPhysicalFacade {
    scope: RoadmapScope,
    headers: PhysicalHeaderAuthority,
    page_records: PhysicalPageRecordAuthority,
    extent_records: crate::PhysicalExtentRecordAuthority,
    references: PhysicalReferenceAuthority,
    storage: PlatformPhysicalFacadeStorage,
    counters: PlatformPhysicalFacadeCounterSnapshot,
    next_root_generation: u64,
}

impl PlatformPhysicalFacade {
    pub fn open_s1(
        readiness: AcceptedHandoffReadiness,
        request: PlatformPhysicalOpenRequest,
    ) -> Result<Self, PlatformPhysicalFacadeDenial> {
        reopen::verify_handoff_readiness(&readiness)?;
        Ok(Self::new(
            readiness.scope(),
            request.headers().clone(),
            PlatformPhysicalFacadeStorage::empty(),
            PlatformPhysicalFacadeCounterSnapshot::empty().with_open(),
        ))
    }

    pub fn reopen_s1(
        readiness: AcceptedHandoffReadiness,
        request: PlatformPhysicalOpenRequest,
        replay_artifact: PlatformPhysicalReplayArtifact,
    ) -> Result<Self, PlatformPhysicalFacadeDenial> {
        replay_artifact.reopen_s1(readiness, request)
    }

    pub fn append_physical_record(
        &mut self,
        request: PlatformPhysicalAppendRequest<'_>,
    ) -> Result<PlatformPhysicalAppendReport, PlatformPhysicalFacadeDenial> {
        let append = append::append_physical_record(
            &mut self.storage,
            &self.page_records,
            &self.extent_records,
            self.counters,
            request,
        )?;
        self.counters = append.counters();
        Ok(append.report())
    }

    pub fn page_layout<'a>(
        &'a mut self,
        rule: &AdmittedPageLayoutRule,
    ) -> Result<AdmittedPageLayoutFamily<'a>, PlatformPhysicalFacadeDenial> {
        let admission = PageLayoutFamilyHome::s8().admit(rule)?;
        Ok(AdmittedPageLayoutFamily::new(self, admission))
    }

    pub fn frame_layout<'a>(
        &'a mut self,
        rule: &AdmittedFrameLayoutRule,
    ) -> Result<AdmittedFrameLayoutFamily<'a>, PlatformPhysicalFacadeDenial> {
        let admission = FrameLayoutFamilyHome::s8().admit(rule)?;
        Ok(AdmittedFrameLayoutFamily::new(self, admission))
    }

    pub fn segment_layout<'a>(
        &'a mut self,
        rule: &AdmittedSegmentLayoutRule,
    ) -> Result<AdmittedSegmentLayoutFamily<'a>, PlatformPhysicalFacadeDenial> {
        let admission = SegmentLayoutFamilyHome::s8().admit(rule)?;
        Ok(AdmittedSegmentLayoutFamily::new(self, admission))
    }

    pub fn extent_layout<'a>(
        &'a mut self,
        rule: &AdmittedExtentLayoutRule,
    ) -> Result<AdmittedExtentLayoutFamily<'a>, PlatformPhysicalFacadeDenial> {
        let admission = ExtentLayoutFamilyHome::s8().admit(rule)?;
        Ok(AdmittedExtentLayoutFamily::new(self, admission))
    }

    pub fn root_manifest_layout<'a>(
        &'a mut self,
        rule: &AdmittedRootManifestLayoutRule,
    ) -> Result<AdmittedRootDiscoveryLayoutFamily<'a>, PlatformPhysicalFacadeDenial> {
        let admission = RootDiscoveryLayoutFamilyHome::s8().admit(rule)?;
        Ok(AdmittedRootDiscoveryLayoutFamily::new(self, admission))
    }

    pub fn manifest_index_layout<'a>(
        &'a mut self,
        rule: &AdmittedManifestIndexLayoutRule,
    ) -> Result<AdmittedManifestLayoutFamily<'a>, PlatformPhysicalFacadeDenial> {
        let admission = ManifestLayoutFamilyHome::s8().admit(rule)?;
        Ok(AdmittedManifestLayoutFamily::new(self, admission))
    }

    pub fn allocation_layout<'a>(
        &'a mut self,
        rule: &AdmittedAllocationLayoutRule,
    ) -> Result<AdmittedAllocationLayoutFamily<'a>, PlatformPhysicalFacadeDenial> {
        let admission = AllocationLayoutFamilyHome::s8().admit(rule)?;
        Ok(AdmittedAllocationLayoutFamily::new(self, admission))
    }

    pub fn free_space_layout<'a>(
        &'a mut self,
        rule: &AdmittedFreeSpaceLayoutRule,
    ) -> Result<AdmittedFreeSpaceLayoutFamily<'a>, PlatformPhysicalFacadeDenial> {
        let admission = FreeSpaceLayoutFamilyHome::s8().admit(rule)?;
        Ok(AdmittedFreeSpaceLayoutFamily::new(self, admission))
    }

    pub fn fragmentation_layout<'a>(
        &'a mut self,
        rule: &AdmittedFragmentationLayoutRule,
    ) -> Result<AdmittedFragmentationLayoutFamily<'a>, PlatformPhysicalFacadeDenial> {
        let admission = FragmentationLayoutFamilyHome::s8().admit(rule)?;
        Ok(AdmittedFragmentationLayoutFamily::new(self, admission))
    }

    pub fn publish_physical_root(
        &mut self,
    ) -> Result<PlatformPhysicalRootPublicationReport, PlatformPhysicalFacadeDenial> {
        let published = root_publication::encode_next_root_publication(
            &mut self.next_root_generation,
            &self.storage,
            &self.headers,
            self.references,
        )?;
        self.storage.replace_manifest_bytes(
            Some(published.root.root_publication()),
            vec![published.root_manifest],
            published.segment_manifest,
            published.extent_manifest,
            published.free_space_map,
        );
        self.counters = self
            .counters
            .with_root_publication()
            .with_flush()
            .with_rename();
        Ok(PlatformPhysicalRootPublicationReport::new(
            self.headers.clone(),
            self.storage.persisted_layout(),
            self.counters,
        ))
    }

    pub fn publish_interrupted_physical_root(
        &mut self,
    ) -> Result<PlatformPhysicalRootPublicationReport, PlatformPhysicalFacadeDenial> {
        let first = root_publication::encode_next_root_publication(
            &mut self.next_root_generation,
            &self.storage,
            &self.headers,
            self.references,
        )?;
        let second = root_publication::encode_next_root_publication(
            &mut self.next_root_generation,
            &self.storage,
            &self.headers,
            self.references,
        )?;
        self.storage.replace_manifest_bytes(
            None,
            vec![first.root_manifest, second.root_manifest],
            first.segment_manifest,
            first.extent_manifest,
            first.free_space_map,
        );
        self.counters = self.counters.with_root_publication().with_flush();
        Err(PlatformPhysicalFacadeDenial::new(
            PlatformPhysicalFacadeDenialKind::AmbiguousRootPublication,
        ))
    }

    pub fn scan_physical_layout(
        &mut self,
    ) -> Result<PlatformPhysicalScanReport, PlatformPhysicalFacadeDenial> {
        let verifier_report = scan::verify_persisted_layout_for_scan(&self.storage, &self.headers)?;
        let runtime_report = scan::collect_runtime_layout_observation(&self.storage);
        self.counters = self.counters.with_scan();
        Ok(scan::construct_scan_report(
            runtime_report,
            verifier_report,
            self.counters,
            self.scope,
        ))
    }

    /// Reject a proposed whole-store access before the scan/verifier path is
    /// entered. The returned receipt is runtime evidence, not a caller-made
    /// label over unrelated execution counters.
    pub fn reject_hidden_broad_scan(
        &mut self,
        request: PlatformPhysicalLayoutAccessRequest,
    ) -> PlatformPhysicalHiddenScanDenialReceipt {
        debug_assert!(matches!(
            request.intent(),
            PlatformPhysicalLayoutAccessIntent::HiddenBroadScan
        ));
        let counters_before = self.counters;
        self.counters = self.counters.with_full_store_materialization_rejection();
        PlatformPhysicalHiddenScanDenialReceipt::from_rejected_request(
            request,
            counters_before,
            self.counters,
        )
    }

    pub fn execute_explicit_degraded_exact_scan(
        &mut self,
        request: PlatformPhysicalLayoutAccessRequest,
    ) -> Result<PlatformPhysicalDegradedExactScanReceipt, PlatformPhysicalFacadeDenial> {
        if !matches!(
            request.intent(),
            PlatformPhysicalLayoutAccessIntent::ExplicitDegradedExactScan
        ) || request.budget_rows() == 0
        {
            return Err(PlatformPhysicalFacadeDenial::new(
                PlatformPhysicalFacadeDenialKind::OfflineVerifierDenied,
            ));
        }
        let scan = self.scan_physical_layout()?;
        let observed_rows = scan.runtime_report().discovered_references().len() as u64;
        if observed_rows > request.budget_rows() {
            return Err(PlatformPhysicalFacadeDenial::new(
                PlatformPhysicalFacadeDenialKind::OfflineVerifierDenied,
            ));
        }
        Ok(PlatformPhysicalDegradedExactScanReceipt::new(
            request,
            observed_rows,
            scan.counters(),
        ))
    }

    pub const fn counters(&self) -> PlatformPhysicalFacadeCounterSnapshot {
        self.counters
    }

    pub(crate) const fn storage_ref(&self) -> &PlatformPhysicalFacadeStorage {
        &self.storage
    }

    pub(crate) const fn headers_ref(&self) -> &PhysicalHeaderAuthority {
        &self.headers
    }

    pub(crate) const fn page_records_ref(&self) -> &PhysicalPageRecordAuthority {
        &self.page_records
    }

    pub(crate) const fn extent_records_ref(&self) -> &crate::PhysicalExtentRecordAuthority {
        &self.extent_records
    }

    pub(crate) fn new(
        scope: RoadmapScope,
        headers: PhysicalHeaderAuthority,
        storage: PlatformPhysicalFacadeStorage,
        counters: PlatformPhysicalFacadeCounterSnapshot,
    ) -> Self {
        Self {
            scope,
            page_records: PhysicalPageRecordAuthority::s1(headers.clone()),
            extent_records: crate::PhysicalExtentRecordAuthority::s1(headers.clone()),
            references: PhysicalReferenceAuthority::s1(),
            headers,
            storage,
            counters,
            next_root_generation: 1,
        }
    }

    pub(crate) fn ensure_admitted_reference(
        &self,
        reference: PhysicalReference,
    ) -> Result<(), PlatformPhysicalFacadeDenial> {
        if self.storage.has_admitted_reference(reference) {
            Ok(())
        } else {
            Err(PlatformPhysicalFacadeDenial::new(
                PlatformPhysicalFacadeDenialKind::MissingPhysicalRecord,
            ))
        }
    }

    pub(crate) fn mark_locate(&mut self) -> PlatformPhysicalFacadeCounterSnapshot {
        self.counters = self.counters.with_locate();
        self.counters
    }

    pub(crate) fn mark_read(&mut self) -> PlatformPhysicalFacadeCounterSnapshot {
        self.counters = self.counters.with_read();
        self.counters
    }
}

pub(crate) fn map_verifier_denial_for_reopen(
    denial: crate::OfflineVerifierDenial,
) -> PlatformPhysicalFacadeDenial {
    let kind = match denial.kind() {
        crate::OfflineVerifierDenialKind::MissingRootManifest => {
            PlatformPhysicalFacadeDenialKind::MissingPhysicalRoot
        }
        crate::OfflineVerifierDenialKind::AmbiguousRootManifest => {
            PlatformPhysicalFacadeDenialKind::AmbiguousRootPublication
        }
        _ => PlatformPhysicalFacadeDenialKind::OfflineVerifierDenied,
    };
    PlatformPhysicalFacadeDenial::new(kind).with_verifier_denial(denial)
}

pub use counters::PlatformPhysicalFacadeCounterSnapshot;
pub use denials::{PlatformPhysicalFacadeDenial, PlatformPhysicalFacadeDenialKind};
pub use evidence::PlatformPhysicalFacadeEvidence;
pub use replay_artifact::PlatformPhysicalReplayArtifact;
pub use reports::{
    PlatformPhysicalAppendReport, PlatformPhysicalFramedRecord, PlatformPhysicalLocateReport,
    PlatformPhysicalRootPublicationReport, PlatformPhysicalRuntimeLayoutReport,
    PlatformPhysicalScanReport,
};
pub use requests::{
    PlatformPhysicalAppendRequest, PlatformPhysicalOpenRequest, PlatformPhysicalRecordTarget,
};
pub use runtime_receipt::{
    PlatformPhysicalDegradedExactScanReceipt, PlatformPhysicalHiddenScanDenialReceipt,
    PlatformPhysicalLayoutAccessIntent, PlatformPhysicalLayoutAccessRequest,
    PlatformPhysicalRuntimeOperation, PlatformPhysicalRuntimeReceipt,
    PlatformPhysicalRuntimeReceiptDenial, PlatformPhysicalRuntimeStrategy,
};
