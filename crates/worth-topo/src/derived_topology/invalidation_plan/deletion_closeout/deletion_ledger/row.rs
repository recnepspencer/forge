use serde::Serialize;

use super::super::super::inventory::{
    DerivedInvalidationAuthorityInventoryRow, DerivedInvalidationAuthorityOwner,
    DerivedInvalidationOldAuthorityKind, DerivedInvalidationProductCategory,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DerivedInvalidationDeletionDisposition {
    MigratedAuthorityDeleted,
    OldAuthorityDenied,
}

impl DerivedInvalidationDeletionDisposition {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MigratedAuthorityDeleted => "migrated_authority_deleted",
            Self::OldAuthorityDenied => "old_authority_denied",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationDeletionRow {
    source_path: String,
    surface: String,
    product_category: DerivedInvalidationProductCategory,
    authority_kind: DerivedInvalidationOldAuthorityKind,
    owner: DerivedInvalidationAuthorityOwner,
    disposition: DerivedInvalidationDeletionDisposition,
    inventory_row_digest: String,
    row_digest: String,
}

impl DerivedInvalidationDeletionRow {
    pub(crate) fn from_inventory_row(
        row: &DerivedInvalidationAuthorityInventoryRow,
        disposition: DerivedInvalidationDeletionDisposition,
    ) -> Self {
        let row_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:derived-invalidation-deletion-row:v1".to_string(),
            format!("source:{}", row.source_path()),
            format!("surface:{}", row.surface()),
            format!("category:{}", row.product_category().as_str()),
            format!("authority:{}", row.authority_kind().as_str()),
            format!("owner:{}", row.owner().as_str()),
            format!("disposition:{}", disposition.as_str()),
            format!("inventory-row:{}", row.row_digest()),
        ]);
        Self {
            source_path: row.source_path().to_string(),
            surface: row.surface().to_string(),
            product_category: row.product_category(),
            authority_kind: row.authority_kind(),
            owner: row.owner(),
            disposition,
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

    pub const fn disposition(&self) -> DerivedInvalidationDeletionDisposition {
        self.disposition
    }

    pub fn inventory_row_digest(&self) -> &str {
        &self.inventory_row_digest
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}
