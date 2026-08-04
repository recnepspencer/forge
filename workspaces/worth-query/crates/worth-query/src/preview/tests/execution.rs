use crate::harness::fixtures::execution_preflights;
use crate::harness::fixtures::preview_bridge::active_preview_artifacts;
use crate::preview::{
    admit_promotion_eligible_preview_session_plan_binding,
    admit_read_only_preview_session_plan_binding, bind_preflight_to_preview_session,
    execute_preview_session_plan, execute_promotion_eligible_preview_session_plan,
    execute_read_only_preview_session_plan, PreviewBindingFailureClass, PreviewEvaluationClass,
    PreviewExecutionFailureClass, PreviewSessionQueryContext,
};

#[test]
fn preview_execution_envelope_preserves_zero_rediscovery_invariants() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let (_runtime, active, execution_record) =
        active_preview_artifacts("preview-execution-envelope");
    let binding = bind_preflight_to_preview_session(
        preflight,
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::read_only(),
        ),
    )
    .expect("preview binding should succeed");

    let execution =
        execute_preview_session_plan(&binding).expect("preview execution should succeed");

    assert_eq!(
        execution
            .counters()
            .binding_counters()
            .preview_lifecycle_rediscovery_count(),
        0
    );
    assert_eq!(
        execution
            .counters()
            .binding_counters()
            .preview_executor_rediscovery_count(),
        0
    );
    assert_eq!(
        execution
            .counters()
            .execution_counters()
            .executor_semantic_rediscovery_count(),
        0
    );
    assert_eq!(execution.counters().preview_execution_envelope_count(), 1);
    assert_eq!(execution.counters().preview_execution_count(), 1);
    assert_eq!(execution.counters().preview_read_only_execution_count(), 1);
    assert_eq!(execution.counters().preview_promotable_execution_count(), 0);
    assert_eq!(
        execution.counters().preview_comparison_shape_check_width,
        execution.comparison_eligibility().shape_check_width()
    );
    assert_eq!(
        execution
            .counters()
            .preview_work_avoided_by_explicit_basis_count(),
        1
    );
    assert_eq!(
        execution
            .counters()
            .preview_workflow_foundation_admission_count(),
        1
    );
    assert_eq!(
        execution
            .counters()
            .preview_workflow_foundation_denial_count(),
        0
    );
    assert_eq!(
        execution.binding.basis().binding_tuple().digest(),
        binding.basis().binding_tuple().digest()
    );
    assert_eq!(
        execution.execution.report().result_digest(),
        &execution.report().result_digest
    );
    assert_eq!(
        execution.report().binding_digest,
        binding.basis().binding_tuple().digest()
    );
    assert_eq!(
        execution.report().basis_digest(),
        execution.execution.report().basis_digest().as_str()
    );
    assert_eq!(
        execution.report().preview_session_identity(),
        binding.basis().binding_tuple().preview_session_identity()
    );
    assert_eq!(
        execution.report().lifecycle_state_kind(),
        binding.basis().binding_tuple().lifecycle_state_kind()
    );
    assert_eq!(
        execution.report().execution_record_identity(),
        binding
            .basis()
            .binding_tuple()
            .execution_record_identity()
            .expect("binding should carry execution record identity")
    );
    assert_eq!(
        &execution.report().query_digest,
        binding.preflight().plan().query().validated_query_digest()
    );
    assert!(!execution.report().preview_execution_digest.is_empty());
    assert_eq!(
        execution.report().comparison_eligibility_digest(),
        execution.comparison_eligibility().digest()
    );
    assert_eq!(
        execution.report().workflow_foundation_digest(),
        execution.workflow_foundation().artifact_for_reporting()
    );
}

