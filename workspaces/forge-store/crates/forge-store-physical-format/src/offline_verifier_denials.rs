use crate::{
    ExtentRecordDenial, ManifestDiscoveryDenial, OfflineVerifierCounterSnapshot, PageRecordDenial,
    PhysicalHeaderDecodeDenial, PhysicalVocabularyError,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfflineVerifierDenialKind {
    MissingRootManifest,
    AmbiguousRootManifest,
    MalformedRootManifest,
    MalformedSegmentManifest,
    MalformedExtentManifest,
    MalformedFreeSpaceMap,
    MalformedManifestMembership,
    MissingPersistedPage,
    MissingPersistedExtent,
    HeaderDecodeDenied,
    PageRecordDenied,
    ExtentRecordDenied,
    ManifestDiscoveryDenied,
    BackendResidueDiscoverySource,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfflineVerifierDenial {
    kind: OfflineVerifierDenialKind,
    counters: OfflineVerifierCounterSnapshot,
    vocabulary_error: Option<PhysicalVocabularyError>,
    header_denial: Option<PhysicalHeaderDecodeDenial>,
    manifest_denial: Option<ManifestDiscoveryDenial>,
    page_denial: Option<PageRecordDenial>,
    extent_denial: Option<ExtentRecordDenial>,
}

impl OfflineVerifierDenial {
    pub const fn new(
        kind: OfflineVerifierDenialKind,
        counters: OfflineVerifierCounterSnapshot,
    ) -> Self {
        Self {
            kind,
            counters,
            vocabulary_error: None,
            header_denial: None,
            manifest_denial: None,
            page_denial: None,
            extent_denial: None,
        }
    }

    pub const fn with_vocabulary_error(mut self, error: PhysicalVocabularyError) -> Self {
        self.vocabulary_error = Some(error);
        self
    }

    pub const fn with_header_denial(mut self, denial: PhysicalHeaderDecodeDenial) -> Self {
        self.header_denial = Some(denial);
        self
    }

    pub const fn with_manifest_denial(mut self, denial: ManifestDiscoveryDenial) -> Self {
        self.manifest_denial = Some(denial);
        self
    }

    pub const fn with_page_denial(mut self, denial: PageRecordDenial) -> Self {
        self.page_denial = Some(denial);
        self
    }

    pub const fn with_extent_denial(mut self, denial: ExtentRecordDenial) -> Self {
        self.extent_denial = Some(denial);
        self
    }

    pub const fn kind(&self) -> OfflineVerifierDenialKind {
        self.kind
    }

    pub const fn counters(&self) -> OfflineVerifierCounterSnapshot {
        self.counters
    }

    pub const fn vocabulary_error(&self) -> Option<PhysicalVocabularyError> {
        self.vocabulary_error
    }

    pub const fn header_denial(&self) -> Option<&PhysicalHeaderDecodeDenial> {
        self.header_denial.as_ref()
    }

    pub const fn manifest_denial(&self) -> Option<&ManifestDiscoveryDenial> {
        self.manifest_denial.as_ref()
    }

    pub const fn page_denial(&self) -> Option<&PageRecordDenial> {
        self.page_denial.as_ref()
    }

    pub const fn extent_denial(&self) -> Option<&ExtentRecordDenial> {
        self.extent_denial.as_ref()
    }
}
