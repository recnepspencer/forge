use super::{
    admit_preview_live_session_plan, admit_scoped_preview_live_session_plan,
    admit_scoped_preview_session_plan_binding,
    admit_scoped_preview_session_plan_binding_from_preview_binding,
    bind_preflight_to_preview_session, execute_preview_live_session_plan,
    execute_scoped_preview_live_session_plan, scoped_observation_basis_for_preview_binding,
    PreviewEvaluationClass, PreviewLiveFailureClass, PreviewSessionQueryContext,
};
use crate::basis_lifecycle::{basis_lifecycle, BasisFamily};
use crate::harness::fixtures::{execution_preflights, preview_bridge::active_preview_artifacts};
use crate::live::promote_preflight_bundle_to_live;

#[test]
fn scoped_preview_live_admission_preserves_existing_live_report_shape() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let live_plan =
        promote_preflight_bundle_to_live(&preflight).expect("live promotion should succeed");
    let (_runtime, active, execution_record) = active_preview_artifacts("scoped-preview-live");
    let preview_binding = bind_preflight_to_preview_session(
        preflight.clone(),
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::read_only(),
        ),
    )
    .expect("preview binding should succeed");
    let scoped_basis = basis_lifecycle()
        .current_head()
        .observe()
        .expect("current-head observation basis should scope");
    let scoped_binding =
        admit_scoped_preview_session_plan_binding(scoped_basis, preview_binding.clone())
            .expect("scoped preview binding should admit");

    let scoped_preview_live =
        admit_scoped_preview_live_session_plan(scoped_binding, live_plan.clone())
            .expect("scoped preview-live should admit");
    let scoped_execution = execute_scoped_preview_live_session_plan(&scoped_preview_live)
        .expect("scoped preview-live execution should succeed");

    let unscoped_preview_live = admit_preview_live_session_plan(preview_binding, live_plan)
        .expect("legacy preview-live should admit");
    let unscoped_execution = execute_preview_live_session_plan(&unscoped_preview_live)
        .expect("legacy preview-live execution should succeed");

    assert_eq!(scoped_preview_live.report(), unscoped_preview_live.report());
    assert_eq!(scoped_execution.counters(), unscoped_execution.counters());
}

#[test]
fn scoped_preview_live_admission_denies_mismatched_scoped_basis_semantics() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let (_runtime, active, execution_record) =
        active_preview_artifacts("scoped-preview-live-mismatch");
    let preview_binding = bind_preflight_to_preview_session(
        preflight,
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::read_only(),
        ),
    )
    .expect("preview binding should succeed");
    let scoped_basis = basis_lifecycle()
        .runtime_snapshot("snapshot:other", "binding:other")
        .observe()
        .expect("runtime-snapshot observation basis should scope");

    let error = admit_scoped_preview_session_plan_binding(scoped_basis, preview_binding)
        .expect_err("mismatched scoped basis should deny");

    assert_eq!(
        error.failure_class(),
        &PreviewLiveFailureClass::PreviewLiveScopedBasisMismatch
    );
    assert_eq!(
        error.counters().preview_live_broad_fallback_denial_count(),
        1
    );
}

#[test]
fn scoped_preview_live_admission_preserves_underlying_preview_live_denials() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let alternate_preflight = execution_preflights::alternate_basis_runtime_preflight();
    let live_plan = promote_preflight_bundle_to_live(&alternate_preflight)
        .expect("alternate live promotion should succeed");
    let (_runtime, active, execution_record) =
        active_preview_artifacts("scoped-preview-live-underlying-denial");
    let preview_binding = bind_preflight_to_preview_session(
        preflight,
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::read_only(),
        ),
    )
    .expect("preview binding should succeed");
    let scoped_basis = basis_lifecycle()
        .current_head()
        .observe()
        .expect("current-head observation basis should scope");
    let scoped_binding = admit_scoped_preview_session_plan_binding(scoped_basis, preview_binding)
        .expect("scoped preview binding should admit");

    let error = admit_scoped_preview_live_session_plan(scoped_binding, live_plan)
        .expect_err("underlying preview-live basis mismatch should still deny");

    assert_eq!(
        error.failure_class(),
        &PreviewLiveFailureClass::PreviewLiveBasisMismatch
    );
}

#[test]
fn scoped_preview_binding_adapter_derives_current_head_observation_semantics() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let (_runtime, active, execution_record) =
        active_preview_artifacts("scoped-preview-binding-adapter");
    let preview_binding = bind_preflight_to_preview_session(
        preflight,
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::read_only(),
        ),
    )
    .expect("preview binding should succeed");

    let scoped_basis = scoped_observation_basis_for_preview_binding(&preview_binding)
        .expect("preview binding should derive scoped observation basis");
    let admitted = admit_scoped_preview_session_plan_binding_from_preview_binding(preview_binding)
        .expect("preview binding should admit through scoped adapter");

    assert_eq!(scoped_basis.family(), BasisFamily::CurrentHead);
    assert_eq!(admitted.scoped_basis(), &scoped_basis);
}
