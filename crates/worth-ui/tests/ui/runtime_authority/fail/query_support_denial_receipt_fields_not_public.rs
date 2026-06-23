use worth_ui::facade::{
    WorthUiQuerySupportDenialKind, WorthUiQuerySupportDenialReceipt, WorthUiQuerySupportStatus,
};

fn main() {
    let _ = WorthUiQuerySupportDenialReceipt {
        kind: WorthUiQuerySupportDenialKind::Unsupported,
        support_status: WorthUiQuerySupportStatus::Unsupported,
        support_receipt_digest: 7,
        runtime_hook_count: 0,
        denied_binding_count: 0,
    };
}
