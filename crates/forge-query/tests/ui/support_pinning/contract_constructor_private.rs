use forge_query::facade::consumer_kit::{
    ForgeQuerySupportPinContract, ForgeQuerySupportPinContractSchemaVersion,
};

fn main() {
    let _ = ForgeQuerySupportPinContract {
        consumer_name: String::new(),
        contract_schema_version: ForgeQuerySupportPinContractSchemaVersion::current(),
        contract_schema_identity: String::new(),
        pinned_vocabulary_identity: String::new(),
        support_snapshot_schema_identity: String::new(),
        source_matrix_digest: String::new(),
        requirements: Vec::new(),
        observed_rows: Vec::new(),
        contract_digest: String::new(),
    };
}
