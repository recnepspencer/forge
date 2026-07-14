use super::{
    WorthServerQueryDependencyAuditRow, WorthServerQueryDependencyClosurePosture,
    WorthServerQueryDependencyCoveredPathInventory, WorthServerQueryDependencySupportPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerQueryDependencyAuditReceipt {
    covered_path_inventory: WorthServerQueryDependencyCoveredPathInventory,
    rows: Vec<WorthServerQueryDependencyAuditRow>,
    support_posture: WorthServerQueryDependencySupportPosture,
    audit_digest: String,
}

impl WorthServerQueryDependencyAuditReceipt {
    pub(crate) fn new(
        covered_path_inventory: WorthServerQueryDependencyCoveredPathInventory,
        rows: Vec<WorthServerQueryDependencyAuditRow>,
    ) -> Self {
        let support_posture = WorthServerQueryDependencySupportPosture::from_rows(&rows);
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

    pub fn covered_path_inventory(&self) -> &WorthServerQueryDependencyCoveredPathInventory {
        &self.covered_path_inventory
    }

    pub fn rows(&self) -> &[WorthServerQueryDependencyAuditRow] {
        &self.rows
    }

    pub fn row(
        &self,
        path_kind: super::WorthServerQueryDependencyAuditPathKind,
    ) -> Option<&WorthServerQueryDependencyAuditRow> {
        self.rows.iter().find(|row| row.path_kind() == path_kind)
    }

    pub fn ordinary_rows(&self) -> Vec<&WorthServerQueryDependencyAuditRow> {
        self.rows.iter().filter(|row| row.ordinary_path()).collect()
    }

    pub fn rows_with_closure_posture(
        &self,
        posture: WorthServerQueryDependencyClosurePosture,
    ) -> Vec<&WorthServerQueryDependencyAuditRow> {
        self.rows
            .iter()
            .filter(|row| row.closure_posture() == posture)
            .collect()
    }

    pub fn support_posture(&self) -> &WorthServerQueryDependencySupportPosture {
        &self.support_posture
    }

    pub fn is_runtime_ready_for_phase_one(&self) -> bool {
        self.support_posture.runtime_ready_for_phase_one()
    }

    pub fn audit_digest(&self) -> &str {
        &self.audit_digest
    }
}

pub type WorthServerQueryDependencyAudit = WorthServerQueryDependencyAuditReceipt;
