use serde::Serialize;

use super::deletion_audit::DerivedInvalidationDeletionAudit;
use super::deletion_ledger::DerivedInvalidationDeletionLedger;
use super::residue_audit::DerivedInvalidationResidueAudit;
use super::source_firewall::DerivedInvalidationDeletionSourceFirewall;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationDeletionCounters {
    deletion_row_count: usize,
    residue_row_count: usize,
    source_firewall_violation_count: usize,
    ordinary_dirty_path_count: usize,
    ordinary_whole_view_rebuild_count: usize,
    counters_digest: String,
}

impl DerivedInvalidationDeletionCounters {
    pub(crate) fn from_products(
        deletion_ledger: &DerivedInvalidationDeletionLedger,
        residue_audit: &DerivedInvalidationResidueAudit,
        source_firewall: &DerivedInvalidationDeletionSourceFirewall,
        deletion_audit: &DerivedInvalidationDeletionAudit,
    ) -> Self {
        let deletion_row_count = deletion_ledger.rows().len();
        let residue_row_count = residue_audit.rows().len();
        let source_firewall_violation_count = source_firewall.violations().len();
        let ordinary_dirty_path_count = deletion_audit.ordinary_dirty_path_count();
        let ordinary_whole_view_rebuild_count = deletion_audit.ordinary_whole_view_rebuild_count();
        let counters_digest = super::super::catalog::catalog_digest([
            "worth-topo:derived-invalidation-deletion-counters:v1".to_string(),
            format!("deletion-rows:{deletion_row_count}"),
            format!("residue-rows:{residue_row_count}"),
            format!("source-firewall-violations:{source_firewall_violation_count}"),
            format!("ordinary-dirty-paths:{ordinary_dirty_path_count}"),
            format!("ordinary-whole-view-rebuilds:{ordinary_whole_view_rebuild_count}"),
            format!("deletion-audit:{}", deletion_audit.audit_digest()),
        ]);
        Self {
            deletion_row_count,
            residue_row_count,
            source_firewall_violation_count,
            ordinary_dirty_path_count,
            ordinary_whole_view_rebuild_count,
            counters_digest,
        }
    }

    pub const fn deletion_row_count(&self) -> usize {
        self.deletion_row_count
    }

    pub const fn residue_row_count(&self) -> usize {
        self.residue_row_count
    }

    pub const fn source_firewall_violation_count(&self) -> usize {
        self.source_firewall_violation_count
    }

    pub const fn ordinary_dirty_path_count(&self) -> usize {
        self.ordinary_dirty_path_count
    }

    pub const fn ordinary_whole_view_rebuild_count(&self) -> usize {
        self.ordinary_whole_view_rebuild_count
    }

    pub fn counters_digest(&self) -> &str {
        &self.counters_digest
    }
}
