use worth_kernel::query_obligation_selection::selection_substrate::{
    QueryObligationSelectionInput, QueryObligationSelectionSubstrate,
};
use worth_spatial::facade::workload_vocabulary::SpatialEvidenceLookupProduct;

fn lookup_product() -> SpatialEvidenceLookupProduct {
    todo!("compile-fail fixture only needs the type boundary")
}

fn main() {
    let input: QueryObligationSelectionInput = lookup_product();
    let _ = QueryObligationSelectionSubstrate::select_execution_backed_obligations(input);
}
