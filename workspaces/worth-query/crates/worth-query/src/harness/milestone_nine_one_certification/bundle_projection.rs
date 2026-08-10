use super::{
    MilestoneNineOneCertificationMatrix, MilestoneNineOneCertificationRow,
    MilestoneNineOneRejectionRow, MILESTONE_NINE_ONE_REQUIRED_COMPILE_FAIL_TARGETS,
};
use crate::harness::certification::digest_parts;

pub(super) fn bundle_digest_parts(matrix: &MilestoneNineOneCertificationMatrix) -> Vec<String> {
    matrix
        .rows
        .iter()
        .flat_map(|row: &MilestoneNineOneCertificationRow| {
            [
                format!(
                    "{}:control:{}",
                    row.row_name, row.control_lane.certification_bundle_digest
                ),
                format!(
                    "{}:hostile:{}",
                    row.row_name, row.hostile_lane.certification_bundle_digest
                ),
                format!(
                    "{}:parity:{}",
                    row.row_name, row.parity_lane.certification_bundle_digest
                ),
            ]
        })
        .chain(
            matrix
                .rejection_rows
                .iter()
                .flat_map(|row: &MilestoneNineOneRejectionRow| {
                    [
                        format!(
                            "{}:control:{}",
                            row.row_name, row.control_lane.certification_bundle_digest
                        ),
                        format!(
                            "{}:hostile:{}",
                            row.row_name, row.hostile_lane.failure_digest
                        ),
                        format!(
                            "{}:parity:{}",
                            row.row_name, row.parity_lane.certification_bundle_digest
                        ),
                    ]
                }),
        )
        .collect()
}

pub(super) fn coverage_digest_parts(matrix: &MilestoneNineOneCertificationMatrix) -> Vec<String> {
    matrix
        .rows
        .iter()
        .map(|row| {
            format!(
                "canonical:{}:{:?}:{:?}:{:?}",
                row.row_name, row.perturbation_class, row.hostile_expectation, row.parity_anchor
            )
        })
        .chain(
            matrix
                .rejection_rows
                .iter()
                .map(|row| format!("rejection:{}:{:?}", row.row_name, row.perturbation_class)),
        )
        .collect()
}

pub(super) fn compile_fail_boundary_digest() -> String {
    let mut parts = MILESTONE_NINE_ONE_REQUIRED_COMPILE_FAIL_TARGETS
        .iter()
        .flat_map(|target| {
            [
                format!("target:{target}"),
                format!(
                    "stderr:{}",
                    target.trim_end_matches(".rs").to_string() + ".stderr"
                ),
            ]
        })
        .collect::<Vec<_>>();
    parts.sort();
    digest_parts(&parts)
}
