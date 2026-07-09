mod append;
mod counters;
mod denials;
mod evidence;
mod locate;
mod replay_artifact;
mod reopen;
mod reports;
mod requests;
mod root_publication;
mod scan;
mod shortcut_rejection;
mod storage;
#[cfg(test)]
mod tests;

use crate::{
    PhysicalHeaderAuthority, PhysicalPageRecordAuthority, PhysicalReference,
    PhysicalReferenceAuthority,
};
use worth_store_contracts::{AcceptedHandoffReadiness, RoadmapScope};
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

    pub fn locate_physical_record(
        &mut self,
        reference: PhysicalReference,
    ) -> Result<PlatformPhysicalLocateReport<'_>, PlatformPhysicalFacadeDenial> {
        self.reject_unadmitted_reference(reference)?;
        self.counters = self.counters.with_locate();
        locate::classify_and_locate_record(
            &self.storage,
            &self.page_records,
            &self.extent_records,
            self.references,
            self.counters,
            reference,
        )
    }

    pub fn read_physical_record(
        &mut self,
        reference: PhysicalReference,
    ) -> Result<PlatformPhysicalLocateReport<'_>, PlatformPhysicalFacadeDenial> {
        self.reject_unadmitted_reference(reference)?;
        self.counters = self.counters.with_read();
        locate::classify_and_locate_record(
            &self.storage,
            &self.page_records,
            &self.extent_records,
            self.references,
            self.counters,
            reference,
        )
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

    pub const fn counters(&self) -> PlatformPhysicalFacadeCounterSnapshot {
        self.counters
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

    fn reject_unadmitted_reference(
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
