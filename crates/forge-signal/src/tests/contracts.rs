use crate::facade::{
    DependencyGraphContract, RawPathComputeContract, StructuralStateBoundaryContract,
    TransactionRuntimeContract,
};

#[test]
fn boundary_contract_markers_are_public() {
    let _dep = DependencyGraphContract;
    let _structural = StructuralStateBoundaryContract;
    let _raw = RawPathComputeContract;
    let _txn = TransactionRuntimeContract;
}
