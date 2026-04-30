use crate::harness::certification::digest_parts;
use crate::runtime::ForgeQueryInspection;

use super::AspectApiFinalizationCertificationMatrix;

pub(super) fn bundle_digest_parts(
    matrix: &AspectApiFinalizationCertificationMatrix,
) -> Vec<String> {
    let mut parts = vec!["aspect_api_finalization_certification_bundle_v1".to_string()];
    parts.extend(matrix.rows.iter().flat_map(|row| {
        [
            format!("row:{}", row.row_name),
            format!("control:{}", row.control_lane.receipt_digest),
            format!("hostile:{}", row.hostile_lane.receipt_digest),
            format!("parity:{}", row.parity_lane.receipt_digest),
        ]
    }));
    parts.extend(matrix.rejection_rows.iter().flat_map(|row| {
        [
            format!("rejection:{}", row.row_name),
            format!("failure:{}", row.hostile_lane.failure_digest),
        ]
    }));
    parts
}

pub(super) fn coverage_digest_parts(
    matrix: &AspectApiFinalizationCertificationMatrix,
) -> Vec<String> {
    let mut parts = vec!["aspect_api_finalization_coverage_v1".to_string()];
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

pub(super) fn inspection_digest(inspection: &ForgeQueryInspection) -> String {
    match inspection {
        ForgeQueryInspection::WriteReceipt(inspection) => {
            inspection.inspection_digest().to_string()
        }
        ForgeQueryInspection::BatchWriteReceipt(inspection) => {
            inspection.inspection_digest().to_string()
        }
        other => panic!("expected mutation inspection, got {other:?}"),
    }
}

pub(super) fn touched_aspect_digest(paths: &[String]) -> String {
    digest_parts(
        &paths
            .iter()
            .map(|path| format!("aspect:{path}"))
            .collect::<Vec<_>>(),
    )
}
