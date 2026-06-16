use super::{
    admit_scoped_query_basis_context, build_scoped_query_basis_result_bundle,
    execute_and_build_scoped_query_basis_result_bundle, execute_scoped_query_basis_context,
    QueryBasisContextRequest, QueryContextAdmissionFailureClass, QueryContextBindingSource,
    QueryContextExecutionFamily, ScopedQueryBasisContext, ScopedQueryContextAdmissionError,
};
use crate::facade::{
    admit_historical_evaluation_path, admit_preview_workflow_foundation,
    bind_preflight_to_preview_session, materialization_metadata_from_resolved,
    resolve_historical_materialization_path, HistoricalCapabilityDescriptor,
    HistoricalEvaluationRequest, HistoricalMaterializationDescriptor,
    HistoricalPathReuseDescriptor, PreviewEvaluationClass, PreviewSessionQueryContext,
};
use crate::harness::fixtures::{execution_preflights, preview_bridge::active_preview_artifacts};
use crate::query_basis_lifecycle::{BasisCapabilityAdmission, BasisIntentDenialKind};

#[test]
fn scoped_query_context_current_head_uses_observation_basis() {
    let preflight = execution_preflights::direct_runtime_preflight();

    let scoped = admit_scoped_query_basis_context(
        QueryBasisContextRequest::current_branch_head(),
        QueryContextBindingSource::RuntimeCurrent(&preflight),
    )
    .expect("current-head scoped query context should admit");
    let execution = execute_scoped_query_basis_context(&scoped)
        .expect("scoped current-head execution should succeed");
    let bundle = build_scoped_query_basis_result_bundle(&scoped, execution.clone())
        .expect("scoped current-head bundle should build");

    match scoped {
        ScopedQueryBasisContext::Observation(scoped) => match scoped.scoped_basis().admission() {
            BasisCapabilityAdmission::Admitted(capability) => {
                assert_eq!(capability.operation_lane().as_str(), "observation");
            }
            other => panic!("unexpected observation admission posture: {other:?}"),
        },
        other => panic!("unexpected scoped query context variant: {other:?}"),
    }
    assert_eq!(
        execution.family(),
        &QueryContextExecutionFamily::RuntimeCurrent
    );
    assert_eq!(bundle.context().family().as_str(), "current_branch_head");
    assert_eq!(
        bundle.execution().family(),
        &QueryContextExecutionFamily::RuntimeCurrent
    );
}

#[test]
fn scoped_query_context_historical_snapshot_uses_materialization_basis() {
    let query_preflight = execution_preflights::direct_runtime_preflight();
    let request = HistoricalEvaluationRequest::retained_snapshot(
        "history:scoped-snapshot",
        1,
        1,
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let capability = HistoricalCapabilityDescriptor::retained_snapshot(
        "history:scoped-snapshot",
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let admission = admit_historical_evaluation_path(request, capability)
        .expect("retained history should admit");
    let resolved = resolve_historical_materialization_path(
        admission.clone(),
        HistoricalMaterializationDescriptor::retained_snapshot("history:scoped-snapshot"),
    )
    .expect("retained history should resolve");
    let metadata = materialization_metadata_from_resolved(resolved);

    let scoped = admit_scoped_query_basis_context(
        QueryBasisContextRequest::historical_snapshot("history:scoped-snapshot"),
        QueryContextBindingSource::Historical {
            query_preflight: &query_preflight,
            admission: &admission,
            metadata: &metadata,
        },
    )
    .expect("historical scoped query context should admit");
    let execution = execute_scoped_query_basis_context(&scoped)
        .expect("scoped historical execution should succeed");

    match scoped {
        ScopedQueryBasisContext::Materialization(scoped) => {
            assert_eq!(
                scoped.scoped_basis().capability().operation_lane().as_str(),
                "materialization"
            );
        }
        other => panic!("unexpected scoped query context variant: {other:?}"),
    }
    assert_eq!(
        execution.family(),
        &QueryContextExecutionFamily::HistoricalMaterialized
    );
}

#[test]
fn scoped_query_context_preview_derived_historical_uses_observation_basis() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let (_runtime, active, execution_record) =
        active_preview_artifacts("scoped-query-context-preview");
    let binding = bind_preflight_to_preview_session(
        preflight,
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::promotion_eligible(),
        ),
    )
    .expect("preview binding should succeed");
    let foundation =
        admit_preview_workflow_foundation(&binding).expect("preview foundation should admit");
    let preview_session_identity = foundation.preview_session_identity().bridge_admission_evidence();

    let scoped = admit_scoped_query_basis_context(
        QueryBasisContextRequest::preview_derived_historical(preview_session_identity.terminal_projection_for_reporting().to_string()),
        QueryContextBindingSource::PreviewDerivedHistorical(&foundation),
    )
    .expect("preview-derived scoped query context should admit");

    match scoped {
        ScopedQueryBasisContext::Observation(scoped) => match scoped.scoped_basis().admission() {
            BasisCapabilityAdmission::Advisory(capability) => {
                assert_eq!(
                    scoped.context().family().as_str(),
                    "preview_derived_historical"
                );
                assert_eq!(capability.operation_lane().as_str(), "observation");
            }
            other => panic!("unexpected preview-derived observation posture: {other:?}"),
        },
        other => panic!("unexpected scoped query context variant: {other:?}"),
    }
}

#[test]
fn scoped_query_context_preserves_query_context_source_pairing_denials() {
    let preflight = execution_preflights::direct_runtime_preflight();

    let error = admit_scoped_query_basis_context(
        QueryBasisContextRequest::current_branch_head(),
        QueryContextBindingSource::RuntimeBranch(&preflight),
    )
    .expect_err("source pairing mismatch should still deny");

    match error {
        ScopedQueryContextAdmissionError::Context(error) => {
            assert_eq!(
                error.failure_class(),
                &QueryContextAdmissionFailureClass::InvalidBasisPairing
            );
        }
        other => panic!("unexpected scoped query context error: {other:?}"),
    }
}

#[test]
fn scoped_query_context_diff_comparison_denies_typed_before_legacy_binding() {
    let preflight = execution_preflights::direct_runtime_preflight();

    let error = admit_scoped_query_basis_context(
        QueryBasisContextRequest::diff_comparison_for_internal_denial_testing("diff"),
        QueryContextBindingSource::RuntimeCurrent(&preflight),
    )
    .expect_err("diff comparison should deny before legacy binding");

    match error {
        ScopedQueryContextAdmissionError::Intent(denial) => match denial.kind() {
            BasisIntentDenialKind::UnsupportedCompatibilityFamily { family, owner } => {
                assert_eq!(family, &"diff_comparison");
                assert_eq!(owner, &"forge_query::query_context");
            }
            other => panic!("unexpected denial kind: {other:?}"),
        },
        other => panic!("unexpected scoped query context error: {other:?}"),
    }
}

#[test]
fn execute_and_build_scoped_query_basis_result_bundle_keeps_execution_boundary_explicit() {
    let preflight = execution_preflights::direct_runtime_preflight();

    let scoped = admit_scoped_query_basis_context(
        QueryBasisContextRequest::current_branch_head(),
        QueryContextBindingSource::RuntimeCurrent(&preflight),
    )
    .expect("current-head scoped query context should admit");

    let bundle = execute_and_build_scoped_query_basis_result_bundle(&scoped)
        .expect("explicit execute-and-build helper should succeed");

    assert_eq!(bundle.context().family().as_str(), "current_branch_head");
    assert_eq!(
        bundle.execution().family(),
        &QueryContextExecutionFamily::RuntimeCurrent
    );
}
