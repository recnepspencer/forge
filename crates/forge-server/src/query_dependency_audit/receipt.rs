use super::{
    ForgeServerQueryDependencyAuditRow, ForgeServerQueryDependencyClosurePosture,
    ForgeServerQueryDependencyCoveredPathInventory, ForgeServerQueryDependencySupportPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerQueryDependencyAuditReceipt {
    covered_path_inventory: ForgeServerQueryDependencyCoveredPathInventory,
    rows: Vec<ForgeServerQueryDependencyAuditRow>,
    support_posture: ForgeServerQueryDependencySupportPosture,
    audit_digest: String,
}

impl ForgeServerQueryDependencyAuditReceipt {
    pub(crate) fn new(
        covered_path_inventory: ForgeServerQueryDependencyCoveredPathInventory,
        rows: Vec<ForgeServerQueryDependencyAuditRow>,
    ) -> Self {
        let support_posture = ForgeServerQueryDependencySupportPosture::from_rows(&rows);
        let audit_digest = rows
            .iter()
            .map(|row| row.canonical_digest())
            .collect::<Vec<_>>()
            .join("|");
        Self {
            covered_path_inventory,
            rows,
            support_posture,
            audit_digest,
        }
    }

    pub fn covered_path_inventory(&self) -> &ForgeServerQueryDependencyCoveredPathInventory {
        &self.covered_path_inventory
    }

    pub fn rows(&self) -> &[ForgeServerQueryDependencyAuditRow] {
        &self.rows
    }

    pub fn row(
        &self,
        path_kind: super::ForgeServerQueryDependencyAuditPathKind,
    ) -> Option<&ForgeServerQueryDependencyAuditRow> {
        self.rows.iter().find(|row| row.path_kind() == path_kind)
    }

    pub fn ordinary_rows(&self) -> Vec<&ForgeServerQueryDependencyAuditRow> {
        self.rows.iter().filter(|row| row.ordinary_path()).collect()
    }

    pub fn rows_with_closure_posture(
        &self,
        posture: ForgeServerQueryDependencyClosurePosture,
    ) -> Vec<&ForgeServerQueryDependencyAuditRow> {
        self.rows
            .iter()
            .filter(|row| row.closure_posture() == posture)
            .collect()
    }

    pub fn support_posture(&self) -> &ForgeServerQueryDependencySupportPosture {
        &self.support_posture
    }

    pub fn is_runtime_ready_for_phase_one(&self) -> bool {
        self.support_posture.runtime_ready_for_phase_one()
    }

    pub fn audit_digest(&self) -> &str {
        &self.audit_digest
    }
}

pub type ForgeServerQueryDependencyAudit = ForgeServerQueryDependencyAuditReceipt;
