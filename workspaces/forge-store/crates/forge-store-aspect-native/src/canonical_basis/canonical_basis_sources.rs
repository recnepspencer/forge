use crate::{
    StoreCanonicalBasisFamily, StoreCanonicalBasisFieldRole, StoreCanonicalBasisLane,
    StoreCanonicalBasisSourceDenial, StoreCanonicalBasisSourceKind, StoreCanonicalBasisSourceOwner,
};

const FORBIDDEN_TEXT_SOURCES: &[StoreCanonicalBasisFieldRole] = &[
    StoreCanonicalBasisFieldRole::TerminalProjection,
    StoreCanonicalBasisFieldRole::OperatorDisplay,
    StoreCanonicalBasisFieldRole::DocumentChecksum,
    StoreCanonicalBasisFieldRole::CompatibilityText,
    StoreCanonicalBasisFieldRole::DigestText,
    StoreCanonicalBasisFieldRole::RawJsonPayload,
];

const ASPECT_STATE: &[StoreCanonicalBasisSourceKind] =
    &[StoreCanonicalBasisSourceKind::FoundationalAspectState];
const ASPECT_PATCH: &[StoreCanonicalBasisSourceKind] =
    &[StoreCanonicalBasisSourceKind::FoundationalAspectPatch];
const RECEIPT: &[StoreCanonicalBasisSourceKind] =
    &[StoreCanonicalBasisSourceKind::FoundationalReceipt];
const DIAGNOSTIC: &[StoreCanonicalBasisSourceKind] =
    &[StoreCanonicalBasisSourceKind::FoundationalDiagnostic];
const PERFORMANCE: &[StoreCanonicalBasisSourceKind] =
    &[StoreCanonicalBasisSourceKind::FoundationalPerformanceEvidence];
const HANDOFF: &[StoreCanonicalBasisSourceKind] =
    &[StoreCanonicalBasisSourceKind::StoreReadinessHandoff];
const SOURCE_MANIFEST: &[StoreCanonicalBasisSourceKind] =
    &[StoreCanonicalBasisSourceKind::StoreSourceManifest];
const PAGE_HEADER: &[StoreCanonicalBasisSourceKind] =
    &[StoreCanonicalBasisSourceKind::StorePageHeader];
const PHYSICAL_WITNESS: &[StoreCanonicalBasisSourceKind] =
    &[StoreCanonicalBasisSourceKind::StorePhysicalWitness];
const PHYSICAL_RECORD: &[StoreCanonicalBasisSourceKind] =
    &[StoreCanonicalBasisSourceKind::StorePhysicalFormatRecord];
const PHYSICAL_INTEGRITY: &[StoreCanonicalBasisSourceKind] =
    &[StoreCanonicalBasisSourceKind::StorePhysicalIntegrityEvidence];
const WAL: &[StoreCanonicalBasisSourceKind] = &[StoreCanonicalBasisSourceKind::StoreWalRecord];
const RECOVERY_RECEIPT: &[StoreCanonicalBasisSourceKind] =
    &[StoreCanonicalBasisSourceKind::StoreRecoveryReceipt];
const RECOVERY_PERFORMANCE: &[StoreCanonicalBasisSourceKind] =
    &[StoreCanonicalBasisSourceKind::StoreRecoveryPerformanceEvidence];

