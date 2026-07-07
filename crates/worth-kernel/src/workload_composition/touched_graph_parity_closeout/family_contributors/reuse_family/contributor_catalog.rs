use schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityFamilyKind;

use crate::workload_composition::planner_owned_routing::{
    current_worth_touched_graph_conflict_compiled_product_reuse_route_packet,
    current_worth_touched_graph_conflict_selected_route_packet,
};
use crate::workload_composition::touched_graph_parity_closeout::family_contributors::{
    KernelTouchedGraphParityCoverageContributor, KernelTouchedGraphParityCoverageError,
};

use super::equivalence_row::current_equivalence_contributor_row;
use super::error::{ReuseFamilyContributorCatalogError, ReuseFamilyContributorCatalogErrorKind};
use super::reuse_row::current_reuse_contributor_row;
use super::row::{
    reuse_family_coverage_contributor_rows_from_catalog, ReuseFamilyContributorCatalogRow,
    ReuseFamilyContributorRowKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReuseFamilyContributorCatalog {
    rows: Vec<ReuseFamilyContributorCatalogRow>,
}

pub fn current_reuse_family_contributor_catalog(
) -> Result<ReuseFamilyContributorCatalog, ReuseFamilyContributorCatalogError> {
    ReuseFamilyContributorCatalog::new(vec![
        current_equivalence_contributor_row()?,
        current_reuse_contributor_row()?,
    ])
}

pub(crate) fn reuse_family_coverage_contributor_rows(
) -> Result<Vec<KernelTouchedGraphParityCoverageContributor>, KernelTouchedGraphParityCoverageError>
{
    let catalog = current_reuse_family_contributor_catalog()
        .map_err(|error| KernelTouchedGraphParityCoverageError::new(error.detail()))?;
    reuse_family_coverage_contributor_rows_from_catalog(catalog.rows())
}

#[cfg(test)]
pub(crate) fn current_compiled_product_equivalence_coverage_contributor(
) -> Result<KernelTouchedGraphParityCoverageContributor, KernelTouchedGraphParityCoverageError> {
    current_reuse_family_contributor_catalog()
        .map(|catalog| {
            catalog
                .rows()
                .iter()
                .find(|row| row.kind() == ReuseFamilyContributorRowKind::Equivalence)
                .expect("equivalence row")
                .coverage_contributor()
                .clone()
        })
        .map_err(|error| KernelTouchedGraphParityCoverageError::new(error.detail()))
}

#[cfg(test)]
pub(crate) fn current_compiled_product_reuse_coverage_contributor(
) -> Result<KernelTouchedGraphParityCoverageContributor, KernelTouchedGraphParityCoverageError> {
    current_reuse_family_contributor_catalog()
        .map(|catalog| {
            catalog
                .rows()
                .iter()
                .find(|row| row.kind() == ReuseFamilyContributorRowKind::Reuse)
                .expect("reuse row")
                .coverage_contributor()
                .clone()
        })
        .map_err(|error| KernelTouchedGraphParityCoverageError::new(error.detail()))
}

impl ReuseFamilyContributorCatalog {
    pub fn new(
        rows: Vec<ReuseFamilyContributorCatalogRow>,
    ) -> Result<Self, ReuseFamilyContributorCatalogError> {
        validate_catalog_rows(&rows)?;
        Ok(Self { rows })
    }

    pub fn rows(&self) -> &[ReuseFamilyContributorCatalogRow] {
        &self.rows
    }

    #[cfg(test)]
    pub(crate) fn new_unvalidated_for_testing(rows: Vec<ReuseFamilyContributorCatalogRow>) -> Self {
        Self { rows }
    }
}

pub(crate) fn validate_reuse_family_contributor_catalog(
    catalog: &ReuseFamilyContributorCatalog,
) -> Result<(), ReuseFamilyContributorCatalogError> {
    validate_catalog_rows(catalog.rows())
}

fn validate_catalog_rows(
    rows: &[ReuseFamilyContributorCatalogRow],
) -> Result<(), ReuseFamilyContributorCatalogError> {
    if rows.len() != 2 {
        return Err(ReuseFamilyContributorCatalogError::new(
            ReuseFamilyContributorCatalogErrorKind::MissingRequiredRow,
            "reuse-family contributor catalog requires exactly equivalence and reuse rows",
        ));
    }

    let current_route = current_worth_touched_graph_conflict_compiled_product_reuse_route_packet()
        .map_err(|error| {
            ReuseFamilyContributorCatalogError::new(
                ReuseFamilyContributorCatalogErrorKind::CurrentSurfaceUnavailable,
                error.detail(),
            )
        })?;
    let selected_route =
        current_worth_touched_graph_conflict_selected_route_packet().map_err(|error| {
            ReuseFamilyContributorCatalogError::new(
                ReuseFamilyContributorCatalogErrorKind::CurrentSurfaceUnavailable,
                error.detail(),
            )
        })?;

    let mut has_equivalence = false;
    let mut has_reuse = false;
    for row in rows {
        match row.kind() {
            ReuseFamilyContributorRowKind::Equivalence => has_equivalence = true,
            ReuseFamilyContributorRowKind::Reuse => has_reuse = true,
        }
        if row.family_kind() != TouchedGraphParityFamilyKind::CompiledProductReuse {
            return Err(ReuseFamilyContributorCatalogError::new(
                ReuseFamilyContributorCatalogErrorKind::MismatchedReuseSemantics,
                "reuse-family contributor row must remain in the shared CompiledProductReuse family kind",
            ));
        }
        if row.route_packet_identity().is_empty()
            || row.topology_selected_family_identity().is_empty()
            || row.topology_selected_product_identity_digest().is_empty()
            || row.topology_equivalence_policy_identity_digest().is_empty()
            || row.certified_topology_equivalence_basis_digest().is_empty()
            || row
                .topology_selected_reuse_basis_identity_digest()
                .is_empty()
            || row.spatial_selected_family_identity().is_empty()
            || row.spatial_selected_product_identity_digest().is_empty()
            || row.spatial_equivalence_policy_identity_digest().is_empty()
            || row.certified_spatial_equivalence_basis_digest().is_empty()
            || row
                .spatial_selected_reuse_basis_identity_digest()
                .is_empty()
        {
            return Err(ReuseFamilyContributorCatalogError::new(
                ReuseFamilyContributorCatalogErrorKind::MissingCarriedIdentity,
                "reuse-family contributor row must carry explicit equivalence, compatibility, reuse, and denial identities",
            ));
        }
        if row.ordinary_path_live_caller_surface()
            != "current_worth_touched_graph_conflict_selected_route_packet"
            || row.ordinary_path_live_caller_path()
                != "crates/worth-kernel/src/workload_composition/planner_owned_routing/selected_route/current.rs"
        {
            return Err(ReuseFamilyContributorCatalogError::new(
                ReuseFamilyContributorCatalogErrorKind::MissingCarriedIdentity,
                "reuse-family contributor row must name the exact selected-route ordinary caller seam",
            ));
        }
        if row.route_packet_identity() != current_route.packet_identity()
            && row.route_packet_identity()
                != selected_route.compiled_product_reuse_route_packet_identity()
        {
            return Err(ReuseFamilyContributorCatalogError::new(
                ReuseFamilyContributorCatalogErrorKind::MismatchedReuseSemantics,
                "reuse-family contributor row must carry the selected-route compiled-product reuse packet identity",
            ));
        }
    }

    if !(has_equivalence && has_reuse) {
        return Err(ReuseFamilyContributorCatalogError::new(
            ReuseFamilyContributorCatalogErrorKind::MissingRequiredRow,
            "reuse-family contributor catalog requires one equivalence row and one reuse row",
        ));
    }
    Ok(())
}
