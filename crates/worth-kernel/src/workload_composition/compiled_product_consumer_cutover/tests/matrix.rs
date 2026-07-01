use super::*;

#[test]
fn kernel_consumer_matrix_classifies_every_current_consumer() {
    let matrix = current_kernel_compiled_product_consumer_dependency_matrix()
        .expect("matrix should classify the current consumer clusters");
    let declared_surfaces = current_coverage_targets()
        .expect("current coverage targets")
        .into_iter()
        .flat_map(|target| target.covered_reuse_surfaces().to_vec())
        .collect::<BTreeSet<_>>();

    assert_eq!(declared_surfaces, expected_current_surfaces());
    assert_eq!(
        inventory_current_surfaces().expect("inventory subset"),
        expected_current_surfaces()
    );

    assert_cluster_contract(
        &matrix,
        KernelCompiledProductConsumerClusterIdentity::TopologyDerivedProjectionEquivalence,
        KernelCompiledProductConsumerResponsibility::TopologyDerived,
        KernelCompiledProductFamilyClass::TopologyDerivedEquivalenceContract,
        KernelCompiledProductFutureCutoverLane::TopologyDerivedConsumerCutover,
        None,
        "topology.selected-equivalence.derived-semantic-parity",
    );
    assert_cluster_contract(
        &matrix,
        KernelCompiledProductConsumerClusterIdentity::TopologyDerivedInvalidationDisposition,
        KernelCompiledProductConsumerResponsibility::TopologyDerived,
        KernelCompiledProductFamilyClass::TopologyDerivedInvalidationDisposition,
        KernelCompiledProductFutureCutoverLane::TopologyDerivedConsumerCutover,
        None,
        "topology.selected-equivalence.derived-semantic-parity",
    );
    assert_cluster_contract(
        &matrix,
        KernelCompiledProductConsumerClusterIdentity::LookupConsumedWorkload,
        KernelCompiledProductConsumerResponsibility::SpatialEvidenceDerived,
        KernelCompiledProductFamilyClass::SpatialEvidenceLookupIndex,
        KernelCompiledProductFutureCutoverLane::SpatialCompiledProductConsumerCutover,
        None,
        "spatial.selected_equivalence.evidence_lookup:v1",
    );
    assert_cluster_contract(
        &matrix,
        KernelCompiledProductConsumerClusterIdentity::LookupConsumedBatchExecution,
        KernelCompiledProductConsumerResponsibility::SpatialEvidenceDerived,
        KernelCompiledProductFamilyClass::SpatialEvidenceLookupIndex,
        KernelCompiledProductFutureCutoverLane::SpatialCompiledProductConsumerCutover,
        None,
        "spatial.selected_equivalence.evidence_lookup:v1",
    );
    assert_cluster_contract(
        &matrix,
        KernelCompiledProductConsumerClusterIdentity::RetainedReplayBatchExecutionCarryForward,
        KernelCompiledProductConsumerResponsibility::RetainedReplay,
        KernelCompiledProductFamilyClass::SpatialRetainedReplayWorkload,
        KernelCompiledProductFutureCutoverLane::SpatialCompiledProductConsumerCutover,
        None,
        "spatial.retained_replay.workload:v1",
    );
    assert_cluster_contract(
        &matrix,
        KernelCompiledProductConsumerClusterIdentity::ReplayUndoBoundary,
        KernelCompiledProductConsumerResponsibility::OrdinarySweep,
        KernelCompiledProductFamilyClass::ReplayUndoBoundaryProof,
        KernelCompiledProductFutureCutoverLane::ReplayUndoCompiledProductConsumerCutover,
        None,
        "replay_undo.boundary.consumer:v1",
    );
    assert_cluster_contract(
        &matrix,
        KernelCompiledProductConsumerClusterIdentity::OrdinaryConsumerCutoverSummary,
        KernelCompiledProductConsumerResponsibility::OrdinarySweep,
        KernelCompiledProductFamilyClass::KernelOrdinaryConsumerCutoverSummary,
        KernelCompiledProductFutureCutoverLane::OrdinarySweepConsumerCutover,
        None,
        "kernel.ordinary_consumer_cutover.summary:v1",
    );
    assert_cluster_contract(
        &matrix,
        KernelCompiledProductConsumerClusterIdentity::ConflictPublicCloseout,
        KernelCompiledProductConsumerResponsibility::PublicCloseout,
        KernelCompiledProductFamilyClass::KernelPublicCloseoutProofChain,
        KernelCompiledProductFutureCutoverLane::PublicCloseoutCompiledProductConsumerCutover,
        None,
        "kernel.public_closeout.proof_chain:v1",
    );
    assert_cluster_contract(
        &matrix,
        KernelCompiledProductConsumerClusterIdentity::ConflictPublicCloseoutSeed,
        KernelCompiledProductConsumerResponsibility::PublicCloseout,
        KernelCompiledProductFamilyClass::KernelPublicCloseoutSeed,
        KernelCompiledProductFutureCutoverLane::PublicCloseoutCompiledProductConsumerCutover,
        None,
        "kernel.public_closeout.seed:v1",
    );
    assert_cluster_contract(
        &matrix,
        KernelCompiledProductConsumerClusterIdentity::SpatialEvidenceLookupPublicCloseout,
        KernelCompiledProductConsumerResponsibility::PublicCloseout,
        KernelCompiledProductFamilyClass::SpatialEvidenceLookupPublicCloseout,
        KernelCompiledProductFutureCutoverLane::PublicCloseoutCompiledProductConsumerCutover,
        None,
        "spatial.evidence_lookup.public_closeout:v1",
    );
    assert_cluster_contract(
        &matrix,
        KernelCompiledProductConsumerClusterIdentity::ReplayUndoPublicCloseoutReadModel,
        KernelCompiledProductConsumerResponsibility::QueryBacked,
        KernelCompiledProductFamilyClass::QueryProjectionConsumption,
        KernelCompiledProductFutureCutoverLane::QueryProjectionConsumerCutover,
        Some(KernelCompiledProductQueryBoundaryLane::ProjectionConsumption),
        "query.projection_consumption.read_model:v1",
    );
    assert_cluster_contract(
        &matrix,
        KernelCompiledProductConsumerClusterIdentity::KernelConflictPublicCloseoutBoundaryTraceability,
        KernelCompiledProductConsumerResponsibility::QueryBacked,
        KernelCompiledProductFamilyClass::QueryLowerRuntimeBoundaryEnvelope,
        KernelCompiledProductFutureCutoverLane::QueryBoundaryEnvelopeConsumerCutover,
        Some(KernelCompiledProductQueryBoundaryLane::LowerRuntimeBoundaryEnvelope),
        "query.lower_runtime_boundary_envelope:v1",
    );
}

