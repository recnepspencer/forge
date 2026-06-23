use crate::runtime::validation_reload::driver::WorthUiValidationPreparedReload;
use crate::runtime::validation_reload::evidence::WorthUiValidationReloadEvidenceBuilder;
use crate::runtime::{
    WorthUiDurableStateFamily, WorthUiDurableStateInventoryBuilder, WorthUiRuntimeHost,
    WorthUiValidationReloadEvidence, WorthUiValidationReloadStage, WorthUiValidationReloadStatus,
};

pub(super) fn active_authoring_snapshot_digest(runtime: &WorthUiRuntimeHost) -> Option<u64> {
    runtime
        .active_authoring_snapshot()
        .map(|snapshot| snapshot.digest().as_u64())
}

pub(super) fn platform_inventory(
    runtime: &WorthUiRuntimeHost,
) -> WorthUiDurableStateInventoryBuilder {
    runtime
        .durable_state_inventory()
        .register_platform_family(WorthUiDurableStateFamily::focus_chain())
        .register_platform_family(WorthUiDurableStateFamily::scroll_anchor())
        .register_platform_family(WorthUiDurableStateFamily::selection_range())
        .register_platform_family(WorthUiDurableStateFamily::text_edit_buffer())
        .register_platform_family(WorthUiDurableStateFamily::splitter_position())
        .register_platform_family(WorthUiDurableStateFamily::tab_state())
        .register_platform_family(WorthUiDurableStateFamily::panel_visibility())
}

pub(super) fn denied_reload(
    evidence: WorthUiValidationReloadEvidenceBuilder,
    runtime: &WorthUiRuntimeHost,
    stage: WorthUiValidationReloadStage,
) -> WorthUiValidationPreparedReload {
    WorthUiValidationPreparedReload {
        runtime_instance_id: runtime.instance_id(),
        evidence: finish_evidence(
            evidence,
            runtime,
            WorthUiValidationReloadStatus::Denied(stage),
        ),
        changed_fact_mapping_receipt: None,
        ready: None,
        candidate_plan: None,
        candidate_authoring_snapshot: None,
    }
}

pub(super) fn denied_reload_with_detail(
    evidence: WorthUiValidationReloadEvidenceBuilder,
    runtime: &WorthUiRuntimeHost,
    stage: WorthUiValidationReloadStage,
    detail: impl Into<String>,
) -> WorthUiValidationPreparedReload {
    let after = runtime.inspect_active();
    WorthUiValidationPreparedReload {
        runtime_instance_id: runtime.instance_id(),
        evidence: evidence.finish_denied(
            stage,
            detail,
            after.artifact_digest(),
            after.active_plan_digest(),
        ),
        changed_fact_mapping_receipt: None,
        ready: None,
        candidate_plan: None,
        candidate_authoring_snapshot: None,
    }
}

pub(super) fn finish_evidence(
    evidence: WorthUiValidationReloadEvidenceBuilder,
    runtime: &WorthUiRuntimeHost,
    status: WorthUiValidationReloadStatus,
) -> WorthUiValidationReloadEvidence {
    let after = runtime.inspect_active();
    evidence.finish(status, after.artifact_digest(), after.active_plan_digest())
}
