use worth_query_host::facade::primary_graph::{
    expire_recovery_handle, WorthQueryRecoveryCurrentDecision, WorthQueryRecoveryHandle,
};

fn current_evidence_cannot_expire(
    handle: WorthQueryRecoveryHandle,
    current: &WorthQueryRecoveryCurrentDecision,
) {
    let _ = expire_recovery_handle(handle, current);
}

fn main() {}
