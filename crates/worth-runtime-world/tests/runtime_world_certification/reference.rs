use worth_runtime_world::facade::ProductBranchObservationMismatchAxis;

#[test]
fn reference_comparison_vocabulary_keeps_each_dynamic_axis_distinct() {
    let axes = [
        ProductBranchObservationMismatchAxis::OwnerIdentity,
        ProductBranchObservationMismatchAxis::BranchIdentity,
        ProductBranchObservationMismatchAxis::LifecycleIncarnation,
        ProductBranchObservationMismatchAxis::ReferenceGeneration,
        ProductBranchObservationMismatchAxis::SelectedCompositeCommit,
        ProductBranchObservationMismatchAxis::CompositeBasis,
    ];

    for (index, axis) in axes.iter().enumerate() {
        assert!(axes[..index].iter().all(|prior| prior != axis));
    }
}
