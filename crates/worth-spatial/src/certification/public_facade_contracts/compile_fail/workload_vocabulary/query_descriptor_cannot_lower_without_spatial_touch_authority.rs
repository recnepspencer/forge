use forge_query::facade::runtime::{
    ForgeQueryGraphTouchDescriptor, ForgeQueryGraphTouchReadVerb,
};
use worth_spatial::facade::workload_vocabulary::lower_spatial_touch_authority_to_query_descriptor;

fn main() {
    let descriptor = ForgeQueryGraphTouchDescriptor::read_family(
        "worth.spatial.evidence_touch",
        [ForgeQueryGraphTouchReadVerb::ObservesCollection],
    )
    .unwrap();

    let _ = lower_spatial_touch_authority_to_query_descriptor(&descriptor, &descriptor);
}
