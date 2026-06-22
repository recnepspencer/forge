use worth_kernel::workload_composition::WorthWorkload;
use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanEdgeSplitRequest;

fn main() {
    let workload: WorthWorkload = todo!();
    let split_request: PlanarBooleanEdgeSplitRequest = todo!();
    let _ = workload.require_boolean_split(&split_request);
}
