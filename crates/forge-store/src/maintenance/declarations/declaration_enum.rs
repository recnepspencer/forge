use super::{
    declaration_id::MaintenanceDeclarationId,
    payloads::{
        AuthoritativeReclaimMaintenanceDeclaration, CompactionMaintenanceDeclaration,
        MaintenanceDeclarationClass, RebuildMaintenanceDeclaration, ReclaimMaintenanceDeclaration,
        RetentionMaintenanceDeclaration,
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

    pub fn id(&self) -> &MaintenanceDeclarationId {
        match self {
            Self::Retention { id, .. }
            | Self::Compaction { id, .. }
            | Self::Reclaim { id, .. }
            | Self::AuthoritativeReclaim { id, .. }
            | Self::Rebuild { id, .. } => id,
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
        }
    }

    pub fn retained_basis_label(&self) -> Option<&str> {
        match self {
            Self::Retention { .. } => None,
            Self::Compaction { declaration, .. } => Some(declaration.retained_basis_label()),
            Self::Reclaim { declaration, .. } => Some(declaration.retained_basis_label()),
            Self::AuthoritativeReclaim { .. } => None,
            Self::Rebuild { declaration, .. } => Some(declaration.retained_basis_label()),
        }
    }
}
