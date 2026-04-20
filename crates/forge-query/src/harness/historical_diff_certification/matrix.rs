use super::lane::{
    HistoricalDiffCertificationMatrix, HistoricalDiffLane, HistoricalDiffPerturbationClass,
    HistoricalDiffRejection,
};
use super::row_catalog::{
    HistoricalDiffCanonicalRowSpec, HistoricalDiffRejectionRowSpec,
    HISTORICAL_DIFF_CANONICAL_ROW_SPECS, HISTORICAL_DIFF_REJECTION_ROW_SPECS,
};
use crate::facade::{
    admit_historical_evaluation_path, admit_preview_workflow_foundation,
    bind_preflight_to_preview_session, materialization_metadata_from_resolved,
    resolve_historical_materialization_path, HistoricalCapabilityDescriptor,
    HistoricalEvaluationRequest, HistoricalMaterializationDescriptor,
    HistoricalPathReuseDescriptor, PreviewEvaluationClass, PreviewSessionQueryContext,
};
use crate::harness::certification::{
    digest_parts, CanonicalCertificationRow, ParityAnchor, RejectionCertificationRow,
};
use crate::harness::fixtures::{execution_preflights, preview_bridge::active_preview_artifacts};
use crate::query_context::{
    admit_query_basis_context, attach_diff_query_metadata, attach_query_basis_metadata,
    bind_diff_query_context, bind_query_basis_context, execute_query_basis_context,
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
        let preview = preview_lane();
        let branch_diff = branch_diff_lane();

        HistoricalDiffCertificationMatrix {
            suite_name: "Branch-Scoped, Historical, And Diff Query Context Test",
            rows: HISTORICAL_DIFF_CANONICAL_ROW_SPECS
                .iter()
                .map(|spec| {
                    canonical_row(spec, &current, &branch, &historical, &preview, &branch_diff)
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
    let context = admit_query_basis_context(
        bind_query_basis_context(
            QueryBasisContextRequest::current_branch_head(),
            QueryContextBindingSource::RuntimeCurrent(&preflight),
        )
        .expect("current context should bind"),
    )
    .expect("current context should admit");
    let result = execute_query_basis_context(&context)
        .expect("current query-context execution should succeed");
    let metadata = attach_query_basis_metadata(&context, &result).expect("metadata should shape");

    HistoricalDiffLane::from_basis_metadata(&metadata, context.counters())
}

fn branch_lane() -> HistoricalDiffLane {
    let preflight = execution_preflights::alternate_basis_runtime_preflight();
    let context = admit_query_basis_context(
        bind_query_basis_context(
            QueryBasisContextRequest::branch_head("branch:snapshot-2"),
            QueryContextBindingSource::RuntimeBranch(&preflight),
        )
        .expect("branch context should bind"),
    )
    .expect("branch context should admit");
    let result = execute_query_basis_context(&context)
        .expect("branch query-context execution should succeed");
    let metadata = attach_query_basis_metadata(&context, &result).expect("metadata should shape");

    HistoricalDiffLane::from_basis_metadata(&metadata, context.counters())
}

fn historical_lane() -> HistoricalDiffLane {
    let preflight = execution_preflights::direct_runtime_preflight();
    let request = HistoricalEvaluationRequest::retained_snapshot(
        "history:snapshot-1",
        1,
        1,
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let capability = HistoricalCapabilityDescriptor::retained_snapshot(
        "history:snapshot-1",
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let admission =
        admit_historical_evaluation_path(request, capability).expect("history should admit");
    let resolved = resolve_historical_materialization_path(
        admission.clone(),
        HistoricalMaterializationDescriptor::retained_snapshot("history:snapshot-1"),
    )
    .expect("history should resolve");
    let metadata = materialization_metadata_from_resolved(resolved);
    let context = admit_query_basis_context(
        bind_query_basis_context(
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
    let shaped = attach_query_basis_metadata(&context, &result).expect("metadata should shape");

    HistoricalDiffLane::from_basis_metadata(&shaped, context.counters())
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
    let context = admit_query_basis_context(
        bind_query_basis_context(
            QueryBasisContextRequest::preview_derived_historical(
                foundation.preview_session_identity().as_str(),
            ),
            QueryContextBindingSource::PreviewDerivedHistorical(&foundation),
        )
        .expect("preview-derived context should bind"),
    )
    .expect("preview-derived context should admit");
    let result = execute_query_basis_context(&context)
        .expect("preview-derived query-context execution should succeed");
    let metadata = attach_query_basis_metadata(&context, &result)
        .expect("preview-derived metadata should shape");

    HistoricalDiffLane::from_basis_metadata(&metadata, context.counters())
}

fn branch_diff_lane() -> HistoricalDiffLane {
    let left_preflight = execution_preflights::direct_runtime_preflight();
    let right_preflight = execution_preflights::alternate_basis_runtime_preflight();
    let left = admit_query_basis_context(
        bind_query_basis_context(
            QueryBasisContextRequest::current_branch_head(),
            QueryContextBindingSource::RuntimeCurrent(&left_preflight),
        )
        .expect("left should bind"),
    )
    .expect("left should admit");
    let right = admit_query_basis_context(
        bind_query_basis_context(
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
    let metadata = attach_diff_query_metadata(&diff, &left_result, &right_result)
        .expect("diff metadata should shape");

    HistoricalDiffLane {
        query_digest: metadata.query_digest().to_string(),
        basis_digest: metadata.left_basis_digest().to_string(),
        result_digest: change_set.result_digest().to_string(),
        replay_digest: digest_parts(&[
            format!("query:{}", metadata.query_digest()),
            format!("left_basis:{}", metadata.left_basis_digest()),
            format!("right_basis:{}", metadata.right_basis_digest()),
            format!("diff_result:{}", change_set.result_digest()),
            format!("change_rows:{}", change_set.rows().len()),
            format!("drift:{}", metadata.drift_outcome().as_str()),
        ]),
        basis_family: left.family().as_str().to_string(),
        cost_class: left.cost_class().as_str().to_string(),
        budget_class: left.budget_class().as_str().to_string(),
        historical_admission_class: "none".to_string(),
        comparison_family: metadata.comparison_basis_family().as_str().to_string(),
        prediction_drift_outcome: "none".to_string(),
        counter_snapshot_digest: digest_parts(&[
            format!(
                "comparison_lookups:{}",
                diff.counters().comparison_basis_lookup_count()
            ),
            format!("scope_width:{}", diff.counters().comparison_scope_width()),
            format!("diff_breadth:{}", diff.counters().diff_input_breadth()),
            format!("binding_width:{}", diff.counters().basis_binding_width()),
            format!(
                "historical_width:{}",
                diff.counters().historical_lookup_width()
            ),
            format!("denial_width:{}", diff.counters().denial_width()),
            format!(
                "basis_rediscovery:{}",
                diff.counters().basis_rediscovery_count()
            ),
            format!(
                "historical_path_rediscovery:{}",
                diff.counters().historical_path_rediscovery_count()
            ),
        ]),
    }
}

fn canonical_row(
    spec: &HistoricalDiffCanonicalRowSpec,
    current: &HistoricalDiffLane,
    branch: &HistoricalDiffLane,
    historical: &HistoricalDiffLane,
    preview: &HistoricalDiffLane,
    branch_diff: &HistoricalDiffLane,
) -> CanonicalCertificationRow<HistoricalDiffPerturbationClass, HistoricalDiffLane> {
    let (control_lane, hostile_lane) = match spec.row_name {
        "current-vs-branch-basis-explicitness" => (current.clone(), branch.clone()),
        "current-vs-historical-basis-explicitness" => (current.clone(), historical.clone()),
        "historical-materialization-path-explicitness" => (historical.clone(), historical.clone()),
        "diff-comparison-family-explicitness" => (branch_diff.clone(), branch.clone()),
        "result-shape-parity-across-basis-variants" => (current.clone(), current.clone()),
        "preview-derived-historical-basis-explicitness" => (historical.clone(), preview.clone()),
        other => panic!("unexpected historical diff canonical row {other}"),
    };

    CanonicalCertificationRow {
        row_name: spec.row_name,
        perturbation_class: spec.perturbation_class,
        hostile_expectation: spec.hostile_expectation,
        parity_anchor: ParityAnchor::Control,
        control_lane: control_lane.clone(),
        hostile_lane,
        parity_lane: control_lane,
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
            &bind_query_basis_context(
                QueryBasisContextRequest::historical_snapshot("history:unsupported"),
                QueryContextBindingSource::HistoricalCapability(
                    &HistoricalCapabilityDescriptor::retained_snapshot(
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
        "store-backed-historical-deferred-debt" => HistoricalDiffRejection::from_error(
            &bind_query_basis_context(
                QueryBasisContextRequest::historical_commit("history:store"),
                QueryContextBindingSource::HistoricalCapability(
                    &HistoricalCapabilityDescriptor::new(
                        "history:store",
                        None,
                        false,
                        false,
                        false,
                        true,
                        HistoricalPathReuseDescriptor::no_reuse(),
                    ),
                ),
            )
            .expect_err("store historical should remain deferred"),
        ),
        "forbidden-basis-substitution" => {
            HistoricalDiffRejection::from_error(&basis_substitution_error())
        }
        "raw-storage-delta-leakage-forbidden" => {
            let left_preflight = execution_preflights::direct_runtime_preflight();
            let left = admit_query_basis_context(
                bind_query_basis_context(
                    QueryBasisContextRequest::current_branch_head(),
                    QueryContextBindingSource::RuntimeCurrent(&left_preflight),
                )
                .expect("left should bind"),
            )
            .expect("left should admit");
            HistoricalDiffRejection::from_error(
                &bind_diff_query_context(&left, &left)
                    .expect_err("identical basis diff should deny broad comparison"),
            )
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

fn preview_lane_context() -> crate::query_context::AdmittedQueryBasisContext {
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
    admit_query_basis_context(
        bind_query_basis_context(
            QueryBasisContextRequest::preview_derived_historical(
                foundation.preview_session_identity().as_str(),
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
    let left = admit_query_basis_context(
        bind_query_basis_context(
            QueryBasisContextRequest::current_branch_head(),
            QueryContextBindingSource::RuntimeCurrent(&left_preflight),
        )
        .expect("left should bind"),
    )
    .expect("left should admit");
    let right = admit_query_basis_context(
        bind_query_basis_context(
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
    let request = HistoricalEvaluationRequest::retained_snapshot(
        "history:snapshot-1",
        1,
        1,
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let capability = HistoricalCapabilityDescriptor::retained_snapshot(
        "history:snapshot-1",
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let admission =
        admit_historical_evaluation_path(request, capability).expect("history should admit");
    let resolved = resolve_historical_materialization_path(
        admission.clone(),
        HistoricalMaterializationDescriptor::retained_snapshot("history:snapshot-1"),
    )
    .expect("history should resolve");
    let metadata = materialization_metadata_from_resolved(resolved);

    bind_query_basis_context(
        QueryBasisContextRequest::historical_snapshot("history:other"),
        QueryContextBindingSource::Historical {
            query_preflight: &preflight,
            admission: &admission,
            metadata: &metadata,
        },
    )
    .expect_err("basis substitution must deny")
}
