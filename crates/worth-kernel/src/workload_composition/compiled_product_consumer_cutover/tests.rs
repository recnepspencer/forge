use std::collections::BTreeSet;

use super::consumer_class::KernelCompiledProductConsumerResponsibility;
use super::current_coverage_targets;
use super::dependency_matrix::{
    current_kernel_compiled_product_consumer_dependency_matrix,
    KernelCompiledProductConsumerDependencyMatrix,
};
use super::dependency_row::{
    KernelCompiledProductConsumerClusterIdentity, KernelCompiledProductConsumerDependencyRow,
};
use super::error::{
    KernelCompiledProductConsumerDependencyError, KernelCompiledProductConsumerDependencyErrorKind,
};
use super::family_class::KernelCompiledProductFamilyClass;
use super::future_cutover_lane::KernelCompiledProductFutureCutoverLane;
use super::proof_basis::KernelCompiledProductProofBasis;
use super::query_boundary_lane::KernelCompiledProductQueryBoundaryLane;
use crate::workload_composition::{
    current_compiled_product_reuse_inventory, CompiledProductReuseAuthorityKind,
    CompiledProductReuseDisposition, CompiledProductReuseSurfaceIdentity as Surface,
};

#[path = "../../certification/public_facade_contracts/contracts/public_api_compiled_product_consumer_cutover.rs"]
mod public_api_compiled_product_consumer_cutover;

