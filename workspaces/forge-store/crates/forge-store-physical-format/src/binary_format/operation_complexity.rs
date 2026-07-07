#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalOperationKind {
    HeaderDecode,
    PhysicalReferenceValidation,
    LocateByReference,
    ManifestLookup,
    RootManifestOpen,
    AppendRecordPlacement,
    ManifestTraversal,
    OfflineVerifierWalk,
}

impl PhysicalOperationKind {
    pub const fn s1_required() -> [Self; 8] {
        [
            Self::HeaderDecode,
            Self::PhysicalReferenceValidation,
            Self::LocateByReference,
            Self::ManifestLookup,
            Self::RootManifestOpen,
            Self::AppendRecordPlacement,
            Self::ManifestTraversal,
            Self::OfflineVerifierWalk,
        ]
    }

    pub const fn contract_name(self) -> &'static str {
        match self {
            Self::HeaderDecode => "header_decode",
            Self::PhysicalReferenceValidation => "physical_reference_validation",
            Self::LocateByReference => "page_slot_locate",
            Self::ManifestLookup => "manifest_lookup",
            Self::RootManifestOpen => "root_manifest_open",
            Self::AppendRecordPlacement => "append_record_placement",
            Self::ManifestTraversal => "manifest_traversal",
            Self::OfflineVerifierWalk => "offline_verifier_walk",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalLocalityClass {
    Constant,
    PageLocal,
    SegmentLocal,
    ExtentLocal,
    RootManifest,
    FreeSpaceClass,
    ManifestDeclaredTraversal,
    FullScan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalComplexityStatus {
    Declared,
    Verified,
    Debt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalOperationEvidenceRequirement {
    CounterReceipt,
    AlgorithmReview,
    HostileFixture,
    ScaleProperty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalOperationComplexityContract {
    operation: PhysicalOperationKind,
    locality: PhysicalLocalityClass,
    status: PhysicalComplexityStatus,
    asymptotic_bound: &'static str,
    requirements: [PhysicalOperationEvidenceRequirement; 4],
}

impl PhysicalOperationComplexityContract {
    pub const fn s1_required(operation: PhysicalOperationKind) -> Self {
        Self {
            operation,
            locality: s1_locality(operation),
            status: PhysicalComplexityStatus::Declared,
            asymptotic_bound: s1_bound(operation),
            requirements: [
                PhysicalOperationEvidenceRequirement::CounterReceipt,
                PhysicalOperationEvidenceRequirement::AlgorithmReview,
                PhysicalOperationEvidenceRequirement::HostileFixture,
                PhysicalOperationEvidenceRequirement::ScaleProperty,
            ],
        }
    }

    pub const fn debt_for_tests(operation: PhysicalOperationKind) -> Self {
        Self {
            operation,
            locality: s1_locality(operation),
            status: PhysicalComplexityStatus::Debt,
            asymptotic_bound: s1_bound(operation),
            requirements: [
                PhysicalOperationEvidenceRequirement::CounterReceipt,
                PhysicalOperationEvidenceRequirement::AlgorithmReview,
                PhysicalOperationEvidenceRequirement::HostileFixture,
                PhysicalOperationEvidenceRequirement::ScaleProperty,
            ],
        }
    }

    pub const fn operation(self) -> PhysicalOperationKind {
        self.operation
    }

    pub const fn locality(self) -> PhysicalLocalityClass {
        self.locality
    }

    pub const fn status(self) -> PhysicalComplexityStatus {
        self.status
    }

    pub const fn asymptotic_bound(self) -> &'static str {
        self.asymptotic_bound
    }

    pub const fn requirements(self) -> [PhysicalOperationEvidenceRequirement; 4] {
        self.requirements
    }

    pub const fn is_s1_declared(self) -> bool {
        matches!(self.status, PhysicalComplexityStatus::Declared)
    }
}

const fn s1_locality(operation: PhysicalOperationKind) -> PhysicalLocalityClass {
    match operation {
        PhysicalOperationKind::HeaderDecode
        | PhysicalOperationKind::PhysicalReferenceValidation => PhysicalLocalityClass::Constant,
        PhysicalOperationKind::LocateByReference => PhysicalLocalityClass::PageLocal,
        PhysicalOperationKind::ManifestLookup => PhysicalLocalityClass::SegmentLocal,
        PhysicalOperationKind::RootManifestOpen => PhysicalLocalityClass::RootManifest,
        PhysicalOperationKind::AppendRecordPlacement => PhysicalLocalityClass::FreeSpaceClass,
        PhysicalOperationKind::ManifestTraversal => {
            PhysicalLocalityClass::ManifestDeclaredTraversal
        }
        PhysicalOperationKind::OfflineVerifierWalk => {
            PhysicalLocalityClass::ManifestDeclaredTraversal
        }
    }
}

const fn s1_bound(operation: PhysicalOperationKind) -> &'static str {
    match operation {
        PhysicalOperationKind::HeaderDecode => "O(1)",
        PhysicalOperationKind::PhysicalReferenceValidation => "O(1)",
        PhysicalOperationKind::LocateByReference => "O(1)",
        PhysicalOperationKind::ManifestLookup => {
            "O(log manifest_entries) or O(1) with admitted index"
        }
        PhysicalOperationKind::RootManifestOpen => "O(root_entries)",
        PhysicalOperationKind::AppendRecordPlacement => {
            "O(candidate_free_space_classes + admitted_candidate_scan_bound)"
        }
        PhysicalOperationKind::ManifestTraversal => {
            "O(root_entries + segment_entries + extent_entries)"
        }
        PhysicalOperationKind::OfflineVerifierWalk => {
            "O(root_entries + segment_entries + extent_entries + pages_walked + extents_walked)"
        }
    }
}
