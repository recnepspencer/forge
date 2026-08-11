use super::{PreviewCertificationLane, PreviewCertificationMatrix, PreviewCertificationRejection};
use crate::preview::{
    PreviewBindingCounters, PreviewComparisonCounters, PreviewExecutionCounters,
    PreviewLiveCounters,
};

pub(super) fn bundle_digest_parts(matrix: &PreviewCertificationMatrix) -> Vec<String> {
    let mut parts = vec![format!("suite:{}", matrix.suite_name)];
    for row in &matrix.rows {
        parts.push(format!("canonical:{}", row.row_name));
        parts.extend(lane_digest_parts(&row.control_lane, "control"));
        parts.extend(lane_digest_parts(&row.hostile_lane, "hostile"));
        parts.extend(lane_digest_parts(&row.parity_lane, "parity"));
    }
    for row in &matrix.rejection_rows {
        parts.push(format!("rejection:{}", row.row_name));
        parts.extend(lane_digest_parts(&row.control_lane, "control"));
        parts.extend(rejection_digest_parts(
            &row.hostile_lane,
            "hostile_rejection",
        ));
        parts.extend(lane_digest_parts(&row.parity_lane, "parity"));
    }
    parts
}

pub(super) fn coverage_digest_parts(matrix: &PreviewCertificationMatrix) -> Vec<String> {
    let mut parts = vec![format!("suite:{}", matrix.suite_name)];
    parts.extend(
        matrix
            .rows
            .iter()
            .map(|row| format!("canonical:{}", row.row_name)),
    );
    parts.extend(
        matrix
            .rejection_rows
            .iter()
            .map(|row| format!("rejection:{}", row.row_name)),
    );
    parts
}

fn lane_digest_parts(bundle: &PreviewCertificationLane, label: &str) -> Vec<String> {
    let mut parts = vec![
        format!("{label}_query_digest:{}", bundle.query_digest),
        format!("{label}_result_shape_digest:{}", bundle.result_shape_digest),
        format!(
            "{label}_preview_session_identity:{}",
            bundle.preview_session_identity
        ),
        format!(
            "{label}_evaluation_class:{}",
            bundle.evaluation_class.as_str()
        ),
        format!(
            "{label}_lifecycle_state_kind:{}",
            bundle.lifecycle_state_kind.as_str()
        ),
        format!("{label}_binding_digest:{}", bundle.binding_digest),
        format!(
            "{label}_preview_execution_digest:{}",
            bundle.preview_execution_digest
        ),
        format!(
            "{label}_comparison_eligibility_digest:{}",
            bundle.comparison_eligibility_digest
        ),
        format!(
            "{label}_workflow_foundation_digest:{}",
            bundle.workflow_foundation_digest
        ),
    ];
    if let Some(digest) = bundle.promotion_parity_digest.as_ref() {
        parts.push(format!("{label}_promotion_parity_digest:{digest}"));
    }
    if let Some(digest) = bundle.preview_live_digest.as_ref() {
        parts.push(format!("{label}_preview_live_digest:{digest}"));
    }
    if let Some(digest) = bundle.preview_live_subscription_digest.as_ref() {
        parts.push(format!("{label}_preview_live_subscription_digest:{digest}"));
    }
    if let Some(family) = bundle.preview_live_family.as_ref() {
        parts.push(format!("{label}_preview_live_family:{family}"));
    }
    parts.extend(counter_digest_parts(&bundle.counters, label));
    parts.extend(execution_counter_digest_parts(
        &bundle.execution_counters,
        label,
    ));
    if let Some(comparison_counters) = bundle.comparison_counters.as_ref() {
        parts.extend(comparison_counter_digest_parts(comparison_counters, label));
    }
    if let Some(preview_live_counters) = bundle.preview_live_counters.as_ref() {
        parts.extend(preview_live_counter_digest_parts(
            preview_live_counters,
            label,
        ));
    }
    parts
}

fn rejection_digest_parts(bundle: &PreviewCertificationRejection, label: &str) -> Vec<String> {
    let mut parts = vec![format!(
        "{label}_failure_class:{}",
        bundle.failure_class.as_str()
    )];
    if let Some(counters) = bundle.counters.as_ref() {
        parts.extend(counter_digest_parts(counters, label));
    }
    if let Some(counters) = bundle.execution_counters.as_ref() {
        parts.extend(execution_counter_digest_parts(counters, label));
    }
    if let Some(counters) = bundle.comparison_counters.as_ref() {
        parts.extend(comparison_counter_digest_parts(counters, label));
    }
    if let Some(counters) = bundle.preview_live_counters.as_ref() {
        parts.extend(preview_live_counter_digest_parts(counters, label));
    }
    parts
}

