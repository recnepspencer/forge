use serde::Serialize;

use super::{CoveredDerivedProductMigrationStatus, CoveredDerivedProductStatusRow};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CoveredDerivedProductMigrationCounters {
    required_family_count: usize,
    migrated_family_count: usize,
    deleted_old_authority_count: usize,
    certification_residue_only_count: usize,
    ordinary_consumable_family_count: usize,
    selected_family_count: usize,
    counters_digest: String,
}

impl CoveredDerivedProductMigrationCounters {
    pub(crate) fn from_rows(
        rows: &[CoveredDerivedProductStatusRow],
        selected_count: usize,
    ) -> Self {
        let migrated_family_count = rows
            .iter()
            .filter(|row| row.status() == CoveredDerivedProductMigrationStatus::Migrated)
            .count();
        let deleted_old_authority_count = rows
            .iter()
            .filter(|row| row.status() == CoveredDerivedProductMigrationStatus::DeletedOldAuthority)
            .count();
        let certification_residue_only_count = rows
            .iter()
            .filter(|row| {
                row.status() == CoveredDerivedProductMigrationStatus::CertificationResidueOnly
            })
            .count();
        let ordinary_consumable_family_count = rows
            .iter()
            .filter(|row| row.ordinary_invalidation_consumable())
            .count();
        let required_family_count = rows.len();
        let counters_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:covered-derived-product-migration-counters:v1".to_string(),
            format!("required-families:{required_family_count}"),
            format!("migrated-families:{migrated_family_count}"),
            format!("deleted-old-authority:{deleted_old_authority_count}"),
            format!("certification-residue-only:{certification_residue_only_count}"),
            format!("ordinary-consumable:{ordinary_consumable_family_count}"),
            format!("selected-families:{selected_count}"),
        ]);
        Self {
            required_family_count,
            migrated_family_count,
            deleted_old_authority_count,
            certification_residue_only_count,
            ordinary_consumable_family_count,
            selected_family_count: selected_count,
            counters_digest,
        }
    }

    pub const fn required_family_count(&self) -> usize {
        self.required_family_count
    }

    pub const fn migrated_family_count(&self) -> usize {
        self.migrated_family_count
    }

    pub const fn deleted_old_authority_count(&self) -> usize {
        self.deleted_old_authority_count
    }

    pub const fn certification_residue_only_count(&self) -> usize {
        self.certification_residue_only_count
    }

    pub const fn ordinary_consumable_family_count(&self) -> usize {
        self.ordinary_consumable_family_count
    }

    pub const fn selected_family_count(&self) -> usize {
        self.selected_family_count
    }

    pub fn counters_digest(&self) -> &str {
        &self.counters_digest
    }
}
