use super::{
    declaration_id::MaintenanceDeclarationId,
    payloads::{
        AuthoritativeReclaimMaintenanceDeclaration, CompactionMaintenanceDeclaration,
        DerivedFamilyRebuildMaintenanceDeclaration, MaintenanceAuditMaintenanceDeclaration,
        MaintenanceDeclarationClass, RebuildMaintenanceDeclaration, ReclaimMaintenanceDeclaration,
        ReplicationPreparationMaintenanceDeclaration, RetentionMaintenanceDeclaration,
        SnapshotRefreshMaintenanceDeclaration, TierMoveMaintenanceDeclaration,
        TierPlacementMaintenanceDeclaration,
    },
};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum MaintenanceDeclaration {
    Retention {
        id: MaintenanceDeclarationId,
        declaration: RetentionMaintenanceDeclaration,
    },
    Compaction {
        id: MaintenanceDeclarationId,
        declaration: CompactionMaintenanceDeclaration,
    },
    Reclaim {
        id: MaintenanceDeclarationId,
        declaration: ReclaimMaintenanceDeclaration,
    },
    AuthoritativeReclaim {
        id: MaintenanceDeclarationId,
        declaration: AuthoritativeReclaimMaintenanceDeclaration,
    },
    Rebuild {
        id: MaintenanceDeclarationId,
        declaration: RebuildMaintenanceDeclaration,
    },
    DerivedFamilyRebuild {
        id: MaintenanceDeclarationId,
        declaration: DerivedFamilyRebuildMaintenanceDeclaration,
    },
    SnapshotRefresh {
        id: MaintenanceDeclarationId,
        declaration: SnapshotRefreshMaintenanceDeclaration,
    },
    ReplicationPreparation {
        id: MaintenanceDeclarationId,
        declaration: ReplicationPreparationMaintenanceDeclaration,
    },
    MaintenanceAudit {
        id: MaintenanceDeclarationId,
        declaration: MaintenanceAuditMaintenanceDeclaration,
    },
    TierPlacementProposal {
        id: MaintenanceDeclarationId,
        declaration: TierPlacementMaintenanceDeclaration,
    },
    TierMoveExecution {
        id: MaintenanceDeclarationId,
        declaration: TierMoveMaintenanceDeclaration,
    },
}

impl MaintenanceDeclaration {
    pub(crate) fn retention(
        id: MaintenanceDeclarationId,
        declaration: RetentionMaintenanceDeclaration,
    ) -> Self {
        Self::Retention { id, declaration }
    }

    pub(crate) fn compaction(
        id: MaintenanceDeclarationId,
        declaration: CompactionMaintenanceDeclaration,
    ) -> Self {
        Self::Compaction { id, declaration }
    }

    pub(crate) fn reclaim(
        id: MaintenanceDeclarationId,
        declaration: ReclaimMaintenanceDeclaration,
    ) -> Self {
        Self::Reclaim { id, declaration }
    }

    pub(crate) fn authoritative_reclaim(
        id: MaintenanceDeclarationId,
        declaration: AuthoritativeReclaimMaintenanceDeclaration,
    ) -> Self {
        Self::AuthoritativeReclaim { id, declaration }
    }

    #[allow(dead_code)]
    pub(crate) fn rebuild(
        id: MaintenanceDeclarationId,
        declaration: RebuildMaintenanceDeclaration,
    ) -> Self {
        Self::Rebuild { id, declaration }
    }

    pub(crate) fn tier_placement_proposal(
        id: MaintenanceDeclarationId,
        declaration: TierPlacementMaintenanceDeclaration,
    ) -> Self {
        Self::TierPlacementProposal { id, declaration }
    }

    pub(crate) fn snapshot_refresh(
        id: MaintenanceDeclarationId,
        declaration: SnapshotRefreshMaintenanceDeclaration,
    ) -> Self {
        Self::SnapshotRefresh { id, declaration }
    }

    pub(crate) fn derived_family_rebuild(
        id: MaintenanceDeclarationId,
        declaration: DerivedFamilyRebuildMaintenanceDeclaration,
    ) -> Self {
        Self::DerivedFamilyRebuild { id, declaration }
    }

    pub(crate) fn replication_preparation(
        id: MaintenanceDeclarationId,
        declaration: ReplicationPreparationMaintenanceDeclaration,
    ) -> Self {
        Self::ReplicationPreparation { id, declaration }
    }

    pub(crate) fn maintenance_audit(
        id: MaintenanceDeclarationId,
        declaration: MaintenanceAuditMaintenanceDeclaration,
    ) -> Self {
        Self::MaintenanceAudit { id, declaration }
    }

    pub(crate) fn tier_move_execution(
        id: MaintenanceDeclarationId,
        declaration: TierMoveMaintenanceDeclaration,
    ) -> Self {
        Self::TierMoveExecution { id, declaration }
    }

    pub fn id(&self) -> &MaintenanceDeclarationId {
        match self {
            Self::Retention { id, .. }
            | Self::Compaction { id, .. }
            | Self::Reclaim { id, .. }
            | Self::AuthoritativeReclaim { id, .. }
            | Self::Rebuild { id, .. }
            | Self::DerivedFamilyRebuild { id, .. }
            | Self::SnapshotRefresh { id, .. }
            | Self::ReplicationPreparation { id, .. }
            | Self::MaintenanceAudit { id, .. }
            | Self::TierPlacementProposal { id, .. }
            | Self::TierMoveExecution { id, .. } => id,
        }
    }

    pub fn class(&self) -> MaintenanceDeclarationClass {
        match self {
            Self::Retention { .. } => MaintenanceDeclarationClass::Retention,
            Self::Compaction { .. } => MaintenanceDeclarationClass::Compaction,
            Self::Reclaim { .. } | Self::AuthoritativeReclaim { .. } => {
                MaintenanceDeclarationClass::Reclaim
            }
            Self::Rebuild { .. } => MaintenanceDeclarationClass::Rebuild,
            Self::DerivedFamilyRebuild { .. } => MaintenanceDeclarationClass::DerivedFamilyRebuild,
            Self::SnapshotRefresh { .. } => MaintenanceDeclarationClass::SnapshotRefresh,
            Self::ReplicationPreparation { .. } => {
                MaintenanceDeclarationClass::ReplicationPreparation
            }
            Self::MaintenanceAudit { .. } => MaintenanceDeclarationClass::MaintenanceAudit,
            Self::TierPlacementProposal { .. } => {
                MaintenanceDeclarationClass::TierPlacementProposal
            }
            Self::TierMoveExecution { .. } => MaintenanceDeclarationClass::TierMoveExecution,
        }
    }

    pub fn retained_basis_label(&self) -> Option<&str> {
        match self {
            Self::Retention { .. } => None,
            Self::Compaction { declaration, .. } => Some(declaration.retained_basis_label()),
            Self::Reclaim { declaration, .. } => Some(declaration.retained_basis_label()),
            Self::AuthoritativeReclaim { .. } => None,
            Self::Rebuild { declaration, .. } => Some(declaration.retained_basis_label()),
            Self::DerivedFamilyRebuild { declaration, .. } => {
                Some(declaration.retained_basis_label())
            }
            Self::SnapshotRefresh { .. }
            | Self::ReplicationPreparation { .. }
            | Self::MaintenanceAudit { .. }
            | Self::TierPlacementProposal { .. }
            | Self::TierMoveExecution { .. } => None,
        }
    }
}
