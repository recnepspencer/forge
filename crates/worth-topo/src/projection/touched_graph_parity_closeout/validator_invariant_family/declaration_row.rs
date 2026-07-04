use std::collections::BTreeSet;

use schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityFamilyKind;

use super::current_milestone_nine::current_projection_validator_invariant_milestone_nine_closeout;
use crate::projection::touched_graph_parity_closeout::contributor_catalog::{
    TopologyContributorCatalogRowKind, TopologyContributorCoverageAuthority,
    TopologyContributorLocalLanguagePosture, TopologyFamilyContributorCatalogRow,
};
use crate::projection::touched_graph_parity_closeout::validator_invariant_family::current_topology_validator_invariant_coverage_contributor;
use crate::projection::touched_graph_parity_closeout::TopologyTouchedGraphParityCoverageError;

const PRODUCED_FIELDS: &[&str] = &[
    "public_proof.query_selected_obligation_digests",
    "public_proof.enforcement_receipt_digests",
    "public_proof.execution_proof_digest",
    "public_proof.support_matrix_digest",
    "milestone_ten_seed.seed_digest",
];

pub fn current_topology_validator_invariant_declaration_row(
) -> Result<TopologyFamilyContributorCatalogRow, TopologyTouchedGraphParityCoverageError> {
    let closeout = current_projection_validator_invariant_milestone_nine_closeout()?;
    let coverage = closeout
        .public_proof()
        .query_selected_obligation_digests()
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    TopologyFamilyContributorCatalogRow::new(
        TopologyContributorCatalogRowKind::ValidatorInvariantFamily,
        TouchedGraphParityFamilyKind::ValidatorInvariantRouting,
        "current_topology_validator_invariant_milestone_nine_closeout",
        PRODUCED_FIELDS,
        TopologyContributorCoverageAuthority::ValidatorRuleIdentities(coverage),
        TopologyContributorLocalLanguagePosture::ExplicitlyBlocked {
            legacy_surface: "operator-local-validator-expectation",
            blocking_surface: "current_topology_validator_invariant_milestone_nine_closeout",
        },
        current_topology_validator_invariant_coverage_contributor()?,
    )
}
