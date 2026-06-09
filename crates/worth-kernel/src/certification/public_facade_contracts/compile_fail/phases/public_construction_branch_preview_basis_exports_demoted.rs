use worth_kernel::facade::authoring::construction::{
    prepare_primitive_construction_branch_preview_basis_artifact,
    PrimitiveConstructionBasisAdmissionLaneReceipt, PrimitiveConstructionBranchPreviewBasisArtifact,
};

fn main() {
    let _ = prepare_primitive_construction_branch_preview_basis_artifact;
    let _ = std::mem::size_of::<PrimitiveConstructionBasisAdmissionLaneReceipt>();
    let _ = std::mem::size_of::<PrimitiveConstructionBranchPreviewBasisArtifact>();
}
