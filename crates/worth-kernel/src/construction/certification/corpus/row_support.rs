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
    let counts = result.evidence().birth_completeness_report();
    Ok(counts.supported_vertex_count()
        + counts.supported_edge_count()
        + counts.supported_loop_count()
        + counts.supported_wire_count()
        + counts.supported_face_count()
        + counts.supported_shell_count()
        + counts.supported_body_count())
}

pub(super) fn birth_attachment_breadth(result: &PreparedPrimitiveConstructionResult) -> usize {
    result
        .evidence()
        .birth_mapping_report()
        .rows()
        .iter()
        .map(|row| row.mapped_count())
        .sum()
}

pub(super) fn certification_breadth(result: &PreparedPrimitiveConstructionResult) -> usize {
    result
        .evidence()
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
    let report = prepare_primitive_construction_rejection_locality_report(vec![request]);
    match report.rows() {
        [row] => Ok(row.clone()),
        [] => Err("rejected siege case did not produce a locality row".to_string()),
        _ => Err("single rejected siege case produced multiple locality rows".to_string()),
    }
}
