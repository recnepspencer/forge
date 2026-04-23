use super::{
    classification_error, cost_surface_for_program_path, stable_digest,
    CompletedSupportProgramAction, SubscriptionSupportActionOrigin, SubscriptionSupportArtifactId,
    SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind,
    SubscriptionSupportOperationalBasis, SubscriptionSupportOperationalVerdict,
    SubscriptionSupportPlanFamily, SubscriptionSupportResultCostSurface, SubscriptionSupportRole,
    SupportActionId, SupportProgramDensityClass, SupportProgramPathPlan,
};
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportAffectedSet {
    family_id: SubscriptionSupportFamilyId,
    family_kind: SubscriptionSupportFamilyKind,
    support_role: SubscriptionSupportRole,
    affected_set_digest: SupportAffectedSetDigest,
    affected_bases: Vec<SubscriptionSupportOperationalBasis>,
}

impl SupportAffectedSet {
    pub(crate) fn from_retention_bases(
        affected_bases: Vec<SubscriptionSupportOperationalBasis>,
    ) -> Result<Self, StoreError> {
        let Some(first) = affected_bases.first() else {
            return Err(classification_error(
                "subscription-support retention affected sets must not be empty",
            ));
        };
        if first.action_origin() != SubscriptionSupportActionOrigin::Retention {
            return Err(classification_error(
                "subscription-support retention affected sets require retention-origin bases",
            ));
        }
        for basis in &affected_bases {
            if basis.action_origin() != SubscriptionSupportActionOrigin::Retention {
                return Err(classification_error(
                    "subscription-support retention affected sets cannot mix action origins",
                ));
            }
            if basis.family_id() != first.family_id()
                || basis.family_kind() != first.family_kind()
                || basis.support_role() != first.support_role()
            {
                return Err(classification_error(
                    "subscription-support retention affected sets must be family-local",
                ));
            }
        }
        let affected_set_digest = SupportAffectedSetDigest::from_bases(&affected_bases)?;
        Ok(Self {
            family_id: first.family_id().clone(),
            family_kind: first.family_kind(),
            support_role: first.support_role(),
            affected_set_digest,
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct SupportAffectedSetDigest(String);

impl SupportAffectedSetDigest {
    pub(crate) fn from_bases(
        affected_bases: &[SubscriptionSupportOperationalBasis],
    ) -> Result<Self, StoreError> {
        Ok(Self(stable_digest(&affected_bases)?))
    }

    pub(crate) fn from_persisted(value: impl Into<String>) -> Result<Self, StoreError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(classification_error(
                "subscription-support affected-set digests must be non-empty",
            ));
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportRetentionDecision {
    evidence: SubscriptionSupportRetentionDecisionEvidence,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
enum SubscriptionSupportRetentionDecisionEvidence {
    RetainExact,
    RetainDegraded {
        weakened_condition: String,
    },
    CompactExact {
        compacted_basis_digest: String,
    },
    ReclaimWithRebuild {
        retained_rebuild_basis_digest: String,
        maintenance_admission_key: String,
    },
    ReclaimWithoutRebuild {
        missing_rebuild_basis_reason: String,
    },
    ExpireByPolicy {
        policy_reason: String,
    },
}

#[allow(dead_code)]
impl SubscriptionSupportRetentionDecision {
    pub(crate) fn retain_exact() -> Self {
        Self {
            evidence: SubscriptionSupportRetentionDecisionEvidence::RetainExact,
        }
    }

    pub(crate) fn retain_degraded(
        weakened_condition: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(
            SubscriptionSupportRetentionDecisionEvidence::RetainDegraded {
                weakened_condition: require_non_empty(
                    "weakened support condition",
                    weakened_condition,
                )?,
            }
            .into(),
        )
    }

    pub(crate) fn compact_exact(
        compacted_basis_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(SubscriptionSupportRetentionDecisionEvidence::CompactExact {
            compacted_basis_digest: require_non_empty(
                "compacted support basis",
                compacted_basis_digest,
            )?,
        }
        .into())
    }

    pub(crate) fn reclaim_with_rebuild(
        retained_rebuild_basis_digest: impl Into<String>,
        maintenance_admission_key: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(
            SubscriptionSupportRetentionDecisionEvidence::ReclaimWithRebuild {
                retained_rebuild_basis_digest: require_non_empty(
                    "retained rebuild basis",
                    retained_rebuild_basis_digest,
                )?,
                maintenance_admission_key: require_non_empty(
                    "maintenance admission",
                    maintenance_admission_key,
                )?,
            }
            .into(),
        )
    }

    pub(crate) fn reclaim_without_rebuild(
        missing_rebuild_basis_reason: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(
            SubscriptionSupportRetentionDecisionEvidence::ReclaimWithoutRebuild {
                missing_rebuild_basis_reason: require_non_empty(
                    "missing rebuild basis reason",
                    missing_rebuild_basis_reason,
                )?,
            }
            .into(),
        )
    }

    pub(crate) fn expire_by_policy(policy_reason: impl Into<String>) -> Result<Self, StoreError> {
        Ok(
            SubscriptionSupportRetentionDecisionEvidence::ExpireByPolicy {
                policy_reason: require_non_empty("retention policy reason", policy_reason)?,
            }
            .into(),
        )
    }

    pub fn verdict(&self) -> SubscriptionSupportOperationalVerdict {
        match &self.evidence {
            SubscriptionSupportRetentionDecisionEvidence::RetainExact
            | SubscriptionSupportRetentionDecisionEvidence::CompactExact { .. } => {
                SubscriptionSupportOperationalVerdict::ExactResumePreserved
            }
            SubscriptionSupportRetentionDecisionEvidence::RetainDegraded { .. } => {
                SubscriptionSupportOperationalVerdict::DegradedResumePreserved
            }
            SubscriptionSupportRetentionDecisionEvidence::ReclaimWithRebuild { .. } => {
                SubscriptionSupportOperationalVerdict::RebuildRequired
            }
            SubscriptionSupportRetentionDecisionEvidence::ReclaimWithoutRebuild { .. } => {
                SubscriptionSupportOperationalVerdict::NotResumable
            }
            SubscriptionSupportRetentionDecisionEvidence::ExpireByPolicy { .. } => {
                SubscriptionSupportOperationalVerdict::RejectedByPolicy
            }
        }
    }

    pub fn is_reclaim(&self) -> bool {
        matches!(
            self.evidence,
            SubscriptionSupportRetentionDecisionEvidence::ReclaimWithRebuild { .. }
                | SubscriptionSupportRetentionDecisionEvidence::ReclaimWithoutRebuild { .. }
        )
    }

    pub fn kind(&self) -> SubscriptionSupportRetentionDecisionKind {
        match &self.evidence {
            SubscriptionSupportRetentionDecisionEvidence::RetainExact => {
                SubscriptionSupportRetentionDecisionKind::RetainExact
            }
            SubscriptionSupportRetentionDecisionEvidence::RetainDegraded { .. } => {
                SubscriptionSupportRetentionDecisionKind::RetainDegraded
            }
            SubscriptionSupportRetentionDecisionEvidence::CompactExact { .. } => {
                SubscriptionSupportRetentionDecisionKind::CompactExact
            }
            SubscriptionSupportRetentionDecisionEvidence::ReclaimWithRebuild { .. } => {
                SubscriptionSupportRetentionDecisionKind::ReclaimWithRebuild
            }
            SubscriptionSupportRetentionDecisionEvidence::ReclaimWithoutRebuild { .. } => {
                SubscriptionSupportRetentionDecisionKind::ReclaimWithoutRebuild
            }
            SubscriptionSupportRetentionDecisionEvidence::ExpireByPolicy { .. } => {
                SubscriptionSupportRetentionDecisionKind::ExpireByPolicy
            }
        }
    }

    pub fn weakened_condition(&self) -> Option<&str> {
        match &self.evidence {
            SubscriptionSupportRetentionDecisionEvidence::RetainDegraded { weakened_condition } => {
                Some(weakened_condition)
            }
            _ => None,
        }
    }

    pub fn compacted_basis_digest(&self) -> Option<&str> {
        match &self.evidence {
            SubscriptionSupportRetentionDecisionEvidence::CompactExact {
                compacted_basis_digest,
            } => Some(compacted_basis_digest),
            _ => None,
        }
    }

    pub fn retained_rebuild_basis_digest(&self) -> Option<&str> {
        match &self.evidence {
            SubscriptionSupportRetentionDecisionEvidence::ReclaimWithRebuild {
                retained_rebuild_basis_digest,
                ..
            } => Some(retained_rebuild_basis_digest),
            _ => None,
        }
    }

    pub fn maintenance_admission_key(&self) -> Option<&str> {
        match &self.evidence {
            SubscriptionSupportRetentionDecisionEvidence::ReclaimWithRebuild {
                maintenance_admission_key,
                ..
            } => Some(maintenance_admission_key),
            _ => None,
        }
    }

    pub fn missing_rebuild_basis_reason(&self) -> Option<&str> {
        match &self.evidence {
            SubscriptionSupportRetentionDecisionEvidence::ReclaimWithoutRebuild {
                missing_rebuild_basis_reason,
            } => Some(missing_rebuild_basis_reason),
            _ => None,
        }
    }

    pub fn policy_reason(&self) -> Option<&str> {
        match &self.evidence {
            SubscriptionSupportRetentionDecisionEvidence::ExpireByPolicy { policy_reason } => {
                Some(policy_reason)
            }
            _ => None,
        }
    }
}

impl From<SubscriptionSupportRetentionDecisionEvidence> for SubscriptionSupportRetentionDecision {
    fn from(evidence: SubscriptionSupportRetentionDecisionEvidence) -> Self {
        Self { evidence }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum SubscriptionSupportRetentionDecisionKind {
    RetainExact,
    RetainDegraded,
    CompactExact,
    ReclaimWithRebuild,
    ReclaimWithoutRebuild,
    ExpireByPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RetainedSupportArtifactSet {
    affected_set: SupportAffectedSet,
    decision_kind: SubscriptionSupportRetentionDecisionKind,
    weakened_condition: Option<String>,
}

impl RetainedSupportArtifactSet {
    pub(crate) fn exact(affected_set: SupportAffectedSet) -> Self {
        Self {
            affected_set,
            decision_kind: SubscriptionSupportRetentionDecisionKind::RetainExact,
            weakened_condition: None,
        }
    }

    pub(crate) fn degraded(
        affected_set: SupportAffectedSet,
        weakened_condition: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            affected_set,
            decision_kind: SubscriptionSupportRetentionDecisionKind::RetainDegraded,
            weakened_condition: Some(require_non_empty(
                "weakened support condition",
                weakened_condition,
            )?),
        })
    }

    pub fn affected_set(&self) -> &SupportAffectedSet {
        &self.affected_set
    }

    pub fn decision_kind(&self) -> SubscriptionSupportRetentionDecisionKind {
        self.decision_kind
    }

    pub fn weakened_condition(&self) -> Option<&str> {
        self.weakened_condition.as_deref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReclaimedSupportArtifactSet {
    affected_set: SupportAffectedSet,
    decision_kind: SubscriptionSupportRetentionDecisionKind,
    retained_rebuild_basis_digest: Option<String>,
    maintenance_admission_key: Option<String>,
    missing_rebuild_basis_reason: Option<String>,
}

impl ReclaimedSupportArtifactSet {
    pub(crate) fn rebuildable(
        affected_set: SupportAffectedSet,
        retained_rebuild_basis_digest: impl Into<String>,
        maintenance_admission_key: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            affected_set,
            decision_kind: SubscriptionSupportRetentionDecisionKind::ReclaimWithRebuild,
            retained_rebuild_basis_digest: Some(require_non_empty(
                "retained rebuild basis",
                retained_rebuild_basis_digest,
            )?),
            maintenance_admission_key: Some(require_non_empty(
                "maintenance admission",
                maintenance_admission_key,
            )?),
            missing_rebuild_basis_reason: None,
        })
    }

    pub(crate) fn non_resumable(
        affected_set: SupportAffectedSet,
        missing_rebuild_basis_reason: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            affected_set,
            decision_kind: SubscriptionSupportRetentionDecisionKind::ReclaimWithoutRebuild,
            retained_rebuild_basis_digest: None,
            maintenance_admission_key: None,
            missing_rebuild_basis_reason: Some(require_non_empty(
                "missing rebuild basis reason",
                missing_rebuild_basis_reason,
            )?),
        })
    }

    pub fn affected_set(&self) -> &SupportAffectedSet {
        &self.affected_set
    }

    pub fn decision_kind(&self) -> SubscriptionSupportRetentionDecisionKind {
        self.decision_kind
    }

    pub fn retained_rebuild_basis_digest(&self) -> Option<&str> {
        self.retained_rebuild_basis_digest.as_deref()
    }

    pub fn maintenance_admission_key(&self) -> Option<&str> {
        self.maintenance_admission_key.as_deref()
    }

    pub fn missing_rebuild_basis_reason(&self) -> Option<&str> {
        self.missing_rebuild_basis_reason.as_deref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompactedSupportBasis {
    affected_set: SupportAffectedSet,
    compacted_basis_digest: String,
}

impl CompactedSupportBasis {
    pub(crate) fn new(
        affected_set: SupportAffectedSet,
        compacted_basis_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            affected_set,
            compacted_basis_digest: require_non_empty(
                "compacted support basis",
                compacted_basis_digest,
            )?,
        })
    }

    pub fn affected_set(&self) -> &SupportAffectedSet {
        &self.affected_set
    }

    pub fn compacted_basis_digest(&self) -> &str {
        &self.compacted_basis_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExpiredSupportArtifactSet {
    affected_set: SupportAffectedSet,
    policy_reason: String,
}

impl ExpiredSupportArtifactSet {
    pub(crate) fn new(
        affected_set: SupportAffectedSet,
        policy_reason: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            affected_set,
            policy_reason: require_non_empty("policy expiration", policy_reason)?,
        })
    }

    pub fn affected_set(&self) -> &SupportAffectedSet {
        &self.affected_set
    }

    pub fn policy_reason(&self) -> &str {
        &self.policy_reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum SubscriptionSupportRetentionMaterialization {
    Retained(RetainedSupportArtifactSet),
    Compacted(CompactedSupportBasis),
    Reclaimed(ReclaimedSupportArtifactSet),
    Expired(ExpiredSupportArtifactSet),
}

impl SubscriptionSupportRetentionMaterialization {
    pub(crate) fn from_decision(
        affected_set: SupportAffectedSet,
        decision: &SubscriptionSupportRetentionDecision,
    ) -> Result<Self, StoreError> {
        match &decision.evidence {
            SubscriptionSupportRetentionDecisionEvidence::RetainExact => Ok(Self::Retained(
                RetainedSupportArtifactSet::exact(affected_set),
            )),
            SubscriptionSupportRetentionDecisionEvidence::RetainDegraded { weakened_condition } => {
                Ok(Self::Retained(RetainedSupportArtifactSet::degraded(
                    affected_set,
                    weakened_condition.clone(),
                )?))
            }
            SubscriptionSupportRetentionDecisionEvidence::CompactExact {
                compacted_basis_digest,
            } => Ok(Self::Compacted(CompactedSupportBasis::new(
                affected_set,
                compacted_basis_digest.clone(),
            )?)),
            SubscriptionSupportRetentionDecisionEvidence::ReclaimWithRebuild {
                retained_rebuild_basis_digest,
                maintenance_admission_key,
            } => Ok(Self::Reclaimed(ReclaimedSupportArtifactSet::rebuildable(
                affected_set,
                retained_rebuild_basis_digest.clone(),
                maintenance_admission_key.clone(),
            )?)),
            SubscriptionSupportRetentionDecisionEvidence::ReclaimWithoutRebuild {
                missing_rebuild_basis_reason,
            } => Ok(Self::Reclaimed(ReclaimedSupportArtifactSet::non_resumable(
                affected_set,
                missing_rebuild_basis_reason.clone(),
            )?)),
            SubscriptionSupportRetentionDecisionEvidence::ExpireByPolicy { policy_reason } => {
                Ok(Self::Expired(ExpiredSupportArtifactSet::new(
                    affected_set,
                    policy_reason.clone(),
                )?))
            }
        }
    }

    pub fn affected_set(&self) -> &SupportAffectedSet {
        match self {
            Self::Retained(set) => set.affected_set(),
            Self::Compacted(basis) => basis.affected_set(),
            Self::Reclaimed(set) => set.affected_set(),
            Self::Expired(set) => set.affected_set(),
        }
    }

    pub fn materialization_kind(&self) -> SubscriptionSupportRetentionDecisionKind {
        match self {
            Self::Retained(set) => set.decision_kind(),
            Self::Compacted(_) => SubscriptionSupportRetentionDecisionKind::CompactExact,
            Self::Reclaimed(set) => set.decision_kind(),
            Self::Expired(_) => SubscriptionSupportRetentionDecisionKind::ExpireByPolicy,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportRetentionBatchPlan {
    action_id: SupportActionId,
    affected_set: SupportAffectedSet,
    path_plan: SupportProgramPathPlan,
    decision: SubscriptionSupportRetentionDecision,
}

impl SupportRetentionBatchPlan {
    pub(crate) fn new(
        action_id: SupportActionId,
        affected_set: SupportAffectedSet,
        path_plan: SupportProgramPathPlan,
        decision: SubscriptionSupportRetentionDecision,
    ) -> Result<Self, StoreError> {
        if path_plan.density_class() == SupportProgramDensityClass::StoreGlobalDebt {
            return Err(classification_error(
                "subscription-support retention cannot admit store-global density",
            ));
        }
        if path_plan.batch_width() != affected_set.affected_count() {
            return Err(classification_error(
                "subscription-support retention plan width must match affected-set breadth",
            ));
        }
        Ok(Self {
            action_id,
            affected_set,
            path_plan,
            decision,
        })
    }

    pub fn action_id(&self) -> &SupportActionId {
        &self.action_id
    }

    pub fn affected_set(&self) -> &SupportAffectedSet {
        &self.affected_set
    }

    pub fn path_plan(&self) -> &SupportProgramPathPlan {
        &self.path_plan
    }

    pub fn decision(&self) -> &SubscriptionSupportRetentionDecision {
        &self.decision
    }

    pub fn verdict(&self) -> SubscriptionSupportOperationalVerdict {
        self.decision.verdict()
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        SupportActionId,
        SupportAffectedSet,
        SupportProgramPathPlan,
        SubscriptionSupportRetentionDecision,
    ) {
        (
            self.action_id,
            self.affected_set,
            self.path_plan,
            self.decision,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportRetentionPlan {
    batch_plan: SupportRetentionBatchPlan,
}

impl SubscriptionSupportRetentionPlan {
    #[allow(dead_code)]
    pub(crate) fn new(batch_plan: SupportRetentionBatchPlan) -> Self {
        Self { batch_plan }
    }

    pub fn batch_plan(&self) -> &SupportRetentionBatchPlan {
        &self.batch_plan
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportRetentionSurvivalWitness {
    verdict: SubscriptionSupportOperationalVerdict,
    affected_count: u64,
    affected_set_digest: SupportAffectedSetDigest,
}

impl SupportRetentionSurvivalWitness {
    pub(crate) fn new(
        completed_action: &CompletedSupportProgramAction,
        expected_verdict: SubscriptionSupportOperationalVerdict,
        affected_set: &SupportAffectedSet,
    ) -> Result<Self, StoreError> {
        if completed_action.envelope().action_origin() != SubscriptionSupportActionOrigin::Retention
        {
            return Err(classification_error(
                "subscription-support retention survival witnesses require retention-origin envelopes",
            ));
        }
        if completed_action.envelope().verdict() != expected_verdict {
            return Err(classification_error(
                "subscription-support retention survival witness verdict drift",
            ));
        }
        Ok(Self {
            verdict: expected_verdict,
            affected_count: affected_set.affected_count(),
            affected_set_digest: affected_set.affected_set_digest().clone(),
        })
    }

    pub fn verdict(&self) -> SubscriptionSupportOperationalVerdict {
        self.verdict
    }

    pub fn affected_count(&self) -> u64 {
        self.affected_count
    }

    pub fn affected_set_digest(&self) -> &SupportAffectedSetDigest {
        &self.affected_set_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportReclaimConsequence {
    completed_action: CompletedSupportProgramAction,
    survival_witness: SupportRetentionSurvivalWitness,
    retention_record: SupportRetentionParticipationRecord,
    reclaimed_artifacts: ReclaimedSupportArtifactSet,
}

impl SupportReclaimConsequence {
    pub(crate) fn new(
        completed_action: CompletedSupportProgramAction,
        survival_witness: SupportRetentionSurvivalWitness,
        retention_record: SupportRetentionParticipationRecord,
        materialization: SubscriptionSupportRetentionMaterialization,
    ) -> Result<Self, StoreError> {
        let SubscriptionSupportRetentionMaterialization::Reclaimed(reclaimed_artifacts) =
            materialization
        else {
            return Err(classification_error(
                "subscription-support reclaim consequences require reclaimed support materialization",
            ));
        };
        Ok(Self {
            completed_action,
            survival_witness,
            retention_record,
            reclaimed_artifacts,
        })
    }

    pub fn completed_action(&self) -> &CompletedSupportProgramAction {
        &self.completed_action
    }

    pub fn survival_witness(&self) -> &SupportRetentionSurvivalWitness {
        &self.survival_witness
    }

    pub fn retention_record(&self) -> &SupportRetentionParticipationRecord {
        &self.retention_record
    }

    pub fn reclaimed_artifacts(&self) -> &ReclaimedSupportArtifactSet {
        &self.reclaimed_artifacts
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportPostActionReport {
    completed_action: CompletedSupportProgramAction,
    survival_witness: SupportRetentionSurvivalWitness,
    retention_record: SupportRetentionParticipationRecord,
    materialization: SubscriptionSupportRetentionMaterialization,
    cost_surface: SubscriptionSupportResultCostSurface,
}

impl SubscriptionSupportPostActionReport {
    pub(crate) fn new(
        completed_action: CompletedSupportProgramAction,
        survival_witness: SupportRetentionSurvivalWitness,
        materialization: SubscriptionSupportRetentionMaterialization,
        decision_kind: SubscriptionSupportRetentionDecisionKind,
        path_plan: &SupportProgramPathPlan,
    ) -> Result<Self, StoreError> {
        let retention_record = SupportRetentionParticipationRecord::new(
            &completed_action,
            &survival_witness,
            &materialization,
            decision_kind,
        )?;
        Ok(Self {
            completed_action,
            survival_witness,
            retention_record,
            materialization,
            cost_surface: cost_surface_for_program_path(
                SubscriptionSupportPlanFamily::RetentionParticipationPlan,
                path_plan,
            ),
        })
    }

    pub fn completed_action(&self) -> &CompletedSupportProgramAction {
        &self.completed_action
    }

    pub fn survival_witness(&self) -> &SupportRetentionSurvivalWitness {
        &self.survival_witness
    }

    pub fn retention_record(&self) -> &SupportRetentionParticipationRecord {
        &self.retention_record
    }

    pub fn materialization(&self) -> &SubscriptionSupportRetentionMaterialization {
        &self.materialization
    }

    pub fn cost_surface(&self) -> SubscriptionSupportResultCostSurface {
        self.cost_surface
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportRetentionParticipationRecord {
    action_id: SupportActionId,
    family_id: SubscriptionSupportFamilyId,
    family_kind: SubscriptionSupportFamilyKind,
    support_role: SubscriptionSupportRole,
    affected_set_digest: SupportAffectedSetDigest,
    affected_artifact_ids: Vec<SubscriptionSupportArtifactId>,
    affected_count: u64,
    decision_kind: SubscriptionSupportRetentionDecisionKind,
    verdict: SubscriptionSupportOperationalVerdict,
    action_origin: SubscriptionSupportActionOrigin,
}

impl SupportRetentionParticipationRecord {
    pub(crate) fn new(
        completed_action: &CompletedSupportProgramAction,
        survival_witness: &SupportRetentionSurvivalWitness,
        materialization: &SubscriptionSupportRetentionMaterialization,
        decision_kind: SubscriptionSupportRetentionDecisionKind,
    ) -> Result<Self, StoreError> {
        if completed_action.envelope().action_origin() != SubscriptionSupportActionOrigin::Retention
        {
            return Err(classification_error(
                "subscription-support retention participation records require retention-origin envelopes",
            ));
        }
        if materialization.affected_set().affected_count() != survival_witness.affected_count() {
            return Err(classification_error(
                "subscription-support retention participation record breadth drift",
            ));
        }
        if materialization.affected_set().affected_set_digest()
            != survival_witness.affected_set_digest()
        {
            return Err(classification_error(
                "subscription-support retention participation record affected-set digest drift",
            ));
        }
        if decision_kind != materialization.materialization_kind() {
            return Err(classification_error(
                "subscription-support retention participation record decision kind drift",
            ));
        }
        let affected_set = materialization.affected_set();
        Ok(Self {
            action_id: completed_action.envelope().action_id().clone(),
            family_id: affected_set.family_id().clone(),
            family_kind: affected_set.family_kind(),
            support_role: affected_set.support_role(),
            affected_set_digest: affected_set.affected_set_digest().clone(),
            affected_artifact_ids: affected_set.affected_artifact_ids(),
            affected_count: affected_set.affected_count(),
            decision_kind,
            verdict: survival_witness.verdict(),
            action_origin: completed_action.envelope().action_origin(),
        })
    }

    pub fn action_id(&self) -> &SupportActionId {
        &self.action_id
    }

    pub fn affected_set_digest(&self) -> &SupportAffectedSetDigest {
        &self.affected_set_digest
    }

    pub fn affected_count(&self) -> u64 {
        self.affected_count
    }

    pub fn decision_kind(&self) -> SubscriptionSupportRetentionDecisionKind {
        self.decision_kind
    }

    pub fn verdict(&self) -> SubscriptionSupportOperationalVerdict {
        self.verdict
    }
}

fn require_non_empty(label: &'static str, value: impl Into<String>) -> Result<String, StoreError> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(classification_error(format!(
            "subscription-support retention {label} evidence must be non-empty"
        )));
    }
    Ok(value)
}
