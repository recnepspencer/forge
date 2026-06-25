use super::catalog::WorthGraphReadDeclarationCatalog;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadDeclarationCatalogSummary {
    source_candidate_count: usize,
    catalog_record_count: usize,
    merged_source_row_count: usize,
    catalog_digest: String,
}

impl WorthGraphReadDeclarationCatalogSummary {
    pub(crate) fn from_catalog(catalog: &WorthGraphReadDeclarationCatalog) -> Self {
        let catalog_record_count = catalog.records().len();
        Self {
            source_candidate_count: catalog.source_candidate_count(),
            catalog_record_count,
            merged_source_row_count: catalog
                .source_candidate_count()
                .saturating_sub(catalog_record_count),
            catalog_digest: catalog.catalog_digest().to_string(),
        }
    }

    pub const fn source_candidate_count(&self) -> usize {
        self.source_candidate_count
    }

    pub const fn catalog_record_count(&self) -> usize {
        self.catalog_record_count
    }

    pub const fn merged_source_row_count(&self) -> usize {
        self.merged_source_row_count
    }

    pub fn catalog_digest(&self) -> &str {
        &self.catalog_digest
    }
}
