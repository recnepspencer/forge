use forge_query::facade::consumer_kit::{
    load_support_pin_contract_terminal_json_document,
    load_support_snapshot_terminal_json_document, ForgeQuerySupportPinContractSchemaVersion,
    ForgeQuerySupportSnapshotSchemaVersion,
};

fn main() {
    let _ = load_support_snapshot_terminal_json_document(
        "{}",
        ForgeQuerySupportSnapshotSchemaVersion::current(),
    );
    let _ = load_support_pin_contract_terminal_json_document(
        "{}",
        ForgeQuerySupportPinContractSchemaVersion::current(),
    );
}