#[test]
fn kernel_consumer_matrix_classifies_every_ordinary_consumer() {
    let matrix = current_kernel_compiled_product_consumer_dependency_matrix()
        .expect("phase 5 matrix should classify the current kernel consumer clusters");
    let declared_surfaces = current_coverage_targets()
        .expect("current coverage targets")
        .into_iter()
        .flat_map(|target| target.covered_reuse_surfaces().to_vec())
        .collect::<BTreeSet<_>>();

    assert_eq!(declared_surfaces, expected_phase_five_surfaces());
    assert_eq!(
        inventory_phase_five_surfaces().expect("phase 5 inventory subset"),
        expected_phase_five_surfaces()
    );

    assert_cluster_contract(
        &matrix,
        KernelCompiledProductConsumerClusterIdentity::LookupConsumedWorkload,
        KernelCompiledProductConsumerResponsibility::SpatialEvidenceDerived,
        KernelCompiledProductFamilyClass::SpatialEvidenceLookupIndex,
        KernelCompiledProductFutureCutoverLane::SpatialCompiledProductConsumerCutover,
        None,
        "spatial.evidence_lookup.index:v1",
    );
    assert_cluster_contract(
        &matrix,
        KernelCompiledProductConsumerClusterIdentity::LookupConsumedBatchExecution,
        KernelCompiledProductConsumerResponsibility::SpatialEvidenceDerived,
        KernelCompiledProductFamilyClass::SpatialEvidenceLookupIndex,
        KernelCompiledProductFutureCutoverLane::SpatialCompiledProductConsumerCutover,
        None,
        "spatial.evidence_lookup.index:v1",
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
        .expect("phase 5 matrix should classify query-backed consumer clusters");

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
fn matrix_rejects_missing_phase_five_covered_surface() {
    let targets = current_coverage_targets().expect("current coverage targets");
    let retained_targets = targets
        .iter()
        .copied()
        .filter(|target| {
            !target
                .covered_reuse_surfaces()
                .contains(&Surface::CurrentEvidenceLookupPublicCloseout)
        })
        .collect::<Vec<_>>();
    let rows = retained_targets
        .iter()
        .map(|target| target.lower_row())
        .collect::<Result<Vec<_>, KernelCompiledProductConsumerDependencyError>>()
        .expect("rows should still lower");

    let error = KernelCompiledProductConsumerDependencyMatrix::new(rows, &retained_targets)
        .expect_err(
            "dropping one covered public-closeout surface must fail kernel matrix coverage",
        );

    assert_eq!(
        error.kind(),
        KernelCompiledProductConsumerDependencyErrorKind::MissingCoveredReuseSurface
    );
}

fn assert_cluster_contract(
    matrix: &KernelCompiledProductConsumerDependencyMatrix,
    cluster: KernelCompiledProductConsumerClusterIdentity,
    responsibility: KernelCompiledProductConsumerResponsibility,
    family_class: KernelCompiledProductFamilyClass,
    future_cutover_lane: KernelCompiledProductFutureCutoverLane,
    query_boundary_lane: Option<KernelCompiledProductQueryBoundaryLane>,
    equivalence_policy_identity: &str,
) {
    let row = matrix.require_cluster(cluster).unwrap_or_else(|_| {
        panic!(
            "missing required kernel consumer cluster {}",
            cluster.as_str()
        )
    });

    assert_eq!(row.responsibility(), responsibility);
    assert_eq!(row.family_class(), family_class);
    assert_eq!(row.future_cutover_lane(), future_cutover_lane);
    assert_eq!(row.query_boundary_lane(), query_boundary_lane);
    assert_eq!(
        row.proof_basis().equivalence_policy_identity(),
        equivalence_policy_identity
    );
    assert!(!row.proof_basis().source_authority_basis().is_empty());
    assert!(!row.proof_basis().locality_footprint_basis().is_empty());
    assert!(!row.proof_basis().prior_proof_basis().is_empty());
    assert!(!row.proof_basis().evidence_support_basis().is_empty());
    assert!(!row.current_source_path().is_empty());
    assert!(!row.current_consumer_surface().is_empty());
    assert!(!row.reason().is_empty());
}

fn expected_phase_five_surfaces() -> BTreeSet<Surface> {
    [
        Surface::LookupConsumedWorkloadCompositionAdmit,
        Surface::WorthWorkloadAdmitLookupConsumedWorkload,
        Surface::WorthWorkloadAdmitLookupConsumedBatchExecutionCluster,
        Surface::CurrentWorthWorkloadOrdinaryConsumerCutover,
        Surface::CurrentWorthTouchedGraphConflictPublicCloseout,
        Surface::CurrentWorthTouchedGraphConflictMilestoneFourteenSeed,
        Surface::CurrentEvidenceLookupPublicCloseout,
        Surface::ReplayUndoPublicCloseoutReadModelProjection,
        Surface::KernelConflictPublicCloseoutBoundaryTraceability,
    ]
    .into_iter()
    .collect()
}

fn inventory_phase_five_surfaces(
) -> Result<BTreeSet<Surface>, KernelCompiledProductConsumerDependencyError> {
    let inventory = current_compiled_product_reuse_inventory().map_err(|error| {
        KernelCompiledProductConsumerDependencyError::new(
            KernelCompiledProductConsumerDependencyErrorKind::DeclaredCoveredReuseSurfaceNotInventoryBacked,
            format!("cannot load compiled-product reuse inventory for phase 5 test proof: {error:?}"),
        )
    })?;
    Ok(inventory
        .rows()
        .iter()
        .filter(|row| phase_five_row_is_required(row))
        .map(|row| row.surface_identity())
        .collect())
}

fn phase_five_row_is_required(
    row: &crate::workload_composition::CompiledProductReuseInventoryRow,
) -> bool {
    if !matches!(
        row.disposition(),
        CompiledProductReuseDisposition::Migrate | CompiledProductReuseDisposition::Cap
    ) {
        return false;
    }
    if !row.ordinary_path()
        && row.old_authority_kind()
            != CompiledProductReuseAuthorityKind::CloseoutConsumerReusePressure
        && row.old_authority_kind()
            != CompiledProductReuseAuthorityKind::PublicReadModelReuseDescriptor
    {
        return false;
    }
    row.source_path()
        .starts_with("crates/worth-kernel/src/workload_composition/")
        || row
            .source_path()
            .starts_with("crates/worth-kernel/src/replay_undo_consumer_cutover/")
        || row.surface_identity() == Surface::CurrentEvidenceLookupPublicCloseout
}
