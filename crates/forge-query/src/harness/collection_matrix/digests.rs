use super::model::{
    CollectionCertificationBundle, CollectionCertificationMatrix, CollectionCertificationRow,
    CollectionRejectionBundle, CollectionRejectionRow,
};

fn bundle_parts(bundle: &CollectionCertificationBundle) -> Vec<String> {
    vec![
        format!("profile:{:?}", bundle.profile),
        format!("query:{}", bundle.query_digest),
        format!("plan:{}", bundle.plan_digest),
        format!("result:{}", bundle.result_digest),
        format!("basis:{}", bundle.basis_digest),
        format!("delivery:{}", bundle.delivery_digest),
        format!("cursor:{}", bundle.cursor_progress_report),
        format!(
            "counter:rediscovery:{}",
            bundle.counter_snapshot.executor_semantic_rediscovery_count()
        ),
    ]
}

fn rejection_parts(bundle: &CollectionRejectionBundle) -> Vec<String> {
    vec![
        format!("profile:{:?}", bundle.profile),
        format!("failure_class:{}", bundle.failure_class),
        format!("failure_digest:{}", bundle.failure_digest),
    ]
}

fn canonical_row_parts(row: &CollectionCertificationRow) -> Vec<String> {
    let mut parts = vec![
        format!("row:{}", row.row_name),
        format!("perturbation:{:?}", row.perturbation_class),
        format!("hostile_expectation:{:?}", row.hostile_expectation),
    ];
    parts.extend(bundle_parts(&row.control_lane));
    parts.extend(bundle_parts(&row.hostile_lane));
    parts.extend(bundle_parts(&row.parity_lane));
    parts
}

fn rejection_row_parts(row: &CollectionRejectionRow) -> Vec<String> {
    let mut parts = vec![
        format!("row:{}", row.row_name),
        format!("perturbation:{:?}", row.perturbation_class),
    ];
    parts.extend(bundle_parts(&row.control_lane));
    parts.extend(rejection_parts(&row.hostile_lane));
    parts.extend(bundle_parts(&row.parity_lane));
    parts
}

pub(super) fn bundle_digest_parts(matrix: &CollectionCertificationMatrix) -> Vec<String> {
    let mut parts = vec![format!("suite:{}", matrix.suite_name)];
    for row in &matrix.rows {
        parts.extend(canonical_row_parts(row));
    }
    for row in &matrix.rejection_rows {
        parts.extend(rejection_row_parts(row));
    }
    parts
}

pub(super) fn coverage_digest_parts(matrix: &CollectionCertificationMatrix) -> Vec<String> {
    let mut parts = vec![format!("suite:{}", matrix.suite_name)];
    parts.extend(matrix.rows.iter().map(|row| format!("canonical:{}", row.row_name)));
    parts.extend(
        matrix
            .rejection_rows
            .iter()
            .map(|row| format!("rejection:{}", row.row_name)),
    );
    parts
}
