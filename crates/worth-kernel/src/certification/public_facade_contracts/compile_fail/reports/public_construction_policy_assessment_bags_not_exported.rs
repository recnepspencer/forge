use worth_kernel::facade::authoring::construction::{
    PrimitiveConstructionArbitrationPolicy, PrimitiveConstructionPolicyPressure,
    PrimitiveConstructionPolicyProfile,
};

fn main() {
    let _ = core::mem::size_of::<PrimitiveConstructionArbitrationPolicy>();
    let _ = core::mem::size_of::<PrimitiveConstructionPolicyPressure>();
    let _ = core::mem::size_of::<PrimitiveConstructionPolicyProfile>();
}
