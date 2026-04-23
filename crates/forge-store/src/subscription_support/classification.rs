use super::{
    classification_error, SubscriptionSupportArtifactId, SubscriptionSupportDeclarationDigest,
};
use crate::failure::StoreError;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubscriptionSupportDensityClass {
    SparseIdentityClassification,
    FamilyBatchClassificationDebt,
    RestartShardBatchClassification,
    FamilyLocalBatch,
    BasisLocalBatch,
    PortabilityScopeBatch,
    MaintenanceKeyBatch,
    StoreGlobalDebt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SubscriptionSupportDriftCause {
    SubscriptionSupportFamilyMismatch,
    SubscriptionSupportCompatibilityDrift,
    SubscriptionSupportBasisDrift,
    SubscriptionSupportSchemaDrift,
    SubscriptionSupportCursorDrift,
    SubscriptionSupportCheckpointDrift,
    SubscriptionSupportDigestMismatch,
    SubscriptionSupportPlacementUnavailable,
    SubscriptionSupportSessionMemoryMissing,
}

impl SubscriptionSupportDriftCause {
    pub(crate) fn precedence_rank(self) -> u8 {
        match self {
            Self::SubscriptionSupportFamilyMismatch => 0,
            Self::SubscriptionSupportCompatibilityDrift => 1,
            Self::SubscriptionSupportBasisDrift => 2,
            Self::SubscriptionSupportSchemaDrift => 3,
            Self::SubscriptionSupportCursorDrift => 4,
            Self::SubscriptionSupportCheckpointDrift => 5,
            Self::SubscriptionSupportDigestMismatch => 6,
            Self::SubscriptionSupportSessionMemoryMissing => 7,
            Self::SubscriptionSupportPlacementUnavailable => 8,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubscriptionResumeClassification {
    Exact,
    Degraded,
    RebuildRequired,
    NotResumable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubscriptionSupportPlanFamily {
    ExactResumeClassificationPlan,
    DegradedResumeClassificationPlan,
    RebuildPlanClassificationPlan,
    DeniedResumeClassificationPlan,
    RetentionParticipationPlan,
    PortabilityParticipationPlan,
    MaintenanceParticipationPlan,
    OperationalRejectionPlan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubscriptionSupportAllocationScope {
    NoAllocation,
    FamilyLocalScratch,
    RestartShardBatch,
    ActionLocal,
    FamilyLocalBatch,
    PortabilityManifest,
    OperatorReport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportPayloadBudget {
    max_payload_bytes: u64,
    max_support_rows: u64,
}

impl SubscriptionSupportPayloadBudget {
    pub fn new(max_payload_bytes: u64, max_support_rows: u64) -> Result<Self, StoreError> {
        if max_payload_bytes == 0 || max_support_rows == 0 {
            return Err(classification_error(
                "subscription-support classification budgets must be non-zero",
            ));
        }
        Ok(Self {
            max_payload_bytes,
            max_support_rows,
        })
    }

    pub fn admits(&self, payload_bytes: u64, support_rows: u64) -> bool {
        payload_bytes <= self.max_payload_bytes && support_rows <= self.max_support_rows
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportClassificationPlan {
    pub(crate) plan_family: SubscriptionSupportPlanFamily,
    pub(crate) budget: SubscriptionSupportPayloadBudget,
    pub(crate) allocation_scope: SubscriptionSupportAllocationScope,
    pub(crate) density_class: SubscriptionSupportDensityClass,
    pub(crate) restart_shard: Option<String>,
}

impl SubscriptionSupportClassificationPlan {
    pub fn new(
        plan_family: SubscriptionSupportPlanFamily,
        budget: SubscriptionSupportPayloadBudget,
        allocation_scope: SubscriptionSupportAllocationScope,
        density_class: SubscriptionSupportDensityClass,
        restart_shard: Option<String>,
    ) -> Result<Self, StoreError> {
        if restart_shard
            .as_ref()
            .is_some_and(|value| value.trim().is_empty())
        {
            return Err(classification_error(
                "subscription-support restart shards must be non-empty when present",
            ));
        }
        Ok(Self {
            plan_family,
            budget,
            allocation_scope,
            density_class,
            restart_shard,
        })
    }

    pub fn exact_sparse_identity() -> Result<Self, StoreError> {
        Self::new(
            SubscriptionSupportPlanFamily::ExactResumeClassificationPlan,
            SubscriptionSupportPayloadBudget::new(16 * 1024, 64)?,
            SubscriptionSupportAllocationScope::NoAllocation,
            SubscriptionSupportDensityClass::SparseIdentityClassification,
            None,
        )
    }

    pub fn budget(&self) -> SubscriptionSupportPayloadBudget {
        self.budget
    }

    pub fn plan_family(&self) -> SubscriptionSupportPlanFamily {
        self.plan_family
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportResultCostSurface {
    plan_family: SubscriptionSupportPlanFamily,
    density_class: SubscriptionSupportDensityClass,
    decoded_payload_bytes: u64,
    scanned_support_rows: u64,
    restart_shards_touched: u64,
    allocation_scope: SubscriptionSupportAllocationScope,
}

impl SubscriptionSupportResultCostSurface {
    pub fn new(
        plan_family: SubscriptionSupportPlanFamily,
        density_class: SubscriptionSupportDensityClass,
        decoded_payload_bytes: u64,
        scanned_support_rows: u64,
        restart_shards_touched: u64,
        allocation_scope: SubscriptionSupportAllocationScope,
    ) -> Self {
        Self {
            plan_family,
            density_class,
            decoded_payload_bytes,
            scanned_support_rows,
            restart_shards_touched,
            allocation_scope,
        }
    }

    pub fn plan_family(&self) -> SubscriptionSupportPlanFamily {
        self.plan_family
    }

    pub fn density_class(&self) -> SubscriptionSupportDensityClass {
        self.density_class
    }

    pub fn decoded_payload_bytes(&self) -> u64 {
        self.decoded_payload_bytes
    }

    pub fn scanned_support_rows(&self) -> u64 {
        self.scanned_support_rows
    }

    pub fn restart_shards_touched(&self) -> u64 {
        self.restart_shards_touched
    }

    pub fn allocation_scope(&self) -> SubscriptionSupportAllocationScope {
        self.allocation_scope
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportClassificationReport {
    pub(crate) artifact_id: SubscriptionSupportArtifactId,
    pub(crate) declaration_digest: SubscriptionSupportDeclarationDigest,
    pub(crate) classification: SubscriptionResumeClassification,
    pub(crate) primary_cause: Option<SubscriptionSupportDriftCause>,
    pub(crate) suppressed_causes: Vec<SubscriptionSupportDriftCause>,
    pub(crate) cost_surface: SubscriptionSupportResultCostSurface,
    pub(crate) counter_snapshot: super::SubscriptionSupportCounterSnapshot,
}

impl SubscriptionSupportClassificationReport {
    pub fn artifact_id(&self) -> &SubscriptionSupportArtifactId {
        &self.artifact_id
    }

    pub fn declaration_digest(&self) -> &SubscriptionSupportDeclarationDigest {
        &self.declaration_digest
    }

    pub fn classification(&self) -> SubscriptionResumeClassification {
        self.classification
    }

    pub fn primary_cause(&self) -> Option<SubscriptionSupportDriftCause> {
        self.primary_cause
    }

    pub fn suppressed_causes(&self) -> &[SubscriptionSupportDriftCause] {
        &self.suppressed_causes
    }

    pub fn cost_surface(&self) -> SubscriptionSupportResultCostSurface {
        self.cost_surface
    }

    pub fn counter_snapshot(&self) -> &super::SubscriptionSupportCounterSnapshot {
        &self.counter_snapshot
    }
}

pub(crate) fn classify_causes(
    causes: Vec<SubscriptionSupportDriftCause>,
) -> (
    Option<SubscriptionSupportDriftCause>,
    Vec<SubscriptionSupportDriftCause>,
) {
    let mut unique = causes.into_iter().collect::<BTreeSet<_>>();
    let primary = unique
        .iter()
        .copied()
        .min_by_key(|cause| cause.precedence_rank());
    if let Some(primary) = primary {
        unique.remove(&primary);
    }
    let mut suppressed = unique.into_iter().collect::<Vec<_>>();
    suppressed.sort_by_key(|cause| cause.precedence_rank());
    (primary, suppressed)
}
