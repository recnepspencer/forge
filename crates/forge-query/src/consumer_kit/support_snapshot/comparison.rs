use crate::runtime::ForgeQueryRuntimePublicSupportMatrix;

use super::error::{ForgeQuerySupportSnapshotError, ForgeQuerySupportSnapshotErrorKind};
use super::snapshot::ForgeQuerySupportSnapshot;

impl ForgeQuerySupportSnapshot {
    pub fn assert_equivalent_to_live_matrix(
        &self,
        matrix: &ForgeQueryRuntimePublicSupportMatrix,
    ) -> Result<(), ForgeQuerySupportSnapshotError> {
        let matrix_digest = matrix
            .matrix_digest()
            .terminal_projection_for_reporting()
            .to_string();
        if self.source_matrix_digest() != matrix_digest {
            return Err(ForgeQuerySupportSnapshotError::with_expected_found(
                ForgeQuerySupportSnapshotErrorKind::SourceMatrixDigestMismatch,
                format!(
                    "support snapshot source matrix digest mismatch: expected {matrix_digest}, found {}",
                    self.source_matrix_digest()
                ),
                matrix_digest,
                self.source_matrix_digest().to_string(),
            ));
        }
        if self.rows().len() != matrix.rows().len() {
            return Err(ForgeQuerySupportSnapshotError::new(
                ForgeQuerySupportSnapshotErrorKind::RowCountMismatch,
                format!(
                    "support snapshot row count mismatch: expected {}, found {}",
                    matrix.rows().len(),
                    self.rows().len()
                ),
            ));
        }
        for (index, (snapshot_row, matrix_row)) in
            self.rows().iter().zip(matrix.rows().iter()).enumerate()
        {
            let expected_family = matrix_row.facade_family().map(|family| family.as_str());
            let expected_digest = matrix_row.row_digest().terminal_projection_for_reporting();
            compare_row_field(
                index,
                "surface",
                matrix_row.surface(),
                matrix_row.surface(),
                snapshot_row.surface(),
            )?;
            compare_row_field(
                index,
                "facade_family",
                matrix_row.surface(),
                expected_family.unwrap_or("matrix-only"),
                snapshot_row.facade_family().unwrap_or("matrix-only"),
            )?;
            compare_row_field(
                index,
                "status",
                matrix_row.surface(),
                matrix_row.status().as_str(),
                snapshot_row.status(),
            )?;
            compare_row_field(
                index,
                "teaching_posture",
                matrix_row.surface(),
                matrix_row.teaching_posture().as_str(),
                snapshot_row.teaching_posture(),
            )?;
            compare_row_field(
                index,
                "owner_milestone",
                matrix_row.surface(),
                matrix_row.owner_milestone(),
                snapshot_row.owner_milestone(),
            )?;
            compare_row_field(
                index,
                "extension_rule",
                matrix_row.surface(),
                matrix_row.extension_rule(),
                snapshot_row.extension_rule(),
            )?;
            compare_row_field(
                index,
                "parallel_api_forbidden",
                matrix_row.surface(),
                bool_label(matrix_row.parallel_api_forbidden()),
                bool_label(snapshot_row.parallel_api_forbidden()),
            )?;
            compare_row_field(
                index,
                "admission_fail_closed",
                matrix_row.surface(),
                bool_label(matrix_row.admission_fail_closed()),
                bool_label(snapshot_row.admission_fail_closed()),
            )?;
            compare_row_field(
                index,
                "support_contract_digest",
                matrix_row.surface(),
                matrix_row.support_contract_digest().unwrap_or("none"),
                snapshot_row.support_contract_digest().unwrap_or("none"),
            )?;
            compare_row_field(
                index,
                "live_row_digest",
                matrix_row.surface(),
                expected_digest,
                snapshot_row.live_row_digest(),
            )?;
        }
        Ok(())
    }
}

fn compare_row_field(
    row_index: usize,
    field: &'static str,
    surface: &str,
    expected: &str,
    found: &str,
) -> Result<(), ForgeQuerySupportSnapshotError> {
    if expected == found {
        Ok(())
    } else {
        Err(ForgeQuerySupportSnapshotError::with_row_field_mismatch(
            ForgeQuerySupportSnapshotErrorKind::RowMismatch,
            format!(
                "support snapshot row {row_index} field {field} mismatch: expected {expected}, found {found}"
            ),
            row_index,
            field,
            surface,
            expected,
            found,
        ))
    }
}

fn bool_label(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}
