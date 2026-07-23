use super::{
    admit_and_scope_legacy_query_basis_context_for_test, attach_diff_query_metadata,
    attach_query_basis_metadata, bind_diff_query_context, bind_legacy_query_basis_context,
    execute_query_basis_context, shape_query_diff_change_set, ComparisonBasisFamily,
    HistoricalAdmissionClass, HistoricalMaterializationCostClass, QueryBasisContextRequest,
    QueryContextAdmissionFailureClass, QueryContextBindingSource, QueryContextBudgetClass,
    QueryContextCostClass, QueryContextExecutionFamily, QueryContextFamily,
    QueryContextPredictionDriftOutcome,
};
use crate::facade::foundation::{
    admit_historical_evaluation_path, materialization_metadata_from_resolved,
    resolve_historical_materialization_path, HistoricalCapabilityDescriptor,
    HistoricalEvaluationRequest, HistoricalMaterializationDescriptor,
    HistoricalPathReuseDescriptor,
};
use crate::facade::policy::{
    admit_preview_workflow_foundation, bind_preflight_to_preview_session, PreviewEvaluationClass,
    PreviewSessionQueryContext,
};
use crate::harness::fixtures::{execution_preflights, preview_bridge::active_preview_artifacts};
use crate::memory_workspace::WorthQuerySnapshotIdentity;
use worth_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts;

#[test]
fn current_branch_head_context_binding_preserves_runtime_digests() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let binding = bind_legacy_query_basis_context(
        QueryBasisContextRequest::current_branch_head(),
        QueryContextBindingSource::RuntimeCurrent(&preflight),
    )
    .expect("current runtime basis should bind");
    let admitted = admit_and_scope_legacy_query_basis_context_for_test(binding)
        .expect("current runtime basis should admit");

    assert_eq!(admitted.family(), &QueryContextFamily::CurrentBranchHead);
    assert_eq!(
        admitted.query_digest(),
        preflight.plan().query().validated_query_digest().as_str()
    );
    assert_eq!(
        admitted.basis_digest(),
        preflight.basis().proof().digest().as_str()
    );
    assert_eq!(
        admitted.cost_class(),
        &QueryContextCostClass::CurrentHeadNarrow
    );
    assert_eq!(
        admitted.budget_class(),
        &QueryContextBudgetClass::NarrowSingleBasis
    );
    assert_eq!(
        admitted.prediction_drift_outcome(),
        Some(&QueryContextPredictionDriftOutcome::PendingExecution)
    );
    assert_eq!(admitted.counters().query_basis_binding_count(), 1);
    assert_eq!(admitted.counters().historical_basis_lookup_count(), 0);
    assert_eq!(admitted.counters().basis_binding_width(), 1);
    assert_eq!(admitted.counters().denial_width(), 0);
    assert_eq!(admitted.counters().basis_rediscovery_count(), 0);
}

#[test]
fn legacy_query_basis_execution_cannot_fabricate_count_aggregate_results() {
    let preflight = execution_preflights::aggregate_rollup_collection_preflight();
    let binding = bind_legacy_query_basis_context(
        QueryBasisContextRequest::current_branch_head(),
        QueryContextBindingSource::RuntimeCurrent(&preflight),
    )
    .expect("count preflight should remain bindable for inspection");
    let admitted = admit_and_scope_legacy_query_basis_context_for_test(binding)
        .expect("count context should remain inspectable");

    let error = execute_query_basis_context(&admitted)
        .expect_err("legacy query-basis execution must not produce a count result");

    assert_eq!(
        error.failure_class(),
        &QueryContextAdmissionFailureClass::UnsupportedHistoricalBasis
    );
}

#[test]
fn alternate_branch_head_context_binding_is_explicitly_distinct() {
    let preflight = execution_preflights::alternate_basis_runtime_preflight();
    let binding = bind_legacy_query_basis_context(
        QueryBasisContextRequest::branch_head("branch:snapshot-2"),
        QueryContextBindingSource::RuntimeBranch(&preflight),
    )
    .expect("alternate runtime basis should bind");
    let admitted = admit_and_scope_legacy_query_basis_context_for_test(binding)
        .expect("alternate runtime basis should admit");

    assert_eq!(admitted.family(), &QueryContextFamily::BranchHead);
    assert_eq!(
        admitted.basis_digest(),
        preflight.basis().proof().digest().as_str()
    );
    assert_eq!(
        admitted.cost_class(),
        &QueryContextCostClass::BranchHeadNarrow
    );
    assert_eq!(admitted.counters().basis_binding_width(), 1);
    assert_eq!(admitted.counters().basis_rediscovery_count(), 0);
}

