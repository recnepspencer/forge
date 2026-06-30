use worth_kernel::workload_composition::{
    current_kernel_compiled_product_consumer_dependency_matrix,
    KernelCompiledProductConsumerClusterIdentity,
    KernelCompiledProductQueryBoundaryLane,
};

fn main() {
    let matrix = current_kernel_compiled_product_consumer_dependency_matrix().unwrap();
    let row = matrix
        .require_cluster(KernelCompiledProductConsumerClusterIdentity::ReplayUndoPublicCloseoutReadModel)
        .unwrap();
    let _: Option<KernelCompiledProductQueryBoundaryLane> = Some("projection-consumption");
    let _ = row.query_boundary_lane() == Some("projection-consumption");
}
