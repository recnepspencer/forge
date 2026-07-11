use crate::{GenerationIntegrityReport, PhysicalBoundaryLocalization, WalTailIntegrityPosture};
use forge_store_physical_format::{
    AllocationClassKind, CheckpointAdjacencyPosture, PhysicalCellReuseDomain, PhysicalReference,
    PhysicalReferenceKind, PhysicalReferenceScope, PhysicalScopeFamily,
    RootManifestIntegrityPosture,
};

pub(crate) const fn scope_family_token(scope: PhysicalReferenceScope) -> &'static str {
    match scope.family() {
        PhysicalScopeFamily::Page => "page",
        PhysicalScopeFamily::Frame => "frame",
        PhysicalScopeFamily::WalFrame => "wal-frame",
        PhysicalScopeFamily::Manifest => "manifest",
        PhysicalScopeFamily::ChunkLike => "chunk-like",
        PhysicalScopeFamily::DerivedIndex => "derived-index",
    }
}

pub(crate) const fn owner_domain_token(domain: PhysicalCellReuseDomain) -> &'static str {
    match domain {
        PhysicalCellReuseDomain::SlotAllocation => "slot-allocation",
        PhysicalCellReuseDomain::ExtentAllocation => "extent-allocation",
        PhysicalCellReuseDomain::FreeSpaceReuse => "free-space-reuse",
        PhysicalCellReuseDomain::RootPublication => "root-publication",
        PhysicalCellReuseDomain::Page => "page",
        PhysicalCellReuseDomain::Segment => "segment",
    }
}

pub(crate) const fn allocation_class_token(class: AllocationClassKind) -> &'static str {
    match class {
        AllocationClassKind::OrdinaryRecordPage => "ordinary-record-page",
        AllocationClassKind::LargeRecordExtent => "large-record-extent",
        AllocationClassKind::RootManifest => "root-manifest",
        AllocationClassKind::SegmentManifest => "segment-manifest",
        AllocationClassKind::ExtentManifest => "extent-manifest",
        AllocationClassKind::FreeSpaceMap => "free-space-map",
    }
}

pub(crate) const fn reference_kind_token(reference: PhysicalReference) -> &'static str {
    match reference.kind() {
        PhysicalReferenceKind::PageSlot => "page-slot",
        PhysicalReferenceKind::ExtentBacked => "extent-backed",
        PhysicalReferenceKind::FreeSpaceReuse => "free-space-reuse",
        PhysicalReferenceKind::RootPublication => "root-publication",
    }
}

pub(crate) const fn root_posture_token(posture: RootManifestIntegrityPosture) -> &'static str {
    match posture {
        RootManifestIntegrityPosture::CurrentRootAdmitted(_) => "current-root-admitted",
        RootManifestIntegrityPosture::MissingRoot => "missing-root",
        RootManifestIntegrityPosture::AmbiguousRoot => "ambiguous-root",
        RootManifestIntegrityPosture::DamagedRoot => "damaged-root",
        RootManifestIntegrityPosture::TornRootPointer => "torn-root-pointer",
        RootManifestIntegrityPosture::MultipleValidRoots => "multiple-valid-roots",
        RootManifestIntegrityPosture::RootGenerationMismatch => "root-generation-mismatch",
        RootManifestIntegrityPosture::ResidueRootRejected => "residue-root-rejected",
        RootManifestIntegrityPosture::RecoveryBlockingRootDamage => "recovery-blocking-root-damage",
        RootManifestIntegrityPosture::WrongRootPosture => "wrong-root-posture",
    }
}

pub(crate) const fn checkpoint_adjacency_token(
    posture: CheckpointAdjacencyPosture,
) -> &'static str {
    match posture {
        CheckpointAdjacencyPosture::NotApplicable => "not-applicable",
        CheckpointAdjacencyPosture::CheckpointAdjacent => "checkpoint-adjacent",
        CheckpointAdjacencyPosture::NotCheckpointAdjacent => "not-checkpoint-adjacent",
        CheckpointAdjacencyPosture::MismatchedCheckpointAdjacency => {
            "mismatched-checkpoint-adjacency"
        }
    }
}

pub(crate) const fn boundary_localization_token(
    boundary: PhysicalBoundaryLocalization,
) -> &'static str {
    match boundary {
        PhysicalBoundaryLocalization::PageHeader => "page-header",
        PhysicalBoundaryLocalization::PageBody => "page-body",
        PhysicalBoundaryLocalization::FrameHeader => "frame-header",
        PhysicalBoundaryLocalization::FrameBody => "frame-body",
        PhysicalBoundaryLocalization::LengthField => "length-field",
        PhysicalBoundaryLocalization::SlotDirectory => "slot-directory",
        PhysicalBoundaryLocalization::SlotState(_) => "slot-state",
        PhysicalBoundaryLocalization::ExtentBoundary => "extent-boundary",
        PhysicalBoundaryLocalization::AmbiguousBoundary => "ambiguous-boundary",
    }
}

pub(crate) const fn wal_tail_posture_token(posture: WalTailIntegrityPosture) -> &'static str {
    match posture {
        WalTailIntegrityPosture::IntactTail => "intact-tail",
        WalTailIntegrityPosture::TornTail => "torn-tail",
        WalTailIntegrityPosture::UnsupportedTailIntegrity => "unsupported-tail-integrity",
        WalTailIntegrityPosture::UnknownTailIntegrity => "unknown-tail-integrity",
        WalTailIntegrityPosture::CheckpointAdjacentDamage => "checkpoint-adjacent-damage",
        WalTailIntegrityPosture::RecoveryPrecedenceRequired => "recovery-precedence-required",
    }
}

pub(crate) const fn generation_report_token(report: GenerationIntegrityReport) -> &'static str {
    match report {
        GenerationIntegrityReport::SamePhysicalGeneration { .. } => "same-physical-generation",
        GenerationIntegrityReport::StalePhysicalGeneration { .. } => "stale-physical-generation",
        GenerationIntegrityReport::MisplacedPhysicalIdentity { .. } => {
            "misplaced-physical-identity"
        }
    }
}
