use worth_query::facade::consumer_kit::{
    load_support_pin_contract_terminal_json_document,
    load_support_snapshot_terminal_json_document, WorthQuerySupportPinContractSchemaVersion,
    WorthQuerySupportSnapshotSchemaVersion,
};

fn main() {
    let _ = load_support_snapshot_terminal_json_document(
        "{}",
        WorthQuerySupportSnapshotSchemaVersion::current(),
    );
    let _ = load_support_pin_contract_terminal_json_document(
        "{}",
        WorthQuerySupportPinContractSchemaVersion::current(),
    );
}
