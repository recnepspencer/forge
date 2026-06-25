use crate::facade_root_publication::encode_root_publication;
use crate::facade_storage::PlatformPhysicalFacadeStorage;
use crate::{
    ManifestDiscoveryAuthority, MinimalManifestVerifierReport, OfflinePhysicalVerifier,
    OfflineVerifierDenialKind, PersistedPhysicalLayout, PhysicalGeneration,
    PhysicalHeaderAuthority, PhysicalPageRecordAuthority, PhysicalReference,
    PhysicalReferenceAuthority, PhysicalReferenceKind, PhysicalShortcutBoundaryDenial,
    PlatformPhysicalAppendReport, PlatformPhysicalAppendRequest,
    PlatformPhysicalFacadeCounterSnapshot, PlatformPhysicalFacadeDenial,
    PlatformPhysicalFacadeDenialKind, PlatformPhysicalLocateReport, PlatformPhysicalOpenRequest,
    PlatformPhysicalRootPublicationReport, PlatformPhysicalRuntimeLayoutReport,
    PlatformPhysicalScanReport,
};
use forge_store_contracts::{AcceptedHandoffReadiness, RoadmapScope};

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
        readiness.physical_authority_scope().map_err(|_| {
            PlatformPhysicalFacadeDenial::new(
                PlatformPhysicalFacadeDenialKind::HandoffReadinessRejected,
            )
        })?;
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
        layout: PersistedPhysicalLayout,
    ) -> Result<Self, PlatformPhysicalFacadeDenial> {
        readiness.physical_authority_scope().map_err(|_| {
            PlatformPhysicalFacadeDenial::new(
                PlatformPhysicalFacadeDenialKind::HandoffReadinessRejected,
            )
        })?;
        let verifier = OfflinePhysicalVerifier::s1(request.headers().clone());
        let verifier_report = verifier
            .verify(&layout)
            .map_err(map_verifier_denial_for_reopen)?;
        let storage = PlatformPhysicalFacadeStorage::from_persisted_layout(
            &layout,
            verifier_report.layout().discovered_references().to_vec(),
        );
        Ok(Self::new(
            readiness.scope(),
            request.headers().clone(),
            storage,
            PlatformPhysicalFacadeCounterSnapshot::empty()
                .with_open()
                .with_reopen(),
        ))
    }

    pub fn append_physical_record(
        &mut self,
        request: PlatformPhysicalAppendRequest<'_>,
    ) -> Result<PlatformPhysicalAppendReport, PlatformPhysicalFacadeDenial> {
        let append = crate::facade_append::append_physical_record(
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
        match reference.kind() {
            PhysicalReferenceKind::PageSlot => crate::facade_locate::locate_page_slot(
                &self.storage,
                &self.page_records,
                self.references,
                self.counters,
                reference,
            ),
            PhysicalReferenceKind::ExtentBacked => crate::facade_locate::locate_extent(
                &self.storage,
                &self.extent_records,
                self.references,
                self.counters,
                reference,
            ),
            _ => Err(PlatformPhysicalFacadeDenial::new(
                PlatformPhysicalFacadeDenialKind::MissingPhysicalRecord,
            )),
        }
    }

    pub fn read_physical_record(
        &mut self,
        reference: PhysicalReference,
    ) -> Result<PlatformPhysicalLocateReport<'_>, PlatformPhysicalFacadeDenial> {
        self.reject_unadmitted_reference(reference)?;
        self.counters = self.counters.with_read();
        match reference.kind() {
            PhysicalReferenceKind::PageSlot => crate::facade_locate::locate_page_slot(
                &self.storage,
                &self.page_records,
                self.references,
                self.counters,
                reference,
            ),
            PhysicalReferenceKind::ExtentBacked => crate::facade_locate::locate_extent(
                &self.storage,
                &self.extent_records,
                self.references,
                self.counters,
                reference,
            ),
            _ => Err(PlatformPhysicalFacadeDenial::new(
                PlatformPhysicalFacadeDenialKind::MissingPhysicalRecord,
            )),
        }
    }

    pub fn publish_physical_root(
        &mut self,
    ) -> Result<PlatformPhysicalRootPublicationReport, PlatformPhysicalFacadeDenial> {
        let published = self.encode_next_root_publication()?;
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
            self.storage.persisted_layout(),
            self.counters,
        ))
    }

    pub fn publish_interrupted_physical_root(
        &mut self,
    ) -> Result<PlatformPhysicalRootPublicationReport, PlatformPhysicalFacadeDenial> {
        let first = self.encode_next_root_publication()?;
        let second = self.encode_next_root_publication()?;
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
        let verifier_report = self.verify_persisted_layout_for_scan()?;
        let runtime_report = self.runtime_layout_report();
        self.counters = self.counters.with_scan();
        Ok(PlatformPhysicalScanReport::new(
            runtime_report,
            verifier_report,
            self.counters,
            self.scope,
        ))
    }

    pub fn reject_full_store_heap_materialization(
        &mut self,
    ) -> Result<(), PlatformPhysicalFacadeDenial> {
        self.counters = self.counters.with_full_store_materialization_rejection();
        Err(PlatformPhysicalFacadeDenial::new(
            PlatformPhysicalFacadeDenialKind::FullStoreMaterializationRejected,
        )
        .with_shortcut_denial(PhysicalShortcutBoundaryDenial::full_store_heap_materialization()))
    }

    pub fn reject_backend_residue_guess(&mut self) -> Result<(), PlatformPhysicalFacadeDenial> {
        self.counters = self.counters.with_backend_residue_guess_rejection();
        Err(PlatformPhysicalFacadeDenial::new(
            PlatformPhysicalFacadeDenialKind::BackendResidueGuessRejected,
        )
        .with_shortcut_denial(PhysicalShortcutBoundaryDenial::backend_residue_guessing()))
    }

    pub fn reject_live_runtime_cache_shortcut(
        &mut self,
    ) -> Result<(), PlatformPhysicalFacadeDenial> {
        Err(PlatformPhysicalFacadeDenial::new(
            PlatformPhysicalFacadeDenialKind::ShortcutBoundaryRejected,
        )
        .with_shortcut_denial(PhysicalShortcutBoundaryDenial::live_runtime_cache()))
    }

    pub fn reject_backend_private_map_shortcut(
        &mut self,
    ) -> Result<(), PlatformPhysicalFacadeDenial> {
        Err(PlatformPhysicalFacadeDenial::new(
            PlatformPhysicalFacadeDenialKind::ShortcutBoundaryRejected,
        )
        .with_shortcut_denial(PhysicalShortcutBoundaryDenial::backend_private_map()))
    }

    pub fn reject_raw_debug_dump_shortcut(&mut self) -> Result<(), PlatformPhysicalFacadeDenial> {
        Err(PlatformPhysicalFacadeDenial::new(
            PlatformPhysicalFacadeDenialKind::ShortcutBoundaryRejected,
        )
        .with_shortcut_denial(PhysicalShortcutBoundaryDenial::raw_debug_dump()))
    }

    pub const fn counters(&self) -> PlatformPhysicalFacadeCounterSnapshot {
        self.counters
    }

    fn new(
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

    fn encode_next_root_publication(
        &mut self,
    ) -> Result<crate::facade_root_publication::EncodedRootPublication, PlatformPhysicalFacadeDenial>
    {
        let generation = PhysicalGeneration::from_raw(self.next_root_generation).map_err(|_| {
            PlatformPhysicalFacadeDenial::new(PlatformPhysicalFacadeDenialKind::MissingPhysicalRoot)
        })?;
        self.next_root_generation += 1;
        let published =
            encode_root_publication(&self.storage, generation, self.headers.byte_order());
        ManifestDiscoveryAuthority::s1()
            .reopen_from_root(
                &published.root,
                self.references
                    .admit_root_publication(published.root.root_publication()),
            )
            .map_err(|denial| {
                PlatformPhysicalFacadeDenial::new(
                    PlatformPhysicalFacadeDenialKind::ManifestDiscoveryDenied,
                )
                .with_manifest_denial(denial)
            })?;
        Ok(published)
    }

    fn verify_persisted_layout_for_scan(
        &self,
    ) -> Result<MinimalManifestVerifierReport, PlatformPhysicalFacadeDenial> {
        OfflinePhysicalVerifier::s1(self.headers.clone())
            .verify(&self.storage.persisted_layout())
            .map_err(map_verifier_denial_for_reopen)
    }

    fn runtime_layout_report(&self) -> PlatformPhysicalRuntimeLayoutReport {
        PlatformPhysicalRuntimeLayoutReport::new(
            self.storage.runtime_discovered_references(),
            self.storage.runtime_traversal_report(),
        )
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

fn map_verifier_denial_for_reopen(
    denial: crate::OfflineVerifierDenial,
) -> PlatformPhysicalFacadeDenial {
    let kind = match denial.kind() {
        OfflineVerifierDenialKind::MissingRootManifest => {
            PlatformPhysicalFacadeDenialKind::MissingPhysicalRoot
        }
        OfflineVerifierDenialKind::AmbiguousRootManifest => {
            PlatformPhysicalFacadeDenialKind::AmbiguousRootPublication
        }
        _ => PlatformPhysicalFacadeDenialKind::OfflineVerifierDenied,
    };
    PlatformPhysicalFacadeDenial::new(kind).with_verifier_denial(denial)
}