#[test]
fn consumer_matrix_rejects_unbound_product_dependencies() {
    let error = KernelCompiledProductConsumerDependencyRow::new(
        KernelCompiledProductConsumerClusterIdentity::ReplayUndoPublicCloseoutReadModel,
        "crates/worth-kernel/src/workload_composition/public_closeout/public_closeout.rs",
        "current_worth_touched_graph_conflict_public_closeout",
        KernelCompiledProductConsumerResponsibility::QueryBacked,
        KernelCompiledProductFamilyClass::QueryProjectionConsumption,
        KernelCompiledProductFutureCutoverLane::QueryProjectionConsumerCutover,
        KernelCompiledProductProofBasis::new(
            "query authority",
            "projection footprint",
            "receipt-backed proof",
            "projection support",
            "query.projection:v1",
        ),
        None,
        "hostile consumer omitted the real Query lane",
    )
    .expect_err("query-backed consumer rows must bind a real Query lane");

    assert_eq!(
        error.kind(),
        KernelCompiledProductConsumerDependencyErrorKind::QueryBackedConsumerMissingRealQueryLane
    );
}

#[test]
fn query_backed_consumers_name_real_query_boundary_lane() {
    let matrix = current_kernel_compiled_product_consumer_dependency_matrix()
        .expect("matrix should classify query-backed consumer clusters");

    let query_rows = matrix
        .rows()
        .iter()
        .filter(|row| {
            row.responsibility() == KernelCompiledProductConsumerResponsibility::QueryBacked
        })
        .collect::<Vec<_>>();

    assert_eq!(query_rows.len(), 2);
    assert!(query_rows
        .iter()
        .all(|row| row.query_boundary_lane().is_some()));
    assert!(query_rows.iter().any(|row| {
        row.cluster_identity()
            == KernelCompiledProductConsumerClusterIdentity::ReplayUndoPublicCloseoutReadModel
            && row.query_boundary_lane()
                == Some(KernelCompiledProductQueryBoundaryLane::ProjectionConsumption)
    }));
    assert!(query_rows.iter().any(|row| {
        row.cluster_identity()
            == KernelCompiledProductConsumerClusterIdentity::KernelConflictPublicCloseoutBoundaryTraceability
            && row.query_boundary_lane()
                == Some(KernelCompiledProductQueryBoundaryLane::LowerRuntimeBoundaryEnvelope)
    }));
}

#[test]
fn matrix_rejects_missing_covered_surface() {
    let targets = current_coverage_targets().expect("current coverage targets");
    let retained_targets = targets
        .iter()
        .copied()
        .filter(|target| {
            !target
                .covered_reuse_surfaces()
                .contains(&Surface::BuildDerivedEquivalenceContract)
        })
        .collect::<Vec<_>>();
    let rows = retained_targets
        .iter()
        .map(|target| target.lower_row())
        .collect::<Result<Vec<_>, KernelCompiledProductConsumerDependencyError>>()
        .expect("rows should still lower");

    let error = KernelCompiledProductConsumerDependencyMatrix::new(rows, &retained_targets)
        .expect_err("dropping one covered topology surface must fail matrix coverage");

    assert_eq!(
        error.kind(),
        KernelCompiledProductConsumerDependencyErrorKind::MissingCoveredReuseSurface
    );
}
