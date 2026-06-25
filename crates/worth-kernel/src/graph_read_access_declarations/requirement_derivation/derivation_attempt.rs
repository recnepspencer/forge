use super::super::read_family_catalog::WorthGraphReadDeclarationCatalogRecord;
use super::stable_identity_digest::stable_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadRequirementDerivationAttempt {
    catalog_record_digest: String,
    query_family_anchor_digest: String,
    has_query_read_family_artifact: bool,
    has_access_shape_artifact: bool,
    has_selectivity_shape_artifact: bool,
    attempted_query_requirement_derivation: bool,
    attempt_digest: String,
}

impl WorthGraphReadRequirementDerivationAttempt {
    pub(crate) fn anchor_only(record: &WorthGraphReadDeclarationCatalogRecord) -> Self {
        Self::new(record, false, false, false, false)
    }

    fn new(
        record: &WorthGraphReadDeclarationCatalogRecord,
        has_query_read_family_artifact: bool,
        has_access_shape_artifact: bool,
        has_selectivity_shape_artifact: bool,
        attempted_query_requirement_derivation: bool,
    ) -> Self {
        let catalog_record_digest = record.declaration_identity_digest().to_string();
        let query_family_anchor_digest = record
            .query_family_anchor()
            .family_digest_seed()
            .to_string();
        let attempt_digest = stable_digest(&[
            "worth_graph_read_requirement_derivation_attempt_v1".to_string(),
            format!("catalog_record:{catalog_record_digest}"),
            format!("query_family_anchor:{query_family_anchor_digest}"),
            format!("has_query_read_family:{has_query_read_family_artifact}"),
            format!("has_access_shape:{has_access_shape_artifact}"),
            format!("has_selectivity_shape:{has_selectivity_shape_artifact}"),
            format!("attempted_query_derivation:{attempted_query_requirement_derivation}"),
        ]);
        Self {
            catalog_record_digest,
            query_family_anchor_digest,
            has_query_read_family_artifact,
            has_access_shape_artifact,
            has_selectivity_shape_artifact,
            attempted_query_requirement_derivation,
            attempt_digest,
        }
    }

    pub fn catalog_record_digest(&self) -> &str {
        &self.catalog_record_digest
    }

    pub fn query_family_anchor_digest(&self) -> &str {
        &self.query_family_anchor_digest
    }

    pub const fn has_query_read_family_artifact(&self) -> bool {
        self.has_query_read_family_artifact
    }

    pub const fn has_access_shape_artifact(&self) -> bool {
        self.has_access_shape_artifact
    }

    pub const fn has_selectivity_shape_artifact(&self) -> bool {
        self.has_selectivity_shape_artifact
    }

    pub const fn attempted_query_requirement_derivation(&self) -> bool {
        self.attempted_query_requirement_derivation
    }

    pub fn attempt_digest(&self) -> &str {
        &self.attempt_digest
    }
}
