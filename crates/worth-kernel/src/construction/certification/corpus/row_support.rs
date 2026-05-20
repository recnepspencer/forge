use crate::construction::diagnostics::{
    prepare_primitive_construction_rejection_locality_report,
    PrimitiveConstructionRejectionLocalityRow,
};
use crate::construction::request::PrimitiveConstructionRequest;
use crate::construction::result::PreparedPrimitiveConstructionResult;

pub(super) fn construction_breadth(
    request: &PrimitiveConstructionRequest,
) -> Result<usize, String> {
    let intent = request.clone().admit().map_err(|error| error.to_string())?;
    let scaffold = intent.build_scaffold().map_err(|error| error.to_string())?;
    let counts = scaffold.topology_counts();
    Ok(counts.vertex_count()
        + counts.edge_count()
        + counts.loop_count()
        + counts.wire_count()
        + counts.face_count()
        + counts.shell_count()
        + counts.body_count())
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
        .topology_fact_report()
        .rows()
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
