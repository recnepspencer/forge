use super::catalog::WorthGraphReadDeclarationCatalog;
use super::catalog_record::WorthGraphReadDeclarationCatalogRecord;
use crate::graph_read_access_inventory::WorthGraphReadDeletionLedgerItem;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessDeclarationPhaseThreeSeed {
    catalog_records: Vec<WorthGraphReadDeclarationCatalogRecord>,
    deletion_items: Vec<WorthGraphReadDeletionLedgerItem>,
    catalog_digest: String,
}

impl WorthGraphReadAccessDeclarationPhaseThreeSeed {
    pub(crate) fn from_catalog(
        catalog: &WorthGraphReadDeclarationCatalog,
        deletion_items: Vec<WorthGraphReadDeletionLedgerItem>,
    ) -> Self {
        Self {
            catalog_records: catalog.records().to_vec(),
            deletion_items,
            catalog_digest: catalog.catalog_digest().to_string(),
        }
    }

    pub fn catalog_records(&self) -> &[WorthGraphReadDeclarationCatalogRecord] {
        &self.catalog_records
    }

    pub fn catalog_digest(&self) -> &str {
        &self.catalog_digest
    }

    pub fn deletion_items(&self) -> &[WorthGraphReadDeletionLedgerItem] {
        &self.deletion_items
    }

    pub const fn claims_execution_authority(&self) -> bool {
        false
    }
}
