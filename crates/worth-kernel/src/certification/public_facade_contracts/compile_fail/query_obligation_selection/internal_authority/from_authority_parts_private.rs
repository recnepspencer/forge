use forge_query::facade::runtime::{
    ForgeQueryGraphObligationOperatingWorldDescriptor, ForgeQueryGraphTouchDescriptor,
};
use worth_kernel::query_obligation_selection::selection_substrate::{
    QueryObligationSelectionAuthorityKind, QueryObligationSelectionInput,
};

fn main() {
    let descriptor =
        ForgeQueryGraphTouchDescriptor::read_family("fake.collection", []).unwrap();
    let operating_world =
        ForgeQueryGraphObligationOperatingWorldDescriptor::any_committed_authority();
    let _ = QueryObligationSelectionInput::from_authority_parts(
        descriptor,
        operating_world,
        "copied-authority",
        QueryObligationSelectionAuthorityKind::SpatialQueryDescriptor,
    );
}
