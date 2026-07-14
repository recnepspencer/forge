use super::lane::{
    HistoricalDiffCertificationMatrix, HistoricalDiffLane, HistoricalDiffPerturbationClass,
    HistoricalDiffRejection,
};
use super::row_catalog::{
    HistoricalDiffCanonicalRowSpec, HistoricalDiffRejectionRowSpec,
    HISTORICAL_DIFF_CANONICAL_ROW_SPECS, HISTORICAL_DIFF_REJECTION_ROW_SPECS,
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
use crate::harness::certification::{
    CanonicalCertificationRow, ParityAnchor, RejectionCertificationRow,
};
use crate::harness::fixtures::{execution_preflights, preview_bridge::active_preview_artifacts};
use crate::query_context::{
    admit_and_scope_legacy_query_basis_context_for_test, bind_diff_query_context,
    bind_legacy_query_basis_context, build_query_basis_result_bundle,
    build_query_diff_result_bundle, execute_query_basis_context, reject_raw_storage_delta_access,
    shape_query_diff_change_set, QueryBasisContextRequest, QueryContextAdmissionError,
    QueryContextBindingSource,
};

pub struct MilestoneSixHistoricalDiffCertificationAdapter;

impl MilestoneSixHistoricalDiffCertificationAdapter {
    pub fn branch_scoped_historical_and_diff_query_context_test(
    ) -> HistoricalDiffCertificationMatrix {
        let current = current_lane();
        let branch = branch_lane();
        let historical = historical_lane();
        let store_historical = store_historical_lane();
        let preview = preview_lane();
        let branch_diff = branch_diff_lane();
        let current_historical_diff = current_historical_diff_lane();

        HistoricalDiffCertificationMatrix {
            suite_name: "Historical / Diff / Basis Parity Test",
            rows: HISTORICAL_DIFF_CANONICAL_ROW_SPECS
                .iter()
                .map(|spec| {
                    canonical_row(
                        spec,
                        &current,
                        &branch,
                        &historical,
                        &store_historical,
                        &preview,
                        &branch_diff,
                        &current_historical_diff,
                    )
                })
                .collect(),
            rejection_rows: HISTORICAL_DIFF_REJECTION_ROW_SPECS
                .iter()
                .map(rejection_row)
                .collect(),
        }
    }
}

fn current_lane() -> HistoricalDiffLane {
    let preflight = execution_preflights::direct_runtime_preflight();
    let context = admit_and_scope_legacy_query_basis_context_for_test(
        bind_legacy_query_basis_context(
            QueryBasisContextRequest::current_branch_head(),
            QueryContextBindingSource::RuntimeCurrent(&preflight),
        )
        .expect("current context should bind"),
    )
    .expect("current context should admit");
    let result = execute_query_basis_context(&context)
        .expect("current query-context execution should succeed");
    let bundle = build_query_basis_result_bundle(&context, result)
        .expect("basis result bundle should shape");

    HistoricalDiffLane::from_basis_result_bundle(&bundle)
}

fn branch_lane() -> HistoricalDiffLane {
    let preflight = execution_preflights::alternate_basis_runtime_preflight();
    let context = admit_and_scope_legacy_query_basis_context_for_test(
        bind_legacy_query_basis_context(
            QueryBasisContextRequest::branch_head("branch:snapshot-2"),
            QueryContextBindingSource::RuntimeBranch(&preflight),
        )
        .expect("branch context should bind"),
    )
    .expect("branch context should admit");
    let result = execute_query_basis_context(&context)
        .expect("branch query-context execution should succeed");
    let bundle = build_query_basis_result_bundle(&context, result)
        .expect("basis result bundle should shape");

    HistoricalDiffLane::from_basis_result_bundle(&bundle)
}

fn historical_lane() -> HistoricalDiffLane {
    let preflight = execution_preflights::direct_runtime_preflight();
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
    let admission =
        admit_historical_evaluation_path(request, capability).expect("history should admit");
    let resolved = resolve_historical_materialization_path(
        admission.clone(),
        HistoricalMaterializationDescriptor::retained_snapshot_for_test("history:snapshot-1"),
    )
    .expect("history should resolve");
    let metadata = materialization_metadata_from_resolved(resolved);
    let context = admit_and_scope_legacy_query_basis_context_for_test(
        bind_legacy_query_basis_context(
            QueryBasisContextRequest::historical_snapshot("history:snapshot-1"),
            QueryContextBindingSource::Historical {
                query_preflight: &preflight,
                admission: &admission,
                metadata: &metadata,
            },
        )
        .expect("historical context should bind"),
    )
    .expect("historical context should admit");
    let result = execute_query_basis_context(&context)
        .expect("historical query-context execution should succeed");
    let bundle = build_query_basis_result_bundle(&context, result)
        .expect("basis result bundle should shape");

    HistoricalDiffLane::from_basis_result_bundle(&bundle)
}

fn store_historical_lane() -> HistoricalDiffLane {
    let preflight = execution_preflights::store_detail_preflight();
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
    let admission =
        admit_historical_evaluation_path(request, capability).expect("history should admit");
    let resolved = resolve_historical_materialization_path(
        admission.clone(),
        HistoricalMaterializationDescriptor::retained_snapshot_for_test("history:snapshot-1"),
    )
    .expect("history should resolve");
    let metadata = materialization_metadata_from_resolved(resolved);
    let context = admit_and_scope_legacy_query_basis_context_for_test(
        bind_legacy_query_basis_context(
            QueryBasisContextRequest::historical_snapshot("history:snapshot-1"),
            QueryContextBindingSource::Historical {
                query_preflight: &preflight,
                admission: &admission,
                metadata: &metadata,
            },
        )
        .expect("store historical context should bind"),
    )
    .expect("store historical context should admit");
    let result = execute_query_basis_context(&context)
        .expect("store historical query-context execution should succeed");
    let bundle = build_query_basis_result_bundle(&context, result)
        .expect("basis result bundle should shape");

    HistoricalDiffLane::from_basis_result_bundle(&bundle)
}

fn preview_lane() -> HistoricalDiffLane {
    let preflight = execution_preflights::direct_runtime_preflight();
    let (_runtime, active, execution_record) = active_preview_artifacts("historical-diff-preview");
    let preview_binding = bind_preflight_to_preview_session(
        preflight,
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::promotion_eligible(),
        ),
    )
    .expect("preview binding should succeed");
    let foundation = admit_preview_workflow_foundation(&preview_binding)
        .expect("preview foundation should admit");
    let preview_session_identity = foundation
        .preview_session_identity()
        .bridge_admission_evidence();
    let context = admit_and_scope_legacy_query_basis_context_for_test(
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
    let result = execute_query_basis_context(&context)
        .expect("preview-derived query-context execution should succeed");
    let bundle = build_query_basis_result_bundle(&context, result)
        .expect("basis result bundle should shape");

    HistoricalDiffLane::from_basis_result_bundle(&bundle)
}

fn branch_diff_lane() -> HistoricalDiffLane {
    let left_preflight = execution_preflights::direct_runtime_preflight();
    let right_preflight = execution_preflights::alternate_basis_runtime_preflight();
    let left = admit_and_scope_legacy_query_basis_context_for_test(
        bind_legacy_query_basis_context(
            QueryBasisContextRequest::current_branch_head(),
            QueryContextBindingSource::RuntimeCurrent(&left_preflight),
        )
        .expect("left should bind"),
    )
    .expect("left should admit");
    let right = admit_and_scope_legacy_query_basis_context_for_test(
        bind_legacy_query_basis_context(
            QueryBasisContextRequest::branch_head("branch:snapshot-2"),
            QueryContextBindingSource::RuntimeBranch(&right_preflight),
        )
        .expect("right should bind"),
    )
    .expect("right should admit");
    let diff = bind_diff_query_context(&left, &right).expect("diff should bind");
    let left_result =
        execute_query_basis_context(&left).expect("left query-context execution should succeed");
    let right_result =
        execute_query_basis_context(&right).expect("right query-context execution should succeed");
    let change_set = shape_query_diff_change_set(&diff, &left_result, &right_result)
        .expect("diff change-set should shape");
    let bundle = build_query_diff_result_bundle(&diff, change_set, &left_result, &right_result)
        .expect("diff result bundle should shape");

    HistoricalDiffLane::from_diff_result_bundle(
        &bundle,
        left_result.counters().context_execution_count(),
        right_result.counters().context_execution_count(),
        left_result.counters().executor_rediscovery_count()
            + right_result.counters().executor_rediscovery_count(),
    )
}

fn current_historical_diff_lane() -> HistoricalDiffLane {
    let current_preflight = execution_preflights::direct_runtime_preflight();
    let historical_preflight = execution_preflights::direct_runtime_preflight();
    let current = admit_and_scope_legacy_query_basis_context_for_test(
        bind_legacy_query_basis_context(
            QueryBasisContextRequest::current_branch_head(),
            QueryContextBindingSource::RuntimeCurrent(&current_preflight),
        )
        .expect("current should bind"),
    )
    .expect("current should admit");
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
    let admission =
        admit_historical_evaluation_path(request, capability).expect("history should admit");
    let resolved = resolve_historical_materialization_path(
        admission.clone(),
        HistoricalMaterializationDescriptor::retained_snapshot_for_test("history:snapshot-1"),
    )
    .expect("history should resolve");
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
        .expect("historical should bind"),
    )
    .expect("historical should admit");
    let diff = bind_diff_query_context(&current, &historical).expect("diff should bind");
    let current_result = execute_query_basis_context(&current)
        .expect("current query-context execution should succeed");
    let historical_result = execute_query_basis_context(&historical)
        .expect("historical query-context execution should succeed");
    let change_set = shape_query_diff_change_set(&diff, &current_result, &historical_result)
        .expect("current-to-historical diff change-set should shape");
    let bundle =
        build_query_diff_result_bundle(&diff, change_set, &current_result, &historical_result)
            .expect("current-to-historical diff result bundle should shape");

    HistoricalDiffLane::from_diff_result_bundle(
        &bundle,
        current_result.counters().context_execution_count(),
        historical_result.counters().context_execution_count(),
        current_result.counters().executor_rediscovery_count()
            + historical_result.counters().executor_rediscovery_count(),
    )
}

