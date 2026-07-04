use schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityFamilyKind;

use crate::derived_invalidation_authority_inventory::{
    current_derived_invalidation_authority_inventory, DerivedInvalidationAuthorityInventoryCloseout,
};
use crate::derived_invalidation_family_catalog::current_derived_invalidation_family_catalog;
use crate::projection::touched_graph_parity_closeout::contributor_catalog::{
    TopologyContributorCatalogRowKind, TopologyContributorCoverageAuthority,
    TopologyContributorLocalLanguagePosture, TopologyFamilyContributorCatalogRow,
};
use crate::projection::touched_graph_parity_closeout::invalidation_family::current_topology_invalidation_coverage_contributor;
use crate::projection::touched_graph_parity_closeout::TopologyTouchedGraphParityCoverageError;

const PRODUCED_FIELDS: &[&str] = &[
    "phase_two_seed.seed_digest",
    "catalog.catalog_digest",
    "catalog.families.*.family_digest",
    "catalog.families.*.identity",
    "catalog.counters.family_count",
];

pub fn current_topology_invalidation_declaration_row(
) -> Result<TopologyFamilyContributorCatalogRow, TopologyTouchedGraphParityCoverageError> {
    let inventory_closeout = DerivedInvalidationAuthorityInventoryCloseout::close(
        current_derived_invalidation_authority_inventory(),
    )
    .map_err(|error| TopologyTouchedGraphParityCoverageError::new(format!("{error:?}")))?;
    let catalog =
        current_derived_invalidation_family_catalog(inventory_closeout.phase_two_seed().clone())
            .map_err(|error| TopologyTouchedGraphParityCoverageError::new(format!("{error:?}")))?;
    let coverage = catalog
        .families()
        .iter()
        .map(|family| family.identity().as_str().to_string())
        .collect::<Vec<_>>();

    TopologyFamilyContributorCatalogRow::new(
        TopologyContributorCatalogRowKind::InvalidationFamily,
        TouchedGraphParityFamilyKind::Invalidation,
        "current_derived_invalidation_family_catalog",
        PRODUCED_FIELDS,
        TopologyContributorCoverageAuthority::InvalidationStageIdentities(coverage),
        TopologyContributorLocalLanguagePosture::ExplicitlyBlocked {
            legacy_surface: "operator-local-invalidation-routing-array",
            blocking_surface: "current_derived_invalidation_family_catalog",
        },
        current_topology_invalidation_coverage_contributor()?,
    )
}
