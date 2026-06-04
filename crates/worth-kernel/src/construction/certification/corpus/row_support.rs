use crate::construction::diagnostics::{
    prepare_primitive_construction_rejection_locality_report,
    PrimitiveConstructionRejectionLocalityRow,
};
use crate::construction::request::PrimitiveConstructionRequest;
use crate::construction::result::{
    prepare_primitive_construction_result, PreparedPrimitiveConstructionResult,
};

pub(super) fn construction_breadth(
    request: &PrimitiveConstructionRequest,
) -> Result<usize, String> {
    let result = prepare_primitive_construction_result(request.clone())
        .map_err(|error| error.to_string())?;
    Ok(result
        .birth_consequence()
        .rows()
        .iter()
        .map(|row| row.mapped_count())
        .sum())
}

pub(super) fn birth_attachment_breadth(result: &PreparedPrimitiveConstructionResult) -> usize {
    result
        .birth_consequence()
        .rows()
        .iter()
        .map(|row| row.mapped_count())
        .sum()
}

pub(super) fn certification_breadth(result: &PreparedPrimitiveConstructionResult) -> usize {
    result
        .topology_query_handoff()
        .topology_query_envelope()
        .fact_rows()
        .iter()
        .map(|row| row.fact_count())
        .sum()
}

pub(super) fn rejection_locality_row_for(
    request: PrimitiveConstructionRequest,
) -> Result<PrimitiveConstructionRejectionLocalityRow, String> {
    let report = prepare_primitive_construction_rejection_locality_report(vec![request.into()]);
    match report.rows() {
        [row] => Ok(row.clone()),
        [] => Err("rejected siege case did not produce a locality row".to_string()),
        _ => Err("single rejected siege case produced multiple locality rows".to_string()),
    }
}
