use super::admission::{
    CompatibilityAdmissionCounters, CompatibilityDecision, CompatibilityRejection,
    CompatibilityRejectionKind, CompatibilityRelation, DerivedReuseCompatibilityReceipt,
    ReadCompatibilityReceipt,
};
use super::catalog::{
    CompatibilityAuthorityClassification, CompatibilityFamilyKind, CompatibilityRegistrySnapshot,
    DerivedFamilyDeclaration,
};
use super::decoding::{CompatibilityCheckedArtifact, QuarantinedDecodedArtifact};
use super::manifests::{
    ArtifactCompatibilityWindow, ArtifactFamilyId, ArtifactFormatVersion, ArtifactSemanticVersion,
    CompatibilityManifestDigest,
};
use serde::Serialize;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DerivedLaneCompatibilityPosture {
    ReuseAdmitted,
    SupportAdmitted,
    InvalidatedForRebuild,
    RebuildAdmitted,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedCompatibilityReusePlan {
    family_id: ArtifactFamilyId,
    manifest_digest: CompatibilityManifestDigest,
    observed_semantic_version: ArtifactSemanticVersion,
    relation: CompatibilityRelation,
    posture: DerivedReusePosture,
    reason: String,
    reuse_receipt: Option<DerivedReuseCompatibilityReceipt>,
}

impl DerivedCompatibilityReusePlan {
    fn reuse(
        artifact: &QuarantinedDecodedArtifact,
        read_receipt: &ReadCompatibilityReceipt,
    ) -> Self {
        Self {
            family_id: artifact.family_id().clone(),
            manifest_digest: artifact.manifest_digest().clone(),
            observed_semantic_version: artifact.semantic_version(),
            relation: read_receipt.receipt().relation(),
            posture: DerivedReusePosture::ReuseAdmitted,
            reason: "derived artifact semantic version is native to the admitted reader"
                .to_string(),
            reuse_receipt: Some(DerivedReuseCompatibilityReceipt::new(
                read_receipt.receipt().clone(),
            )),
        }
    }

    fn rebuild(
        artifact: &QuarantinedDecodedArtifact,
        read_receipt: &ReadCompatibilityReceipt,
    ) -> Self {
        Self {
            family_id: artifact.family_id().clone(),
            manifest_digest: artifact.manifest_digest().clone(),
            observed_semantic_version: artifact.semantic_version(),
            relation: read_receipt.receipt().relation(),
            posture: DerivedReusePosture::RebuildRequired,
            reason:
                "derived artifact requires rebuild because admitted read relation is not native"
                    .to_string(),
            reuse_receipt: None,
        }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn posture(&self) -> DerivedReusePosture {
        self.posture
    }

    pub fn relation(&self) -> CompatibilityRelation {
        self.relation
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub(crate) fn reuse_receipt(&self) -> Option<&DerivedReuseCompatibilityReceipt> {
        self.reuse_receipt.as_ref()
    }
}

pub(crate) fn plan_exact_derived_reuse(
    counters: &mut CompatibilityAdmissionCounters,
    derived_family: &DerivedFamilyDeclaration,
    artifact: &QuarantinedDecodedArtifact,
    read_receipt: &ReadCompatibilityReceipt,
) -> Result<DerivedCompatibilityReusePlan, CompatibilityRejection> {
    if artifact.family_id() != derived_family.declaration().family_id()
        || artifact.family_id() != read_receipt.receipt().family_id()
        || artifact.manifest_digest() != read_receipt.receipt().manifest_digest()
        || artifact.semantic_version() != read_receipt.receipt().observed_semantic_version()
    {
        counters.record_derived_reuse_incompatible();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::DerivedReuseIncompatible,
            artifact.family_id().clone(),
            "derived family declaration, receipt, and artifact do not describe the same derived artifact",
        ));
    }

    match read_receipt.receipt().relation() {
        CompatibilityRelation::Native => {
            Ok(DerivedCompatibilityReusePlan::reuse(artifact, read_receipt))
        }
        CompatibilityRelation::BackwardRead
        | CompatibilityRelation::ForwardRead
        | CompatibilityRelation::AdapterRequired
        | CompatibilityRelation::DerivedRebuildRequired => {
            counters.record_derived_rebuild_required();
            Ok(DerivedCompatibilityReusePlan::rebuild(
                artifact,
                read_receipt,
            ))
        }
        CompatibilityRelation::Incompatible => {
            counters.record_derived_reuse_incompatible();
            Err(CompatibilityRejection::new(
                CompatibilityRejectionKind::DerivedReuseIncompatible,
                artifact.family_id().clone(),
                "incompatible read relation cannot prove derived reuse",
            ))
        }
    }
}

pub(crate) fn admit_checked_derived_reuse(
    checked_artifact: CompatibilityCheckedArtifact,
    reuse_plan: &DerivedCompatibilityReusePlan,
) -> Result<DerivedCompatibilityReuseWitness, CompatibilityRejection> {
    if reuse_plan.reuse_receipt().is_none() {
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::DerivedRebuildIncompatible,
            reuse_plan.family_id().clone(),
            "derived reuse plan requires rebuild before reuse",
        ));
    }
    match checked_artifact.decision() {
        CompatibilityDecision::Admit(CompatibilityRelation::Native)
            if checked_artifact.family_id() == reuse_plan.family_id() =>
        {
            Ok(DerivedCompatibilityReuseWitness::new(
                reuse_plan.family_id().clone(),
            ))
        }
        _ => Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::DerivedReuseIncompatible,
            checked_artifact.family_id().clone(),
            "checked artifact decision is not native derived reuse",
        )),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationPlan {
    family_id: ArtifactFamilyId,
    observed_format_version: ArtifactFormatVersion,
    observed_semantic_version: ArtifactSemanticVersion,
    required_window: ArtifactCompatibilityWindow,
    reason_code: DerivedInvalidationReason,
    reason: String,
}

