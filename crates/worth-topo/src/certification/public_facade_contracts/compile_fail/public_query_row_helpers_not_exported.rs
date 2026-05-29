use topology::facade::{
    derived_read_diagnostics_from_query_rows, equivalence_contract_from_diagnostics_rows,
    interpreted_topology_from_materialized_rows, naming_attachment_report_from_query_rows,
    validation_report_from_query_rows,
};

fn main() {
    let _ = derived_read_diagnostics_from_query_rows;
    let _ = equivalence_contract_from_diagnostics_rows;
    let _ = interpreted_topology_from_materialized_rows;
    let _ = naming_attachment_report_from_query_rows;
    let _ = validation_report_from_query_rows;
}




