use super::super::MaintenanceDeclaration;
use super::descriptor::{MaintenanceWorkDescriptor, MaintenanceWorkDescriptorBasis};

use super::budgets::{
    CpuBudgetUnits, ForegroundLatencyGuard, FreshnessWindow, IoBudgetUnits,
    MaintenanceDescriptorDemand, MemoryBudgetUnits, PlanGeneration, PublicationSlotBudget,
    SupersessionEpoch,
};

use super::classes::{
    BackgroundReservationFamily, MaintenanceDebtFamily, MaintenanceEscalationDecision,
    MaintenanceExecutionPosture, MaintenanceReservationFamily, MaintenanceWorkClass,
    TierWorkContainerClass,
};

use super::identities::{
    locality_scope_token_string, LocalityScopeToken, MaintenanceEquivalenceKey,
    MaintenanceLocalityScope, MaintenanceWorkIdentity,
};

impl MaintenanceDeclaration {
    pub fn work_descriptor(&self) -> MaintenanceWorkDescriptor {
        let work_class = self.work_class();
        let locality_scope = self.locality_scope();
        let locality_scope_token =
            LocalityScopeToken::new(locality_scope_token_string(&locality_scope));
        let work_identity = MaintenanceWorkIdentity::new(self.id().as_str().to_string());
        let equivalence_key = MaintenanceEquivalenceKey::new(self.equivalence_key_string());
        MaintenanceWorkDescriptor::new(MaintenanceWorkDescriptorBasis {
            declaration_id: self.id().clone(),
            work_class,
            execution_posture: self.execution_posture(),
            locality_scope,
            locality_scope_token,
            demand: self.predicted_demand(),
            reservation_family: self.reservation_family(),
            work_identity,
            equivalence_key,
            plan_generation: PlanGeneration::new(0),
            supersession_epoch: SupersessionEpoch::new(0),
            freshness_window: FreshnessWindow::new(1),
            debt_family: self.debt_family(),
            escalation_decision: MaintenanceEscalationDecision::StayBackground,
            tier_work_container_class: self.tier_work_container_class(),
            recovered_from_restart: false,
        })
    }

    pub fn work_class(&self) -> MaintenanceWorkClass {
        match self {
            Self::Retention { .. } => MaintenanceWorkClass::RetentionAudit,
            Self::Compaction { .. } => MaintenanceWorkClass::CompactionMaintenance,
            Self::Reclaim { .. } => MaintenanceWorkClass::DerivedArtifactReclaim,
            Self::AuthoritativeReclaim { .. } => MaintenanceWorkClass::AuthoritativeReclaim,
            Self::Rebuild { .. } => MaintenanceWorkClass::RetainedRangeRebuild,
            Self::DerivedFamilyRebuild { .. } => MaintenanceWorkClass::DerivedFamilyRebuild,
            Self::SnapshotRefresh { .. } => MaintenanceWorkClass::SnapshotRefresh,
            Self::ReplicationPreparation { .. } => MaintenanceWorkClass::ReplicationPreparation,
            Self::MaintenanceAudit { .. } => MaintenanceWorkClass::MaintenanceAudit,
            Self::TierPlacementProposal { .. } => MaintenanceWorkClass::TierPlacementProposal,
            Self::TierMoveExecution { .. } => MaintenanceWorkClass::TierMoveExecution,
        }
    }

    pub fn execution_posture(&self) -> MaintenanceExecutionPosture {
        match self.work_class() {
            MaintenanceWorkClass::RetentionAudit
            | MaintenanceWorkClass::CompactionMaintenance
            | MaintenanceWorkClass::DerivedArtifactReclaim
            | MaintenanceWorkClass::AuthoritativeReclaim
            | MaintenanceWorkClass::SnapshotRefresh
            | MaintenanceWorkClass::MaintenanceAudit => {
                MaintenanceExecutionPosture::ForegroundAware
            }
            MaintenanceWorkClass::RetainedRangeRebuild
            | MaintenanceWorkClass::DerivedFamilyRebuild
            | MaintenanceWorkClass::ReplicationPreparation
            | MaintenanceWorkClass::TierPlacementProposal
            | MaintenanceWorkClass::TierMoveExecution => {
                MaintenanceExecutionPosture::FullyDeferrable
            }
        }
    }

