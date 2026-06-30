use worth_kernel::workload_composition::{
    current_kernel_compiled_product_consumer_dependency_matrix,
    KernelCompiledProductConsumerClusterIdentity, KernelCompiledProductConsumerResponsibility,
    KernelCompiledProductFamilyClass, KernelCompiledProductQueryBoundaryLane,
};

#[test]
fn public_api_exposes_kernel_consumer_dependency_matrix_boundary() {
    let matrix = current_kernel_compiled_product_consumer_dependency_matrix()
        .expect("public workload-composition boundary should expose the kernel consumer matrix");

    let lookup = matrix
        .require_cluster(KernelCompiledProductConsumerClusterIdentity::LookupConsumedWorkload)
        .expect("lookup-consumed workload cluster should remain exported");
    assert_eq!(
        lookup.responsibility(),
        KernelCompiledProductConsumerResponsibility::SpatialEvidenceDerived
    );
    assert_eq!(
        lookup.family_class(),
        KernelCompiledProductFamilyClass::SpatialEvidenceLookupIndex
    );
    assert!(lookup.query_boundary_lane().is_none());
}

#[test]
fn public_api_exposes_query_backed_consumer_lanes_as_typed_boundary_values() {
    let matrix = current_kernel_compiled_product_consumer_dependency_matrix()
        .expect("public workload-composition boundary should expose query-backed kernel consumers");

    let projection = matrix
        .require_cluster(
            KernelCompiledProductConsumerClusterIdentity::ReplayUndoPublicCloseoutReadModel,
        )
        .expect("query projection cluster should remain exported");
    assert_eq!(
        projection.responsibility(),
        KernelCompiledProductConsumerResponsibility::QueryBacked
    );
    assert_eq!(
        projection.query_boundary_lane(),
        Some(KernelCompiledProductQueryBoundaryLane::ProjectionConsumption)
    );
}

#[test]
fn public_api_preserves_split_batch_execution_consumer_clusters() {
    let matrix = current_kernel_compiled_product_consumer_dependency_matrix().expect(
        "public workload-composition boundary should expose the split batch-execution clusters",
    );

    let lookup = matrix
        .require_cluster(KernelCompiledProductConsumerClusterIdentity::LookupConsumedBatchExecution)
        .expect("lookup batch-execution cluster should remain exported");
    assert_eq!(
        lookup.responsibility(),
        KernelCompiledProductConsumerResponsibility::SpatialEvidenceDerived
    );
    assert_eq!(
        lookup.family_class(),
        KernelCompiledProductFamilyClass::SpatialEvidenceLookupIndex
    );
    assert!(lookup.query_boundary_lane().is_none());

    let retained_replay = matrix
        .require_cluster(
            KernelCompiledProductConsumerClusterIdentity::RetainedReplayBatchExecutionCarryForward,
        )
        .expect("retained replay carry-forward cluster should remain exported");
    assert_eq!(
        retained_replay.responsibility(),
        KernelCompiledProductConsumerResponsibility::RetainedReplay
    );
    assert_eq!(
        retained_replay.family_class(),
        KernelCompiledProductFamilyClass::SpatialRetainedReplayWorkload
    );
    assert!(retained_replay.query_boundary_lane().is_none());
}

#[test]
fn public_api_exposes_public_closeout_and_boundary_traceability_clusters() {
    let matrix = current_kernel_compiled_product_consumer_dependency_matrix().expect(
        "public workload-composition boundary should expose public-closeout consumer clusters",
    );

    let closeout = matrix
        .require_cluster(KernelCompiledProductConsumerClusterIdentity::ConflictPublicCloseout)
        .expect("public closeout cluster should remain exported");
    assert_eq!(
        closeout.responsibility(),
        KernelCompiledProductConsumerResponsibility::PublicCloseout
    );
    assert_eq!(
        closeout.family_class(),
        KernelCompiledProductFamilyClass::KernelPublicCloseoutProofChain
    );

    let boundary = matrix
        .require_cluster(
            KernelCompiledProductConsumerClusterIdentity::KernelConflictPublicCloseoutBoundaryTraceability,
        )
        .expect("query-backed boundary traceability cluster should remain exported");
    assert_eq!(
        boundary.responsibility(),
        KernelCompiledProductConsumerResponsibility::QueryBacked
    );
    assert_eq!(
        boundary.family_class(),
        KernelCompiledProductFamilyClass::QueryLowerRuntimeBoundaryEnvelope
    );
    assert_eq!(
        boundary.query_boundary_lane(),
        Some(KernelCompiledProductQueryBoundaryLane::LowerRuntimeBoundaryEnvelope)
    );
}
