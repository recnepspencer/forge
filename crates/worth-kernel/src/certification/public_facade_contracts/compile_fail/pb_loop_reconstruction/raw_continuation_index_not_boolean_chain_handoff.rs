use worth_kernel::workload_composition::BooleanChainIntegrationHandoff;
use worth_spatial::facade::planar_boolean_loop_reconstruction::PlanarBooleanFragmentContinuationIndex;

fn require_boolean_chain(_: &BooleanChainIntegrationHandoff) {}

fn main() {
    let continuation_index: PlanarBooleanFragmentContinuationIndex = panic!();
    require_boolean_chain(&continuation_index);
}
