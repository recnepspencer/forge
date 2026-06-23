use forge_query::facade::consumer_kit::ForgeQueryGraphObligationInMemoryProof;
use worth_kernel::query_obligation_selection::selection_substrate::{
    QueryObligationSelectionInput, QuerySelectedGraphObligations,
};

fn main() {
    let _:
        fn(
            QueryObligationSelectionInput,
            ForgeQueryGraphObligationInMemoryProof,
        ) -> QuerySelectedGraphObligations =
        QuerySelectedGraphObligations::from_query_proof;
}
