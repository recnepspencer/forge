#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StoreCanonicalBasisFamily {
    AspectBoundaryFact,
    AspectPatchBoundaryFact,
    BoundaryReceiptEvidence,
    DiagnosticEvidence,
    PerformanceReceiptEvidence,
    ReadinessHandoff,
    S2EntryBoundaryEvidence,
    IntegrityCloseoutHandoff,
    PhysicalSourceManifest,
    PhysicalPageHeader,
    PhysicalPageRecord,
    PhysicalExtentRecord,
    PhysicalReference,
    PhysicalOfflineVerifierEvidence,
    PhysicalHeaderDecodeEvidence,
    PhysicalFormatEvidence,
    PhysicalManifestDiscoveryEvidence,
    PhysicalIdentityEvidence,
    PhysicalFoundationEvidence,
    PhysicalIntegrityChecksumCoverage,
    PhysicalIntegrityEvidence,
    PhysicalIntegrityQuarantineReceipt,
    PhysicalIntegrityScrubReceipt,
    PhysicalIntegrityCloseoutEvidence,
    WalFrameIntegrityEvidence,
    WalRecord,
    RecoveryIntegrityHandoff,
    RecoveryWalReplayReceipt,
    RecoveryCheckpointValidityReceipt,
    RecoveryVettedRecordReceipt,
    RecoveryPerformanceReport,
}

impl StoreCanonicalBasisFamily {
    pub const ALL: [Self; 31] = [
        Self::AspectBoundaryFact,
        Self::AspectPatchBoundaryFact,
        Self::BoundaryReceiptEvidence,
        Self::DiagnosticEvidence,
        Self::PerformanceReceiptEvidence,
        Self::ReadinessHandoff,
        Self::S2EntryBoundaryEvidence,
        Self::IntegrityCloseoutHandoff,
        Self::PhysicalSourceManifest,
        Self::PhysicalPageHeader,
        Self::PhysicalPageRecord,
        Self::PhysicalExtentRecord,
        Self::PhysicalReference,
        Self::PhysicalOfflineVerifierEvidence,
        Self::PhysicalHeaderDecodeEvidence,
        Self::PhysicalFormatEvidence,
        Self::PhysicalManifestDiscoveryEvidence,
        Self::PhysicalIdentityEvidence,
        Self::PhysicalFoundationEvidence,
        Self::PhysicalIntegrityChecksumCoverage,
        Self::PhysicalIntegrityEvidence,
        Self::PhysicalIntegrityQuarantineReceipt,
        Self::PhysicalIntegrityScrubReceipt,
        Self::PhysicalIntegrityCloseoutEvidence,
        Self::WalFrameIntegrityEvidence,
        Self::WalRecord,
        Self::RecoveryIntegrityHandoff,
        Self::RecoveryWalReplayReceipt,
        Self::RecoveryCheckpointValidityReceipt,
        Self::RecoveryVettedRecordReceipt,
        Self::RecoveryPerformanceReport,
    ];

