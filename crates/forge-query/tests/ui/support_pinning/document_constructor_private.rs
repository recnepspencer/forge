use forge_query::facade::consumer_kit::ForgeQuerySupportPinContractDocument;

fn main() {
    let _ = ForgeQuerySupportPinContractDocument {
        schema_version: 1,
        schema_identity: String::new(),
        pinned_vocabulary_identity: String::new(),
        support_snapshot_schema_identity: String::new(),
        source_matrix_digest: String::new(),
        consumer_name: String::new(),
        contract_digest: String::new(),
        document_digest: String::new(),
        requirements: Vec::new(),
        observed_rows: Vec::new(),
    };
}