fn execution_counter_digest_parts(counters: &PreviewExecutionCounters, label: &str) -> Vec<String> {
    vec![
        format!(
            "{label}_preview_execution_envelope_count:{}",
            counters.preview_execution_envelope_count()
        ),
        format!(
            "{label}_preview_execution_count:{}",
            counters.preview_execution_count()
        ),
        format!(
            "{label}_preview_promotable_execution_count:{}",
            counters.preview_promotable_execution_count()
        ),
        format!(
            "{label}_preview_read_only_execution_count:{}",
            counters.preview_read_only_execution_count()
        ),
        format!(
            "{label}_preview_comparison_eligibility_proof_count:{}",
            counters.preview_comparison_eligibility_proof_count()
        ),
        format!(
            "{label}_preview_comparison_shape_check_width:{}",
            counters.preview_comparison_shape_check_width()
        ),
        format!(
            "{label}_preview_workflow_foundation_artifact_lookup_count:{}",
            counters.preview_workflow_foundation_artifact_lookup_count()
        ),
        format!(
            "{label}_preview_workflow_foundation_admission_count:{}",
            counters.preview_workflow_foundation_admission_count()
        ),
        format!(
            "{label}_preview_workflow_foundation_denial_count:{}",
            counters.preview_workflow_foundation_denial_count()
        ),
        format!(
            "{label}_preview_work_avoided_by_explicit_basis_count:{}",
            counters.preview_work_avoided_by_explicit_basis_count()
        ),
    ]
}

fn comparison_counter_digest_parts(
    counters: &PreviewComparisonCounters,
    label: &str,
) -> Vec<String> {
    vec![
        format!(
            "{label}_preview_promotion_comparison_count:{}",
            counters.preview_promotion_comparison_count()
        ),
        format!(
            "{label}_preview_promotion_comparison_denial_count:{}",
            counters.preview_promotion_comparison_denial_count()
        ),
        format!(
            "{label}_preview_comparison_eligibility_proof_count:{}",
            counters.preview_comparison_eligibility_proof_count()
        ),
        format!(
            "{label}_preview_comparison_shape_check_width:{}",
            counters.preview_comparison_shape_check_width()
        ),
        format!(
            "{label}_preview_basis_pair_width:{}",
            counters.preview_basis_pair_width()
        ),
    ]
}

fn counter_digest_parts(counters: &PreviewBindingCounters, label: &str) -> Vec<String> {
    vec![
        format!(
            "{label}_preview_session_admission_count:{}",
            counters.preview_session_admission_count()
        ),
        format!(
            "{label}_preview_basis_resolution_count:{}",
            counters.preview_basis_resolution_count()
        ),
        format!(
            "{label}_preview_lifecycle_lookup_count:{}",
            counters.preview_lifecycle_lookup_count()
        ),
        format!(
            "{label}_preview_lifecycle_rediscovery_count:{}",
            counters.preview_lifecycle_rediscovery_count()
        ),
        format!(
            "{label}_preview_invalid_basis_denial_count:{}",
            counters.preview_invalid_basis_denial_count()
        ),
        format!(
            "{label}_preview_invalid_lifecycle_denial_count:{}",
            counters.preview_invalid_lifecycle_denial_count()
        ),
        format!(
            "{label}_preview_broad_fallback_denial_count:{}",
            counters.preview_broad_fallback_denial_count()
        ),
        format!(
            "{label}_preview_executor_rediscovery_count:{}",
            counters.preview_executor_rediscovery_count()
        ),
        format!(
            "{label}_preview_replay_bundle_lookup_count:{}",
            counters.preview_replay_bundle_lookup_count()
        ),
        format!(
            "{label}_preview_bridge_promotion_linkage_count:{}",
            counters.preview_bridge_promotion_linkage_count()
        ),
    ]
}

fn preview_live_counter_digest_parts(counters: &PreviewLiveCounters, label: &str) -> Vec<String> {
    vec![
        format!(
            "{label}_preview_live_admission_count:{}",
            counters.preview_live_admission_count()
        ),
        format!(
            "{label}_preview_live_execution_count:{}",
            counters.preview_live_execution_count()
        ),
        format!(
            "{label}_preview_live_lifecycle_check_count:{}",
            counters.preview_live_lifecycle_check_count()
        ),
        format!(
            "{label}_preview_live_drift_denial_count:{}",
            counters.preview_live_drift_denial_count()
        ),
        format!(
            "{label}_preview_live_rebind_available_count:{}",
            counters.preview_live_rebind_available_count()
        ),
        format!(
            "{label}_preview_live_broad_fallback_denial_count:{}",
            counters.preview_live_broad_fallback_denial_count()
        ),
    ]
}
