use serde::Serialize;

use super::super::admission::{
    CompatibilityAdmissionCounters, CompatibilityDecision, CompatibilityRejection,
    CompatibilityRejectionKind, CompatibilityRelation, DerivedReuseCompatibilityReceipt,
    ReadCompatibilityReceipt,
};

use super::super::catalog::DerivedFamilyDeclaration;

use super::super::decoding::{CompatibilityCheckedArtifact, QuarantinedDecodedArtifact};

use super::super::manifests::{
    ArtifactCompatibilityWindow, ArtifactFamilyId, ArtifactFormatVersion, ArtifactSemanticVersion,
    CompatibilityManifestDigest,
};

use super::declarations::{
    DerivedBasisCompatibilityPosture, DerivedCompatibilityReuseWitness, DerivedInvalidationReason,
    DerivedReusePosture,
};

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

    pub(super) fn rebuild(
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
    pub(super) fn reusable(family_id: ArtifactFamilyId) -> Self {
        Self {
            family_id,
            posture: DerivedBasisCompatibilityPosture::ReuseStillValid,
            invalidation: None,
            rebuild_requirement: None,
        }
    }

    pub(super) fn rebuild(invalidation: DerivedInvalidationPlan) -> Self {
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