    pub fn locality_scope(&self) -> MaintenanceLocalityScope {
        match self {
            Self::Retention { .. } => MaintenanceLocalityScope::StoreGlobalLocalityScope,
            Self::Compaction { declaration, .. } => {
                let family_label = declaration
                    .family_labels()
                    .first()
                    .cloned()
                    .unwrap_or_else(|| declaration.retained_basis_label().to_string());
                MaintenanceLocalityScope::ArtifactFamilyLocalityScope { family_label }
            }
            Self::Reclaim { declaration, .. } => {
                MaintenanceLocalityScope::ArtifactFamilyLocalityScope {
                    family_label: declaration.artifact_family().to_string(),
                }
            }
            Self::AuthoritativeReclaim { declaration, .. } => {
                MaintenanceLocalityScope::BranchLocalityScope {
                    branch_label: declaration.branch_id().0.clone(),
                }
            }
            Self::Rebuild { declaration, .. } => {
                MaintenanceLocalityScope::ArtifactFamilyLocalityScope {
                    family_label: declaration.family_label().to_string(),
                }
            }
            Self::DerivedFamilyRebuild { declaration, .. } => {
                MaintenanceLocalityScope::ArtifactFamilyLocalityScope {
                    family_label: declaration.family_label().to_string(),
                }
            }
            Self::SnapshotRefresh { declaration, .. } => {
                MaintenanceLocalityScope::ArtifactFamilyLocalityScope {
                    family_label: declaration.locality_label().to_string(),
                }
            }
            Self::ReplicationPreparation { declaration, .. } => {
                MaintenanceLocalityScope::ArtifactFamilyLocalityScope {
                    family_label: declaration.locality_label().to_string(),
                }
            }
            Self::MaintenanceAudit { declaration, .. } => {
                MaintenanceLocalityScope::ArtifactFamilyLocalityScope {
                    family_label: declaration.locality_label().to_string(),
                }
            }
            Self::TierPlacementProposal { declaration, .. } => {
                MaintenanceLocalityScope::ArtifactFamilyLocalityScope {
                    family_label: declaration.locality_label().to_string(),
                }
            }
            Self::TierMoveExecution { declaration, .. } => {
                MaintenanceLocalityScope::ArtifactFamilyLocalityScope {
                    family_label: declaration.locality_label().to_string(),
                }
            }
        }
    }

    pub fn reservation_family(&self) -> MaintenanceReservationFamily {
        MaintenanceReservationFamily::Background(BackgroundReservationFamily::Maintenance)
    }

    pub fn predicted_demand(&self) -> MaintenanceDescriptorDemand {
        match self {
            Self::Retention { .. } => MaintenanceDescriptorDemand::new(
                IoBudgetUnits::new(1),
                CpuBudgetUnits::new(1),
                MemoryBudgetUnits::new(1),
                PublicationSlotBudget::new(0),
                ForegroundLatencyGuard::new(1),
            ),
            Self::Compaction { .. } => MaintenanceDescriptorDemand::new(
                IoBudgetUnits::new(4),
                CpuBudgetUnits::new(3),
                MemoryBudgetUnits::new(2),
                PublicationSlotBudget::new(1),
                ForegroundLatencyGuard::new(2),
            ),
            Self::Reclaim { .. } | Self::AuthoritativeReclaim { .. } => {
                MaintenanceDescriptorDemand::new(
                    IoBudgetUnits::new(2),
                    CpuBudgetUnits::new(1),
                    MemoryBudgetUnits::new(1),
                    PublicationSlotBudget::new(0),
                    ForegroundLatencyGuard::new(1),
                )
            }
            Self::Rebuild { .. } => MaintenanceDescriptorDemand::new(
                IoBudgetUnits::new(3),
                CpuBudgetUnits::new(2),
                MemoryBudgetUnits::new(2),
                PublicationSlotBudget::new(1),
                ForegroundLatencyGuard::new(1),
            ),
            Self::DerivedFamilyRebuild { .. } => MaintenanceDescriptorDemand::new(
                IoBudgetUnits::new(2),
                CpuBudgetUnits::new(2),
                MemoryBudgetUnits::new(1),
                PublicationSlotBudget::new(1),
                ForegroundLatencyGuard::new(1),
            ),
            Self::SnapshotRefresh { .. } => MaintenanceDescriptorDemand::new(
                IoBudgetUnits::new(1),
                CpuBudgetUnits::new(1),
                MemoryBudgetUnits::new(2),
                PublicationSlotBudget::new(1),
                ForegroundLatencyGuard::new(1),
            ),
            Self::ReplicationPreparation { .. } => MaintenanceDescriptorDemand::new(
                IoBudgetUnits::new(2),
                CpuBudgetUnits::new(1),
                MemoryBudgetUnits::new(1),
                PublicationSlotBudget::new(1),
                ForegroundLatencyGuard::new(1),
            ),
            Self::MaintenanceAudit { .. } => MaintenanceDescriptorDemand::new(
                IoBudgetUnits::new(1),
                CpuBudgetUnits::new(1),
                MemoryBudgetUnits::new(1),
                PublicationSlotBudget::new(0),
                ForegroundLatencyGuard::new(1),
            ),
            Self::TierPlacementProposal { .. } => MaintenanceDescriptorDemand::new(
                IoBudgetUnits::new(1),
                CpuBudgetUnits::new(1),
                MemoryBudgetUnits::new(1),
                PublicationSlotBudget::new(0),
                ForegroundLatencyGuard::new(1),
            ),
            Self::TierMoveExecution { .. } => MaintenanceDescriptorDemand::new(
                IoBudgetUnits::new(2),
                CpuBudgetUnits::new(2),
                MemoryBudgetUnits::new(1),
                PublicationSlotBudget::new(0),
                ForegroundLatencyGuard::new(1),
            ),
        }
    }

