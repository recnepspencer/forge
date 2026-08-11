use crate::harness::live_certification::{
    LiveCertificationMatrix, LiveCertificationRow, LiveRejectionRow,
};

pub(super) fn canonical_row<'a>(
    matrix: &'a LiveCertificationMatrix,
    row_name: &str,
) -> &'a LiveCertificationRow {
    matrix
        .rows
        .iter()
        .find(|row| row.row_name == row_name)
        .unwrap_or_else(|| panic!("missing canonical row {row_name}"))
}

pub(super) fn rejection_row<'a>(
    matrix: &'a LiveCertificationMatrix,
    row_name: &str,
) -> &'a LiveRejectionRow {
    matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == row_name)
        .unwrap_or_else(|| panic!("missing rejection row {row_name}"))
}