    pub const fn canonical_basis_family_label(self) -> &'static str {
        match self {
            Self::AspectBoundaryFact => "aspect boundary fact",
            Self::AspectPatchBoundaryFact => "aspect patch boundary fact",
            Self::BoundaryReceiptEvidence => "boundary receipt evidence",
            Self::DiagnosticEvidence => "diagnostic evidence",
            Self::PerformanceReceiptEvidence => "performance receipt evidence",
            Self::ReadinessHandoff => "readiness handoff",
            Self::S2EntryBoundaryEvidence => "S2 entry boundary evidence",
            Self::IntegrityCloseoutHandoff => "integrity closeout handoff",
            Self::PhysicalSourceManifest => "physical source manifest",
            Self::PhysicalPageHeader => "physical page header",
            Self::PhysicalPageRecord => "physical page record",
            Self::PhysicalExtentRecord => "physical extent record",
            Self::PhysicalReference => "physical reference",
            Self::PhysicalOfflineVerifierEvidence => "physical offline verifier evidence",
            Self::PhysicalHeaderDecodeEvidence => "physical header decode evidence",
            Self::PhysicalFormatEvidence => "physical format evidence",
            Self::PhysicalManifestDiscoveryEvidence => "physical manifest discovery evidence",
            Self::PhysicalIdentityEvidence => "physical identity evidence",
            Self::PhysicalFoundationEvidence => "physical foundation evidence",
            Self::PhysicalIntegrityChecksumCoverage => "physical integrity checksum coverage",
            Self::PhysicalIntegrityEvidence => "physical integrity evidence",
            Self::PhysicalIntegrityQuarantineReceipt => "physical integrity quarantine receipt",
            Self::PhysicalIntegrityScrubReceipt => "physical integrity scrub receipt",
            Self::PhysicalIntegrityCloseoutEvidence => "physical integrity closeout evidence",
            Self::WalFrameIntegrityEvidence => "WAL frame integrity evidence",
            Self::WalRecord => "WAL record",
            Self::RecoveryIntegrityHandoff => "recovery integrity handoff",
            Self::RecoveryWalReplayReceipt => "recovery WAL replay receipt",
            Self::RecoveryCheckpointValidityReceipt => "recovery checkpoint validity receipt",
            Self::RecoveryVettedRecordReceipt => "recovery vetted record receipt",
            Self::RecoveryPerformanceReport => "recovery performance report",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StoreCanonicalBasisSourceKind {
    FoundationalAspectState,
    FoundationalAspectPatch,
    FoundationalReceipt,
    FoundationalDiagnostic,
    FoundationalPerformanceEvidence,
    StoreReadinessHandoff,
    StoreSourceManifest,
    StorePageHeader,
    StorePhysicalWitness,
    StorePhysicalFormatRecord,
    StorePhysicalIntegrityEvidence,
    StoreWalRecord,
    StoreRecoveryReceipt,
    StoreRecoveryPerformanceEvidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StoreCanonicalBasisFieldRole {
    NativeSourceManifest,
    NativePageHeader,
    NativePhysicalWitness,
    NativeReceipt,
    NativeDiagnostic,
    NativePerformanceReport,
    TerminalProjection,
    OperatorDisplay,
    DocumentChecksum,
    CompatibilityText,
    DigestText,
    RawJsonPayload,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StoreCanonicalBasisLane {
    AspectValueState,
    AspectPatch,
    Receipt,
    Diagnostic,
    PerformanceEvidence,
    PhysicalSourceManifest,
    PhysicalRecord,
    PhysicalIntegrity,
    Wal,
    Recovery,
    Handoff,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreCanonicalBasisSourceDenial {
    MissingSourceOwner {
        family: StoreCanonicalBasisFamily,
        classifying_subsystem: &'static str,
    },
    WrongNativeSourceKind {
        family: StoreCanonicalBasisFamily,
        source: StoreCanonicalBasisSourceKind,
    },
    ForbiddenFieldRole {
        field_role: StoreCanonicalBasisFieldRole,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreCanonicalBasisSourceOwner {
    family: StoreCanonicalBasisFamily,
    owner_crate: &'static str,
    classifying_subsystem: &'static str,
    allowed_sources: &'static [StoreCanonicalBasisSourceKind],
    denied_fields: &'static [StoreCanonicalBasisFieldRole],
    foundational_lane: StoreCanonicalBasisLane,
}

impl StoreCanonicalBasisSourceOwner {
    pub const fn new(
        family: StoreCanonicalBasisFamily,
        owner_crate: &'static str,
        classifying_subsystem: &'static str,
        allowed_sources: &'static [StoreCanonicalBasisSourceKind],
        denied_fields: &'static [StoreCanonicalBasisFieldRole],
        foundational_lane: StoreCanonicalBasisLane,
    ) -> Self {
        Self {
            family,
            owner_crate,
            classifying_subsystem,
            allowed_sources,
            denied_fields,
            foundational_lane,
        }
    }

    pub const fn family(&self) -> StoreCanonicalBasisFamily {
        self.family
    }

    pub const fn owner_crate(&self) -> &'static str {
        self.owner_crate
    }

    pub const fn classifying_subsystem(&self) -> &'static str {
        self.classifying_subsystem
    }

    pub const fn foundational_lane(&self) -> StoreCanonicalBasisLane {
        self.foundational_lane
    }

    pub fn allows_source(&self, source: StoreCanonicalBasisSourceKind) -> bool {
        self.allowed_sources.contains(&source)
    }

    pub fn primary_source_kind(&self) -> Option<StoreCanonicalBasisSourceKind> {
        self.allowed_sources.first().copied()
    }

    pub fn denies_field(&self, field_role: StoreCanonicalBasisFieldRole) -> bool {
        self.denied_fields.contains(&field_role)
    }
}
mod canonical_basis_construction;
mod canonical_basis_denial;
pub(crate) mod canonical_basis_domains;
mod canonical_basis_entries;
mod canonical_basis_sources;

pub use canonical_basis_construction::{
    StoreCanonicalBasisConstruction, StoreCanonicalBasisConstructionOutcome,
};
pub use canonical_basis_denial::StoreCanonicalBasisConstructionDenial;
pub use canonical_basis_domains::StoreCanonicalBasisDomainMismatch;
pub use canonical_basis_sources::{
    canonical_basis_source_owner_for_family, certify_canonical_basis_field_role,
    certify_canonical_basis_source, STORE_CANONICAL_BASIS_SOURCE_OWNERS,
};