impl DerivedInvalidationPlan {
    pub(crate) fn new(
        family_id: ArtifactFamilyId,
        observed_format_version: ArtifactFormatVersion,
        observed_semantic_version: ArtifactSemanticVersion,
        required_window: ArtifactCompatibilityWindow,
        reason_code: DerivedInvalidationReason,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            family_id,
            observed_format_version,
            observed_semantic_version,
            required_window,
            reason_code,
            reason: reason.into(),
        }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn reason_code(&self) -> DerivedInvalidationReason {
        self.reason_code
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedRebuildRequirement {
    family_id: ArtifactFamilyId,
    observed_semantic_version: ArtifactSemanticVersion,
    required_window: ArtifactCompatibilityWindow,
    reason: String,
}

impl DerivedRebuildRequirement {
    pub(crate) fn from_reuse_plan(
        plan: &DerivedCompatibilityReusePlan,
        required_window: ArtifactCompatibilityWindow,
    ) -> Option<Self> {
        if plan.posture() != DerivedReusePosture::RebuildRequired {
            return None;
        }
        Some(Self {
            family_id: plan.family_id().clone(),
            observed_semantic_version: plan.observed_semantic_version,
            required_window,
            reason: plan.reason().to_string(),
        })
    }

    fn from_invalidation(
        invalidation: &DerivedInvalidationPlan,
        required_window: ArtifactCompatibilityWindow,
    ) -> Self {
        Self {
            family_id: invalidation.family_id.clone(),
            observed_semantic_version: invalidation.observed_semantic_version,
            required_window,
            reason: invalidation.reason.clone(),
        }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn observed_semantic_version(&self) -> ArtifactSemanticVersion {
        self.observed_semantic_version
    }

    pub fn target_semantic_version(&self) -> ArtifactSemanticVersion {
        self.required_window.maximum_semantic()
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedBasisCompatibilityPlan {
    family_id: ArtifactFamilyId,
    posture: DerivedBasisCompatibilityPosture,
    invalidation: Option<DerivedInvalidationPlan>,
    rebuild_requirement: Option<DerivedRebuildRequirement>,
}

impl DerivedBasisCompatibilityPlan {
    fn reusable(family_id: ArtifactFamilyId) -> Self {
        Self {
            family_id,
            posture: DerivedBasisCompatibilityPosture::ReuseStillValid,
            invalidation: None,
            rebuild_requirement: None,
        }
    }

    fn rebuild(invalidation: DerivedInvalidationPlan) -> Self {
        let rebuild_requirement = DerivedRebuildRequirement::from_invalidation(
            &invalidation,
            invalidation.required_window.clone(),
        );
        Self {
            family_id: invalidation.family_id.clone(),
            posture: DerivedBasisCompatibilityPosture::InvalidateAndRebuild,
            invalidation: Some(invalidation),
            rebuild_requirement: Some(rebuild_requirement),
        }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn posture(&self) -> DerivedBasisCompatibilityPosture {
        self.posture
    }

    pub fn invalidation(&self) -> Option<&DerivedInvalidationPlan> {
        self.invalidation.as_ref()
    }

    pub fn rebuild_requirement(&self) -> Option<&DerivedRebuildRequirement> {
        self.rebuild_requirement.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedBasisCompatibilityInput {
    lane_declaration: DerivedCompatibilityLaneDeclaration,
    derived_family: DerivedFamilyDeclaration,
    required_window: ArtifactCompatibilityWindow,
}

impl DerivedBasisCompatibilityInput {
    pub fn new(
        lane_declaration: DerivedCompatibilityLaneDeclaration,
        derived_family: DerivedFamilyDeclaration,
        required_window: ArtifactCompatibilityWindow,
    ) -> Self {
        Self {
            lane_declaration,
            derived_family,
            required_window,
        }
    }

    pub fn lane_declaration(&self) -> &DerivedCompatibilityLaneDeclaration {
        &self.lane_declaration
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedLaneReuseAdmission {
    family_id: ArtifactFamilyId,
    lane_kind: DerivedCompatibilityLaneKind,
    posture: DerivedLaneCompatibilityPosture,
}

impl DerivedLaneReuseAdmission {
    pub(crate) fn new(
        family_id: ArtifactFamilyId,
        lane_kind: DerivedCompatibilityLaneKind,
        posture: DerivedLaneCompatibilityPosture,
    ) -> Self {
        Self {
            family_id,
            lane_kind,
            posture,
        }
    }

    pub fn lane_kind(&self) -> DerivedCompatibilityLaneKind {
        self.lane_kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedLaneInvalidation {
    lane_kind: DerivedCompatibilityLaneKind,
    invalidation: DerivedInvalidationPlan,
}

impl DerivedLaneInvalidation {
    pub(crate) fn new(
        lane_kind: DerivedCompatibilityLaneKind,
        invalidation: DerivedInvalidationPlan,
    ) -> Self {
        Self {
            lane_kind,
            invalidation,
        }
    }

    pub fn lane_kind(&self) -> DerivedCompatibilityLaneKind {
        self.lane_kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedLaneRebuildRequirement {
    lane_kind: DerivedCompatibilityLaneKind,
    requirement: DerivedRebuildRequirement,
}

impl DerivedLaneRebuildRequirement {
    pub(crate) fn new(
        lane_kind: DerivedCompatibilityLaneKind,
        requirement: DerivedRebuildRequirement,
    ) -> Self {
        Self {
            lane_kind,
            requirement,
        }
    }

    pub fn requirement(&self) -> &DerivedRebuildRequirement {
        &self.requirement
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedLaneRejection {
    family_id: ArtifactFamilyId,
    lane_kind: DerivedCompatibilityLaneKind,
    reason: String,
}

impl DerivedLaneRejection {
    pub(crate) fn new(
        family_id: ArtifactFamilyId,
        lane_kind: DerivedCompatibilityLaneKind,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            family_id,
            lane_kind,
            reason: reason.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BulkResumeCompatibilityRejection {
    family_id: ArtifactFamilyId,
    interpretation: BulkResumeInterpretation,
}

impl BulkResumeCompatibilityRejection {
    pub(crate) fn new(
        family_id: ArtifactFamilyId,
        interpretation: BulkResumeInterpretation,
    ) -> Self {
        Self {
            family_id,
            interpretation,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum BulkResumeInterpretation {
    NativeResume,
    ChangedInterpretationRejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BulkResumeCompatibilityPlan {
    family_id: ArtifactFamilyId,
    interpretation: BulkResumeInterpretation,
}

impl BulkResumeCompatibilityPlan {
    pub(crate) fn new(
        family_id: ArtifactFamilyId,
        interpretation: BulkResumeInterpretation,
    ) -> Self {
        Self {
            family_id,
            interpretation,
        }
    }

    pub fn interpretation(&self) -> BulkResumeInterpretation {
        self.interpretation
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum TierCompatibilityNonAuthorityPosture {
    PlacementSupportOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TierManifestCompatibilityPlan {
    family_id: ArtifactFamilyId,
    posture: TierCompatibilityNonAuthorityPosture,
}

impl TierManifestCompatibilityPlan {
    pub(crate) fn new(family_id: ArtifactFamilyId) -> Self {
        Self {
            family_id,
            posture: TierCompatibilityNonAuthorityPosture::PlacementSupportOnly,
        }
    }

    pub fn posture(&self) -> TierCompatibilityNonAuthorityPosture {
        self.posture
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TierManifestCompatibilityRejection {
    family_id: ArtifactFamilyId,
    reason: String,
}

impl TierManifestCompatibilityRejection {
    pub(crate) fn new(family_id: ArtifactFamilyId, reason: impl Into<String>) -> Self {
        Self {
            family_id,
            reason: reason.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedLaneCompatibilityPlan {
    family_id: ArtifactFamilyId,
    lane_kind: DerivedCompatibilityLaneKind,
    posture: DerivedLaneCompatibilityPosture,
    reuse_admission: Option<DerivedLaneReuseAdmission>,
    invalidation: Option<DerivedLaneInvalidation>,
    rebuild_requirement: Option<DerivedLaneRebuildRequirement>,
    bulk_resume: Option<BulkResumeCompatibilityPlan>,
    tier_manifest: Option<TierManifestCompatibilityPlan>,
}

impl DerivedLaneCompatibilityPlan {
    fn from_basis(
        lane_kind: DerivedCompatibilityLaneKind,
        basis: DerivedBasisCompatibilityPlan,
    ) -> Self {
        let posture = match basis.posture() {
            DerivedBasisCompatibilityPosture::ReuseStillValid => {
                DerivedLaneCompatibilityPosture::ReuseAdmitted
            }
            DerivedBasisCompatibilityPosture::InvalidateAndRebuild => {
                DerivedLaneCompatibilityPosture::InvalidatedForRebuild
            }
        };
        Self {
            family_id: basis.family_id().clone(),
            lane_kind,
            posture,
            reuse_admission: (posture == DerivedLaneCompatibilityPosture::ReuseAdmitted).then(
                || DerivedLaneReuseAdmission::new(basis.family_id().clone(), lane_kind, posture),
            ),
            invalidation: basis
                .invalidation()
                .cloned()
                .map(|invalidation| DerivedLaneInvalidation::new(lane_kind, invalidation)),
            rebuild_requirement: basis
                .rebuild_requirement()
                .cloned()
                .map(|requirement| DerivedLaneRebuildRequirement::new(lane_kind, requirement)),
            bulk_resume: None,
            tier_manifest: None,
        }
    }

    fn from_bulk_resume(
        plan: BulkResumeCompatibilityPlan,
        lane_kind: DerivedCompatibilityLaneKind,
    ) -> Self {
        Self {
            family_id: plan.family_id.clone(),
            lane_kind,
            posture: DerivedLaneCompatibilityPosture::SupportAdmitted,
            reuse_admission: None,
            invalidation: None,
            rebuild_requirement: None,
            bulk_resume: Some(plan),
            tier_manifest: None,
        }
    }

    fn from_tier_manifest(
        plan: TierManifestCompatibilityPlan,
        lane_kind: DerivedCompatibilityLaneKind,
    ) -> Self {
        Self {
            family_id: plan.family_id.clone(),
            lane_kind,
            posture: DerivedLaneCompatibilityPosture::SupportAdmitted,
            reuse_admission: None,
            invalidation: None,
            rebuild_requirement: None,
            bulk_resume: None,
            tier_manifest: Some(plan),
        }
    }

    pub fn lane_kind(&self) -> DerivedCompatibilityLaneKind {
        self.lane_kind
    }

    pub fn posture(&self) -> DerivedLaneCompatibilityPosture {
        self.posture
    }

    pub fn rebuild_requirement(&self) -> Option<&DerivedLaneRebuildRequirement> {
        self.rebuild_requirement.as_ref()
    }

    pub fn bulk_resume(&self) -> Option<&BulkResumeCompatibilityPlan> {
        self.bulk_resume.as_ref()
    }

    pub fn tier_manifest(&self) -> Option<&TierManifestCompatibilityPlan> {
        self.tier_manifest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RetainedAuthorityCompatibilityWitness {
    family_id: ArtifactFamilyId,
}

impl RetainedAuthorityCompatibilityWitness {
    pub(crate) fn new(family_id: ArtifactFamilyId) -> Self {
        Self { family_id }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityMaintenanceAdmissionWitness {
    family_id: ArtifactFamilyId,
    compatibility_lane_id: String,
    maintenance_lane_id: String,
    maintenance_work_class_label: String,
}

impl CompatibilityMaintenanceAdmissionWitness {
    pub(crate) fn new(family_id: ArtifactFamilyId, maintenance_lane_id: impl Into<String>) -> Self {
        Self {
            family_id,
            compatibility_lane_id: "compatibility.derived.legacy".to_string(),
            maintenance_lane_id: maintenance_lane_id.into(),
            maintenance_work_class_label: "DerivedFamilyRebuild".to_string(),
        }
    }

    pub(crate) fn for_lane(
        family_id: ArtifactFamilyId,
        compatibility_lane_id: impl Into<String>,
        maintenance_lane_id: impl Into<String>,
        maintenance_work_class_label: impl Into<String>,
    ) -> Self {
        Self {
            family_id,
            compatibility_lane_id: compatibility_lane_id.into(),
            maintenance_lane_id: maintenance_lane_id.into(),
            maintenance_work_class_label: maintenance_work_class_label.into(),
        }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn maintenance_lane_id(&self) -> &str {
        &self.maintenance_lane_id
    }

    pub fn compatibility_lane_id(&self) -> &str {
        &self.compatibility_lane_id
    }

    pub fn maintenance_work_class_label(&self) -> &str {
        &self.maintenance_work_class_label
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityMaintenanceLaneRequirement {
    family_id: ArtifactFamilyId,
    compatibility_lane_id: String,
    maintenance_work_class_label: String,
}

impl CompatibilityMaintenanceLaneRequirement {
    pub fn new(
        family_id: ArtifactFamilyId,
        compatibility_lane_id: impl Into<String>,
        maintenance_work_class_label: impl Into<String>,
    ) -> Self {
        Self {
            family_id,
            compatibility_lane_id: compatibility_lane_id.into(),
            maintenance_work_class_label: maintenance_work_class_label.into(),
        }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityMaintenanceLaneAdmission {
    witness: CompatibilityMaintenanceAdmissionWitness,
}

impl CompatibilityMaintenanceLaneAdmission {
    pub(crate) fn new(witness: CompatibilityMaintenanceAdmissionWitness) -> Self {
        Self { witness }
    }

    pub fn witness(&self) -> &CompatibilityMaintenanceAdmissionWitness {
        &self.witness
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityMaintenanceLaneRejection {
    family_id: ArtifactFamilyId,
    reason: String,
}

impl CompatibilityMaintenanceLaneRejection {
    pub(crate) fn new(family_id: ArtifactFamilyId, reason: impl Into<String>) -> Self {
        Self {
            family_id,
            reason: reason.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedRebuildCompatibilityPlan {
    family_id: ArtifactFamilyId,
    source_semantic_version: ArtifactSemanticVersion,
    target_semantic_version: ArtifactSemanticVersion,
    maintenance_lane_id: String,
}

impl DerivedRebuildCompatibilityPlan {
    pub(crate) fn new(
        family_id: ArtifactFamilyId,
        source_semantic_version: ArtifactSemanticVersion,
        target_semantic_version: ArtifactSemanticVersion,
        maintenance_lane_id: impl Into<String>,
    ) -> Self {
        Self {
            family_id,
            source_semantic_version,
            target_semantic_version,
            maintenance_lane_id: maintenance_lane_id.into(),
        }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn target_semantic_version(&self) -> ArtifactSemanticVersion {
        self.target_semantic_version
    }

    pub fn maintenance_lane_id(&self) -> &str {
        &self.maintenance_lane_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityRebuildDebt {
    family_id: ArtifactFamilyId,
    debt_record_count: u64,
}

impl CompatibilityRebuildDebt {
    pub(crate) fn new(family_id: ArtifactFamilyId, debt_record_count: u64) -> Self {
        Self {
            family_id,
            debt_record_count,
        }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn debt_record_count(&self) -> u64 {
        self.debt_record_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StaleDerivedVersionRejection {
    family_id: ArtifactFamilyId,
    observed_semantic_version: ArtifactSemanticVersion,
}

impl StaleDerivedVersionRejection {
    pub(crate) fn new(
        family_id: ArtifactFamilyId,
        observed_semantic_version: ArtifactSemanticVersion,
    ) -> Self {
        Self {
            family_id,
            observed_semantic_version,
        }
    }
}

pub(crate) fn plan_derived_basis_compatibility(
    counters: &mut CompatibilityAdmissionCounters,
    derived_family: &DerivedFamilyDeclaration,
    artifact: &QuarantinedDecodedArtifact,
    read_receipt: &ReadCompatibilityReceipt,
    required_window: ArtifactCompatibilityWindow,
) -> Result<DerivedBasisCompatibilityPlan, CompatibilityRejection> {
    if artifact.family_id() != derived_family.declaration().family_id()
        || artifact.family_id() != read_receipt.receipt().family_id()
        || artifact.manifest_digest() != read_receipt.receipt().manifest_digest()
        || artifact.semantic_version() != read_receipt.receipt().observed_semantic_version()
    {
        counters.record_derived_reuse_incompatible();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::DerivedBasisIncompatible,
            artifact.family_id().clone(),
            "derived basis declaration, receipt, and artifact do not describe the same artifact",
        ));
    }

    if !required_window.contains_format(artifact.format_version()) {
        return Ok(plan_invalidation(
            counters,
            artifact,
            required_window,
            DerivedInvalidationReason::FormatWindowMismatch,
            "derived artifact format version is outside the required rebuild window",
        ));
    }

    if !required_window.contains_semantic(artifact.semantic_version()) {
        return Ok(plan_invalidation(
            counters,
            artifact,
            required_window,
            DerivedInvalidationReason::SemanticWindowMismatch,
            "derived artifact semantic version is outside the required rebuild window",
        ));
    }

    if read_receipt.receipt().relation() != CompatibilityRelation::Native {
        return Ok(plan_invalidation(
            counters,
            artifact,
            required_window,
            DerivedInvalidationReason::NonNativeReadRelation,
            "derived artifact was admitted through a non-native read relation",
        ));
    }

    Ok(DerivedBasisCompatibilityPlan::reusable(
        artifact.family_id().clone(),
    ))
}

pub(crate) fn plan_derived_lane_compatibility(
    counters: &mut CompatibilityAdmissionCounters,
    input: &DerivedBasisCompatibilityInput,
    artifact: &QuarantinedDecodedArtifact,
    read_receipt: &ReadCompatibilityReceipt,
) -> Result<DerivedLaneCompatibilityPlan, CompatibilityRejection> {
    counters.record_derived_lane_plan();
    let lane = input.lane_declaration();
    if lane.family_id() != input.derived_family.declaration().family_id()
        || lane.family_id() != artifact.family_id()
    {
        counters.record_derived_lane_rejection();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::DerivedLaneRejected,
            artifact.family_id().clone(),
            "derived compatibility lane, declaration, and artifact family do not match",
        ));
    }

    match lane.lane_kind() {
        DerivedCompatibilityLaneKind::BulkResumeReuse => {
            return plan_bulk_resume_compatibility(counters, lane, artifact, read_receipt).map(
                |plan| DerivedLaneCompatibilityPlan::from_bulk_resume(plan, lane.lane_kind()),
            );
        }
        DerivedCompatibilityLaneKind::TierManifestSupport => {
            return plan_tier_manifest_compatibility(counters, lane, artifact, read_receipt).map(
                |plan| DerivedLaneCompatibilityPlan::from_tier_manifest(plan, lane.lane_kind()),
            );
        }
        DerivedCompatibilityLaneKind::LayoutBlockChunkReuse
            if !input
                .required_window
                .contains_format(artifact.format_version())
                || !input
                    .required_window
                    .contains_semantic(artifact.semantic_version()) =>
        {
            counters.record_derived_layout_basis_rejection();
            return Err(CompatibilityRejection::new(
                CompatibilityRejectionKind::DerivedBasisIncompatible,
                artifact.family_id().clone(),
                "layout/block/chunk compatibility lane rejects basis drift",
            ));
        }
        _ => {}
    }

    let plan = plan_derived_basis_compatibility(
        counters,
        &input.derived_family,
        artifact,
        read_receipt,
        input.required_window.clone(),
    )?;
    match plan.posture() {
        DerivedBasisCompatibilityPosture::ReuseStillValid => {
            counters.record_derived_lane_reuse();
            match lane.lane_kind() {
                DerivedCompatibilityLaneKind::SnapshotReuse => {
                    counters.record_derived_snapshot_reuse();
                }
                DerivedCompatibilityLaneKind::BranchDeltaReuse => {
                    counters.record_derived_delta_reuse();
                }
                _ => {}
            }
        }
        DerivedBasisCompatibilityPosture::InvalidateAndRebuild => {
            counters.record_derived_lane_invalidation();
            if lane.lane_kind() == DerivedCompatibilityLaneKind::MaintenanceSummarySupport {
                counters.record_derived_maintenance_summary_rebuild();
            }
        }
    }
    Ok(DerivedLaneCompatibilityPlan::from_basis(
        lane.lane_kind(),
        plan,
    ))
}

pub(crate) fn plan_bulk_resume_compatibility(
    counters: &mut CompatibilityAdmissionCounters,
    lane: &DerivedCompatibilityLaneDeclaration,
    artifact: &QuarantinedDecodedArtifact,
    read_receipt: &ReadCompatibilityReceipt,
) -> Result<BulkResumeCompatibilityPlan, CompatibilityRejection> {
    if lane.lane_kind() != DerivedCompatibilityLaneKind::BulkResumeReuse
        || lane.family_id() != artifact.family_id()
    {
        counters.record_derived_bulk_resume_rejection();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::BulkResumeCompatibilityRejected,
            artifact.family_id().clone(),
            "bulk resume compatibility requires the bulk resume lane",
        ));
    }
    if read_receipt.receipt().relation() != CompatibilityRelation::Native {
        counters.record_derived_bulk_resume_rejection();
        let _rejection = BulkResumeCompatibilityRejection::new(
            artifact.family_id().clone(),
            BulkResumeInterpretation::ChangedInterpretationRejected,
        );
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::BulkResumeCompatibilityRejected,
            artifact.family_id().clone(),
            "bulk resume support cannot resume under changed semantic interpretation",
        ));
    }
    counters.record_derived_lane_reuse();
    Ok(BulkResumeCompatibilityPlan::new(
        artifact.family_id().clone(),
        BulkResumeInterpretation::NativeResume,
    ))
}

pub(crate) fn plan_tier_manifest_compatibility(
    counters: &mut CompatibilityAdmissionCounters,
    lane: &DerivedCompatibilityLaneDeclaration,
    artifact: &QuarantinedDecodedArtifact,
    read_receipt: &ReadCompatibilityReceipt,
) -> Result<TierManifestCompatibilityPlan, CompatibilityRejection> {
    if lane.lane_kind() != DerivedCompatibilityLaneKind::TierManifestSupport
        || lane.family_id() != artifact.family_id()
    {
        counters.record_tier_manifest_rejection();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::TierManifestCompatibilityRejected,
            artifact.family_id().clone(),
            "tier manifest compatibility requires the tier manifest lane",
        ));
    }
    if read_receipt.receipt().relation() != CompatibilityRelation::Native
        || !lane
            .required_window()
            .contains_format(artifact.format_version())
        || !lane
            .required_window()
            .contains_semantic(artifact.semantic_version())
    {
        counters.record_tier_manifest_rejection();
        let _rejection = TierManifestCompatibilityRejection::new(
            artifact.family_id().clone(),
            "tier manifest semantic drift rejected",
        );
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::TierManifestCompatibilityRejected,
            artifact.family_id().clone(),
            "tier manifest compatibility preserves placement non-authority by rejecting drift",
        ));
    }
    counters.record_tier_non_authority_preserved();
    Ok(TierManifestCompatibilityPlan::new(
        artifact.family_id().clone(),
    ))
}

fn plan_invalidation(
    counters: &mut CompatibilityAdmissionCounters,
    artifact: &QuarantinedDecodedArtifact,
    required_window: ArtifactCompatibilityWindow,
    reason_code: DerivedInvalidationReason,
    reason: &'static str,
) -> DerivedBasisCompatibilityPlan {
    counters.record_derived_invalidation();
    counters.record_derived_rebuild_required();
    DerivedBasisCompatibilityPlan::rebuild(DerivedInvalidationPlan::new(
        artifact.family_id().clone(),
        artifact.format_version(),
        artifact.semantic_version(),
        required_window,
        reason_code,
        reason,
    ))
}

pub(crate) fn prove_retained_authority_for_derived_rebuild(
    family_id: ArtifactFamilyId,
) -> RetainedAuthorityCompatibilityWitness {
    RetainedAuthorityCompatibilityWitness::new(family_id)
}

pub(crate) fn prove_maintenance_admission_for_derived_rebuild(
    counters: &mut CompatibilityAdmissionCounters,
    family_id: ArtifactFamilyId,
    maintenance_lane_id: impl Into<String>,
) -> CompatibilityMaintenanceAdmissionWitness {
    counters.record_maintenance_compatibility_rebuild_admission();
    CompatibilityMaintenanceAdmissionWitness::new(family_id, maintenance_lane_id)
}

pub(crate) fn prove_compatibility_maintenance_lane_admission(
    counters: &mut CompatibilityAdmissionCounters,
    requirement: &CompatibilityMaintenanceLaneRequirement,
    maintenance_lane_id: impl Into<String>,
) -> CompatibilityMaintenanceLaneAdmission {
    counters.record_maintenance_compatibility_rebuild_admission();
    CompatibilityMaintenanceLaneAdmission::new(CompatibilityMaintenanceAdmissionWitness::for_lane(
        requirement.family_id.clone(),
        requirement.compatibility_lane_id.clone(),
        maintenance_lane_id,
        requirement.maintenance_work_class_label.clone(),
    ))
}

pub(crate) fn require_matching_maintenance_lane(
    counters: &mut CompatibilityAdmissionCounters,
    requirement: &CompatibilityMaintenanceLaneRequirement,
    admission: &CompatibilityMaintenanceLaneAdmission,
) -> Result<(), CompatibilityRejection> {
    let witness = admission.witness();
    if witness.family_id() != requirement.family_id()
        || witness.compatibility_lane_id() != requirement.compatibility_lane_id
        || witness.maintenance_work_class_label() != requirement.maintenance_work_class_label
    {
        counters.record_maintenance_lane_mismatch_rejection();
        let _rejection = CompatibilityMaintenanceLaneRejection::new(
            requirement.family_id().clone(),
            "maintenance lane admission does not match compatibility lane requirement",
        );
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::MaintenanceLaneMismatch,
            requirement.family_id().clone(),
            "maintenance lane admission does not match compatibility family, lane, or work class",
        ));
    }
    Ok(())
}

pub(crate) fn defer_derived_rebuild(
    counters: &mut CompatibilityAdmissionCounters,
    requirement: &DerivedRebuildRequirement,
    debt_record_count: u64,
) -> CompatibilityRebuildDebt {
    counters.record_derived_rebuild_debt(debt_record_count);
    CompatibilityRebuildDebt::new(requirement.family_id().clone(), debt_record_count)
}

pub(crate) fn admit_derived_rebuild_maintenance(
    counters: &mut CompatibilityAdmissionCounters,
    requirement: &DerivedRebuildRequirement,
    retained_authority: Option<&RetainedAuthorityCompatibilityWitness>,
    maintenance_admission: Option<&CompatibilityMaintenanceAdmissionWitness>,
) -> Result<DerivedRebuildCompatibilityPlan, CompatibilityRejection> {
    let Some(retained_authority) = retained_authority else {
        counters.record_derived_stale_version_rejection();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::DerivedStaleVersion,
            requirement.family_id().clone(),
            "derived rebuild requires retained authoritative basis proof",
        ));
    };
    if retained_authority.family_id() != requirement.family_id() {
        counters.record_derived_rebuild_incompatible();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::DerivedBasisIncompatible,
            requirement.family_id().clone(),
            "retained authoritative basis proof belongs to a different family",
        ));
    }

    let Some(maintenance_admission) = maintenance_admission else {
        counters.record_maintenance_compatibility_rebuild_rejection();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::DerivedRebuildAdmissionRejected,
            requirement.family_id().clone(),
            "derived rebuild requires Milestone 11 maintenance admission proof",
        ));
    };
    if maintenance_admission.family_id() != requirement.family_id() {
        counters.record_derived_rebuild_incompatible();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::DerivedRebuildAdmissionRejected,
            requirement.family_id().clone(),
            "maintenance admission proof belongs to a different family",
        ));
    }

    Ok(DerivedRebuildCompatibilityPlan::new(
        requirement.family_id().clone(),
        requirement.observed_semantic_version(),
        requirement.target_semantic_version(),
        maintenance_admission.maintenance_lane_id().to_string(),
    ))
}
