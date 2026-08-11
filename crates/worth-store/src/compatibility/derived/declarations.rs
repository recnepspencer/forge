use serde::Serialize;

use super::super::catalog::CompatibilityRegistrySnapshot;
use worth_store_contracts::{CompatibilityAuthorityClassification, CompatibilityFamilyKind};

use super::super::manifests::{ArtifactCompatibilityWindow, ArtifactFamilyId};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedCompatibilityWitness {
    family_id: ArtifactFamilyId,
}

impl DerivedCompatibilityWitness {
    pub(crate) fn new(family_id: ArtifactFamilyId) -> Self {
        Self { family_id }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedCompatibilityReuseWitness {
    family_id: ArtifactFamilyId,
}

impl DerivedCompatibilityReuseWitness {
    pub(crate) fn new(family_id: ArtifactFamilyId) -> Self {
        Self { family_id }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DerivedReusePosture {
    ReuseAdmitted,
    RebuildRequired,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DerivedBasisCompatibilityPosture {
    ReuseStillValid,
    InvalidateAndRebuild,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DerivedInvalidationReason {
    FormatWindowMismatch,
    SemanticWindowMismatch,
    NonNativeReadRelation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum DerivedCompatibilityLaneKind {
    SnapshotReuse,
    BranchDeltaReuse,
    LayoutBlockChunkReuse,
    LiveBasisContinuationReuse,
    BulkResumeReuse,
    RetentionRebuildSupport,
    MaintenanceSummarySupport,
    TierManifestSupport,
}

impl DerivedCompatibilityLaneKind {
    pub const fn label(self) -> &'static str {
        match self {
            Self::SnapshotReuse => "snapshot_reuse",
            Self::BranchDeltaReuse => "branch_delta_reuse",
            Self::LayoutBlockChunkReuse => "layout_block_chunk_reuse",
            Self::LiveBasisContinuationReuse => "live_basis_continuation_reuse",
            Self::BulkResumeReuse => "bulk_resume_reuse",
            Self::RetentionRebuildSupport => "retention_rebuild_support",
            Self::MaintenanceSummarySupport => "maintenance_summary_support",
            Self::TierManifestSupport => "tier_manifest_support",
        }
    }

    pub const fn from_family_kind(kind: CompatibilityFamilyKind) -> Option<Self> {
        match kind {
            CompatibilityFamilyKind::SnapshotRecord => Some(Self::SnapshotReuse),
            CompatibilityFamilyKind::DeltaRecord => Some(Self::BranchDeltaReuse),
            CompatibilityFamilyKind::Milestone6LayoutBlockChunkRecord => {
                Some(Self::LayoutBlockChunkReuse)
            }
            CompatibilityFamilyKind::Milestone8BasisContinuationDescriptor => {
                Some(Self::LiveBasisContinuationReuse)
            }
            CompatibilityFamilyKind::Milestone9BulkRecord => Some(Self::BulkResumeReuse),
            CompatibilityFamilyKind::Milestone10RetentionRebuildRecord => {
                Some(Self::RetentionRebuildSupport)
            }
            CompatibilityFamilyKind::Milestone11MaintenanceRecord => {
                Some(Self::MaintenanceSummarySupport)
            }
            CompatibilityFamilyKind::Milestone13TieringRecord => Some(Self::TierManifestSupport),
            CompatibilityFamilyKind::CommitEnvelope
            | CompatibilityFamilyKind::BranchVersionDagRecord
            | CompatibilityFamilyKind::WalRestartRecord
            | CompatibilityFamilyKind::SchemaLineageCursorCheckpointSupport
            | CompatibilityFamilyKind::EmbeddedCheckpointAuthority => None,
        }
    }

    pub const fn requires_retained_authority(self) -> bool {
        matches!(
            self,
            Self::SnapshotReuse
                | Self::BranchDeltaReuse
                | Self::LayoutBlockChunkReuse
                | Self::RetentionRebuildSupport
                | Self::MaintenanceSummarySupport
        )
    }

    pub const fn requires_maintenance_admission(self) -> bool {
        matches!(
            self,
            Self::MaintenanceSummarySupport | Self::RetentionRebuildSupport
        )
    }

    pub const fn preserves_tier_non_authority(self) -> bool {
        matches!(self, Self::TierManifestSupport)
    }

    pub const fn maintenance_work_class_label(self) -> Option<&'static str> {
        match self {
            Self::MaintenanceSummarySupport => Some("DerivedFamilyRebuild"),
            Self::RetentionRebuildSupport => Some("RetainedRangeRebuild"),
            Self::TierManifestSupport => Some("TierPlacementProposal"),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DerivedCompatibilityLane {
    ExactAcceleration,
    SupportMetadata,
    PlacementSupportNonAuthority,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedCompatibilityLaneDeclaration {
    family_kind: CompatibilityFamilyKind,
    family_id: ArtifactFamilyId,
    lane_kind: DerivedCompatibilityLaneKind,
    lane: DerivedCompatibilityLane,
    required_window: ArtifactCompatibilityWindow,
    retained_authority_required: bool,
    maintenance_admission_required: bool,
    tier_non_authority_required: bool,
    counter_lane_id: String,
    certification_lane_id: String,
}

impl DerivedCompatibilityLaneDeclaration {
    pub(crate) fn new(
        family_kind: CompatibilityFamilyKind,
        required_window: ArtifactCompatibilityWindow,
    ) -> Option<Self> {
        let lane_kind = DerivedCompatibilityLaneKind::from_family_kind(family_kind)?;
        let lane = match lane_kind {
            DerivedCompatibilityLaneKind::SnapshotReuse
            | DerivedCompatibilityLaneKind::BranchDeltaReuse
            | DerivedCompatibilityLaneKind::LayoutBlockChunkReuse => {
                DerivedCompatibilityLane::ExactAcceleration
            }
            DerivedCompatibilityLaneKind::TierManifestSupport => {
                DerivedCompatibilityLane::PlacementSupportNonAuthority
            }
            DerivedCompatibilityLaneKind::LiveBasisContinuationReuse
            | DerivedCompatibilityLaneKind::BulkResumeReuse
            | DerivedCompatibilityLaneKind::RetentionRebuildSupport
            | DerivedCompatibilityLaneKind::MaintenanceSummarySupport => {
                DerivedCompatibilityLane::SupportMetadata
            }
        };
        Some(Self {
            family_kind,
            family_id: family_kind.family_id(),
            lane_kind,
            lane,
            required_window,
            retained_authority_required: lane_kind.requires_retained_authority(),
            maintenance_admission_required: lane_kind.requires_maintenance_admission(),
            tier_non_authority_required: lane_kind.preserves_tier_non_authority(),
            counter_lane_id: format!("counter.derived.lane.{}", lane_kind.label()),
            certification_lane_id: format!("certification.derived.lane.{}", lane_kind.label()),
        })
    }

    pub fn family_kind(&self) -> CompatibilityFamilyKind {
        self.family_kind
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn lane_kind(&self) -> DerivedCompatibilityLaneKind {
        self.lane_kind
    }

    pub fn lane(&self) -> DerivedCompatibilityLane {
        self.lane
    }

    pub fn required_window(&self) -> &ArtifactCompatibilityWindow {
        &self.required_window
    }

    pub fn retained_authority_required(&self) -> bool {
        self.retained_authority_required
    }

    pub fn maintenance_admission_required(&self) -> bool {
        self.maintenance_admission_required
    }

    pub fn tier_non_authority_required(&self) -> bool {
        self.tier_non_authority_required
    }

    pub fn counter_lane_id(&self) -> &str {
        &self.counter_lane_id
    }

    pub fn certification_lane_id(&self) -> &str {
        &self.certification_lane_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedCompatibilityLaneSnapshot {
    declarations: Vec<DerivedCompatibilityLaneDeclaration>,
}

impl DerivedCompatibilityLaneSnapshot {
    pub(crate) fn new(mut declarations: Vec<DerivedCompatibilityLaneDeclaration>) -> Self {
        declarations.sort_by_key(|declaration| declaration.lane_kind().label());
        Self { declarations }
    }

    pub fn declarations(&self) -> &[DerivedCompatibilityLaneDeclaration] {
        &self.declarations
    }

    pub fn get(
        &self,
        lane_kind: DerivedCompatibilityLaneKind,
    ) -> Option<&DerivedCompatibilityLaneDeclaration> {
        self.declarations
            .iter()
            .find(|declaration| declaration.lane_kind() == lane_kind)
    }

    pub fn get_by_family_kind(
        &self,
        family_kind: CompatibilityFamilyKind,
    ) -> Option<&DerivedCompatibilityLaneDeclaration> {
        self.declarations
            .iter()
            .find(|declaration| declaration.family_kind() == family_kind)
    }
}

#[derive(Debug, Default)]
pub struct DerivedCompatibilityLaneRegistry {
    declarations: Vec<DerivedCompatibilityLaneDeclaration>,
}

impl DerivedCompatibilityLaneRegistry {
    pub fn from_compatibility_snapshot(snapshot: &CompatibilityRegistrySnapshot) -> Self {
        let mut registry = Self::default();
        for declaration in snapshot.declarations() {
            if declaration.authority_classification()
                == CompatibilityAuthorityClassification::Derived
            {
                if let Some(lane) = DerivedCompatibilityLaneDeclaration::new(
                    declaration.kind(),
                    declaration.manifest().window().clone(),
                ) {
                    registry.declarations.push(lane);
                }
            }
        }
        registry
    }

    pub fn snapshot(self) -> DerivedCompatibilityLaneSnapshot {
        DerivedCompatibilityLaneSnapshot::new(self.declarations)
    }
}
