use worth_kernel::facade::authoring::construction::{
    prepare_primitive_construction_move_witness_resolution_artifact,
    PrimitiveConstructionMotionWitnessResolutionArtifact,
};

fn main() {
    let _ = prepare_primitive_construction_move_witness_resolution_artifact;
    let _ = std::mem::size_of::<PrimitiveConstructionMotionWitnessResolutionArtifact>();
}
