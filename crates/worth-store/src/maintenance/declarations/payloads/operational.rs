use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MaintenanceAuditMaintenanceDeclaration {
    audit_family: String,
    locality_label: String,
    audit_label: String,
}

impl MaintenanceAuditMaintenanceDeclaration {
    pub(crate) fn new(
        audit_family: impl Into<String>,
        locality_label: impl Into<String>,
        audit_label: impl Into<String>,
    ) -> Self {
        Self {
            audit_family: audit_family.into(),
            locality_label: locality_label.into(),
            audit_label: audit_label.into(),
        }
    }

    pub fn audit_family(&self) -> &str {
        &self.audit_family
    }

    pub fn locality_label(&self) -> &str {
        &self.locality_label
    }

    pub fn audit_label(&self) -> &str {
        &self.audit_label
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TierPlacementMaintenanceDeclaration {
    placement_family: String,
    locality_label: String,
    proposal_label: String,
}

impl TierPlacementMaintenanceDeclaration {
    pub(crate) fn new(
        placement_family: impl Into<String>,
        locality_label: impl Into<String>,
        proposal_label: impl Into<String>,
    ) -> Self {
        Self {
            placement_family: placement_family.into(),
            locality_label: locality_label.into(),
            proposal_label: proposal_label.into(),
        }
    }

    pub fn placement_family(&self) -> &str {
        &self.placement_family
    }

    pub fn locality_label(&self) -> &str {
        &self.locality_label
    }

    pub fn proposal_label(&self) -> &str {
        &self.proposal_label
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TierMoveMaintenanceDeclaration {
    placement_family: String,
    locality_label: String,
    move_label: String,
    cross_locality_debt: bool,
}

impl TierMoveMaintenanceDeclaration {
    pub(crate) fn new(
        placement_family: impl Into<String>,
        locality_label: impl Into<String>,
        move_label: impl Into<String>,
        cross_locality_debt: bool,
    ) -> Self {
        Self {
            placement_family: placement_family.into(),
            locality_label: locality_label.into(),
            move_label: move_label.into(),
            cross_locality_debt,
        }
    }

    pub fn placement_family(&self) -> &str {
        &self.placement_family
    }

    pub fn locality_label(&self) -> &str {
        &self.locality_label
    }

    pub fn move_label(&self) -> &str {
        &self.move_label
    }

    pub fn cross_locality_debt(&self) -> bool {
        self.cross_locality_debt
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SnapshotRefreshMaintenanceDeclaration {
    snapshot_family: String,
    locality_label: String,
    refresh_label: String,
}

impl SnapshotRefreshMaintenanceDeclaration {
    pub(crate) fn new(
        snapshot_family: impl Into<String>,
        locality_label: impl Into<String>,
        refresh_label: impl Into<String>,
    ) -> Self {
        Self {
            snapshot_family: snapshot_family.into(),
            locality_label: locality_label.into(),
            refresh_label: refresh_label.into(),
        }
    }

    pub fn snapshot_family(&self) -> &str {
        &self.snapshot_family
    }

    pub fn locality_label(&self) -> &str {
        &self.locality_label
    }

    pub fn refresh_label(&self) -> &str {
        &self.refresh_label
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReplicationPreparationMaintenanceDeclaration {
    replication_family: String,
    locality_label: String,
    preparation_label: String,
}

impl ReplicationPreparationMaintenanceDeclaration {
    pub(crate) fn new(
        replication_family: impl Into<String>,
        locality_label: impl Into<String>,
        preparation_label: impl Into<String>,
    ) -> Self {
        Self {
            replication_family: replication_family.into(),
            locality_label: locality_label.into(),
            preparation_label: preparation_label.into(),
        }
    }

    pub fn replication_family(&self) -> &str {
        &self.replication_family
    }

    pub fn locality_label(&self) -> &str {
        &self.locality_label
    }

    pub fn preparation_label(&self) -> &str {
        &self.preparation_label
    }
}
