use crate::{
    AuthoritativeReclaimMaintenanceDeclaration, CompactionMaintenanceDeclaration, FreshnessWindow,
    LocalityScopeToken, MaintenanceBatchClass, MaintenanceCoalescingDecision,
    MaintenanceDebtFamily, MaintenanceDebtSummary, MaintenanceDeclaration,
    MaintenanceDeclarationClass, MaintenanceDeclarationId, MaintenanceDescriptorDemand,
    MaintenanceEquivalenceKey, MaintenanceEscalationDecision, MaintenanceEscalationVerdict,
    MaintenanceExecutionPosture, MaintenanceExecutionStatus, MaintenanceExecutionTransition,
    MaintenanceForegroundImpact, MaintenanceLaneKey, MaintenanceLocalityScope,
    MaintenanceLocalitySummary, MaintenancePlanFamily, MaintenanceQueueSummary,
    MaintenanceReadmissionStatus, MaintenanceReservationFamily, MaintenanceReservationSummary,
    MaintenanceReservationTransition, MaintenanceResourceBudgetGrant,
    MaintenanceResourceBudgetSummary, MaintenanceStarvationStatus, MaintenanceWorkClass,
    MaintenanceWorkDescriptor, MaintenanceWorkIdentity, PlanGeneration,
    RebuildMaintenanceDeclaration, ReclaimMaintenanceDeclaration, RetentionMaintenanceDeclaration,
    SupersessionEpoch, TierWorkContainerClass,
};
use forge_relational::facade::history::{BranchId, CommitId};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaintenanceDeclarationRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub batch_id: String,
    pub declaration_class: MaintenanceDeclarationClass,
    pub declaration: MaintenanceDeclaration,
    pub retained_basis_label: Option<String>,
    pub family_label: Option<String>,
    pub debt_link_artifact_id: Option<String>,
    pub work_descriptor: MaintenanceWorkDescriptor,
    pub created_order: u64,
}

