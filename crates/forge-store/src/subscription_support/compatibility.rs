use super::{
    classification_error, cost_surface_for_program_path, stable_digest,
    CompletedSupportProgramAction, SubscriptionSupportActionOrigin, SubscriptionSupportArtifactId,
    SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind,
    SubscriptionSupportOperationalBasis, SubscriptionSupportOperationalVerdict,
    SubscriptionSupportPlanFamily, SubscriptionSupportResultCostSurface, SubscriptionSupportRole,
    SupportActionId, SupportAffectedSetDigest, SupportProgramPathPlan,
};
use crate::{
    failure::StoreError, ArtifactFamilyId, CompatibilityReadAdmissionOutcome,
    CompatibilityRejectionKind, CompatibilityRelation, ReadCompatibilityReceipt,
};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportFamilyVersionWindow {
    family_id: SubscriptionSupportFamilyId,
    family_kind: SubscriptionSupportFamilyKind,
    minimum_reader_version: u16,
    maximum_payload_version: u16,
}

#[allow(dead_code)]
impl SupportFamilyVersionWindow {
    pub(crate) fn new(
        family_id: SubscriptionSupportFamilyId,
        family_kind: SubscriptionSupportFamilyKind,
        minimum_reader_version: u16,
        maximum_payload_version: u16,
    ) -> Result<Self, StoreError> {
        if minimum_reader_version == 0 || maximum_payload_version == 0 {
            return Err(classification_error(
                "subscription-support compatibility version windows require non-zero versions",
            ));
        }
        if minimum_reader_version > maximum_payload_version {
            return Err(classification_error(
                "subscription-support compatibility version windows cannot require a reader newer than the payload",
            ));
        }
        Ok(Self {
            family_id,
            family_kind,
            minimum_reader_version,
            maximum_payload_version,
        })
    }

    pub fn family_id(&self) -> &SubscriptionSupportFamilyId {
        &self.family_id
    }

    pub fn family_kind(&self) -> SubscriptionSupportFamilyKind {
        self.family_kind
    }

    pub fn minimum_reader_version(&self) -> u16 {
        self.minimum_reader_version
    }

