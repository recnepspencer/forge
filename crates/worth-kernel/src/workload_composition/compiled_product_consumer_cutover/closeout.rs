use std::collections::BTreeSet;

use super::coverage_target::KernelCompiledProductConsumerCoverageTarget;
use super::dependency_row::KernelCompiledProductConsumerDependencyRow;
use super::error::{
    KernelCompiledProductConsumerDependencyError, KernelCompiledProductConsumerDependencyErrorKind,
};
use crate::workload_composition::{
    current_compiled_product_reuse_inventory, CompiledProductReuseAuthorityKind,
    CompiledProductReuseDisposition, CompiledProductReuseInventoryRow,
    CompiledProductReuseSurfaceIdentity,
};

pub(super) fn require_complete_cluster_coverage(
    targets: &[KernelCompiledProductConsumerCoverageTarget],
    rows: &[KernelCompiledProductConsumerDependencyRow],
) -> Result<(), KernelCompiledProductConsumerDependencyError> {
    require_declared_clusters_present(targets, rows)?;
    require_inventory_surface_coverage(targets)?;
    Ok(())
}

pub(super) fn current_required_phase_five_reuse_surfaces() -> Result<
    BTreeSet<CompiledProductReuseSurfaceIdentity>,
    KernelCompiledProductConsumerDependencyError,
> {
    let inventory = current_compiled_product_reuse_inventory().map_err(|error| {
        KernelCompiledProductConsumerDependencyError::new(
            KernelCompiledProductConsumerDependencyErrorKind::DeclaredCoveredReuseSurfaceNotInventoryBacked,
            format!("cannot load compiled-product reuse inventory for kernel coverage: {error:?}"),
        )
    })?;
    Ok(inventory
        .rows()
        .iter()
        .filter(|row| phase_five_required_inventory_surface(row))
        .map(CompiledProductReuseInventoryRow::surface_identity)
        .collect())
}

fn require_declared_clusters_present(
    targets: &[KernelCompiledProductConsumerCoverageTarget],
    rows: &[KernelCompiledProductConsumerDependencyRow],
) -> Result<(), KernelCompiledProductConsumerDependencyError> {
    let covered = rows
        .iter()
        .map(KernelCompiledProductConsumerDependencyRow::cluster_identity)
        .collect::<BTreeSet<_>>();

    for target in targets {
        if !covered.contains(&target.cluster_identity()) {
            return Err(KernelCompiledProductConsumerDependencyError::new(
                KernelCompiledProductConsumerDependencyErrorKind::MissingRequiredCluster,
                format!(
                    "kernel compiled-product consumer matrix is missing required cluster `{}`",
                    target.cluster_identity().as_str()
                ),
            ));
        }
    }

    Ok(())
}

fn require_inventory_surface_coverage(
    targets: &[KernelCompiledProductConsumerCoverageTarget],
) -> Result<(), KernelCompiledProductConsumerDependencyError> {
    let inventory = current_compiled_product_reuse_inventory().map_err(inventory_error)?;
    let required_surfaces = current_required_phase_five_reuse_surfaces()?;
    let all_inventory_surfaces = inventory
        .rows()
        .iter()
        .map(CompiledProductReuseInventoryRow::surface_identity)
        .collect::<BTreeSet<_>>();
    let declared_surfaces = targets
        .iter()
        .flat_map(KernelCompiledProductConsumerCoverageTarget::covered_reuse_surfaces)
        .copied()
        .collect::<BTreeSet<_>>();

    for surface in &required_surfaces {
        if !declared_surfaces.contains(surface) {
            return Err(KernelCompiledProductConsumerDependencyError::new(
                KernelCompiledProductConsumerDependencyErrorKind::MissingCoveredReuseSurface,
                format!(
                    "kernel compiled-product consumer matrix is missing covered reuse surface `{:?}`",
                    surface
                ),
            ));
        }
    }
    for surface in &declared_surfaces {
        if !all_inventory_surfaces.contains(surface) {
            return Err(KernelCompiledProductConsumerDependencyError::new(
                KernelCompiledProductConsumerDependencyErrorKind::DeclaredCoveredReuseSurfaceNotInventoryBacked,
                format!(
                    "kernel compiled-product consumer matrix declares surface `{:?}` that is not backed by the compiled-product reuse inventory",
                    surface
                ),
            ));
        }
    }

    Ok(())
}

fn inventory_error(error: impl std::fmt::Debug) -> KernelCompiledProductConsumerDependencyError {
    KernelCompiledProductConsumerDependencyError::new(
        KernelCompiledProductConsumerDependencyErrorKind::DeclaredCoveredReuseSurfaceNotInventoryBacked,
        format!("cannot load compiled-product reuse inventory for kernel coverage: {error:?}"),
    )
}

fn phase_five_required_inventory_surface(row: &CompiledProductReuseInventoryRow) -> bool {
    matches!(
        row.disposition(),
        CompiledProductReuseDisposition::Migrate | CompiledProductReuseDisposition::Cap
    ) && (row.ordinary_path()
        || row.old_authority_kind()
            == CompiledProductReuseAuthorityKind::CloseoutConsumerReusePressure
        || row.old_authority_kind()
            == CompiledProductReuseAuthorityKind::PublicReadModelReuseDescriptor)
        && (row
            .source_path()
            .starts_with("crates/worth-kernel/src/workload_composition/")
            || row
                .source_path()
                .starts_with("crates/worth-kernel/src/replay_undo_consumer_cutover/")
            || row.surface_identity()
                == CompiledProductReuseSurfaceIdentity::CurrentEvidenceLookupPublicCloseout)
}
