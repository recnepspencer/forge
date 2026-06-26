use crate::graph_read_access_inventory::WorthGraphReadAccessInventoryRowIdentity;

use super::stable_identity_digest::stable_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadRequirementSourceTrace {
    catalog_record_digest: String,
    catalog_key_digest: String,
    seed_requirement_evidence_digest: String,
    source_row_identities: Vec<WorthGraphReadAccessInventoryRowIdentity>,
    trace_digest: String,
}

impl WorthGraphReadRequirementSourceTrace {
    pub(crate) fn new(
        catalog_record_digest: impl Into<String>,
        catalog_key_digest: impl Into<String>,
        seed_requirement_evidence_digest: impl Into<String>,
        source_row_identities: Vec<WorthGraphReadAccessInventoryRowIdentity>,
    ) -> Self {
        let catalog_record_digest = catalog_record_digest.into();
        let catalog_key_digest = catalog_key_digest.into();
        let seed_requirement_evidence_digest = seed_requirement_evidence_digest.into();
        let mut trace_parts = vec![
            "worth_graph_read_requirement_source_trace_v1".to_string(),
            format!("catalog_record:{catalog_record_digest}"),
            format!("catalog_key:{catalog_key_digest}"),
            format!("seed_requirement:{seed_requirement_evidence_digest}"),
            format!("source_rows:{}", source_row_identities.len()),
        ];
        trace_parts.extend(source_row_identities.iter().map(|identity| {
            format!(
                "source_row:{}:{}:{}",
                identity.source_path(),
                identity.owner().as_str(),
                identity.current_caller()
            )
        }));
        let trace_digest = stable_digest(&trace_parts);
        Self {
            catalog_record_digest,
            catalog_key_digest,
            seed_requirement_evidence_digest,
            source_row_identities,
            trace_digest,
        }
    }

    pub fn catalog_record_digest(&self) -> &str {
        &self.catalog_record_digest
    }

    pub fn catalog_key_digest(&self) -> &str {
        &self.catalog_key_digest
    }

    pub fn seed_requirement_evidence_digest(&self) -> &str {
        &self.seed_requirement_evidence_digest
    }

    pub fn source_row_identities(&self) -> &[WorthGraphReadAccessInventoryRowIdentity] {
        &self.source_row_identities
    }

    pub fn trace_digest(&self) -> &str {
        &self.trace_digest
    }
}