#[test]
fn retained_historical_context_binding_reuses_admitted_history_artifacts() {
    let query_preflight = execution_preflights::direct_runtime_preflight();
    let request = HistoricalEvaluationRequest::retained_snapshot_for_test(
        "history:snapshot-1",
        1,
        1,
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let capability = HistoricalCapabilityDescriptor::retained_snapshot_for_test(
        "history:snapshot-1",
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let admission = admit_historical_evaluation_path(request, capability)
        .expect("retained history should admit");
    let resolved = resolve_historical_materialization_path(
        admission.clone(),
        HistoricalMaterializationDescriptor::retained_snapshot_for_test("history:snapshot-1"),
    )
    .expect("retained history should resolve");
    let metadata = materialization_metadata_from_resolved(resolved);

    let binding = bind_legacy_query_basis_context(
        QueryBasisContextRequest::historical_snapshot("history:snapshot-1"),
        QueryContextBindingSource::Historical {
            query_preflight: &query_preflight,
            admission: &admission,
            metadata: &metadata,
        },
    )
    .expect("historical context should bind");
    let admitted = admit_and_scope_legacy_query_basis_context_for_test(binding)
        .expect("historical context should admit");
    let execution = execute_query_basis_context(&admitted)
        .expect("historical query-context execution should succeed");
    let shaped = attach_query_basis_metadata(&admitted, &execution)
        .expect("historical metadata should shape");

    assert_eq!(admitted.family(), &QueryContextFamily::HistoricalSnapshot);
    assert_eq!(
        admitted.cost_class(),
        &QueryContextCostClass::HistoricalRetainedBounded
    );
    assert_eq!(
        admitted.budget_class(),
        &QueryContextBudgetClass::HistoricalBounded
    );
    assert_eq!(
        admitted.historical_admission_class(),
        Some(&HistoricalAdmissionClass::RuntimeRetained)
    );
    assert_eq!(
        admitted.historical_materialization_cost_class(),
        Some(&HistoricalMaterializationCostClass::RetainedBounded)
    );
    assert!(admitted.materialization_path_identity_source().is_some());
    assert!(shaped.materialization_path_identity().is_some());
    assert_eq!(
        shaped.historical_materialization_cost_class(),
        Some(&HistoricalMaterializationCostClass::RetainedBounded)
    );
    assert_eq!(admitted.counters().historical_basis_lookup_count(), 1);
    assert_eq!(admitted.counters().historical_lookup_width(), 1);
    assert_eq!(admitted.counters().historical_path_rediscovery_count(), 0);
}

#[test]
fn retained_historical_admission_owns_runtime_snapshot_matching() {
    let snapshot = WorthQuerySnapshotIdentity::from_relational_snapshot(
        RelationalBridgeSnapshotIdentityParts::new(41, 7),
    );
    let admitted_projection = snapshot
        .bridge_admission_evidence()
        .terminal_projection_for_reporting()
        .to_owned();
    let query_preflight = execution_preflights::direct_runtime_preflight();
    let request = HistoricalEvaluationRequest::retained_snapshot_for_test(
        admitted_projection.clone(),
        1,
        1,
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let capability = HistoricalCapabilityDescriptor::retained_snapshot_for_test(
        admitted_projection.clone(),
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let admission = admit_historical_evaluation_path(request, capability)
        .expect("retained snapshot path should admit");
    let resolved = resolve_historical_materialization_path(
        admission.clone(),
        HistoricalMaterializationDescriptor::retained_snapshot_for_test(
            admitted_projection.clone(),
        ),
    )
    .expect("retained snapshot path should resolve");
    let metadata = materialization_metadata_from_resolved(resolved);
    let historical = admit_and_scope_legacy_query_basis_context_for_test(
        bind_legacy_query_basis_context(
            QueryBasisContextRequest::historical_snapshot(admitted_projection.clone()),
            QueryContextBindingSource::Historical {
                query_preflight: &query_preflight,
                admission: &admission,
                metadata: &metadata,
            },
        )
        .expect("historical snapshot context should bind"),
    )
    .expect("historical snapshot context should admit");

    assert!(historical.admits_runtime_snapshot(&snapshot));

    let wrong_snapshot = WorthQuerySnapshotIdentity::from_relational_snapshot(
        RelationalBridgeSnapshotIdentityParts::new(41, 8),
    );
    assert!(!historical.admits_runtime_snapshot(&wrong_snapshot));

    let branch_projection = admit_and_scope_legacy_query_basis_context_for_test(
        bind_legacy_query_basis_context(
            QueryBasisContextRequest::branch_head(admitted_projection),
            QueryContextBindingSource::RuntimeBranch(&query_preflight),
        )
        .expect("runtime branch projection should bind"),
    )
    .expect("runtime branch projection should admit");
    assert!(!branch_projection.admits_runtime_snapshot(&snapshot));
}

#[test]
fn preview_derived_context_binding_preserves_preview_identity() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let (_runtime, active, execution_record) = active_preview_artifacts("query-context-preview");
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
    let preview_session_identity = foundation
        .preview_session_identity()
        .bridge_admission_evidence();

    let binding = bind_legacy_query_basis_context(
        QueryBasisContextRequest::preview_derived_historical(
            preview_session_identity
                .terminal_projection_for_reporting()
                .to_string(),
        ),
        QueryContextBindingSource::PreviewDerivedHistorical(&foundation),
    )
    .expect("preview-derived context should bind");
    let admitted = admit_and_scope_legacy_query_basis_context_for_test(binding)
        .expect("preview-derived context should admit");

    assert_eq!(
        admitted.family(),
        &QueryContextFamily::PreviewDerivedHistorical
    );
    assert_eq!(
        admitted.query_digest(),
        foundation.validated_query_digest().as_str()
    );
    assert_eq!(
        admitted.cost_class(),
        &QueryContextCostClass::PreviewDerivedHistoricalBounded
    );
    assert_eq!(
        admitted.budget_class(),
        &QueryContextBudgetClass::PreviewDerivedBounded
    );
    assert_eq!(
        admitted.preview_provenance_identity_source(),
        Some(preview_session_identity.terminal_projection_for_reporting())
    );
}

#[test]
fn preview_derived_execution_is_query_owned_and_provenance_explicit() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let (_runtime, active, execution_record) = active_preview_artifacts("query-context-preview");
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
    let preview_session_identity = foundation
        .preview_session_identity()
        .bridge_admission_evidence();

    let admitted = admit_and_scope_legacy_query_basis_context_for_test(
        bind_legacy_query_basis_context(
            QueryBasisContextRequest::preview_derived_historical(
                preview_session_identity
                    .terminal_projection_for_reporting()
                    .to_string(),
            ),
            QueryContextBindingSource::PreviewDerivedHistorical(&foundation),
        )
        .expect("preview-derived context should bind"),
    )
    .expect("preview-derived context should admit");
    let execution = execute_query_basis_context(&admitted)
        .expect("preview-derived query-context execution should succeed");
    let metadata = attach_query_basis_metadata(&admitted, &execution)
        .expect("preview-derived metadata should shape");

    assert_eq!(
        execution.family(),
        &QueryContextExecutionFamily::PreviewDerivedHistorical
    );
    assert_eq!(
        execution.preview_provenance_identity(),
        Some(preview_session_identity.terminal_projection_for_reporting())
    );
    assert_eq!(
        execution.prediction_drift_outcome(),
        &QueryContextPredictionDriftOutcome::WithinBudget
    );
    assert_eq!(
        metadata.preview_provenance_identity(),
        Some(preview_session_identity.terminal_projection_for_reporting())
    );
    assert_eq!(
        metadata.prediction_drift_outcome(),
        Some(&QueryContextPredictionDriftOutcome::WithinBudget)
    );
    assert_eq!(
        execution.counters().result_shape_width(),
        foundation.shape_check_width()
    );
    assert_eq!(execution.rows().len(), foundation.shape_check_width());
}

#[test]
fn diff_context_binding_preserves_both_basis_identities() {
    let left_preflight = execution_preflights::direct_runtime_preflight();
    let right_preflight = execution_preflights::alternate_basis_runtime_preflight();
    let left = admit_and_scope_legacy_query_basis_context_for_test(
        bind_legacy_query_basis_context(
            QueryBasisContextRequest::current_branch_head(),
            QueryContextBindingSource::RuntimeCurrent(&left_preflight),
        )
        .expect("left context should bind"),
    )
    .expect("left context should admit");
    let right = admit_and_scope_legacy_query_basis_context_for_test(
        bind_legacy_query_basis_context(
            QueryBasisContextRequest::branch_head("branch:snapshot-2"),
            QueryContextBindingSource::RuntimeBranch(&right_preflight),
        )
        .expect("right context should bind"),
    )
    .expect("right context should admit");
    let diff = bind_diff_query_context(&left, &right).expect("diff context should bind");
    let left_result =
        execute_query_basis_context(&left).expect("left context execution should succeed");
    let right_result =
        execute_query_basis_context(&right).expect("right context execution should succeed");
    let change_set = shape_query_diff_change_set(&diff, &left_result, &right_result)
        .expect("diff change-set should shape");
    let metadata = attach_diff_query_metadata(&diff, &left_result, &right_result, &change_set)
        .expect("diff metadata should shape");

    assert_eq!(diff.family(), &ComparisonBasisFamily::BranchToBranch);
    assert_eq!(
        diff.cost_class(),
        &QueryContextCostClass::DiffComparisonBounded
    );
    assert_eq!(
        diff.budget_class(),
        &QueryContextBudgetClass::ComparisonBounded
    );
    assert_eq!(
        diff.prediction_drift_outcome(),
        &QueryContextPredictionDriftOutcome::PendingComparison
    );
    assert_eq!(diff.prediction_report().comparison_binding_width(), 2);
    assert_eq!(diff.prediction_report().comparison_row_width(), 1);
    assert_eq!(
        change_set.comparison_basis_family(),
        &ComparisonBasisFamily::BranchToBranch
    );
    assert_eq!(
        change_set.prediction_drift_outcome(),
        &QueryContextPredictionDriftOutcome::WithinBudget
    );
    assert_eq!(change_set.left_basis_digest(), left.basis_digest());
    assert_eq!(change_set.right_basis_digest(), right.basis_digest());
    assert!(
        !change_set.rows().is_empty(),
        "diff change-set should remain query-shaped and typed"
    );
    assert_eq!(metadata.left_basis_digest(), left.basis_digest());
    assert_eq!(metadata.right_basis_digest(), right.basis_digest());
    assert_eq!(
        metadata.cost_class(),
        &QueryContextCostClass::DiffComparisonBounded
    );
    assert_eq!(
        metadata.budget_class(),
        &QueryContextBudgetClass::ComparisonBounded
    );
    assert_eq!(metadata.prediction_report().comparison_binding_width(), 2);
    assert_eq!(
        metadata.comparison_result_digest(),
        change_set.result_digest()
    );
    assert_eq!(
        metadata.prediction_drift_outcome(),
        &QueryContextPredictionDriftOutcome::WithinBudget
    );
    assert_eq!(diff.counters().comparison_scope_width(), 2);
    assert_eq!(diff.counters().comparison_basis_lookup_count(), 1);
    assert_eq!(diff.counters().comparison_row_width(), 1);
    assert_eq!(diff.counters().comparison_broadening_denial_count(), 0);
    assert_eq!(diff.counters().comparison_family_rediscovery_count(), 0);
}

#[test]
fn historical_execution_artifact_preserves_requested_admitted_and_resolved_path_identity() {
    let query_preflight = execution_preflights::direct_runtime_preflight();
    let request = HistoricalEvaluationRequest::retained_snapshot_for_test(
        "history:snapshot-1",
        1,
        1,
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let capability = HistoricalCapabilityDescriptor::retained_snapshot_for_test(
        "history:snapshot-1",
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let admission = admit_historical_evaluation_path(request, capability)
        .expect("retained history should admit");
    let resolved = resolve_historical_materialization_path(
        admission.clone(),
        HistoricalMaterializationDescriptor::retained_snapshot_for_test("history:snapshot-1"),
    )
    .expect("retained history should resolve");
    let metadata = materialization_metadata_from_resolved(resolved);

    let admitted = admit_and_scope_legacy_query_basis_context_for_test(
        bind_legacy_query_basis_context(
            QueryBasisContextRequest::historical_snapshot("history:snapshot-1"),
            QueryContextBindingSource::Historical {
                query_preflight: &query_preflight,
                admission: &admission,
                metadata: &metadata,
            },
        )
        .expect("historical context should bind"),
    )
    .expect("historical context should admit");
    let execution = execute_query_basis_context(&admitted)
        .expect("historical query-context execution should succeed");
    let shaped = attach_query_basis_metadata(&admitted, &execution)
        .expect("historical metadata should shape");

    assert_eq!(
        execution.family(),
        &QueryContextExecutionFamily::HistoricalMaterialized
    );
    assert_eq!(
        execution.requested_path_identity(),
        Some(metadata.requested_path_class().as_str())
    );
    assert_eq!(
        execution.admitted_path_identity(),
        Some(metadata.admitted_path_class().as_str())
    );
    assert_eq!(
        execution.resolved_path_identity(),
        Some(metadata.resolved_path_class().as_str())
    );
    assert_eq!(
        execution.prediction_drift_outcome(),
        &QueryContextPredictionDriftOutcome::WithinBudget
    );
    assert_eq!(
        shaped.requested_path_identity(),
        Some(metadata.requested_path_class().as_str())
    );
    assert_eq!(
        shaped.admitted_path_identity(),
        Some(metadata.admitted_path_class().as_str())
    );
    assert_eq!(
        shaped.resolved_path_identity(),
        Some(metadata.resolved_path_class().as_str())
    );
    assert_eq!(
        shaped.prediction_drift_outcome(),
        Some(&QueryContextPredictionDriftOutcome::WithinBudget)
    );
}

#[test]
fn invalid_runtime_current_vs_branch_pairing_rejects_typed_and_early() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let error = bind_legacy_query_basis_context(
        QueryBasisContextRequest::branch_head("branch:snapshot-2"),
        QueryContextBindingSource::RuntimeCurrent(&preflight),
    )
    .expect_err("runtime current source cannot bind branch-head family");

    assert_eq!(
        error.failure_class(),
        &QueryContextAdmissionFailureClass::InvalidBasisPairing
    );
    assert_eq!(error.counters().query_basis_binding_count(), 0);
    assert_eq!(error.counters().basis_binding_width(), 0);
    assert_eq!(error.counters().denial_width(), 1);
}

#[test]
fn store_backed_historical_debt_is_denied_typed_and_early() {
    let capability = HistoricalCapabilityDescriptor::new_for_test(
        "history:store",
        None,
        false,
        false,
        false,
        true,
        HistoricalPathReuseDescriptor::no_reuse(),
    );
    let error = bind_legacy_query_basis_context(
        QueryBasisContextRequest::historical_commit("history:store"),
        QueryContextBindingSource::HistoricalCapability(&capability),
    )
    .expect_err("store-backed history should remain deferred debt");

    assert_eq!(
        error.failure_class(),
        &QueryContextAdmissionFailureClass::StoreBackedHistoricalDeferred
    );
    assert_eq!(error.counters().unsupported_basis_denial_count(), 1);
    assert_eq!(error.counters().historical_lookup_width(), 1);
    assert_eq!(error.counters().denial_width(), 1);
}

#[test]
fn store_backed_retained_historical_binding_preserves_query_owned_parity() {
    let runtime_preflight = execution_preflights::direct_runtime_preflight();
    let store_preflight = execution_preflights::store_detail_preflight();
    let request = HistoricalEvaluationRequest::retained_snapshot_for_test(
        "history:snapshot-1",
        1,
        1,
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let capability = HistoricalCapabilityDescriptor::retained_snapshot_for_test(
        "history:snapshot-1",
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let admission = admit_historical_evaluation_path(request, capability)
        .expect("retained history should admit");
    let resolved = resolve_historical_materialization_path(
        admission.clone(),
        HistoricalMaterializationDescriptor::retained_snapshot_for_test("history:snapshot-1"),
    )
    .expect("retained history should resolve");
    let metadata = materialization_metadata_from_resolved(resolved);

    let runtime = admit_and_scope_legacy_query_basis_context_for_test(
        bind_legacy_query_basis_context(
            QueryBasisContextRequest::historical_snapshot("history:snapshot-1"),
            QueryContextBindingSource::Historical {
                query_preflight: &runtime_preflight,
                admission: &admission,
                metadata: &metadata,
            },
        )
        .expect("runtime historical context should bind"),
    )
    .expect("runtime historical context should admit");
    let store = admit_and_scope_legacy_query_basis_context_for_test(
        bind_legacy_query_basis_context(
            QueryBasisContextRequest::historical_snapshot("history:snapshot-1"),
            QueryContextBindingSource::Historical {
                query_preflight: &store_preflight,
                admission: &admission,
                metadata: &metadata,
            },
        )
        .expect("store historical context should bind"),
    )
    .expect("store historical context should admit");

    let runtime_bundle = attach_query_basis_metadata(
        &runtime,
        &execute_query_basis_context(&runtime).expect("runtime execution should succeed"),
    )
    .expect("runtime metadata should shape");
    let store_bundle = attach_query_basis_metadata(
        &store,
        &execute_query_basis_context(&store).expect("store execution should succeed"),
    )
    .expect("store metadata should shape");

    assert_eq!(
        runtime_bundle.result_digest(),
        store_bundle.result_digest(),
        "store-backed retained history must preserve canonical result parity"
    );
    assert_eq!(
        runtime_bundle.materialization_path_identity(),
        store_bundle.materialization_path_identity()
    );
    assert_eq!(
        runtime_bundle.historical_admission_class(),
        store_bundle.historical_admission_class()
    );
    assert_eq!(
        runtime.basis_authority_family(),
        &crate::basis::BasisAuthorityFamily::Runtime
    );
    assert_eq!(
        store.basis_authority_family(),
        &crate::basis::BasisAuthorityFamily::Store
    );
}

#[test]
fn store_backed_replay_historical_binding_stays_explicit_deferred_debt() {
    let store_preflight = execution_preflights::store_detail_preflight();
    let request = HistoricalEvaluationRequest::delta_replay_for_test(
        "history:snapshot-1",
        1,
        1,
        HistoricalPathReuseDescriptor::no_reuse(),
    );
    let capability = HistoricalCapabilityDescriptor::delta_replay_for_test(
        "history:snapshot-1",
        HistoricalPathReuseDescriptor::no_reuse(),
    );
    let admission =
        admit_historical_evaluation_path(request, capability).expect("replay history should admit");
    let resolved = resolve_historical_materialization_path(
        admission.clone(),
        HistoricalMaterializationDescriptor::delta_replay_for_test("history:snapshot-1"),
    )
    .expect("replay history should resolve");
    let metadata = materialization_metadata_from_resolved(resolved);

    let error = bind_legacy_query_basis_context(
        QueryBasisContextRequest::historical_snapshot("history:snapshot-1"),
        QueryContextBindingSource::Historical {
            query_preflight: &store_preflight,
            admission: &admission,
            metadata: &metadata,
        },
    )
    .expect_err("store-backed replay must remain deferred until a later milestone");

    assert_eq!(
        error.failure_class(),
        &QueryContextAdmissionFailureClass::StoreBackedHistoricalDeferred
    );
    assert_eq!(error.counters().unsupported_basis_denial_count(), 1);
    assert_eq!(error.counters().historical_lookup_width(), 1);
    assert_eq!(error.counters().denial_width(), 1);
}

#[test]
fn store_backed_reconstruction_historical_binding_stays_explicit_deferred_debt() {
    let store_preflight = execution_preflights::store_detail_preflight();
    let request = HistoricalEvaluationRequest::full_reconstruction_for_test(
        "history:snapshot-1",
        1,
        1,
        HistoricalPathReuseDescriptor::no_reuse(),
    );
    let capability = HistoricalCapabilityDescriptor::full_reconstruction_for_test(
        "history:snapshot-1",
        HistoricalPathReuseDescriptor::no_reuse(),
    );
    let admission = admit_historical_evaluation_path(request, capability)
        .expect("reconstruction history should admit");
    let resolved = resolve_historical_materialization_path(
        admission.clone(),
        HistoricalMaterializationDescriptor::full_reconstruction_for_test("history:snapshot-1"),
    )
    .expect("reconstruction history should resolve");
    let metadata = materialization_metadata_from_resolved(resolved);

    let error = bind_legacy_query_basis_context(
        QueryBasisContextRequest::historical_snapshot("history:snapshot-1"),
        QueryContextBindingSource::Historical {
            query_preflight: &store_preflight,
            admission: &admission,
            metadata: &metadata,
        },
    )
    .expect_err("store-backed reconstruction must remain deferred until a later milestone");

    assert_eq!(
        error.failure_class(),
        &QueryContextAdmissionFailureClass::StoreBackedHistoricalDeferred
    );
    assert_eq!(error.counters().unsupported_basis_denial_count(), 1);
    assert_eq!(error.counters().historical_lookup_width(), 1);
    assert_eq!(error.counters().denial_width(), 1);
}

#[test]
fn diff_scope_mismatch_rejects_before_broadening() {
    let left_preflight = execution_preflights::direct_runtime_preflight();
    let right_preflight = execution_preflights::ordered_collection_preflight();
    let left = admit_and_scope_legacy_query_basis_context_for_test(
        bind_legacy_query_basis_context(
            QueryBasisContextRequest::current_branch_head(),
            QueryContextBindingSource::RuntimeCurrent(&left_preflight),
        )
        .expect("left context should bind"),
    )
    .expect("left context should admit");
    let right = admit_and_scope_legacy_query_basis_context_for_test(
        bind_legacy_query_basis_context(
            QueryBasisContextRequest::branch_head("branch:ordered"),
            QueryContextBindingSource::RuntimeBranch(&right_preflight),
        )
        .expect("right context should bind"),
    )
    .expect("right context should admit");
    let error =
        bind_diff_query_context(&left, &right).expect_err("mismatched query scopes must reject");

    assert_eq!(
        error.failure_class(),
        &QueryContextAdmissionFailureClass::DiffScopeMismatch
    );
    assert_eq!(error.counters().basis_substitution_denial_count(), 1);
}

#[test]
fn basis_substitution_attempt_rejects_typed_and_early() {
    let query_preflight = execution_preflights::direct_runtime_preflight();
    let request = HistoricalEvaluationRequest::retained_snapshot_for_test(
        "history:snapshot-1",
        1,
        1,
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let capability = HistoricalCapabilityDescriptor::retained_snapshot_for_test(
        "history:snapshot-1",
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let admission = admit_historical_evaluation_path(request, capability)
        .expect("retained history should admit");
    let resolved = resolve_historical_materialization_path(
        admission.clone(),
        HistoricalMaterializationDescriptor::retained_snapshot_for_test("history:snapshot-1"),
    )
    .expect("retained history should resolve");
    let metadata = materialization_metadata_from_resolved(resolved);

    let error = bind_legacy_query_basis_context(
        QueryBasisContextRequest::historical_snapshot("history:other"),
        QueryContextBindingSource::Historical {
            query_preflight: &query_preflight,
            admission: &admission,
            metadata: &metadata,
        },
    )
    .expect_err("basis substitution must deny");

    assert_eq!(
        error.failure_class(),
        &QueryContextAdmissionFailureClass::BasisSubstitutionForbidden
    );
    assert_eq!(error.counters().basis_substitution_denial_count(), 1);
    assert_eq!(error.counters().historical_lookup_width(), 1);
    assert_eq!(error.counters().denial_width(), 1);
}

#[test]
fn current_to_historical_diff_admission_attaches_cost_budget_and_prediction_posture() {
    let current_preflight = execution_preflights::direct_runtime_preflight();
    let historical_preflight = execution_preflights::direct_runtime_preflight();
    let current = admit_and_scope_legacy_query_basis_context_for_test(
        bind_legacy_query_basis_context(
            QueryBasisContextRequest::current_branch_head(),
            QueryContextBindingSource::RuntimeCurrent(&current_preflight),
        )
        .expect("current context should bind"),
    )
    .expect("current context should admit");
    let request = HistoricalEvaluationRequest::retained_snapshot_for_test(
        "history:snapshot-1",
        1,
        1,
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let capability = HistoricalCapabilityDescriptor::retained_snapshot_for_test(
        "history:snapshot-1",
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let admission = admit_historical_evaluation_path(request, capability)
        .expect("retained history should admit");
    let resolved = resolve_historical_materialization_path(
        admission.clone(),
        HistoricalMaterializationDescriptor::retained_snapshot_for_test("history:snapshot-1"),
    )
    .expect("retained history should resolve");
    let metadata = materialization_metadata_from_resolved(resolved);
    let historical = admit_and_scope_legacy_query_basis_context_for_test(
        bind_legacy_query_basis_context(
            QueryBasisContextRequest::historical_snapshot("history:snapshot-1"),
            QueryContextBindingSource::Historical {
                query_preflight: &historical_preflight,
                admission: &admission,
                metadata: &metadata,
            },
        )
        .expect("historical context should bind"),
    )
    .expect("historical context should admit");

    let diff = bind_diff_query_context(&current, &historical).expect("diff should bind");

    assert_eq!(diff.family(), &ComparisonBasisFamily::CurrentToHistorical);
    assert_eq!(
        diff.cost_class(),
        &QueryContextCostClass::DiffComparisonBounded
    );
    assert_eq!(
        diff.budget_class(),
        &QueryContextBudgetClass::ComparisonBounded
    );
    assert_eq!(
        diff.prediction_drift_outcome(),
        &QueryContextPredictionDriftOutcome::PendingComparison
    );
    assert_eq!(diff.prediction_report().comparison_binding_width(), 2);
    assert_eq!(diff.counters().comparison_basis_lookup_count(), 1);
    assert_eq!(diff.counters().comparison_scope_width(), 2);
    assert_eq!(diff.counters().comparison_family_rediscovery_count(), 0);
}

#[test]
fn diff_change_set_rejects_basis_mismatched_execution_artifacts_before_materialization() {
    let left_preflight = execution_preflights::direct_runtime_preflight();
    let right_preflight = execution_preflights::alternate_basis_runtime_preflight();
    let left = admit_and_scope_legacy_query_basis_context_for_test(
        bind_legacy_query_basis_context(
            QueryBasisContextRequest::current_branch_head(),
            QueryContextBindingSource::RuntimeCurrent(&left_preflight),
        )
        .expect("left context should bind"),
    )
    .expect("left context should admit");
    let right = admit_and_scope_legacy_query_basis_context_for_test(
        bind_legacy_query_basis_context(
            QueryBasisContextRequest::branch_head("branch:snapshot-2"),
            QueryContextBindingSource::RuntimeBranch(&right_preflight),
        )
        .expect("right context should bind"),
    )
    .expect("right context should admit");
    let diff = bind_diff_query_context(&left, &right).expect("diff should bind");
    let left_result =
        execute_query_basis_context(&left).expect("left context execution should succeed");
    let wrong_right_result =
        execute_query_basis_context(&left).expect("left context execution should succeed");

    let error = shape_query_diff_change_set(&diff, &left_result, &wrong_right_result)
        .expect_err("basis-mismatched execution artifact must reject before row shaping");

    assert_eq!(
        error.failure_class(),
        &QueryContextAdmissionFailureClass::ComparisonShapeMismatch
    );
    assert_eq!(error.counters().comparison_basis_lookup_count(), 1);
    assert_eq!(error.counters().comparison_broadening_denial_count(), 0);
    assert_eq!(error.counters().denial_width(), 1);
}

#[test]
fn broadening_required_comparison_denies_before_rich_artifact_shaping() {
    let left_preflight = execution_preflights::ordered_collection_without_traversal_preflight();
    let right_preflight = execution_preflights::alternate_basis_ordered_collection_preflight();
    let left = admit_and_scope_legacy_query_basis_context_for_test(
        bind_legacy_query_basis_context(
            QueryBasisContextRequest::current_branch_head(),
            QueryContextBindingSource::RuntimeCurrent(&left_preflight),
        )
        .expect("left context should bind"),
    )
    .expect("left context should admit");
    let right = admit_and_scope_legacy_query_basis_context_for_test(
        bind_legacy_query_basis_context(
            QueryBasisContextRequest::branch_head("branch:ordered-collection"),
            QueryContextBindingSource::RuntimeBranch(&right_preflight),
        )
        .expect("right context should bind"),
    )
    .expect("right context should admit");
    let diff = bind_diff_query_context(&left, &right).expect("diff should bind");
    let left_result =
        execute_query_basis_context(&left).expect("left context execution should succeed");
    let right_result =
        execute_query_basis_context(&right).expect("right context execution should succeed");

    let error = shape_query_diff_change_set(&diff, &left_result, &right_result)
        .expect_err("ordered collection diff should deny hidden broadening");

    assert_eq!(
        error.failure_class(),
        &QueryContextAdmissionFailureClass::ComparisonBroadeningRequired
    );
    assert_eq!(error.counters().comparison_basis_lookup_count(), 1);
    assert_eq!(error.counters().comparison_broadening_denial_count(), 1);
    assert_eq!(error.counters().comparison_family_rediscovery_count(), 0);
    assert_eq!(error.counters().denial_width(), 1);
}

#[test]
fn historical_reconstruction_broadening_denies_before_rich_execution() {
    let preflight = execution_preflights::ordered_collection_preflight();
    let request = HistoricalEvaluationRequest::full_reconstruction_for_test(
        "history:reconstruction",
        4,
        8,
        HistoricalPathReuseDescriptor::no_reuse(),
    );
    let capability = HistoricalCapabilityDescriptor::full_reconstruction_for_test(
        "history:reconstruction",
        HistoricalPathReuseDescriptor::no_reuse(),
    );
    let admission =
        admit_historical_evaluation_path(request, capability).expect("reconstruction should admit");
    let resolved = resolve_historical_materialization_path(
        admission.clone(),
        HistoricalMaterializationDescriptor::full_reconstruction_for_test("history:reconstruction"),
    )
    .expect("reconstruction should resolve");
    let metadata = materialization_metadata_from_resolved(resolved);
    let admitted = admit_and_scope_legacy_query_basis_context_for_test(
        bind_legacy_query_basis_context(
            QueryBasisContextRequest::historical_snapshot("history:reconstruction"),
            QueryContextBindingSource::Historical {
                query_preflight: &preflight,
                admission: &admission,
                metadata: &metadata,
            },
        )
        .expect("historical reconstruction context should bind"),
    )
    .expect("historical reconstruction context should admit");

    let error = execute_query_basis_context(&admitted)
        .expect_err("reconstruction lane should deny broadening before execution");

    assert_eq!(
        error.failure_class(),
        &QueryContextAdmissionFailureClass::HistoricalPathTooBroadDenied
    );
    assert_eq!(
        error
            .counters()
            .materialization_path_compatibility_check_count(),
        1
    );
    assert_eq!(error.counters().historical_broadening_denial_count(), 1);
    assert_eq!(error.counters().denial_width(), 1);
}
