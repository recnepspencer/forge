use crate::graph_read_access_inventory::WorthGraphReadAccessInventoryRowIdentity;

use super::catalog_key::{stable_digest, WorthGraphReadDeclarationCatalogKey};
use super::query_family_anchor::WorthGraphReadQueryFamilyAnchor;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadDeclarationCatalogRecord {
    key: WorthGraphReadDeclarationCatalogKey,
    declaration_identity_digest: String,
    query_family_anchor: WorthGraphReadQueryFamilyAnchor,
    source_row_identities: Vec<WorthGraphReadAccessInventoryRowIdentity>,
}

impl WorthGraphReadDeclarationCatalogRecord {
    pub(crate) fn new(
        key: WorthGraphReadDeclarationCatalogKey,
        source_row_identity: WorthGraphReadAccessInventoryRowIdentity,
    ) -> Self {
        let declaration_identity_digest = stable_digest(&[
            "worth_graph_read_declaration_catalog_record_v1".to_string(),
            format!("catalog_key:{}", key.key_digest()),
        ]);
        let query_family_anchor = WorthGraphReadQueryFamilyAnchor::from_catalog_key(&key);
        Self {
            key,
            declaration_identity_digest,
            query_family_anchor,
            source_row_identities: vec![source_row_identity],
        }
    }

    pub(crate) fn add_source_row_identity(
        &mut self,
        source_row_identity: WorthGraphReadAccessInventoryRowIdentity,
    ) {
        self.source_row_identities.push(source_row_identity);
        self.source_row_identities.sort();
    }

    pub fn key(&self) -> &WorthGraphReadDeclarationCatalogKey {
        &self.key
    }

    pub fn declaration_identity_digest(&self) -> &str {
        &self.declaration_identity_digest
    }

    pub fn query_family_anchor(&self) -> &WorthGraphReadQueryFamilyAnchor {
        &self.query_family_anchor
    }

    pub fn source_row_identities(&self) -> &[WorthGraphReadAccessInventoryRowIdentity] {
        &self.source_row_identities
    }

    pub fn source_candidate_count(&self) -> usize {
        self.source_row_identities.len()
    }
}
