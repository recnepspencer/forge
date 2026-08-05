use super::*;

pub(in crate::harness::milestone_eight_certification) fn coverage_digest_parts(
    matrix: &MilestoneEightCertificationMatrix,
) -> Vec<String> {
    let mut parts = vec![format!("suite:{}", matrix.suite_name)];
    parts.extend(
        matrix
            .rows
            .iter()
            .map(|row| format!("row:{}", row.row_name)),
    );
    parts.extend(
        matrix
            .rejection_rows
            .iter()
            .map(|row| format!("rejection:{}", row.row_name)),
    );
    parts
}
