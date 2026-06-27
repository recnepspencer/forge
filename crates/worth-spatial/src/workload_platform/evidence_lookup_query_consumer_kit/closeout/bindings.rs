use crate::workload_platform::evidence_lookup_inventory::EvidenceLookupQuerySurface;
use crate::workload_platform::evidence_lookup_query_surface_matrix::EvidenceLookupQuerySurfaceMatrixCloseout;

use super::super::row::EvidenceLookupQueryConsumerKitBindingRow;

pub(super) fn binding_rows_from_matrix(
    matrix: &EvidenceLookupQuerySurfaceMatrixCloseout,
    support_pin_report_digest: &str,
) -> Vec<EvidenceLookupQueryConsumerKitBindingRow> {
    matrix
        .rows()
        .iter()
        .filter(|row| row.query_surface() != EvidenceLookupQuerySurface::NotQuery)
        .map(|row| {
            EvidenceLookupQueryConsumerKitBindingRow::from_matrix_row(
                row.family_identity(),
                row.stage(),
                row.touchpoint(),
                row.query_surface(),
                row.row_digest(),
                row.proof_digest(),
                (row.query_surface() == EvidenceLookupQuerySurface::SupportPinning)
                    .then_some(support_pin_report_digest),
            )
        })
        .collect()
}
