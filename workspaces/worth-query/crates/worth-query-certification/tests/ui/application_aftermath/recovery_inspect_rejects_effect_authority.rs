use worth_query_execution::facade::primary_graph::{
    inspect_recovery_handle, WorthQueryRecoveryEffectAuthority, WorthQueryRecoveryHandle,
};

fn cannot_inspect_with_effect_authority(
    handle: &WorthQueryRecoveryHandle,
    effect: &WorthQueryRecoveryEffectAuthority,
) {
    // Inspect requires InspectAuthority (disclosure-backed), not effect authority.
    let _ = inspect_recovery_handle(handle, effect);
}

fn main() {}
