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
    CompiledProductReuseDisposition, CompiledProductReuseInventoryRow,
    CompiledProductReuseSurfaceIdentity as Surface,
};

#[path = "../../worth_workload/ordinary_consumer_sweep/tests_support_basic_completed_split.rs"]
mod ordinary_consumer_sweep_tests_support_completed_split;
#[path = "../../worth_workload/ordinary_consumer_sweep/tests_support_lookup_routes.rs"]
mod ordinary_consumer_sweep_tests_support_lookup_routes;
#[path = "../../worth_workload/ordinary_consumer_sweep/tests_support_conflict_route_inputs.rs"]
mod ordinary_consumer_sweep_tests_support_conflict_route_inputs;
#[path = "../../../certification/public_facade_contracts/contracts/public_api_compiled_product_consumer_cutover.rs"]
mod public_api_compiled_product_consumer_cutover;

mod matrix;
mod workload_routes;

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

fn expected_current_surfaces() -> BTreeSet<Surface> {
    [
        Surface::BuildDerivedEquivalenceContract,
        Surface::BuildDerivedEquivalenceContractReport,
        Surface::CompareDerivedEquivalenceContracts,
        Surface::DerivedInvalidationPlannedDispositionFromUpdatePosture,
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

fn inventory_current_surfaces(
) -> Result<BTreeSet<Surface>, KernelCompiledProductConsumerDependencyError> {
    let inventory = current_compiled_product_reuse_inventory().map_err(|error| {
        KernelCompiledProductConsumerDependencyError::new(
            KernelCompiledProductConsumerDependencyErrorKind::DeclaredCoveredReuseSurfaceNotInventoryBacked,
            format!("cannot load compiled-product reuse inventory for current matrix proof: {error:?}"),
        )
    })?;
    Ok(inventory
        .rows()
        .iter()
        .filter(|row| current_row_is_required(row))
        .map(|row| row.surface_identity())
        .collect())
}

fn current_row_is_required(row: &CompiledProductReuseInventoryRow) -> bool {
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
        || (row.ordinary_path() && row.source_path().starts_with("crates/worth-topo/src/"))
        || row
            .source_path()
            .starts_with("crates/worth-kernel/src/replay_undo_consumer_cutover/")
        || row.surface_identity() == Surface::CurrentEvidenceLookupPublicCloseout
}
