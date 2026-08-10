use super::{LiveCertificationBundle, LiveCertificationMatrix, LiveRejectionBundle};

pub(super) fn bundle_digest_parts(matrix: &LiveCertificationMatrix) -> Vec<String> {
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

pub(super) fn coverage_digest_parts(matrix: &LiveCertificationMatrix) -> Vec<String> {
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

fn lane_digest_parts(bundle: &LiveCertificationBundle, label: &str) -> Vec<String> {
    let mut parts = vec![
        format!("{label}_profile:{:?}", bundle.profile),
        format!("{label}_query_digest:{}", bundle.query_digest),
        format!("{label}_result_digest:{}", bundle.result_digest),
        format!("{label}_delivery_digest:{}", bundle.delivery_digest),
        format!("{label}_replay_digest:{}", bundle.replay_digest),
        format!(
            "{label}_replay_step_count:{}",
            bundle.replay_step_delivery_digests.len()
        ),
        format!("{label}_family:{}", bundle.family.as_str()),
        format!("{label}_outcome_kind:{}", bundle.outcome_kind.as_str()),
        format!("{label}_outcome_digest:{}", bundle.outcome_digest),
        format!("{label}_basis:{}", bundle.basis_digest),
        format!("{label}_subscription:{}", bundle.subscription_digest),
    ];
    parts.extend(
        bundle
            .replay_step_delivery_digests
            .iter()
            .map(|digest| format!("{label}_replay_step_delivery:{digest}")),
    );
    parts.extend(bundle.counter_snapshot.digest_parts(label));
    parts
}

fn rejection_digest_parts(bundle: &LiveRejectionBundle, label: &str) -> Vec<String> {
    let mut parts = vec![
        format!("{label}_profile:{:?}", bundle.profile),
        format!("{label}_failure_class:{}", bundle.failure_class.as_str()),
        format!("{label}_failure_digest:{}", bundle.failure_digest),
    ];
    parts.extend(bundle.counter_snapshot.digest_parts(label));
    parts
}
