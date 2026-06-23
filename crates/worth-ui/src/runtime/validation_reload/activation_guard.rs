use crate::runtime::{
    WorthUiRuntimeHost, WorthUiValidationReloadEvidence, WorthUiValidationReloadStage,
};

pub(super) fn reject_stale_prepared_reload_activation(
    runtime: &WorthUiRuntimeHost,
    evidence: &WorthUiValidationReloadEvidence,
) -> Result<(), WorthUiValidationReloadStage> {
    let active = runtime.inspect_active();
    if active.artifact_digest() != evidence.active_artifact_digest_before()
        || active.active_plan_digest() != evidence.active_plan_digest_before()
        || active_authoring_snapshot_digest(runtime)
            != evidence.active_authoring_snapshot_digest_before()
    {
        Err(WorthUiValidationReloadStage::PlanSwap)
    } else {
        Ok(())
    }
}

fn active_authoring_snapshot_digest(runtime: &WorthUiRuntimeHost) -> Option<u64> {
    runtime
        .active_authoring_snapshot()
        .map(|snapshot| snapshot.digest().as_u64())
}
