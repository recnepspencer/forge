use worth_kernel::query_obligation_selection::selection_substrate::{
    QueryObligationSelectionInput, QueryObligationSelectionSubstrate,
};

fn raw_spatial_row() -> &'static str {
    "copied spatial evidence row"
}

fn main() {
    let input: QueryObligationSelectionInput = raw_spatial_row();
    let _ = QueryObligationSelectionSubstrate::select_execution_backed_obligations(input);
}