    pub fn maximum_payload_version(&self) -> u16 {
        self.maximum_payload_version
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportCompatibilityReceiptWitness {
    support_family_id: SubscriptionSupportFamilyId,
    support_family_kind: SubscriptionSupportFamilyKind,
    milestone12_family_id: ArtifactFamilyId,
    version_window: SupportFamilyVersionWindow,
    manifest_digest: String,
    registry_snapshot_identity: Option<String>,
    manifest_frontier_identity: Option<String>,
    relation: Option<CompatibilityRelation>,
    rejection_kind: Option<CompatibilityRejectionKind>,
    receipt_digest: String,
}

#[allow(dead_code)]
impl SupportCompatibilityReceiptWitness {
    pub(crate) fn from_read_receipt(
        support_family_id: SubscriptionSupportFamilyId,
        support_family_kind: SubscriptionSupportFamilyKind,
        receipt: &ReadCompatibilityReceipt,
    ) -> Result<Self, StoreError> {
        let inner = receipt.receipt();
        if inner.family_id().as_str() != support_family_id.as_str() {
            return Err(classification_error(
                "subscription-support compatibility receipt witness must match support family id",
            ));
        }
        let observed = inner.observed_semantic_version().value();
        let target = inner.target_semantic_version().value();
        let minimum_reader_version = u16::try_from(observed.min(target)).map_err(|_| {
            classification_error(
                "subscription-support compatibility receipt semantic versions exceed support window range",
            )
        })?;
        let maximum_payload_version = u16::try_from(observed.max(target)).map_err(|_| {
            classification_error(
                "subscription-support compatibility receipt semantic versions exceed support window range",
            )
        })?;
        let version_window = SupportFamilyVersionWindow::new(
            support_family_id.clone(),
            support_family_kind,
            minimum_reader_version,
            maximum_payload_version,
        )?;
        let receipt_digest = stable_digest(inner)?;
        Ok(Self {
            support_family_id,
            support_family_kind,
            milestone12_family_id: inner.family_id().clone(),
            version_window,
            manifest_digest: inner.manifest_digest().as_str().to_string(),
            registry_snapshot_identity: Some(inner.registry_snapshot_identity().to_string()),
            manifest_frontier_identity: Some(inner.manifest_frontier_identity().to_string()),
            relation: Some(inner.relation()),
            rejection_kind: None,
            receipt_digest,
        })
    }

    pub(crate) fn from_read_admission_outcome(
        support_family_id: SubscriptionSupportFamilyId,
        support_family_kind: SubscriptionSupportFamilyKind,
        version_window: SupportFamilyVersionWindow,
        outcome: &CompatibilityReadAdmissionOutcome,
    ) -> Result<Self, StoreError> {
        if outcome.family_id().as_str() != support_family_id.as_str()
            || version_window.family_id() != &support_family_id
            || version_window.family_kind() != support_family_kind
        {
            return Err(classification_error(
                "subscription-support compatibility read outcome witness must match support family",
            ));
        }
        if outcome.is_accepted() && outcome.relation().is_none() {
            return Err(classification_error(
                "accepted subscription-support compatibility read outcomes require a relation",
            ));
        }
        if !outcome.is_accepted() && outcome.rejection_kind().is_none() {
            return Err(classification_error(
                "rejected subscription-support compatibility read outcomes require a typed rejection",
            ));
        }
        let receipt_digest = stable_digest(outcome)?;
        Ok(Self {
            support_family_id,
            support_family_kind,
            milestone12_family_id: outcome.family_id().clone(),
            version_window,
            manifest_digest: outcome.manifest_digest().as_str().to_string(),
            registry_snapshot_identity: None,
            manifest_frontier_identity: None,
            relation: outcome.relation(),
            rejection_kind: outcome.rejection_kind(),
            receipt_digest,
        })
    }

    pub fn support_family_id(&self) -> &SubscriptionSupportFamilyId {
        &self.support_family_id
    }

    pub fn support_family_kind(&self) -> SubscriptionSupportFamilyKind {
        self.support_family_kind
    }

    pub fn milestone12_family_id(&self) -> &ArtifactFamilyId {
        &self.milestone12_family_id
    }

    pub fn version_window(&self) -> &SupportFamilyVersionWindow {
        &self.version_window
    }

    pub fn manifest_digest(&self) -> &str {
        &self.manifest_digest
    }

    pub fn relation(&self) -> Option<CompatibilityRelation> {
        self.relation
    }

    pub fn rejection_kind(&self) -> Option<CompatibilityRejectionKind> {
        self.rejection_kind
    }

    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportManifestAdmissionWitness {
    version_window: SupportFamilyVersionWindow,
    manifest_digest: String,
    compatibility_digest: String,
    compatibility_receipt: SupportCompatibilityReceiptWitness,
}

#[allow(dead_code)]
impl SupportManifestAdmissionWitness {
    pub(crate) fn new(
        version_window: SupportFamilyVersionWindow,
        manifest_digest: impl Into<String>,
        compatibility_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        let manifest_digest = require_non_empty("manifest", manifest_digest)?;
        Ok(Self {
            compatibility_receipt: SupportCompatibilityReceiptWitness {
                support_family_id: version_window.family_id().clone(),
                support_family_kind: version_window.family_kind(),
                milestone12_family_id: ArtifactFamilyId::new(version_window.family_id().as_str()),
                version_window: version_window.clone(),
                manifest_digest: manifest_digest.clone(),
                registry_snapshot_identity: None,
                manifest_frontier_identity: None,
                relation: None,
                rejection_kind: None,
                receipt_digest: "unbound-legacy-support-compatibility-receipt".into(),
            },
            version_window,
            manifest_digest,
            compatibility_digest: require_non_empty("compatibility", compatibility_digest)?,
        })
    }

    pub(crate) fn from_compatibility_receipt(
        compatibility_receipt: SupportCompatibilityReceiptWitness,
        compatibility_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        let manifest_digest = compatibility_receipt.manifest_digest().to_string();
        Ok(Self {
            version_window: compatibility_receipt.version_window().clone(),
            manifest_digest,
            compatibility_digest: require_non_empty("compatibility", compatibility_digest)?,
            compatibility_receipt,
        })
    }

    pub fn version_window(&self) -> &SupportFamilyVersionWindow {
        &self.version_window
    }

    pub fn manifest_digest(&self) -> &str {
        &self.manifest_digest
    }

    pub fn compatibility_digest(&self) -> &str {
        &self.compatibility_digest
    }

    pub fn compatibility_receipt(&self) -> &SupportCompatibilityReceiptWitness {
        &self.compatibility_receipt
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportDecodedRowSemanticAccess {
    admission_witness: SupportManifestAdmissionWitness,
    semantic_digest: String,
}

impl SupportDecodedRowSemanticAccess {
    pub(crate) fn from_manifest_admission(
        admission_witness: SupportManifestAdmissionWitness,
        semantic_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            admission_witness,
            semantic_digest: require_non_empty("decoded semantic row", semantic_digest)?,
        })
    }

    pub fn admission_witness(&self) -> &SupportManifestAdmissionWitness {
        &self.admission_witness
    }

    pub fn semantic_digest(&self) -> &str {
        &self.semantic_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportCompatibilityAffectedSet {
    family_id: SubscriptionSupportFamilyId,
    family_kind: SubscriptionSupportFamilyKind,
    support_role: SubscriptionSupportRole,
    affected_set_digest: SupportAffectedSetDigest,
    affected_bases: Vec<SubscriptionSupportOperationalBasis>,
}

impl SupportCompatibilityAffectedSet {
    pub(crate) fn from_compatibility_bases(
        affected_bases: Vec<SubscriptionSupportOperationalBasis>,
    ) -> Result<Self, StoreError> {
        let Some(first) = affected_bases.first() else {
            return Err(classification_error(
                "subscription-support compatibility affected sets must not be empty",
            ));
        };
        if first.action_origin() != SubscriptionSupportActionOrigin::Compatibility {
            return Err(classification_error(
                "subscription-support compatibility affected sets require compatibility-origin bases",
            ));
        }
        for basis in &affected_bases {
            if basis.action_origin() != SubscriptionSupportActionOrigin::Compatibility {
                return Err(classification_error(
                    "subscription-support compatibility affected sets cannot mix action origins",
                ));
            }
            if basis.family_id() != first.family_id()
                || basis.family_kind() != first.family_kind()
                || basis.support_role() != first.support_role()
            {
                return Err(classification_error(
                    "subscription-support compatibility affected sets must be family-local",
                ));
            }
        }
        Ok(Self {
            family_id: first.family_id().clone(),
            family_kind: first.family_kind(),
            support_role: first.support_role(),
            affected_set_digest: SupportAffectedSetDigest::from_bases(&affected_bases)?,
            affected_bases,
        })
    }

    pub fn family_id(&self) -> &SubscriptionSupportFamilyId {
        &self.family_id
    }

    pub fn family_kind(&self) -> SubscriptionSupportFamilyKind {
        self.family_kind
    }

    pub fn support_role(&self) -> SubscriptionSupportRole {
        self.support_role
    }

    pub fn affected_count(&self) -> u64 {
        self.affected_bases.len() as u64
    }

    pub fn affected_set_digest(&self) -> &SupportAffectedSetDigest {
        &self.affected_set_digest
    }

    pub(crate) fn primary_basis(&self) -> &SubscriptionSupportOperationalBasis {
        &self.affected_bases[0]
    }

    pub(crate) fn affected_artifact_ids(&self) -> Vec<SubscriptionSupportArtifactId> {
        self.affected_bases
            .iter()
            .map(|basis| basis.artifact_id().clone())
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportCompatibilityDecision {
    evidence: SubscriptionSupportCompatibilityDecisionEvidence,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
enum SubscriptionSupportCompatibilityDecisionEvidence {
    ExactCompatibleMigration {
        classifier_equivalence_digest: String,
    },
    DegradedCompatibility {
        drift_reason: String,
    },
    OldReaderRejected {
        reader_version: u16,
        required_minimum_version: u16,
    },
    UnknownFamilyRejected {
        family_id: SubscriptionSupportFamilyId,
    },
    VersionSkewRejected {
        skew_reason: String,
    },
}

#[allow(dead_code)]
impl SubscriptionSupportCompatibilityDecision {
    pub(crate) fn exact_compatible_migration(
        classifier_equivalence_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(
            SubscriptionSupportCompatibilityDecisionEvidence::ExactCompatibleMigration {
                classifier_equivalence_digest: require_non_empty(
                    "classifier equivalence",
                    classifier_equivalence_digest,
                )?,
            }
            .into(),
        )
    }

    pub(crate) fn degraded_compatibility(
        drift_reason: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(
            SubscriptionSupportCompatibilityDecisionEvidence::DegradedCompatibility {
                drift_reason: require_non_empty("compatibility drift reason", drift_reason)?,
            }
            .into(),
        )
    }

    pub(crate) fn old_reader_rejected(
        reader_version: u16,
        required_minimum_version: u16,
    ) -> Result<Self, StoreError> {
        if reader_version == 0 || required_minimum_version == 0 {
            return Err(classification_error(
                "subscription-support old-reader rejection requires non-zero versions",
            ));
        }
        if reader_version >= required_minimum_version {
            return Err(classification_error(
                "subscription-support old-reader rejection requires a reader below the admitted window",
            ));
        }
        Ok(
            SubscriptionSupportCompatibilityDecisionEvidence::OldReaderRejected {
                reader_version,
                required_minimum_version,
            }
            .into(),
        )
    }

    pub(crate) fn unknown_family_rejected(family_id: SubscriptionSupportFamilyId) -> Self {
        SubscriptionSupportCompatibilityDecisionEvidence::UnknownFamilyRejected { family_id }.into()
    }

    pub(crate) fn version_skew_rejected(
        skew_reason: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(
            SubscriptionSupportCompatibilityDecisionEvidence::VersionSkewRejected {
                skew_reason: require_non_empty("version-skew rejection reason", skew_reason)?,
            }
            .into(),
        )
    }

    pub fn verdict(&self) -> SubscriptionSupportOperationalVerdict {
        match &self.evidence {
            SubscriptionSupportCompatibilityDecisionEvidence::ExactCompatibleMigration {
                ..
            } => SubscriptionSupportOperationalVerdict::ExactResumePreserved,
            SubscriptionSupportCompatibilityDecisionEvidence::DegradedCompatibility { .. } => {
                SubscriptionSupportOperationalVerdict::DegradedResumePreserved
            }
            SubscriptionSupportCompatibilityDecisionEvidence::OldReaderRejected { .. }
            | SubscriptionSupportCompatibilityDecisionEvidence::UnknownFamilyRejected { .. }
            | SubscriptionSupportCompatibilityDecisionEvidence::VersionSkewRejected { .. } => {
                SubscriptionSupportOperationalVerdict::RejectedByPolicy
            }
        }
    }

    pub fn kind(&self) -> SubscriptionSupportCompatibilityDecisionKind {
        match &self.evidence {
            SubscriptionSupportCompatibilityDecisionEvidence::ExactCompatibleMigration {
                ..
            } => SubscriptionSupportCompatibilityDecisionKind::ExactCompatibleMigration,
            SubscriptionSupportCompatibilityDecisionEvidence::DegradedCompatibility { .. } => {
                SubscriptionSupportCompatibilityDecisionKind::DegradedCompatibility
            }
            SubscriptionSupportCompatibilityDecisionEvidence::OldReaderRejected { .. } => {
                SubscriptionSupportCompatibilityDecisionKind::OldReaderRejected
            }
            SubscriptionSupportCompatibilityDecisionEvidence::UnknownFamilyRejected { .. } => {
                SubscriptionSupportCompatibilityDecisionKind::UnknownFamilyRejected
            }
            SubscriptionSupportCompatibilityDecisionEvidence::VersionSkewRejected { .. } => {
                SubscriptionSupportCompatibilityDecisionKind::VersionSkewRejected
            }
        }
    }

    fn classifier_equivalence_digest(&self) -> Option<&str> {
        match &self.evidence {
            SubscriptionSupportCompatibilityDecisionEvidence::ExactCompatibleMigration {
                classifier_equivalence_digest,
            } => Some(classifier_equivalence_digest),
            _ => None,
        }
    }

    fn drift_reason(&self) -> Option<&str> {
        match &self.evidence {
            SubscriptionSupportCompatibilityDecisionEvidence::DegradedCompatibility {
                drift_reason,
            }
            | SubscriptionSupportCompatibilityDecisionEvidence::VersionSkewRejected {
                skew_reason: drift_reason,
            } => Some(drift_reason),
            SubscriptionSupportCompatibilityDecisionEvidence::OldReaderRejected {
                reader_version,
                required_minimum_version,
            } => Some(if reader_version < required_minimum_version {
                "reader below admitted support manifest window"
            } else {
                "invalid old-reader compatibility rejection"
            }),
            SubscriptionSupportCompatibilityDecisionEvidence::UnknownFamilyRejected { .. } => {
                Some("unknown subscription-support family")
            }
            _ => None,
        }
    }
}

impl From<SubscriptionSupportCompatibilityDecisionEvidence>
    for SubscriptionSupportCompatibilityDecision
{
    fn from(evidence: SubscriptionSupportCompatibilityDecisionEvidence) -> Self {
        Self { evidence }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SubscriptionSupportCompatibilityDecisionKind {
    ExactCompatibleMigration,
    DegradedCompatibility,
    OldReaderRejected,
    UnknownFamilyRejected,
    VersionSkewRejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum SubscriptionSupportCompatibilityOutcome {
    ExactMigrated(ExactCompatibleSupportMigration),
    Degraded(DegradedCompatibleSupportPosture),
    Rejected(SupportVersionSkewRejection),
}

impl SubscriptionSupportCompatibilityOutcome {
    pub fn outcome_kind(&self) -> SubscriptionSupportCompatibilityDecisionKind {
        match self {
            Self::ExactMigrated(_) => {
                SubscriptionSupportCompatibilityDecisionKind::ExactCompatibleMigration
            }
            Self::Degraded(_) => {
                SubscriptionSupportCompatibilityDecisionKind::DegradedCompatibility
            }
            Self::Rejected(rejection) => rejection.rejection_kind(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExactCompatibleSupportMigration {
    affected_set_digest: SupportAffectedSetDigest,
    manifest_digest: String,
    compatibility_digest: String,
    milestone12_receipt_digest: String,
    milestone12_relation: CompatibilityRelation,
    classifier_equivalence_digest: String,
    migrated_artifact_ids: Vec<SubscriptionSupportArtifactId>,
}

impl ExactCompatibleSupportMigration {
    pub fn affected_set_digest(&self) -> &SupportAffectedSetDigest {
        &self.affected_set_digest
    }

    pub fn manifest_digest(&self) -> &str {
        &self.manifest_digest
    }

    pub fn compatibility_digest(&self) -> &str {
        &self.compatibility_digest
    }

    pub fn classifier_equivalence_digest(&self) -> &str {
        &self.classifier_equivalence_digest
    }

    pub fn milestone12_relation(&self) -> CompatibilityRelation {
        self.milestone12_relation
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DegradedCompatibleSupportPosture {
    affected_set_digest: SupportAffectedSetDigest,
    manifest_digest: String,
    compatibility_digest: String,
    milestone12_receipt_digest: String,
    milestone12_relation: CompatibilityRelation,
    drift_reason: String,
    degraded_artifact_ids: Vec<SubscriptionSupportArtifactId>,
}

impl DegradedCompatibleSupportPosture {
    pub fn affected_set_digest(&self) -> &SupportAffectedSetDigest {
        &self.affected_set_digest
    }

    pub fn drift_reason(&self) -> &str {
        &self.drift_reason
    }

    pub fn milestone12_relation(&self) -> CompatibilityRelation {
        self.milestone12_relation
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportVersionSkewRejection {
    rejection_kind: SubscriptionSupportCompatibilityDecisionKind,
    affected_set_digest: SupportAffectedSetDigest,
    manifest_digest: String,
    compatibility_digest: String,
    milestone12_receipt_digest: String,
    milestone12_rejection_kind: Option<CompatibilityRejectionKind>,
    rejection_reason: String,
    rejected_artifact_ids: Vec<SubscriptionSupportArtifactId>,
}

impl SupportVersionSkewRejection {
    pub fn rejection_kind(&self) -> SubscriptionSupportCompatibilityDecisionKind {
        self.rejection_kind
    }

    pub fn rejection_reason(&self) -> &str {
        &self.rejection_reason
    }

    pub fn milestone12_rejection_kind(&self) -> Option<CompatibilityRejectionKind> {
        self.milestone12_rejection_kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportCompatibilityBatchPlan {
    action_id: SupportActionId,
    affected_set: SupportCompatibilityAffectedSet,
    path_plan: SupportProgramPathPlan,
    manifest_admission: SupportManifestAdmissionWitness,
    semantic_access: SupportDecodedRowSemanticAccess,
    decision: SubscriptionSupportCompatibilityDecision,
}

impl SupportCompatibilityBatchPlan {
    pub(crate) fn new(
        action_id: SupportActionId,
        affected_set: SupportCompatibilityAffectedSet,
        path_plan: SupportProgramPathPlan,
        manifest_admission: SupportManifestAdmissionWitness,
        semantic_access: SupportDecodedRowSemanticAccess,
        decision: SubscriptionSupportCompatibilityDecision,
    ) -> Result<Self, StoreError> {
        if affected_set.family_id() != manifest_admission.version_window().family_id()
            || affected_set.family_kind() != manifest_admission.version_window().family_kind()
        {
            return Err(classification_error(
                "subscription-support compatibility batch manifest admission must match affected family",
            ));
        }
        if manifest_admission != *semantic_access.admission_witness() {
            return Err(classification_error(
                "decoded subscription-support semantic access requires the same manifest admission witness",
            ));
        }
        validate_decision_against_receipt(&decision, manifest_admission.compatibility_receipt())?;
        Ok(Self {
            action_id,
            affected_set,
            path_plan,
            manifest_admission,
            semantic_access,
            decision,
        })
    }

    pub fn affected_set(&self) -> &SupportCompatibilityAffectedSet {
        &self.affected_set
    }

    pub fn path_plan(&self) -> &SupportProgramPathPlan {
        &self.path_plan
    }

    pub fn manifest_admission(&self) -> &SupportManifestAdmissionWitness {
        &self.manifest_admission
    }

    pub fn semantic_access(&self) -> &SupportDecodedRowSemanticAccess {
        &self.semantic_access
    }

    pub fn decision(&self) -> &SubscriptionSupportCompatibilityDecision {
        &self.decision
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        SupportActionId,
        SupportCompatibilityAffectedSet,
        SupportProgramPathPlan,
        SupportManifestAdmissionWitness,
        SupportDecodedRowSemanticAccess,
        SubscriptionSupportCompatibilityDecision,
    ) {
        (
            self.action_id,
            self.affected_set,
            self.path_plan,
            self.manifest_admission,
            self.semantic_access,
            self.decision,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportCompatibilityParticipationRecord {
    action_id: SupportActionId,
    affected_set_digest: SupportAffectedSetDigest,
    manifest_digest: String,
    compatibility_digest: String,
    milestone12_receipt_digest: String,
    milestone12_relation: Option<CompatibilityRelation>,
    milestone12_rejection_kind: Option<CompatibilityRejectionKind>,
    decision_kind: SubscriptionSupportCompatibilityDecisionKind,
    semantic_digest: String,
}

impl SupportCompatibilityParticipationRecord {
    fn new(
        completed_action: &CompletedSupportProgramAction,
        affected_set: &SupportCompatibilityAffectedSet,
        manifest_admission: &SupportManifestAdmissionWitness,
        semantic_access: &SupportDecodedRowSemanticAccess,
        decision_kind: SubscriptionSupportCompatibilityDecisionKind,
    ) -> Result<Self, StoreError> {
        if completed_action.envelope().action_origin()
            != SubscriptionSupportActionOrigin::Compatibility
        {
            return Err(classification_error(
                "subscription-support compatibility participation records require compatibility-origin actions",
            ));
        }
        if manifest_admission != semantic_access.admission_witness() {
            return Err(classification_error(
                "subscription-support compatibility records require admitted semantic access",
            ));
        }
        Ok(Self {
            action_id: completed_action.envelope().action_id().clone(),
            affected_set_digest: affected_set.affected_set_digest().clone(),
            manifest_digest: manifest_admission.manifest_digest().to_string(),
            compatibility_digest: manifest_admission.compatibility_digest().to_string(),
            milestone12_receipt_digest: manifest_admission
                .compatibility_receipt()
                .receipt_digest()
                .to_string(),
            milestone12_relation: manifest_admission.compatibility_receipt().relation(),
            milestone12_rejection_kind: manifest_admission.compatibility_receipt().rejection_kind(),
            decision_kind,
            semantic_digest: semantic_access.semantic_digest().to_string(),
        })
    }

    pub fn affected_set_digest(&self) -> &SupportAffectedSetDigest {
        &self.affected_set_digest
    }

    pub fn decision_kind(&self) -> SubscriptionSupportCompatibilityDecisionKind {
        self.decision_kind
    }

    pub fn milestone12_receipt_digest(&self) -> &str {
        &self.milestone12_receipt_digest
    }

    pub fn milestone12_relation(&self) -> Option<CompatibilityRelation> {
        self.milestone12_relation
    }

    pub fn milestone12_rejection_kind(&self) -> Option<CompatibilityRejectionKind> {
        self.milestone12_rejection_kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportCompatibilityReport {
    completed_action: CompletedSupportProgramAction,
    participation_record: SupportCompatibilityParticipationRecord,
    outcome: SubscriptionSupportCompatibilityOutcome,
    cost_surface: SubscriptionSupportResultCostSurface,
}

impl SubscriptionSupportCompatibilityReport {
    pub(crate) fn new(
        completed_action: CompletedSupportProgramAction,
        affected_set: SupportCompatibilityAffectedSet,
        path_plan: &SupportProgramPathPlan,
        manifest_admission: SupportManifestAdmissionWitness,
        semantic_access: SupportDecodedRowSemanticAccess,
        decision: &SubscriptionSupportCompatibilityDecision,
    ) -> Result<Self, StoreError> {
        let decision_kind = decision.kind();
        let participation_record = SupportCompatibilityParticipationRecord::new(
            &completed_action,
            &affected_set,
            &manifest_admission,
            &semantic_access,
            decision_kind,
        )?;
        let outcome = outcome_from_decision(affected_set, manifest_admission, decision)?;
        if outcome.outcome_kind() != decision_kind {
            return Err(classification_error(
                "subscription-support compatibility outcome kind must match decision kind",
            ));
        }
        Ok(Self {
            completed_action,
            participation_record,
            outcome,
            cost_surface: cost_surface_for_program_path(
                SubscriptionSupportPlanFamily::CompatibilityParticipationPlan,
                path_plan,
            ),
        })
    }

    pub fn completed_action(&self) -> &CompletedSupportProgramAction {
        &self.completed_action
    }

    pub fn participation_record(&self) -> &SupportCompatibilityParticipationRecord {
        &self.participation_record
    }

    pub fn outcome(&self) -> &SubscriptionSupportCompatibilityOutcome {
        &self.outcome
    }

    pub fn cost_surface(&self) -> SubscriptionSupportResultCostSurface {
        self.cost_surface
    }
}

fn outcome_from_decision(
    affected_set: SupportCompatibilityAffectedSet,
    manifest_admission: SupportManifestAdmissionWitness,
    decision: &SubscriptionSupportCompatibilityDecision,
) -> Result<SubscriptionSupportCompatibilityOutcome, StoreError> {
    let affected_set_digest = affected_set.affected_set_digest().clone();
    let artifact_ids = affected_set.affected_artifact_ids();
    let receipt = manifest_admission.compatibility_receipt();
    match decision.kind() {
        SubscriptionSupportCompatibilityDecisionKind::ExactCompatibleMigration => {
            Ok(SubscriptionSupportCompatibilityOutcome::ExactMigrated(
                ExactCompatibleSupportMigration {
                    affected_set_digest,
                    manifest_digest: manifest_admission.manifest_digest().to_string(),
                    compatibility_digest: manifest_admission.compatibility_digest().to_string(),
                    milestone12_receipt_digest: receipt.receipt_digest().to_string(),
                    milestone12_relation: receipt.relation().ok_or_else(|| {
                        classification_error(
                            "exact compatible support migration requires an accepted Milestone 12 relation",
                        )
                    })?,
                    classifier_equivalence_digest: decision
                        .classifier_equivalence_digest()
                        .ok_or_else(|| {
                            classification_error(
                                "exact compatible support migration requires classifier equivalence evidence",
                            )
                        })?
                        .to_string(),
                    migrated_artifact_ids: artifact_ids,
                },
            ))
        }
        SubscriptionSupportCompatibilityDecisionKind::DegradedCompatibility => {
            Ok(SubscriptionSupportCompatibilityOutcome::Degraded(
                DegradedCompatibleSupportPosture {
                    affected_set_digest,
                    manifest_digest: manifest_admission.manifest_digest().to_string(),
                    compatibility_digest: manifest_admission.compatibility_digest().to_string(),
                    milestone12_receipt_digest: receipt.receipt_digest().to_string(),
                    milestone12_relation: receipt.relation().ok_or_else(|| {
                        classification_error(
                            "degraded compatible support posture requires an accepted Milestone 12 relation",
                        )
                    })?,
                    drift_reason: decision
                        .drift_reason()
                        .ok_or_else(|| {
                            classification_error(
                                "degraded compatible support posture requires drift evidence",
                            )
                        })?
                        .to_string(),
                    degraded_artifact_ids: artifact_ids,
                },
            ))
        }
        SubscriptionSupportCompatibilityDecisionKind::OldReaderRejected
        | SubscriptionSupportCompatibilityDecisionKind::UnknownFamilyRejected
        | SubscriptionSupportCompatibilityDecisionKind::VersionSkewRejected => {
            Ok(SubscriptionSupportCompatibilityOutcome::Rejected(
                SupportVersionSkewRejection {
                    rejection_kind: decision.kind(),
                    affected_set_digest,
                    manifest_digest: manifest_admission.manifest_digest().to_string(),
                    compatibility_digest: manifest_admission.compatibility_digest().to_string(),
                    milestone12_receipt_digest: receipt.receipt_digest().to_string(),
                    milestone12_rejection_kind: receipt.rejection_kind(),
                    rejection_reason: decision
                        .drift_reason()
                        .ok_or_else(|| {
                            classification_error(
                                "version-skew support rejection requires typed rejection evidence",
                            )
                        })?
                        .to_string(),
                    rejected_artifact_ids: artifact_ids,
                },
            ))
        }
    }
}

fn validate_decision_against_receipt(
    decision: &SubscriptionSupportCompatibilityDecision,
    receipt: &SupportCompatibilityReceiptWitness,
) -> Result<(), StoreError> {
    match decision.kind() {
        SubscriptionSupportCompatibilityDecisionKind::ExactCompatibleMigration => {
            match receipt.relation() {
                Some(
                    CompatibilityRelation::Native
                    | CompatibilityRelation::BackwardRead
                    | CompatibilityRelation::ForwardRead,
                ) if receipt.rejection_kind().is_none() => Ok(()),
                _ => Err(classification_error(
                    "exact support compatibility migration requires a native/forward/backward Milestone 12 read receipt",
                )),
            }
        }
        SubscriptionSupportCompatibilityDecisionKind::DegradedCompatibility => {
            match receipt.relation() {
                Some(
                    CompatibilityRelation::AdapterRequired
                    | CompatibilityRelation::DerivedRebuildRequired,
                ) if receipt.rejection_kind().is_none() => Ok(()),
                _ => Err(classification_error(
                    "degraded support compatibility requires an admitted adapter or rebuild-required Milestone 12 relation",
                )),
            }
        }
        SubscriptionSupportCompatibilityDecisionKind::OldReaderRejected
        | SubscriptionSupportCompatibilityDecisionKind::UnknownFamilyRejected
        | SubscriptionSupportCompatibilityDecisionKind::VersionSkewRejected => {
            if receipt.rejection_kind().is_some() && receipt.relation().is_none() {
                Ok(())
            } else {
                Err(classification_error(
                    "support compatibility rejection requires a rejected Milestone 12 read admission outcome",
                ))
            }
        }
    }
}

fn require_non_empty(label: &'static str, value: impl Into<String>) -> Result<String, StoreError> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(classification_error(format!(
            "subscription-support compatibility {label} evidence must be non-empty"
        )));
    }
    Ok(value)
}

#[allow(dead_code)]
fn _digest_for_semantic_access(
    access: &SupportDecodedRowSemanticAccess,
) -> Result<String, StoreError> {
    stable_digest(access)
}
