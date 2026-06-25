use forge_query::facade::runtime::{
    ForgeQueryGraphObligationOperatingWorldDescriptor, ForgeQueryGraphTouchDescriptor,
};
use worth_kernel::query_obligation_selection::selection_substrate::{
    QueryObligationSelectionAuthorityKind, QueryObligationSelectionInput,
};

fn main() {
    let _ = QueryObligationSelectionInput {
        touch_descriptor: ForgeQueryGraphTouchDescriptor::read_family("fake.collection", [])
            .unwrap(),
        operating_world: ForgeQueryGraphObligationOperatingWorldDescriptor::any_committed_authority(
        ),
        authority_digest: "copied-authority".to_string(),
        authority_kind: QueryObligationSelectionAuthorityKind::SpatialQueryDescriptor,
        spatial_descriptor: None,
    };
}
