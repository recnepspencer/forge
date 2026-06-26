use serde::Serialize;

use super::deletion_ledger::DerivedInvalidationDeletionLedger;
use super::residue_audit::DerivedInvalidationResidueAudit;
use super::source_firewall::DerivedInvalidationDeletionSourceFirewall;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationDeletionAudit {
    scanned_source_count: usize,
    observed_old_authority_pattern_count: usize,
    ordinary_dirty_path_count: usize,
    ordinary_whole_view_rebuild_count: usize,
    migrated_or_denied_old_authority_count: usize,
    certification_residue_count: usize,
    audit_digest: String,
}

impl DerivedInvalidationDeletionAudit {
    pub(crate) fn from_products(
        deletion_ledger: &DerivedInvalidationDeletionLedger,
        residue_audit: &DerivedInvalidationResidueAudit,
        source_firewall: &DerivedInvalidationDeletionSourceFirewall,
    ) -> Self {
        let ordinary_dirty_path_count = source_firewall
            .violations()
            .iter()
            .filter(|violation| violation.is_dirty_path_authority())
            .count();
        let ordinary_whole_view_rebuild_count = source_firewall
            .violations()
            .iter()
            .filter(|violation| violation.is_whole_view_rebuild_authority())
            .count();
        let migrated_or_denied_old_authority_count = deletion_ledger.rows().len();
        let certification_residue_count = residue_audit.rows().len();
        let scanned_source_count = source_firewall.scanned_source_count();
        let observed_old_authority_pattern_count = source_firewall.observed_pattern_count();
        let audit_digest = super::super::catalog::catalog_digest([
            "worth-topo:derived-invalidation-deletion-audit:v1".to_string(),
            format!("scanned-sources:{scanned_source_count}"),
            format!("observed-patterns:{observed_old_authority_pattern_count}"),
            format!("ordinary-dirty-paths:{ordinary_dirty_path_count}"),
            format!("ordinary-whole-view-rebuilds:{ordinary_whole_view_rebuild_count}"),
            format!("migrated-denied:{migrated_or_denied_old_authority_count}"),
            format!("certification-residue:{certification_residue_count}"),
            format!("source-firewall:{}", source_firewall.report_digest()),
            format!("deletion-ledger:{}", deletion_ledger.ledger_digest()),
            format!("residue-audit:{}", residue_audit.audit_digest()),
        ]);
        Self {
            scanned_source_count,
            observed_old_authority_pattern_count,
            ordinary_dirty_path_count,
            ordinary_whole_view_rebuild_count,
            migrated_or_denied_old_authority_count,
            certification_residue_count,
            audit_digest,
        }
    }

    pub const fn scanned_source_count(&self) -> usize {
        self.scanned_source_count
    }

    pub const fn observed_old_authority_pattern_count(&self) -> usize {
        self.observed_old_authority_pattern_count
    }

    pub const fn ordinary_dirty_path_count(&self) -> usize {
        self.ordinary_dirty_path_count
    }

    pub const fn ordinary_whole_view_rebuild_count(&self) -> usize {
        self.ordinary_whole_view_rebuild_count
    }

    pub const fn migrated_or_denied_old_authority_count(&self) -> usize {
        self.migrated_or_denied_old_authority_count
    }

    pub const fn certification_residue_count(&self) -> usize {
        self.certification_residue_count
    }

    pub fn audit_digest(&self) -> &str {
        &self.audit_digest
    }
}
