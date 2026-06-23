use forge_query::facade::consumer_kit::ForgeQueryGraphObligationExecutionBackedAdoptionProof;
use worth_kernel::query_obligation_selection::selection_substrate::{
    QueryObligationSelectionInput, QuerySelectedGraphObligations,
};

fn main() {
    let _:
        fn(
            QueryObligationSelectionInput,
            ForgeQueryGraphObligationExecutionBackedAdoptionProof,
        ) -> QuerySelectedGraphObligations =
        QuerySelectedGraphObligations::from_query_proof;
}