#[test]
fn read_only_preview_execution_entrypoint_requires_read_only_binding() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let (_runtime, active, execution_record) =
        active_preview_artifacts("preview-read-only-entrypoint");
    let read_only_binding = bind_preflight_to_preview_session(
        preflight.clone(),
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::read_only(),
        ),
    )
    .expect("read-only binding should succeed");
    let promotion_binding = bind_preflight_to_preview_session(
        preflight,
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::promotion_eligible(),
        ),
    )
    .expect("promotion-eligible binding should succeed");

    let read_only_binding = admit_read_only_preview_session_plan_binding(read_only_binding)
        .expect("read-only binding should admit to the read-only execution class");
    let read_only_execution = execute_read_only_preview_session_plan(&read_only_binding)
        .expect("read-only execution entrypoint should accept read-only binding");
    assert_eq!(
        read_only_execution
            .as_preview_execution()
            .binding
            .basis()
            .binding_tuple()
            .evaluation_class(),
        &PreviewEvaluationClass::read_only()
    );

    let mismatch = admit_read_only_preview_session_plan_binding(promotion_binding)
        .expect_err("read-only witness admission should reject promotion-eligible binding");
    assert_eq!(
        mismatch.failure_class(),
        PreviewExecutionFailureClass::InvalidExecutionClass
    );
}

#[test]
fn promotion_eligible_preview_execution_entrypoint_requires_promotion_binding() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let (_runtime, active, execution_record) =
        active_preview_artifacts("preview-promotion-entrypoint");
    let read_only_binding = bind_preflight_to_preview_session(
        preflight.clone(),
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::read_only(),
        ),
    )
    .expect("read-only binding should succeed");
    let promotion_binding = bind_preflight_to_preview_session(
        preflight,
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::promotion_eligible(),
        ),
    )
    .expect("promotion-eligible binding should succeed");

    let promotion_binding =
        admit_promotion_eligible_preview_session_plan_binding(promotion_binding)
            .expect("promotion binding should admit to the promotion execution class");
    let promotion_execution = execute_promotion_eligible_preview_session_plan(&promotion_binding)
        .expect("promotion entrypoint should accept promotion binding");
    assert_eq!(
        promotion_execution
            .as_preview_execution()
            .binding
            .basis()
            .binding_tuple()
            .evaluation_class(),
        &PreviewEvaluationClass::promotion_eligible()
    );
    assert_eq!(
        promotion_execution
            .as_preview_execution()
            .counters()
            .preview_promotable_execution_count(),
        1
    );
    assert_eq!(
        promotion_execution
            .as_preview_execution()
            .counters()
            .preview_read_only_execution_count(),
        0
    );

    let mismatch = admit_promotion_eligible_preview_session_plan_binding(read_only_binding)
        .expect_err("promotion witness admission should reject read-only binding");
    assert_eq!(
        mismatch.failure_class(),
        PreviewExecutionFailureClass::InvalidExecutionClass
    );
}

#[test]
fn preview_execution_failure_classifies_underlying_execution_errors() {
    let preflight = execution_preflights::cdc_collection_preflight();
    let (_runtime, active, execution_record) =
        active_preview_artifacts("preview-execution-underlying-failure");
    let error = bind_preflight_to_preview_session(
        preflight,
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::read_only(),
        ),
    )
    .expect_err("unsupported preview families should reject before execution");

    assert_eq!(
        error.failure_class(),
        &PreviewBindingFailureClass::UnsupportedPreviewQueryFamily
    );

    let supported_preflight = execution_preflights::direct_runtime_preflight();
    let binding = bind_preflight_to_preview_session(
        supported_preflight,
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::read_only(),
        ),
    )
    .expect("supported preview binding should succeed");
    let execution =
        execute_preview_session_plan(&binding).expect("supported preview execution should work");

    assert_eq!(
        execution
            .check_invariants()
            .map(|_| ())
            .map_err(|err| err.failure_class()),
        Ok(())
    );
    assert_ne!(
        PreviewExecutionFailureClass::UnderlyingExecutionFailure,
        PreviewExecutionFailureClass::InternalInvariantBreak
    );
}
