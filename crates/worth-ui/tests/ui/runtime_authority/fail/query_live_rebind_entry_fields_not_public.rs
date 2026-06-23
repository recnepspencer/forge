use worth_ui::facade::{
    WorthUiQueryBindingIdentity, WorthUiQueryLiveRebindEntry, WorthUiQueryLiveRebindOutcome,
};

fn forged_query_binding_identity() -> WorthUiQueryBindingIdentity {
    panic!("fixture should not run")
}

fn forged_query_live_rebind_outcome() -> WorthUiQueryLiveRebindOutcome {
    panic!("fixture should not run")
}

fn main() {
    let _ = WorthUiQueryLiveRebindEntry {
        identity: forged_query_binding_identity(),
        outcome: forged_query_live_rebind_outcome(),
    };
}