impl Serialize for MaintenanceDeclarationRecord {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        PersistedMaintenanceDeclarationRecord::from(self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for MaintenanceDeclarationRecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let persisted = PersistedMaintenanceDeclarationRecord::deserialize(deserializer)?;
        Self::try_from(persisted).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct PersistedMaintenanceDeclarationRecord {
    artifact_id: String,
    family_version: u32,
    batch_id: String,
    declaration_class: MaintenanceDeclarationClass,
    declaration: PersistedMaintenanceDeclaration,
    retained_basis_label: Option<String>,
    family_label: Option<String>,
    debt_link_artifact_id: Option<String>,
    #[serde(default)]
    work_descriptor: Option<PersistedMaintenanceWorkDescriptor>,
    created_order: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct PersistedMaintenanceWorkDescriptor {
    declaration_id: String,
    work_class: MaintenanceWorkClass,
    execution_posture: MaintenanceExecutionPosture,
    locality_scope: MaintenanceLocalityScope,
    locality_scope_token: LocalityScopeToken,
    demand: MaintenanceDescriptorDemand,
    reservation_family: MaintenanceReservationFamily,
    work_identity: String,
    equivalence_key: String,
    plan_generation: PlanGeneration,
    supersession_epoch: SupersessionEpoch,
    freshness_window: FreshnessWindow,
    debt_family: Option<MaintenanceDebtFamily>,
    escalation_decision: MaintenanceEscalationDecision,
    tier_work_container_class: Option<TierWorkContainerClass>,
    recovered_from_restart: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum PersistedMaintenanceDeclaration {
    Retention {
        id: String,
        batch_label: String,
        closure_commit_count: u64,
        declaration_count: u64,
    },
    Compaction {
        id: String,
        retained_basis_label: String,
        retained_head_branch_ids: Vec<BranchId>,
        stable_basis_labels: Vec<String>,
        closure_commit_ids: Vec<CommitId>,
        frontier_commit_ids: Vec<CommitId>,
        family_labels: Vec<String>,
        superseded_families: Vec<(String, String, Option<CommitId>)>,
        rewritten_range_count: u64,
    },
    Reclaim {
        id: String,
        retained_basis_label: String,
        artifact_family: String,
        artifact_id: String,
    },
    AuthoritativeReclaim {
        id: String,
        branch_id: BranchId,
        oldest_retained_commit_id: Option<CommitId>,
        expired_commit_ids: Vec<CommitId>,
    },
    Rebuild {
        id: String,
        retained_basis_label: String,
        family_label: String,
        rebuild_target_id: String,
        debt_link_artifact_id: Option<String>,
    },
}

impl From<&MaintenanceDeclarationRecord> for PersistedMaintenanceDeclarationRecord {
    fn from(record: &MaintenanceDeclarationRecord) -> Self {
        Self {
            artifact_id: record.artifact_id.clone(),
            family_version: record.family_version,
            batch_id: record.batch_id.clone(),
            declaration_class: record.declaration_class,
            declaration: PersistedMaintenanceDeclaration::from(&record.declaration),
            retained_basis_label: record.retained_basis_label.clone(),
            family_label: record.family_label.clone(),
            debt_link_artifact_id: record.debt_link_artifact_id.clone(),
            work_descriptor: Some(PersistedMaintenanceWorkDescriptor::from(
                &record.work_descriptor,
            )),
            created_order: record.created_order,
        }
    }
}

impl TryFrom<PersistedMaintenanceDeclarationRecord> for MaintenanceDeclarationRecord {
    type Error = String;

    fn try_from(record: PersistedMaintenanceDeclarationRecord) -> Result<Self, Self::Error> {
        let declaration = MaintenanceDeclaration::try_from(record.declaration)?;
        let work_descriptor = record
            .work_descriptor
            .map(MaintenanceWorkDescriptor::try_from)
            .transpose()?
            .unwrap_or_else(|| declaration.work_descriptor());
        Ok(Self {
            artifact_id: record.artifact_id,
            family_version: record.family_version,
            batch_id: record.batch_id,
            declaration_class: record.declaration_class,
            declaration,
            retained_basis_label: record.retained_basis_label,
            family_label: record.family_label,
            debt_link_artifact_id: record.debt_link_artifact_id,
            work_descriptor,
            created_order: record.created_order,
        })
    }
}

impl From<&MaintenanceWorkDescriptor> for PersistedMaintenanceWorkDescriptor {
    fn from(descriptor: &MaintenanceWorkDescriptor) -> Self {
        Self {
            declaration_id: descriptor.declaration_id().as_str().to_string(),
            work_class: descriptor.work_class(),
            execution_posture: descriptor.execution_posture(),
            locality_scope: descriptor.locality_scope().clone(),
            locality_scope_token: descriptor.locality_scope_token().clone(),
            demand: descriptor.demand().clone(),
            reservation_family: descriptor.reservation_family(),
            work_identity: descriptor.work_identity().as_str().to_string(),
            equivalence_key: descriptor.equivalence_key().as_str().to_string(),
            plan_generation: descriptor.plan_generation(),
            supersession_epoch: descriptor.supersession_epoch(),
            freshness_window: descriptor.freshness_window(),
            debt_family: descriptor.debt_family(),
            escalation_decision: descriptor.escalation_decision(),
            tier_work_container_class: descriptor.tier_work_container_class(),
            recovered_from_restart: descriptor.recovered_from_restart(),
        }
    }
}

impl TryFrom<PersistedMaintenanceWorkDescriptor> for MaintenanceWorkDescriptor {
    type Error = String;

    fn try_from(descriptor: PersistedMaintenanceWorkDescriptor) -> Result<Self, Self::Error> {
        Ok(MaintenanceWorkDescriptor::new(
            MaintenanceDeclarationId::new(descriptor.declaration_id),
            descriptor.work_class,
            descriptor.execution_posture,
            descriptor.locality_scope,
            descriptor.locality_scope_token,
            descriptor.demand,
            descriptor.reservation_family,
            MaintenanceWorkIdentity::new(descriptor.work_identity),
            MaintenanceEquivalenceKey::new(descriptor.equivalence_key),
            descriptor.plan_generation,
            descriptor.supersession_epoch,
            descriptor.freshness_window,
            descriptor.debt_family,
            descriptor.escalation_decision,
            descriptor.tier_work_container_class,
            descriptor.recovered_from_restart,
        ))
    }
}

impl From<&MaintenanceDeclaration> for PersistedMaintenanceDeclaration {
    fn from(declaration: &MaintenanceDeclaration) -> Self {
        match declaration {
            MaintenanceDeclaration::Retention { id, declaration } => Self::Retention {
                id: id.as_str().to_string(),
                batch_label: declaration.batch_label().to_string(),
                closure_commit_count: declaration.closure_commit_count(),
                declaration_count: declaration.declaration_count(),
            },
            MaintenanceDeclaration::Compaction { id, declaration } => Self::Compaction {
                id: id.as_str().to_string(),
                retained_basis_label: declaration.retained_basis_label().to_string(),
                retained_head_branch_ids: declaration.retained_head_branch_ids().to_vec(),
                stable_basis_labels: declaration.stable_basis_labels().to_vec(),
                closure_commit_ids: declaration.closure_commit_ids().to_vec(),
                frontier_commit_ids: declaration.frontier_commit_ids().to_vec(),
                family_labels: declaration.family_labels().to_vec(),
                superseded_families: declaration.superseded_families().to_vec(),
                rewritten_range_count: declaration.rewritten_range_count(),
            },
            MaintenanceDeclaration::Reclaim { id, declaration } => Self::Reclaim {
                id: id.as_str().to_string(),
                retained_basis_label: declaration.retained_basis_label().to_string(),
                artifact_family: declaration.artifact_family().to_string(),
                artifact_id: declaration.artifact_id().to_string(),
            },
            MaintenanceDeclaration::AuthoritativeReclaim { id, declaration } => {
                Self::AuthoritativeReclaim {
                    id: id.as_str().to_string(),
                    branch_id: declaration.branch_id().clone(),
                    oldest_retained_commit_id: declaration.oldest_retained_commit_id(),
                    expired_commit_ids: declaration.expired_commit_ids().to_vec(),
                }
            }
            MaintenanceDeclaration::Rebuild { id, declaration } => Self::Rebuild {
                id: id.as_str().to_string(),
                retained_basis_label: declaration.retained_basis_label().to_string(),
                family_label: declaration.family_label().to_string(),
                rebuild_target_id: declaration.rebuild_target_id().to_string(),
                debt_link_artifact_id: declaration.debt_link_artifact_id().map(ToString::to_string),
            },
        }
    }
}

impl TryFrom<PersistedMaintenanceDeclaration> for MaintenanceDeclaration {
    type Error = String;

    fn try_from(declaration: PersistedMaintenanceDeclaration) -> Result<Self, Self::Error> {
        Ok(match declaration {
            PersistedMaintenanceDeclaration::Retention {
                id,
                batch_label,
                closure_commit_count,
                declaration_count,
            } => MaintenanceDeclaration::retention(
                MaintenanceDeclarationId::new(id),
                RetentionMaintenanceDeclaration::new(
                    batch_label,
                    closure_commit_count,
                    declaration_count,
                ),
            ),
            PersistedMaintenanceDeclaration::Compaction {
                id,
                retained_basis_label,
                retained_head_branch_ids,
                stable_basis_labels,
                closure_commit_ids,
                frontier_commit_ids,
                family_labels,
                superseded_families,
                rewritten_range_count,
            } => MaintenanceDeclaration::compaction(
                MaintenanceDeclarationId::new(id),
                CompactionMaintenanceDeclaration::new(
                    retained_basis_label,
                    retained_head_branch_ids,
                    stable_basis_labels,
                    closure_commit_ids,
                    frontier_commit_ids,
                    family_labels,
                    superseded_families,
                    rewritten_range_count,
                ),
            ),
            PersistedMaintenanceDeclaration::Reclaim {
                id,
                retained_basis_label,
                artifact_family,
                artifact_id,
            } => MaintenanceDeclaration::reclaim(
                MaintenanceDeclarationId::new(id),
                ReclaimMaintenanceDeclaration::new(
                    retained_basis_label,
                    artifact_family,
                    artifact_id,
                ),
            ),
            PersistedMaintenanceDeclaration::AuthoritativeReclaim {
                id,
                branch_id,
                oldest_retained_commit_id,
                expired_commit_ids,
            } => MaintenanceDeclaration::authoritative_reclaim(
                MaintenanceDeclarationId::new(id),
                AuthoritativeReclaimMaintenanceDeclaration::new(
                    branch_id,
                    oldest_retained_commit_id,
                    expired_commit_ids,
                ),
            ),
            PersistedMaintenanceDeclaration::Rebuild {
                id,
                retained_basis_label,
                family_label,
                rebuild_target_id,
                debt_link_artifact_id,
            } => MaintenanceDeclaration::rebuild(
                MaintenanceDeclarationId::new(id),
                RebuildMaintenanceDeclaration::new(
                    retained_basis_label,
                    family_label,
                    rebuild_target_id,
                    debt_link_artifact_id,
                ),
            ),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceExecutionRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub declaration_id: String,
    pub execution_status: MaintenanceExecutionStatus,
    #[serde(default)]
    pub lane_key: Option<MaintenanceLaneKey>,
    #[serde(default)]
    pub plan_family: Option<MaintenancePlanFamily>,
    pub last_completed_phase: Option<String>,
    #[serde(default)]
    pub pending_reason: Option<String>,
    pub durable_error_kind: Option<String>,
    pub durable_error_message: Option<String>,
    #[serde(default)]
    pub last_quantum_units: Option<u64>,
    #[serde(default)]
    pub reservation_transition: Option<MaintenanceReservationTransition>,
    #[serde(default)]
    pub execution_transition: Option<MaintenanceExecutionTransition>,
    #[serde(default)]
    pub restart_readmission_status: Option<MaintenanceReadmissionStatus>,
    #[serde(default = "MaintenanceForegroundImpact::none")]
    pub foreground_impact: MaintenanceForegroundImpact,
    #[serde(default)]
    pub coalescing_decision: Option<MaintenanceCoalescingDecision>,
    #[serde(default)]
    pub supersession_source: Option<String>,
    #[serde(default)]
    pub resource_budget_grant: Option<MaintenanceResourceBudgetGrant>,
    #[serde(default)]
    pub starvation_status: Option<MaintenanceStarvationStatus>,
    #[serde(default)]
    pub escalation_verdict: Option<MaintenanceEscalationVerdict>,
    #[serde(default)]
    pub explicit_global_scope_debt: bool,
    pub resume_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceBatchRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub batch_class: MaintenanceBatchClass,
    pub declaration_ids: Vec<String>,
    pub declaration_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceCheckpointRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub declaration_id: String,
    pub completed_phase: String,
    pub checkpoint_order: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceQueueSummaryRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub summary: MaintenanceQueueSummary,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceLocalitySummaryRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub summary: MaintenanceLocalitySummary,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceReservationSummaryRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub summary: MaintenanceReservationSummary,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceResourceBudgetSummaryRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub summary: MaintenanceResourceBudgetSummary,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceDebtSummaryRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub lane_key: MaintenanceLaneKey,
    pub summary: MaintenanceDebtSummary,
}
