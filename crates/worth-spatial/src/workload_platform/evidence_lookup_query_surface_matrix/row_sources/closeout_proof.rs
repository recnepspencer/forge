use crate::workload_platform::evidence_lookup_family_catalog::EvidenceLookupFamilyDeclaration;

use super::rows_for_touchpoint;
use crate::workload_platform::evidence_lookup_query_surface_matrix::row::{
    EvidenceLookupQuerySurfaceMatrixRow, EvidenceLookupQuerySurfaceTouchpoint,
};

pub(super) fn rows(
    declarations: &[EvidenceLookupFamilyDeclaration],
) -> Vec<EvidenceLookupQuerySurfaceMatrixRow> {
    rows_for_touchpoint(
        declarations,
        EvidenceLookupQuerySurfaceTouchpoint::PublicCloseoutProof,
    )
}
