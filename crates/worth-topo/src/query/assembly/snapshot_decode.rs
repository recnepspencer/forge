use serde_json::Value;

use super::{WorthTopologyQuerySnapshot, WorthTopologyQuerySurfaceError};
use crate::facade::{
    DerivedTopologyValidationReport, InterpretedTopologyView, MaterializedTopologyView,
    WorthDerivedEquivalenceContractReport, WorthDerivedReadDiagnostics,
    WorthNamingAttachmentReport,
};
use crate::query::{
    derived::decode_single_computed_row, equivalence_contract_from_diagnostics_rows,
};

pub(super) fn snapshot_from_query_rows(
    naming_attachments: WorthNamingAttachmentReport,
    materialized_rows: &[Value],
    interpreted_rows: &[Value],
    validation_rows: &[Value],
    diagnostics_rows: &[Value],
    equivalence_rows: &[Value],
) -> Result<WorthTopologyQuerySnapshot, WorthTopologyQuerySurfaceError> {
    let materialized: MaterializedTopologyView =
        decode_single_computed_row(materialized_rows, "materialized topology")?;
    let interpreted: InterpretedTopologyView =
        decode_single_computed_row(interpreted_rows, "interpreted topology")?;
    let validation: DerivedTopologyValidationReport =
        decode_single_computed_row(validation_rows, "topology validation")?;
    let diagnostics: WorthDerivedReadDiagnostics =
        decode_single_computed_row(diagnostics_rows, "derived read diagnostics")?;
    let equivalence_contract = equivalence_contract_from_diagnostics_rows(diagnostics_rows)?;
    let decoded_equivalence: WorthDerivedEquivalenceContractReport =
        decode_single_computed_row(equivalence_rows, "derived equivalence contract")?;
    if equivalence_contract != decoded_equivalence {
        return Err(WorthTopologyQuerySurfaceError::new(
            "query-derived equivalence contract row and diagnostics-carried equivalence contract diverged",
        ));
    }
    Ok(WorthTopologyQuerySnapshot {
        naming_attachments,
        materialized,
        interpreted,
        validation,
        diagnostics,
        equivalence_contract: decoded_equivalence,
    })
}
