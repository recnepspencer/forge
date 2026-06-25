use serde::Serialize;

use super::super::super::inventory::{
    DerivedInvalidationAuthorityInventoryRow, DerivedInvalidationAuthorityOwner,
    DerivedInvalidationOldAuthorityKind, DerivedInvalidationProductCategory,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationResidueAuditRow {
    source_path: String,
    surface: String,
    product_category: DerivedInvalidationProductCategory,
    authority_kind: DerivedInvalidationOldAuthorityKind,
    owner: DerivedInvalidationAuthorityOwner,
    capped_count: usize,
    blocker: String,
    removal_trigger: String,
    certification_or_bootstrap_only: bool,
    ordinary_invalidation_admissible: bool,
    inventory_row_digest: String,
    row_digest: String,
}

impl DerivedInvalidationResidueAuditRow {
    pub(crate) fn from_inventory_row(row: &DerivedInvalidationAuthorityInventoryRow) -> Self {
        let capped_count = row.cap().unwrap_or(0);
        let ordinary_invalidation_admissible = false;
        let row_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:derived-invalidation-residue-audit-row:v1".to_string(),
            format!("source:{}", row.source_path()),
            format!("surface:{}", row.surface()),
            format!("category:{}", row.product_category().as_str()),
            format!("authority:{}", row.authority_kind().as_str()),
            format!("owner:{}", row.owner().as_str()),
            format!("cap:{capped_count}"),
            format!("blocker:{}", row.blocker()),
            format!("removal-trigger:{}", row.removal_trigger()),
            format!(
                "certification-bootstrap:{}",
                row.certification_or_bootstrap_only()
            ),
            format!("ordinary-admissible:{ordinary_invalidation_admissible}"),
            format!("inventory-row:{}", row.row_digest()),
        ]);
        Self {
            source_path: row.source_path().to_string(),
            surface: row.surface().to_string(),
            product_category: row.product_category(),
            authority_kind: row.authority_kind(),
            owner: row.owner(),
            capped_count,
            blocker: row.blocker().to_string(),
            removal_trigger: row.removal_trigger().to_string(),
            certification_or_bootstrap_only: row.certification_or_bootstrap_only(),
            ordinary_invalidation_admissible,
            inventory_row_digest: row.row_digest().to_string(),
            row_digest,
        }
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub fn surface(&self) -> &str {
        &self.surface
    }

    pub const fn product_category(&self) -> DerivedInvalidationProductCategory {
        self.product_category
    }

    pub const fn authority_kind(&self) -> DerivedInvalidationOldAuthorityKind {
        self.authority_kind
    }

    pub const fn owner(&self) -> DerivedInvalidationAuthorityOwner {
        self.owner
    }

    pub const fn capped_count(&self) -> usize {
        self.capped_count
    }

    pub fn blocker(&self) -> &str {
        &self.blocker
    }

    pub fn removal_trigger(&self) -> &str {
        &self.removal_trigger
    }

    pub const fn certification_or_bootstrap_only(&self) -> bool {
        self.certification_or_bootstrap_only
    }

    pub const fn ordinary_invalidation_admissible(&self) -> bool {
        self.ordinary_invalidation_admissible
    }

    pub fn inventory_row_digest(&self) -> &str {
        &self.inventory_row_digest
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}