fn canonical_row(
    spec: &HistoricalDiffCanonicalRowSpec,
    current: &HistoricalDiffLane,
    branch: &HistoricalDiffLane,
    historical: &HistoricalDiffLane,
    store_historical: &HistoricalDiffLane,
    preview: &HistoricalDiffLane,
    branch_diff: &HistoricalDiffLane,
    current_historical_diff: &HistoricalDiffLane,
) -> CanonicalCertificationRow<HistoricalDiffPerturbationClass, HistoricalDiffLane> {
    let (control_lane, hostile_lane) = match spec.row_name {
        "current-vs-branch-basis-explicitness" => (current.clone(), branch.clone()),
        "current-vs-historical-basis-explicitness" => (current.clone(), historical.clone()),
        "historical-materialization-path-explicitness" => (current.clone(), historical.clone()),
        "runtime-vs-store-historical-parity" => (historical.clone(), store_historical.clone()),
        "diff-comparison-family-explicitness" => (branch_diff.clone(), branch.clone()),
        "branch-to-branch-diff-shaped" => (branch.clone(), branch_diff.clone()),
        "current-to-historical-diff-shaped" => {
            (historical.clone(), current_historical_diff.clone())
        }
        "result-shape-parity-across-basis-variants" => (current.clone(), historical.clone()),
        "preview-derived-historical-basis-explicitness" => (historical.clone(), preview.clone()),
        "admitted-diff-cost-class-explicitness" => (branch.clone(), branch_diff.clone()),
        "prediction-versus-realization-explicitness" => (branch.clone(), branch_diff.clone()),
        other => panic!("unexpected historical diff canonical row {other}"),
    };
    let parity_lane = match spec.row_name {
        "current-vs-branch-basis-explicitness" => current_lane(),
        "current-vs-historical-basis-explicitness" => current_lane(),
        "historical-materialization-path-explicitness" => historical_lane(),
        "runtime-vs-store-historical-parity" => store_historical_lane(),
        "diff-comparison-family-explicitness" => branch_diff_lane(),
        "branch-to-branch-diff-shaped" => branch_diff_lane(),
        "current-to-historical-diff-shaped" => current_historical_diff_lane(),
        "result-shape-parity-across-basis-variants" => current_lane(),
        "preview-derived-historical-basis-explicitness" => preview_lane(),
        "admitted-diff-cost-class-explicitness" => branch_diff_lane(),
        "prediction-versus-realization-explicitness" => branch_diff_lane(),
        other => panic!("unexpected historical diff canonical row {other}"),
    };

    CanonicalCertificationRow {
        row_name: spec.row_name,
        perturbation_class: spec.perturbation_class,
        hostile_expectation: spec.hostile_expectation,
        parity_anchor: ParityAnchor::Control,
        control_lane,
        hostile_lane,
        parity_lane,
    }
}