pub const STORE_CANONICAL_BASIS_SOURCE_OWNERS: &[StoreCanonicalBasisSourceOwner] = &[
    owner(
        StoreCanonicalBasisFamily::AspectBoundaryFact,
        "forge-store-aspect-native",
        "aspect-native authority",
        ASPECT_STATE,
        StoreCanonicalBasisLane::AspectValueState,
    ),
    owner(
        StoreCanonicalBasisFamily::AspectPatchBoundaryFact,
        "forge-store-aspect-native",
        "aspect-native patch authority",
        ASPECT_PATCH,
        StoreCanonicalBasisLane::AspectPatch,
    ),
    owner(
        StoreCanonicalBasisFamily::BoundaryReceiptEvidence,
        "forge-store-aspect-native",
        "aspect-native receipts",
        RECEIPT,
        StoreCanonicalBasisLane::Receipt,
    ),
    owner(
        StoreCanonicalBasisFamily::DiagnosticEvidence,
        "forge-store-aspect-native",
        "aspect-native diagnostics",
        DIAGNOSTIC,
        StoreCanonicalBasisLane::Diagnostic,
    ),
    owner(
        StoreCanonicalBasisFamily::PerformanceReceiptEvidence,
        "forge-store-aspect-native",
        "aspect-native performance",
        PERFORMANCE,
        StoreCanonicalBasisLane::PerformanceEvidence,
    ),
    owner(
        StoreCanonicalBasisFamily::ReadinessHandoff,
        "forge-store-readiness",
        "readiness handoff",
        HANDOFF,
        StoreCanonicalBasisLane::Handoff,
    ),
    owner(
        StoreCanonicalBasisFamily::S2EntryBoundaryEvidence,
        "forge-store-readiness",
        "S2 boundary readiness",
        PHYSICAL_WITNESS,
        StoreCanonicalBasisLane::Handoff,
    ),
    owner(
        StoreCanonicalBasisFamily::S3IntegrityCloseoutHandoff,
        "forge-store-readiness",
        "S3 integrity handoff",
        HANDOFF,
        StoreCanonicalBasisLane::Handoff,
    ),
    owner(
        StoreCanonicalBasisFamily::PhysicalSourceManifest,
        "forge-store-physical-format",
        "source manifest",
        SOURCE_MANIFEST,
        StoreCanonicalBasisLane::PhysicalSourceManifest,
    ),
    owner(
        StoreCanonicalBasisFamily::PhysicalPageHeader,
        "forge-store-physical-format",
        "page header",
        PAGE_HEADER,
        StoreCanonicalBasisLane::PhysicalRecord,
    ),
    owner(
        StoreCanonicalBasisFamily::PhysicalPageRecord,
        "forge-store-physical-format",
        "page record",
        PHYSICAL_RECORD,
        StoreCanonicalBasisLane::PhysicalRecord,
    ),
    owner(
        StoreCanonicalBasisFamily::PhysicalExtentRecord,
        "forge-store-physical-format",
        "extent record",
        PHYSICAL_RECORD,
        StoreCanonicalBasisLane::PhysicalRecord,
    ),
    owner(
        StoreCanonicalBasisFamily::PhysicalReference,
        "forge-store-physical-format",
        "physical reference",
        PHYSICAL_RECORD,
        StoreCanonicalBasisLane::PhysicalRecord,
    ),
    owner(
        StoreCanonicalBasisFamily::PhysicalOfflineVerifierEvidence,
        "forge-store-physical-format",
        "offline verifier",
        PHYSICAL_RECORD,
        StoreCanonicalBasisLane::PhysicalRecord,
    ),
    owner(
        StoreCanonicalBasisFamily::PhysicalHeaderDecodeEvidence,
        "forge-store-physical-format",
        "header decoder",
        PAGE_HEADER,
        StoreCanonicalBasisLane::PhysicalRecord,
    ),
    owner(
        StoreCanonicalBasisFamily::PhysicalFormatEvidence,
        "forge-store-physical-format",
        "physical format evidence",
        PHYSICAL_RECORD,
        StoreCanonicalBasisLane::PhysicalRecord,
    ),
    owner(
        StoreCanonicalBasisFamily::PhysicalManifestDiscoveryEvidence,
        "forge-store-physical-format",
        "manifest discovery",
        SOURCE_MANIFEST,
        StoreCanonicalBasisLane::PhysicalSourceManifest,
    ),
    owner(
        StoreCanonicalBasisFamily::PhysicalIdentityEvidence,
        "forge-store-physical-integrity",
        "physical identity",
        PHYSICAL_INTEGRITY,
        StoreCanonicalBasisLane::PhysicalIntegrity,
    ),
    owner(
        StoreCanonicalBasisFamily::PhysicalFoundationEvidence,
        "forge-store-physical-integrity",
        "physical foundation",
        PHYSICAL_INTEGRITY,
        StoreCanonicalBasisLane::PhysicalIntegrity,
    ),
    owner(
        StoreCanonicalBasisFamily::PhysicalIntegrityChecksumCoverage,
        "forge-store-physical-integrity",
        "checksum coverage",
        PHYSICAL_INTEGRITY,
        StoreCanonicalBasisLane::PhysicalIntegrity,
    ),
    owner(
        StoreCanonicalBasisFamily::PhysicalIntegrityEvidence,
        "forge-store-physical-integrity",
        "integrity evidence",
        PHYSICAL_INTEGRITY,
        StoreCanonicalBasisLane::PhysicalIntegrity,
    ),
    owner(
        StoreCanonicalBasisFamily::PhysicalIntegrityQuarantineReceipt,
        "forge-store-physical-integrity",
        "quarantine receipt",
        PHYSICAL_INTEGRITY,
        StoreCanonicalBasisLane::PhysicalIntegrity,
    ),
    owner(
        StoreCanonicalBasisFamily::PhysicalIntegrityScrubReceipt,
        "forge-store-physical-integrity",
        "scrub receipt",
        PHYSICAL_INTEGRITY,
        StoreCanonicalBasisLane::PhysicalIntegrity,
    ),
    owner(
        StoreCanonicalBasisFamily::PhysicalIntegrityCloseoutEvidence,
        "forge-store-physical-integrity",
        "integrity closeout",
        PHYSICAL_INTEGRITY,
        StoreCanonicalBasisLane::PhysicalIntegrity,
    ),
    owner(
        StoreCanonicalBasisFamily::WalFrameIntegrityEvidence,
        "forge-store-wal",
        "WAL frame integrity",
        WAL,
        StoreCanonicalBasisLane::Wal,
    ),
    owner(
        StoreCanonicalBasisFamily::WalRecord,
        "forge-store-wal",
        "WAL record",
        WAL,
        StoreCanonicalBasisLane::Wal,
    ),
    owner(
        StoreCanonicalBasisFamily::RecoveryIntegrityHandoff,
        "forge-store-recovery-physics",
        "recovery integrity handoff",
        RECOVERY_RECEIPT,
        StoreCanonicalBasisLane::Recovery,
    ),
    owner(
        StoreCanonicalBasisFamily::RecoveryWalReplayReceipt,
        "forge-store-recovery-physics",
        "WAL replay receipt",
        RECOVERY_RECEIPT,
        StoreCanonicalBasisLane::Recovery,
    ),
    owner(
        StoreCanonicalBasisFamily::RecoveryCheckpointValidityReceipt,
        "forge-store-recovery-physics",
        "checkpoint validity receipt",
        RECOVERY_RECEIPT,
        StoreCanonicalBasisLane::Recovery,
    ),
    owner(
        StoreCanonicalBasisFamily::RecoveryVettedRecordReceipt,
        "forge-store-recovery-physics",
        "vetted record receipt",
        RECOVERY_RECEIPT,
        StoreCanonicalBasisLane::Recovery,
    ),
    owner(
        StoreCanonicalBasisFamily::RecoveryPerformanceReport,
        "forge-store-recovery-physics",
        "recovery performance",
        RECOVERY_PERFORMANCE,
        StoreCanonicalBasisLane::Recovery,
    ),
];

