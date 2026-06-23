use worth_kernel::query_obligation_selection::selection_substrate::QueryObligationSelectionInput;
use worth_spatial::facade::workload_vocabulary::SpatialEvidenceQueryTouchDescriptor;

fn main() {
    let _:
        fn(
            QueryObligationSelectionInput,
            SpatialEvidenceQueryTouchDescriptor,
        ) -> QueryObligationSelectionInput =
        QueryObligationSelectionInput::with_spatial_descriptor;
}
