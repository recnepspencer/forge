use worth_ui::facade::WorthUiCandidateOrderingReceipt;

fn main() {
    let _receipt = WorthUiCandidateOrderingReceipt {
        provider_id: String::from("provider"),
        source_revision_digest: 1,
        event_burst_digest: 2,
        debounce_policy_digest: 3,
        sequence: 4,
        receipt_digest: 5,
    };
}
