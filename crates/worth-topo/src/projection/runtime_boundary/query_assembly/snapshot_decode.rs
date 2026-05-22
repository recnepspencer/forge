use super::snapshot_rows::TopologyQuerySnapshotRows;
use super::{TopologyQuerySnapshot, TopologyQuerySurfaceError};
use crate::facade::{
    DerivedEquivalenceContractReport, DerivedReadDiagnostics, DerivedTopologyValidationReport,
    InterpretedTopologyView, MaterializedTopologyView,
};
use crate::projection::{decode_single_computed_row, equivalence_contract_from_diagnostics_rows};

pub(super) fn snapshot_from_query_rows(
    rows: TopologyQuerySnapshotRows,
) -> Result<TopologyQuerySnapshot, TopologyQuerySurfaceError> {
    let materialized: MaterializedTopologyView =
        decode_single_computed_row(&rows.materialized_rows, "materialized topology")?;
    let interpreted: InterpretedTopologyView =
        decode_single_computed_row(&rows.interpreted_rows, "interpreted topology")?;
    let validation: DerivedTopologyValidationReport =
        decode_single_computed_row(&rows.validation_rows, "topology validation")?;
    let diagnostics: DerivedReadDiagnostics =
        decode_single_computed_row(&rows.diagnostics_rows, "derived read diagnostics")?;
    let equivalence_contract = equivalence_contract_from_diagnostics_rows(&rows.diagnostics_rows)?;
    let decoded_equivalence: DerivedEquivalenceContractReport =
        decode_single_computed_row(&rows.equivalence_rows, "derived equivalence contract")?;
    if equivalence_contract != decoded_equivalence {
        return Err(TopologyQuerySurfaceError::new(
            "query-derived equivalence contract row and diagnostics-carried equivalence contract diverged",
        ));
    }
    Ok(TopologyQuerySnapshot {
        naming_attachments: rows.naming_attachments,
        materialized,
        interpreted,
        validation,
        diagnostics,
        equivalence_contract: decoded_equivalence,
    })
}
