use worth_kernel::facade::authoring::construction::{
    PrimitiveConstructionRuntimeBasisError, PrimitiveConstructionRuntimeBasisLaneReport,
};

fn main() {
    let _ = std::mem::size_of::<PrimitiveConstructionRuntimeBasisLaneReport>();
    let _ = std::mem::size_of::<PrimitiveConstructionRuntimeBasisError>();
}
