use worth_kernel::facade::authoring::{
    MoveSpatialIntent, PrimitiveConstructionIntent, SpatialIntentPolicyProfile,
};

fn main() {
    let _ = MoveSpatialIntent::shape("shape-1");
    let _ = PrimitiveConstructionIntent::wire_body;
    let _ = SpatialIntentPolicyProfile::aggressive_snap;
}