    pub fn debt_family(&self) -> Option<MaintenanceDebtFamily> {
        match self {
            Self::Compaction { .. } => Some(MaintenanceDebtFamily::CompactionDebt),
            Self::Rebuild { .. } | Self::DerivedFamilyRebuild { .. } => {
                Some(MaintenanceDebtFamily::RebuildDebt)
            }
            Self::SnapshotRefresh { .. } => Some(MaintenanceDebtFamily::SnapshotDebt),
            Self::ReplicationPreparation { .. } => {
                Some(MaintenanceDebtFamily::ReplicationPreparationDebt)
            }
            Self::TierPlacementProposal { .. } | Self::TierMoveExecution { .. } => {
                Some(MaintenanceDebtFamily::TierPlacementDebt)
            }
            _ => None,
        }
    }

    pub fn tier_work_container_class(&self) -> Option<TierWorkContainerClass> {
        match self {
            Self::TierPlacementProposal { .. } => {
                Some(TierWorkContainerClass::TierPlacementProposal)
            }
            Self::TierMoveExecution { .. } => Some(TierWorkContainerClass::TierMoveExecution),
            _ => None,
        }
    }

    fn equivalence_key_string(&self) -> String {
        match self {
            Self::Retention { declaration, .. } => format!(
                "retention:{}:{}:{}",
                declaration.batch_label(),
                declaration.closure_commit_count(),
                declaration.declaration_count(),
            ),
            Self::Compaction { declaration, .. } => format!(
                "compaction:{}:{}:{}",
                declaration.retained_basis_label(),
                declaration.family_labels().join("|"),
                declaration.rewritten_range_count(),
            ),
            Self::Reclaim { declaration, .. } => format!(
                "reclaim:{}:{}:{}",
                declaration.retained_basis_label(),
                declaration.artifact_family(),
                declaration.artifact_id(),
            ),
            Self::AuthoritativeReclaim { declaration, .. } => format!(
                "authoritative-reclaim:{}:{:?}:{:?}",
                declaration.branch_id().0,
                declaration.oldest_retained_commit_id(),
                declaration.expired_commit_ids(),
            ),
            Self::Rebuild { declaration, .. } => format!(
                "rebuild:{}:{}:{}",
                declaration.retained_basis_label(),
                declaration.family_label(),
                declaration.rebuild_target_id(),
            ),
            Self::DerivedFamilyRebuild { declaration, .. } => format!(
                "derived-family-rebuild:{}:{}:{}",
                declaration.retained_basis_label(),
                declaration.family_label(),
                declaration.rebuild_target_id(),
            ),
            Self::SnapshotRefresh { declaration, .. } => format!(
                "snapshot-refresh:{}:{}:{}",
                declaration.snapshot_family(),
                declaration.locality_label(),
                declaration.refresh_label(),
            ),
            Self::ReplicationPreparation { declaration, .. } => format!(
                "replication-preparation:{}:{}:{}",
                declaration.replication_family(),
                declaration.locality_label(),
                declaration.preparation_label(),
            ),
            Self::MaintenanceAudit { declaration, .. } => format!(
                "maintenance-audit:{}:{}:{}",
                declaration.audit_family(),
                declaration.locality_label(),
                declaration.audit_label(),
            ),
            Self::TierPlacementProposal { declaration, .. } => format!(
                "tier-placement:{}:{}:{}",
                declaration.placement_family(),
                declaration.locality_label(),
                declaration.proposal_label(),
            ),
            Self::TierMoveExecution { declaration, .. } => format!(
                "tier-move:{}:{}:{}:{}",
                declaration.placement_family(),
                declaration.locality_label(),
                declaration.move_label(),
                declaration.cross_locality_debt(),
            ),
        }
    }
}