fn rejection_row(
    spec: &HistoricalDiffRejectionRowSpec,
) -> RejectionCertificationRow<
    HistoricalDiffPerturbationClass,
    HistoricalDiffLane,
    HistoricalDiffRejection,
> {
    let control_lane = current_lane();
    let parity_lane = branch_lane();
    let hostile_lane = match spec.row_name {
        "unsupported-historical-basis" => HistoricalDiffRejection::from_error(
            &bind_legacy_query_basis_context(
                QueryBasisContextRequest::historical_snapshot("history:unsupported"),
                QueryContextBindingSource::HistoricalCapability(
                    &HistoricalCapabilityDescriptor::retained_snapshot_for_test(
                        "history:unsupported",
                        HistoricalPathReuseDescriptor::retained_reuse(),
                    ),
                ),
            )
            .expect_err("raw historical capability should not mint an admitted query context"),
        ),
        "ambiguous-comparison-basis" => {
            let preview = preview_lane_context();
            HistoricalDiffRejection::from_error(
                &bind_diff_query_context(&preview, &preview)
                    .expect_err("preview to preview comparison should stay ambiguous"),
            )
        }
        "diff-scope-mismatch" => HistoricalDiffRejection::from_error(&diff_scope_mismatch_error()),
        "forbidden-basis-substitution" => {
            HistoricalDiffRejection::from_error(&basis_substitution_error())
        }
        "raw-storage-delta-leakage-forbidden" => {
            HistoricalDiffRejection::from_error(&reject_raw_storage_delta_access())
        }
        "historical-broadening-denied" => {
            HistoricalDiffRejection::from_error(&historical_broadening_error())
        }
        "broadening-required-comparison-denial" => {
            HistoricalDiffRejection::from_error(&comparison_broadening_error())
        }
        "declared-result-shape-mismatch" => {
            HistoricalDiffRejection::from_error(&comparison_shape_mismatch_error())
        }
        other => panic!("unexpected historical diff rejection row {other}"),
    };

    RejectionCertificationRow {
        row_name: spec.row_name,
        perturbation_class: spec.perturbation_class,
        control_lane,
        hostile_lane,
        parity_lane,
    }
}

