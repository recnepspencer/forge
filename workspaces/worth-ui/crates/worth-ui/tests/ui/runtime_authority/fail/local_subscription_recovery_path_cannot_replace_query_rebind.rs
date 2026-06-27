use worth_ui::facade::WorthUiQueryBindingRebind;

struct LocalSubscriptionRecoveryPath {
    retry_label: &'static str,
}

fn accepts_query_rebind(_rebind: WorthUiQueryBindingRebind) {}

fn main() {
    accepts_query_rebind(LocalSubscriptionRecoveryPath {
        retry_label: "retry locally",
    });
}
