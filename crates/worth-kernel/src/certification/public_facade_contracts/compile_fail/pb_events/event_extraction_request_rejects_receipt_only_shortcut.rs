use worth_kernel::workload_composition::PlanarBooleanEventExtractionRequest;
use worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlaneReducedOperandPairReceipt;

fn main() {
    let receipt: PlanarBooleanCommonPlaneReducedOperandPairReceipt = todo!();
    let _ = PlanarBooleanEventExtractionRequest::from_reduced_operand_pair_receipt(receipt);
}
