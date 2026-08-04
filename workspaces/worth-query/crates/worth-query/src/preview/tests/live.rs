use crate::harness::fixtures::execution_preflights;
use crate::harness::fixtures::preview_bridge::{
    active_preview_artifacts, discarded_preview_artifacts,
};
use crate::preview::{
    admit_preview_live_session_plan, assess_preview_live_drift, bind_preflight_to_preview_session,
    execute_preview_live_session_plan, PreviewEvaluationClass, PreviewLiveDriftOutcome,
    PreviewLiveFailureClass, PreviewSessionQueryContext,
};

#[test]
fn preview_live_admission_reuses_matching_live_plan_proof() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let live_plan =
        crate::live::promote_preflight_bundle_to_live(&preflight).expect("live promotion");
    let (_runtime, active, execution_record) = active_preview_artifacts("preview-live-admission");
    let preview_binding = bind_preflight_to_preview_session(
        preflight,
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::read_only(),
        ),
    )
    .expect("preview binding should succeed");

    let preview_live = admit_preview_live_session_plan(preview_binding, live_plan)
        .expect("preview-live should admit");
    let execution = execute_preview_live_session_plan(&preview_live)
        .expect("preview-live execution should succeed");

    assert_eq!(
        preview_live
            .report()
            .counters()
            .preview_live_admission_count(),
        1
    );
    assert_eq!(execution.counters().preview_live_execution_count(), 1);
    assert_eq!(
        preview_live.live_plan().descriptor().query_digest(),
        preview_live
            .scoped_binding()
            .preview_binding()
            .preflight()
            .plan()
            .query()
            .validated_query_digest()
    );
    assert_eq!(
        preview_live.report().preview_binding_digest(),
        preview_live
            .scoped_binding()
            .preview_binding()
            .basis()
            .binding_tuple()
            .digest()
    );
}

#[test]
fn preview_live_admission_rejects_mismatched_live_plan() {
    let preview_preflight = execution_preflights::direct_runtime_preflight();
    let mismatched_live_preflight = execution_preflights::ordered_collection_preflight();
    let live_plan = crate::live::promote_preflight_bundle_to_live(&mismatched_live_preflight)
        .expect("mismatched live promotion");
    let (_runtime, active, execution_record) = active_preview_artifacts("preview-live-mismatch");
    let preview_binding = bind_preflight_to_preview_session(
        preview_preflight,
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::read_only(),
        ),
    )
    .expect("preview binding should succeed");

    let error = admit_preview_live_session_plan(preview_binding, live_plan)
        .expect_err("preview-live should reject mismatched live plan proofs");

    assert_eq!(
        error.failure_class(),
        &PreviewLiveFailureClass::PreviewLiveQueryDigestMismatch
    );
    assert_eq!(
        error.counters().preview_live_broad_fallback_denial_count(),
        1
    );
}

#[test]
fn preview_live_drift_denies_when_lifecycle_leaves_active() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let live_plan =
        crate::live::promote_preflight_bundle_to_live(&preflight).expect("live promotion");
    let (_runtime, active, execution_record) =
        active_preview_artifacts("preview-live-drift-denied");
    let preview_binding = bind_preflight_to_preview_session(
        preflight,
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::read_only(),
        ),
    )
    .expect("preview binding should succeed");
    let preview_live = admit_preview_live_session_plan(preview_binding, live_plan)
        .expect("preview-live should admit");
    let (_discarded_runtime, discarded, _discard_record) =
        discarded_preview_artifacts("preview-live-drift-discarded");

    let outcome = assess_preview_live_drift(
        &preview_live,
        PreviewSessionQueryContext::discarded(&discarded, PreviewEvaluationClass::read_only()),
    );

    match outcome {
        PreviewLiveDriftOutcome::DriftDenied(denied) => {
            assert_eq!(
                denied.error().failure_class(),
                &PreviewLiveFailureClass::PreviewLiveLifecycleDrifted
            );
            assert_eq!(
                denied.error().counters().preview_live_drift_denial_count(),
                1
            );
        }
        other => panic!("expected drift denial, got {other:?}"),
    }
}

#[test]
fn preview_live_drift_can_offer_explicit_rebind() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let live_plan =
        crate::live::promote_preflight_bundle_to_live(&preflight).expect("live promotion");
    let (_runtime, active, execution_record) = active_preview_artifacts("preview-live-rebind-old");
    let preview_binding = bind_preflight_to_preview_session(
        preflight,
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::read_only(),
        ),
    )
    .expect("preview binding should succeed");
    let preview_live = admit_preview_live_session_plan(preview_binding, live_plan)
        .expect("preview-live should admit");
    let (_next_runtime, next_active, next_execution_record) =
        active_preview_artifacts("preview-live-rebind-new");

    let outcome = assess_preview_live_drift(
        &preview_live,
        PreviewSessionQueryContext::active(
            &next_active,
            &next_execution_record,
            PreviewEvaluationClass::read_only(),
        ),
    );

    match outcome {
        PreviewLiveDriftOutcome::ExplicitRebindAvailable(rebind) => {
            assert_eq!(rebind.counters().preview_live_rebind_available_count(), 1);
            assert_ne!(
                rebind.prior_preview_live_digest(),
                rebind.rebound_preview_live().scoped_live_digest()
            );
        }
        other => panic!("expected explicit rebind, got {other:?}"),
    }
}