fn historical_broadening_error() -> QueryContextAdmissionError {
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
    let context = admit_and_scope_legacy_query_basis_context_for_test(
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

    execute_query_basis_context(&context)
        .expect_err("reconstruction lane should deny broadening before rich execution")
}

fn preview_lane_context() -> crate::query_context::ScopedQueryBasisContext {
    let preflight = execution_preflights::direct_runtime_preflight();
    let (_runtime, active, execution_record) =
        active_preview_artifacts("historical-diff-preview-rejection");
    let preview_binding = bind_preflight_to_preview_session(
        preflight,
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::promotion_eligible(),
        ),
    )
    .expect("preview binding should succeed");
    let foundation = admit_preview_workflow_foundation(&preview_binding)
        .expect("preview foundation should admit");
    let preview_session_identity = foundation
        .preview_session_identity()
        .bridge_admission_evidence();
    admit_and_scope_legacy_query_basis_context_for_test(
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
    .expect("preview-derived context should admit")
}

fn diff_scope_mismatch_error() -> QueryContextAdmissionError {
    let left_preflight = execution_preflights::direct_runtime_preflight();
    let right_preflight = execution_preflights::ordered_collection_preflight();
    let left = admit_and_scope_legacy_query_basis_context_for_test(
        bind_legacy_query_basis_context(
            QueryBasisContextRequest::current_branch_head(),
            QueryContextBindingSource::RuntimeCurrent(&left_preflight),
        )
        .expect("left should bind"),
    )
    .expect("left should admit");
    let right = admit_and_scope_legacy_query_basis_context_for_test(
        bind_legacy_query_basis_context(
            QueryBasisContextRequest::branch_head("branch:ordered"),
            QueryContextBindingSource::RuntimeBranch(&right_preflight),
        )
        .expect("right should bind"),
    )
    .expect("right should admit");
    bind_diff_query_context(&left, &right).expect_err("mismatched query scopes should deny")
}

fn basis_substitution_error() -> QueryContextAdmissionError {
    let preflight = execution_preflights::direct_runtime_preflight();
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
    let admission =
        admit_historical_evaluation_path(request, capability).expect("history should admit");
    let resolved = resolve_historical_materialization_path(
        admission.clone(),
        HistoricalMaterializationDescriptor::retained_snapshot_for_test("history:snapshot-1"),
    )
    .expect("history should resolve");
    let metadata = materialization_metadata_from_resolved(resolved);

    bind_legacy_query_basis_context(
        QueryBasisContextRequest::historical_snapshot("history:other"),
        QueryContextBindingSource::Historical {
            query_preflight: &preflight,
            admission: &admission,
            metadata: &metadata,
        },
    )
    .expect_err("basis substitution must deny")
}

fn comparison_broadening_error() -> QueryContextAdmissionError {
    let left_preflight = execution_preflights::ordered_collection_without_traversal_preflight();
    let right_preflight = execution_preflights::alternate_basis_ordered_collection_preflight();
    let left = admit_and_scope_legacy_query_basis_context_for_test(
        bind_legacy_query_basis_context(
            QueryBasisContextRequest::current_branch_head(),
            QueryContextBindingSource::RuntimeCurrent(&left_preflight),
        )
        .expect("left should bind"),
    )
    .expect("left should admit");
    let right = admit_and_scope_legacy_query_basis_context_for_test(
        bind_legacy_query_basis_context(
            QueryBasisContextRequest::branch_head("branch:ordered-collection"),
            QueryContextBindingSource::RuntimeBranch(&right_preflight),
        )
        .expect("right should bind"),
    )
    .expect("right should admit");
    let diff = bind_diff_query_context(&left, &right).expect("diff should bind");
    let left_result =
        execute_query_basis_context(&left).expect("left query-context execution should succeed");
    let right_result =
        execute_query_basis_context(&right).expect("right query-context execution should succeed");

    shape_query_diff_change_set(&diff, &left_result, &right_result)
        .expect_err("ordered collection diff should deny hidden broadening")
}

fn comparison_shape_mismatch_error() -> QueryContextAdmissionError {
    let left_preflight = execution_preflights::direct_runtime_preflight();
    let right_preflight = execution_preflights::alternate_basis_runtime_preflight();
    let left = admit_and_scope_legacy_query_basis_context_for_test(
        bind_legacy_query_basis_context(
            QueryBasisContextRequest::current_branch_head(),
            QueryContextBindingSource::RuntimeCurrent(&left_preflight),
        )
        .expect("left should bind"),
    )
    .expect("left should admit");
    let right = admit_and_scope_legacy_query_basis_context_for_test(
        bind_legacy_query_basis_context(
            QueryBasisContextRequest::branch_head("branch:snapshot-2"),
            QueryContextBindingSource::RuntimeBranch(&right_preflight),
        )
        .expect("right should bind"),
    )
    .expect("right should admit");
    let diff = bind_diff_query_context(&left, &right).expect("diff should bind");
    let left_result =
        execute_query_basis_context(&left).expect("left query-context execution should succeed");
    let wrong_right_result =
        execute_query_basis_context(&left).expect("left query-context execution should succeed");

    shape_query_diff_change_set(&diff, &left_result, &wrong_right_result)
        .expect_err("basis-mismatched execution artifact should deny comparison shaping")
}
