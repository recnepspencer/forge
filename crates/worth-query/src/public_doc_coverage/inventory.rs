use super::row::WorthQueryPublicDocCoverageRow;
use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPublicDocCoverageInventory {
    source_inventory_digest: String,
    coverage_digest: String,
    rows: Vec<WorthQueryPublicDocCoverageRow>,
}

impl WorthQueryPublicDocCoverageInventory {
    pub(crate) fn new(
        source_inventory_digest: String,
        rows: Vec<WorthQueryPublicDocCoverageRow>,
    ) -> Self {
        let coverage_digest = hash_parts(
            &rows
                .iter()
                .map(|row| row.coverage_digest().to_string())
                .collect::<Vec<_>>(),
        );
        Self {
            source_inventory_digest,
            coverage_digest,
            rows,
        }
    }

    pub fn current() -> Self {
        super::current::worth_query_public_doc_coverage_inventory()
    }

    pub fn source_inventory_digest(&self) -> &str {
        &self.source_inventory_digest
    }

    pub fn coverage_digest(&self) -> &str {
        &self.coverage_digest
    }

    pub fn rows(&self) -> &[WorthQueryPublicDocCoverageRow] {
        &self.rows
    }

    pub fn row_for_public_name(
        &self,
        public_name: &str,
    ) -> Option<&WorthQueryPublicDocCoverageRow> {
        self.rows
            .iter()
            .find(|row| row.public_name() == public_name)
    }
}
