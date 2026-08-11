use worth_query_execution::facade::primary_graph::{
    dispose_recovery_handle, WorthQueryRecoveryEffectAuthority, WorthQueryRecoveryHandle,
};

fn cannot_dispose_twice(
    handle: WorthQueryRecoveryHandle,
    authority: &WorthQueryRecoveryEffectAuthority,
) {
    let _first = dispose_recovery_handle(handle, authority);
    // Handle was moved — a second transition is unrepresentable.
    let _second = dispose_recovery_handle(handle, authority);
}

fn main() {}
