//! Positive twin: safe-retry is reachable only after re-dispatch performed.

use worth_query_execution::facade::primary_graph::{
    safe_retry_recovery_handle, WorthQueryPerformedExternalRedispatch,
    WorthQueryRecoveryEffectAuthority, WorthQueryRecoveryHandle, WorthQueryRecoveryHandleDenial,
    WorthQueryRecoverySafeRetryAdmission,
};

fn safe_retry_with_performed_redispatch(
    handle: WorthQueryRecoveryHandle,
    authority: &WorthQueryRecoveryEffectAuthority,
    redispatch: WorthQueryPerformedExternalRedispatch,
) -> Result<WorthQueryRecoverySafeRetryAdmission, WorthQueryRecoveryHandleDenial> {
    safe_retry_recovery_handle(handle, authority, redispatch)
}

fn main() {}