#[test]
fn preview_live_drift_maintained_retains_lifecycle_check_counters() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let live_plan =
        crate::live::promote_preflight_bundle_to_live(&preflight).expect("live promotion");
    let (_runtime, active, execution_record) = active_preview_artifacts("preview-live-maintained");
    let preview_binding = bind_preflight_to_preview_session(
        preflight,
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::read_only(),
        ),
    )
    .expect("preview binding should succeed");
    let preview_live = admit_preview_live_session_plan(preview_binding, live_plan)
        .expect("preview-live should admit");

    let outcome = assess_preview_live_drift(
        &preview_live,
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::read_only(),
        ),
    );

    match outcome {
        PreviewLiveDriftOutcome::Maintained(maintained) => {
            assert_eq!(
                maintained.counters().preview_live_lifecycle_check_count(),
                1
            );
            assert_eq!(maintained.counters().preview_live_drift_denial_count(), 0);
            assert_eq!(
                maintained.maintained_preview_live().report().digest(),
                preview_live.report().digest()
            );
        }
        other => panic!("expected maintained preview-live, got {other:?}"),
    }
}

#[test]
fn preview_live_drift_invalid_rebind_basis_stays_typed() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let live_plan =
        crate::live::promote_preflight_bundle_to_live(&preflight).expect("live promotion");
    let (_runtime, active, execution_record) =
        active_preview_artifacts("preview-live-invalid-rebind-basis");
    let preview_binding = bind_preflight_to_preview_session(
        preflight,
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::read_only(),
        ),
    )
    .expect("preview binding should succeed");
    let preview_live = admit_preview_live_session_plan(preview_binding, live_plan)
        .expect("preview-live should admit");

    let outcome = assess_preview_live_drift(
        &preview_live,
        PreviewSessionQueryContext::active_without_execution_record(
            &active,
            PreviewEvaluationClass::read_only(),
        ),
    );

    match outcome {
        PreviewLiveDriftOutcome::DriftDenied(denied) => {
            assert_eq!(
                denied.error().failure_class(),
                &PreviewLiveFailureClass::PreviewLiveRebindBindingRejected
            );
            assert_eq!(
                denied.error().counters().preview_live_drift_denial_count(),
                1
            );
            assert_eq!(
                denied
                    .error()
                    .counters()
                    .preview_live_broad_fallback_denial_count(),
                0,
                "invalid rebind basis must not masquerade as broad fallback"
            );
        }
        other => panic!("expected typed drift denial, got {other:?}"),
    }
}

#[test]
fn preview_live_drift_foreign_execution_record_is_broad_fallback_denial() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let live_plan =
        crate::live::promote_preflight_bundle_to_live(&preflight).expect("live promotion");
    let (_runtime, active, execution_record) =
        active_preview_artifacts("preview-live-broad-fallback");
    let preview_binding = bind_preflight_to_preview_session(
        preflight,
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::read_only(),
        ),
    )
    .expect("preview binding should succeed");
    let preview_live = admit_preview_live_session_plan(preview_binding, live_plan)
        .expect("preview-live should admit");
    let (_foreign_runtime, _foreign_active, foreign_execution_record) =
        active_preview_artifacts("preview-live-broad-fallback-foreign");

    let outcome = assess_preview_live_drift(
        &preview_live,
        PreviewSessionQueryContext::active(
            &active,
            &foreign_execution_record,
            PreviewEvaluationClass::read_only(),
        ),
    );

    match outcome {
        PreviewLiveDriftOutcome::DriftDenied(denied) => {
            assert_eq!(
                denied.error().failure_class(),
                &PreviewLiveFailureClass::PreviewLiveBroadFallbackForbidden
            );
            assert_eq!(
                denied
                    .error()
                    .counters()
                    .preview_live_broad_fallback_denial_count(),
                1
            );
        }
        other => panic!("expected broad fallback denial, got {other:?}"),
    }
}

#[test]
fn preview_live_execution_emits_explicit_execution_counter() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let live_plan =
        crate::live::promote_preflight_bundle_to_live(&preflight).expect("live promotion");
    let (_runtime, active, execution_record) = active_preview_artifacts("preview-live-execution");
    let preview_binding = bind_preflight_to_preview_session(
        preflight,
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::read_only(),
        ),
    )
    .expect("preview binding should succeed");
    let preview_live = admit_preview_live_session_plan(preview_binding, live_plan)
        .expect("preview-live should admit");

    let execution = execute_preview_live_session_plan(&preview_live)
        .expect("preview-live execution should succeed");

    assert_eq!(execution.counters().preview_live_admission_count(), 1);
    assert_eq!(execution.counters().preview_live_execution_count(), 1);
    assert_eq!(execution.counters().preview_live_lifecycle_check_count(), 0);
    assert_eq!(
        execution.preview_live().report().digest(),
        preview_live.report().digest()
    );
}
