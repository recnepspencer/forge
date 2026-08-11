use serde::Serialize;

use super::super::catalog::DerivedFamilyDeclaration;

use super::super::manifests::{ArtifactCompatibilityWindow, ArtifactFamilyId};

use super::declarations::{
    DerivedBasisCompatibilityPosture, DerivedCompatibilityLaneDeclaration,
    DerivedCompatibilityLaneKind,
};

use super::reuse::{
    DerivedBasisCompatibilityPlan, DerivedInvalidationPlan, DerivedLaneCompatibilityPosture,
    DerivedRebuildRequirement,
};

pub struct DerivedBasisCompatibilityInput {
    lane_declaration: DerivedCompatibilityLaneDeclaration,
    pub(super) derived_family: DerivedFamilyDeclaration,
    pub(super) required_window: ArtifactCompatibilityWindow,
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
    pub(super) fn from_basis(
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

    pub(super) fn from_bulk_resume(
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

    pub(super) fn from_tier_manifest(
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
