use worth_query::facade::consumer_kit::{
    load_support_pin_contract_document, load_support_snapshot_document,
    WorthQuerySupportPinContract, WorthQuerySupportPinContractDocument,
    WorthQuerySupportSnapshot, WorthQuerySupportSnapshotDocument,
};

fn snapshot_neutral_json_methods_are_removed(snapshot: WorthQuerySupportSnapshot) {
    let _ = snapshot.to_canonical_json();
    let _ = snapshot.to_stable_json();
    let _ = WorthQuerySupportSnapshotDocument::from_json("{}");
}

fn support_pin_neutral_json_methods_are_removed(contract: WorthQuerySupportPinContract) {
    let _ = contract.to_canonical_json();
    let _ = contract.to_stable_json();
    let _ = WorthQuerySupportPinContractDocument::from_json("{}");
}

fn neutral_loaders_are_removed() {
    let _ = load_support_snapshot_document;
    let _ = load_support_pin_contract_document;
}

fn main() {}
