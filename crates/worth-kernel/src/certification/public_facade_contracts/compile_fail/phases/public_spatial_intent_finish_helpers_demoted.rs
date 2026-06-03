use worth_kernel::facade::authoring::intents::MoveSpatialIntent;
use worth_kernel::facade::authoring::construction::{PrimitiveConstructionIntent, WireBodySpec};

fn main() {
    let _ = MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
        edge_count: 6,
    }))
    .to([1.0, 2.0, 3.0])
    .finish();
}