pub fn canonical_basis_source_owner_for_family(
    family: StoreCanonicalBasisFamily,
) -> Result<&'static StoreCanonicalBasisSourceOwner, StoreCanonicalBasisSourceDenial> {
    STORE_CANONICAL_BASIS_SOURCE_OWNERS
        .iter()
        .find(|owner| owner.family() == family)
        .ok_or(StoreCanonicalBasisSourceDenial::MissingSourceOwner {
            family,
            classifying_subsystem: "Store canonical-basis source ownership",
        })
}

pub fn certify_canonical_basis_source(
    family: StoreCanonicalBasisFamily,
    source: StoreCanonicalBasisSourceKind,
) -> Result<(), StoreCanonicalBasisSourceDenial> {
    let owner = canonical_basis_source_owner_for_family(family)?;
    if owner.allows_source(source) {
        Ok(())
    } else {
        Err(StoreCanonicalBasisSourceDenial::WrongNativeSourceKind { family, source })
    }
}

pub fn certify_canonical_basis_field_role(
    field_role: StoreCanonicalBasisFieldRole,
) -> Result<(), StoreCanonicalBasisSourceDenial> {
    if FORBIDDEN_TEXT_SOURCES.contains(&field_role) {
        Err(StoreCanonicalBasisSourceDenial::ForbiddenFieldRole { field_role })
    } else {
        Ok(())
    }
}

const fn owner(
    family: StoreCanonicalBasisFamily,
    owner_crate: &'static str,
    classifying_subsystem: &'static str,
    allowed_sources: &'static [StoreCanonicalBasisSourceKind],
    foundational_lane: StoreCanonicalBasisLane,
) -> StoreCanonicalBasisSourceOwner {
    StoreCanonicalBasisSourceOwner::new(
        family,
        owner_crate,
        classifying_subsystem,
        allowed_sources,
        FORBIDDEN_TEXT_SOURCES,
        foundational_lane,
    )
}
