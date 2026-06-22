use worth_kernel::workload_composition::{
    PlanarBooleanDeclarationReceipt, PlanarBooleanExecutionLane, PlanarBooleanFamily,
    PlanarBooleanOperandPairIdentity, PlanarBooleanOperation,
};

fn main() {
    let _ = PlanarBooleanDeclarationReceipt::new(
        PlanarBooleanFamily::PlanarRegions,
        PlanarBooleanOperation::Union,
        PlanarBooleanOperandPairIdentity::new("pair").unwrap(),
        PlanarBooleanExecutionLane::BRepNow,
        None,
        "query".to_string(),
    );
}
