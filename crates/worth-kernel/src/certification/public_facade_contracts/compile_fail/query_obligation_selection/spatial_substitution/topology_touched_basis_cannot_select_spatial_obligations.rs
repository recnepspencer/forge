use worth_kernel::query_obligation_selection::selection_substrate::QueryObligationSelectionInput;
use topology::facade::TopologyPrimitiveConstructionBirthDeclaredTouchedBasis;

fn topology_basis() -> TopologyPrimitiveConstructionBirthDeclaredTouchedBasis {
    todo!("compile-fail fixture only needs the type boundary")
}

fn main() {
    let _ = QueryObligationSelectionInput::from_spatial_query_descriptor(&topology_basis());
}
