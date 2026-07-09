use crate::{
    ExtentBackedRecordView, ManifestTraversalReport, MinimalManifestVerifierReport,
    PersistedPhysicalLayout, PhysicalBootstrapCatalogDenial, PhysicalBootstrapCatalogOpenWitness,
    PhysicalHeaderAuthority, PhysicalReference, PlatformPhysicalFacadeCounterSnapshot,
    PlatformPhysicalFacadeEvidence, RecordAppendReport, RecordLocateReport,
};
use worth_store_contracts::RoadmapScope;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformPhysicalAppendReport {
    reference: PhysicalReference,
    counters: PlatformPhysicalFacadeCounterSnapshot,
}

impl PlatformPhysicalAppendReport {
    pub(crate) const fn new(
        reference: PhysicalReference,
        counters: PlatformPhysicalFacadeCounterSnapshot,
    ) -> Self {
        Self {
            reference,
            counters,
        }
    }

    pub const fn reference(self) -> PhysicalReference {
        self.reference
    }

    pub const fn counters(self) -> PlatformPhysicalFacadeCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformPhysicalFramedRecord<'a> {
    PageSlot(crate::FramedRecordView<'a>),
    Extent(ExtentBackedRecordView<'a>),
}

impl<'a> PlatformPhysicalFramedRecord<'a> {
    pub const fn payload(self) -> crate::FramedRecordPayload<'a> {
        match self {
            Self::PageSlot(view) => view.payload(),
            Self::Extent(view) => view.payload(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformPhysicalLocateReport<'a> {
    reference: PhysicalReference,
    framed_record: PlatformPhysicalFramedRecord<'a>,
    counters: PlatformPhysicalFacadeCounterSnapshot,
}

impl<'a> PlatformPhysicalLocateReport<'a> {
    pub(crate) const fn new(
        reference: PhysicalReference,
        framed_record: PlatformPhysicalFramedRecord<'a>,
        counters: PlatformPhysicalFacadeCounterSnapshot,
    ) -> Self {
        Self {
            reference,
            framed_record,
            counters,
        }
    }

    pub const fn reference(self) -> PhysicalReference {
        self.reference
    }

    pub const fn framed_record(self) -> PlatformPhysicalFramedRecord<'a> {
        self.framed_record
    }

    pub const fn counters(self) -> PlatformPhysicalFacadeCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformPhysicalRootPublicationReport {
    headers: PhysicalHeaderAuthority,
    layout: PersistedPhysicalLayout,
    counters: PlatformPhysicalFacadeCounterSnapshot,
}

impl PlatformPhysicalRootPublicationReport {
    pub(crate) fn new(
        headers: PhysicalHeaderAuthority,
        layout: PersistedPhysicalLayout,
        counters: PlatformPhysicalFacadeCounterSnapshot,
    ) -> Self {
        Self {
            headers,
            layout,
            counters,
        }
    }

    pub const fn persisted_layout(&self) -> &PersistedPhysicalLayout {
        &self.layout
    }

    pub fn replay_artifact(&self) -> super::PlatformPhysicalReplayArtifact {
        super::PlatformPhysicalReplayArtifact::from_persisted_layout(
            self.headers.clone(),
            self.layout.clone(),
        )
    }

    pub fn admit_bootstrap_open_witness(
        &self,
    ) -> Result<PhysicalBootstrapCatalogOpenWitness, PhysicalBootstrapCatalogDenial> {
        PhysicalBootstrapCatalogOpenWitness::admit_persisted_layout(&self.headers, &self.layout)
    }

    pub const fn counters(&self) -> PlatformPhysicalFacadeCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformPhysicalRuntimeLayoutReport {
    discovered_references: Vec<PhysicalReference>,
    traversal: ManifestTraversalReport,
    semantic_decode_attempts: u32,
}

impl PlatformPhysicalRuntimeLayoutReport {
    pub(crate) const fn new(
        discovered_references: Vec<PhysicalReference>,
        traversal: ManifestTraversalReport,
    ) -> Self {
        Self {
            discovered_references,
            traversal,
            semantic_decode_attempts: 0,
        }
    }

    pub fn discovered_references(&self) -> &[PhysicalReference] {
        &self.discovered_references
    }

    pub const fn traversal(&self) -> &ManifestTraversalReport {
        &self.traversal
    }

    pub const fn semantic_decode_attempts(&self) -> u32 {
        self.semantic_decode_attempts
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformPhysicalScanReport {
    runtime_report: PlatformPhysicalRuntimeLayoutReport,
    verifier_report: MinimalManifestVerifierReport,
    counters: PlatformPhysicalFacadeCounterSnapshot,
    scope: RoadmapScope,
}

impl PlatformPhysicalScanReport {
    pub(crate) const fn new(
        runtime_report: PlatformPhysicalRuntimeLayoutReport,
        verifier_report: MinimalManifestVerifierReport,
        counters: PlatformPhysicalFacadeCounterSnapshot,
        scope: RoadmapScope,
    ) -> Self {
        Self {
            runtime_report,
            verifier_report,
            counters,
            scope,
        }
    }

    pub const fn runtime_report(&self) -> &PlatformPhysicalRuntimeLayoutReport {
        &self.runtime_report
    }

    pub const fn verifier_report(&self) -> &MinimalManifestVerifierReport {
        &self.verifier_report
    }

    pub const fn counters(&self) -> PlatformPhysicalFacadeCounterSnapshot {
        self.counters
    }

    pub fn platform_evidence(&self) -> PlatformPhysicalFacadeEvidence {
        PlatformPhysicalFacadeEvidence::from_verifier_report(
            self.scope,
            self.counters,
            &self.verifier_report,
        )
    }
}

impl From<RecordAppendReport> for PlatformPhysicalAppendReport {
    fn from(report: RecordAppendReport) -> Self {
        Self::new(
            report.reference(),
            PlatformPhysicalFacadeCounterSnapshot::empty().with_append(),
        )
    }
}

impl<'a> From<RecordLocateReport<'a>> for PlatformPhysicalFramedRecord<'a> {
    fn from(report: RecordLocateReport<'a>) -> Self {
        Self::PageSlot(report.record_view())
    }
}
