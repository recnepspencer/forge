use super::model::{
    PlanningCertificationBundle, PlanningCertificationMatrix, PlanningCertificationRow,
    PlanningRejectionBundle, PlanningRejectionRow,
};

pub(super) fn bundle_digest_parts(matrix: &PlanningCertificationMatrix) -> Vec<String> {
    let mut parts = vec![format!("suite:{}", matrix.suite_name)];
    for row in &matrix.rows {
        parts.extend(row_digest_parts(row));
    }
    for row in &matrix.rejection_rows {
        parts.extend(rejection_row_digest_parts(row));
    }
    parts
}

pub(super) fn coverage_digest_parts(matrix: &PlanningCertificationMatrix) -> Vec<String> {
    let mut parts = vec![
        format!("canonical-rows:{}", matrix.rows.len()),
        format!("rejection-rows:{}", matrix.rejection_rows.len()),
    ];
    for row in &matrix.rows {
        parts.push(format!("canonical:{}", row.row_name));
    }
    for row in &matrix.rejection_rows {
        parts.push(format!("rejection:{}", row.row_name));
    }
    parts
}

fn row_digest_parts(row: &PlanningCertificationRow) -> Vec<String> {
    let mut parts = vec![
        format!("row:{}", row.row_name),
        format!("perturbation:{:?}", row.perturbation_class),
        format!("hostile:{:?}", row.hostile_expectation),
    ];
    parts.extend(bundle_parts(&row.control_lane, "control"));
    parts.extend(bundle_parts(&row.hostile_lane, "hostile"));
    parts.extend(bundle_parts(&row.parity_lane, "parity"));
    parts
}

fn rejection_row_digest_parts(row: &PlanningRejectionRow) -> Vec<String> {
    let mut parts = vec![
        format!("row:{}", row.row_name),
        format!("perturbation:{:?}", row.perturbation_class),
    ];
    parts.extend(bundle_parts(&row.control_lane, "control"));
    parts.extend(rejection_bundle_parts(&row.hostile_lane, "hostile"));
    parts.extend(bundle_parts(&row.parity_lane, "parity"));
    parts
}

fn bundle_parts(bundle: &PlanningCertificationBundle, lane: &str) -> Vec<String> {
    vec![
        format!("lane:{lane}"),
        format!("profile:{:?}", bundle.profile),
        format!("query:{}", bundle.query_digest),
        format!("plan:{}", bundle.plan_digest),
        format!("result:{}", bundle.result_digest),
        format!("basis:{}", bundle.basis_digest),
        format!(
            "reads:{}",
            bundle.counter_snapshot.execution_read_operation_count()
        ),
        format!(
            "emitted:{}",
            bundle.counter_snapshot.execution_records_emitted_count()
        ),
    ]
}

fn rejection_bundle_parts(bundle: &PlanningRejectionBundle, lane: &str) -> Vec<String> {
    vec![
        format!("lane:{lane}"),
        format!("profile:{:?}", bundle.profile),
        format!("failure-class:{}", bundle.failure_class),
        format!("failure-digest:{}", bundle.failure_digest),
    ]
}
