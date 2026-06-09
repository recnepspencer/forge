use worth_kernel::facade::authoring::construction::{PrimitiveConstructionIntent, WireBodySpec};
use worth_kernel::facade::authoring::intents::MoveSpatialIntent;
use worth_spatial::facade::refs::SpatialWitnessCatalog;

fn demo(catalog: &impl SpatialWitnessCatalog) {
    let _ = MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
        edge_count: 6,
    }))
    .to([1.0, 2.0, 3.0])
    .finish_with_catalog(catalog);
}

fn main() {}
