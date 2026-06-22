use crate::ForgeQueryEvidenceIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryConsumerResidueSourceInventory {
    audited_source_paths: Vec<String>,
    skipped_non_rust_file_count: usize,
    inventory_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryConsumerResidueSourceInventory {
    pub(crate) fn sealed(
        mut audited_source_paths: Vec<String>,
        skipped_non_rust_file_count: usize,
        inventory_identity: ForgeQueryEvidenceIdentity,
    ) -> Self {
        audited_source_paths.sort();
        Self {
            audited_source_paths,
            skipped_non_rust_file_count,
            inventory_identity,
        }
    }

    pub fn audited_source_paths(&self) -> &[String] {
        &self.audited_source_paths
    }

    pub fn audited_source_count(&self) -> usize {
        self.audited_source_paths.len()
    }

    pub fn skipped_non_rust_file_count(&self) -> usize {
        self.skipped_non_rust_file_count
    }

    pub fn inventory_digest(&self) -> &str {
        self.inventory_identity.as_str()
    }

    pub fn inventory_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.inventory_identity
    }
}
