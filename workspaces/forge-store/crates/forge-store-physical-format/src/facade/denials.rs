use crate::{
    ExtentRecordDenial, ManifestDiscoveryDenial, OfflineVerifierDenial, PageRecordDenial,
    PhysicalHeaderDecodeDenial, PhysicalReferenceValidationDenial, PhysicalShortcutBoundaryDenial,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformPhysicalFacadeDenialKind {
    HandoffReadinessRejected,
    StoreIdentityMismatch,
    MissingPhysicalRecord,
    MissingPhysicalRoot,
    AmbiguousRootPublication,
    PageRecordDenied,
    ExtentRecordDenied,
    HeaderDecodeDenied,
    ManifestDiscoveryDenied,
    OfflineVerifierDenied,
    ReferenceValidationDenied,
    FullStoreMaterializationRejected,
    BackendResidueGuessRejected,
    ShortcutBoundaryRejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformPhysicalFacadeDenial {
    kind: PlatformPhysicalFacadeDenialKind,
    page_denial: Option<PageRecordDenial>,
    extent_denial: Option<ExtentRecordDenial>,
    header_denial: Option<PhysicalHeaderDecodeDenial>,
    manifest_denial: Option<ManifestDiscoveryDenial>,
    verifier_denial: Option<OfflineVerifierDenial>,
    reference_denial: Option<PhysicalReferenceValidationDenial>,
    shortcut_denial: Option<PhysicalShortcutBoundaryDenial>,
}

impl PlatformPhysicalFacadeDenial {
    pub const fn new(kind: PlatformPhysicalFacadeDenialKind) -> Self {
        Self {
            kind,
            page_denial: None,
            extent_denial: None,
            header_denial: None,
            manifest_denial: None,
            verifier_denial: None,
            reference_denial: None,
            shortcut_denial: None,
        }
    }

    pub const fn kind(&self) -> PlatformPhysicalFacadeDenialKind {
        self.kind
    }

    pub fn with_page_denial(mut self, denial: PageRecordDenial) -> Self {
        self.page_denial = Some(denial);
        self
    }

    pub fn with_extent_denial(mut self, denial: ExtentRecordDenial) -> Self {
        self.extent_denial = Some(denial);
        self
    }

    pub fn with_header_denial(mut self, denial: PhysicalHeaderDecodeDenial) -> Self {
        self.header_denial = Some(denial);
        self
    }

    pub fn with_manifest_denial(mut self, denial: ManifestDiscoveryDenial) -> Self {
        self.manifest_denial = Some(denial);
        self
    }

    pub fn with_verifier_denial(mut self, denial: OfflineVerifierDenial) -> Self {
        self.verifier_denial = Some(denial);
        self
    }

    pub fn with_reference_denial(mut self, denial: PhysicalReferenceValidationDenial) -> Self {
        self.reference_denial = Some(denial);
        self
    }

    pub(crate) fn with_shortcut_denial(mut self, denial: PhysicalShortcutBoundaryDenial) -> Self {
        self.shortcut_denial = Some(denial);
        self
    }

    pub const fn page_denial(&self) -> Option<&PageRecordDenial> {
        self.page_denial.as_ref()
    }

    pub const fn extent_denial(&self) -> Option<&ExtentRecordDenial> {
        self.extent_denial.as_ref()
    }

    pub const fn verifier_denial(&self) -> Option<&OfflineVerifierDenial> {
        self.verifier_denial.as_ref()
    }

    pub const fn shortcut_denial(&self) -> Option<PhysicalShortcutBoundaryDenial> {
        self.shortcut_denial
    }
}
