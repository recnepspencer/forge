use forge_query::facade::runtime::{
    ForgeQueryGraphObligationOperatingWorldDescriptor, ForgeQueryGraphTouchDescriptor,
};
use worth_spatial::facade::workload_vocabulary::SpatialEvidenceQueryTouchDescriptor;

fn main() {
    let _ = SpatialEvidenceQueryTouchDescriptor {
        touch_descriptor: ForgeQueryGraphTouchDescriptor::read_family("worth.spatial.fake", [])
            .unwrap(),
        operating_world: ForgeQueryGraphObligationOperatingWorldDescriptor::any_committed_authority(
        ),
        product_digest: todo!(),
        spatial_touch_digest: todo!(),
        lookup_product_digest: todo!(),
        collection: "worth.spatial.fake".to_string(),
        relation_kind: "fake".to_string(),
        aspect_paths: vec![],
        read_verbs: vec![],
        gap_rows: vec![],
        milestone_five_selection_claimed: true,
        counters: todo!(),
    };
}
