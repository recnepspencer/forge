use crate::workload_composition::planner_owned_routing::WorthTouchedGraphConflictPublicFacade;
use super::error::PublicProjectionContributorCatalogError;
use super::row::{PublicProjectionContributorCatalogRow, PublicProjectionContributorRowKind};

pub(super) fn public_proof_contributor_row_from_public_facade(
    public_facade: &WorthTouchedGraphConflictPublicFacade,
) -> Result<PublicProjectionContributorCatalogRow, PublicProjectionContributorCatalogError> {
    let inspection = public_facade.public_proof();
    let seed = inspection.milestone_fifteen_seed();

    PublicProjectionContributorCatalogRow::new(
        PublicProjectionContributorRowKind::PublicProof,
        "current_worth_touched_graph_conflict_public_facade_with_artifact_policy",
        "current_worth_touched_graph_conflict_public_closeout::{proof_chain,milestone_fifteen_seed,residue_chain,source_firewall_digest}",
        "current_worth_workload_ordinary_consumer_sweep_closeout",
        "crates/worth-kernel/src/workload_composition/worth_workload/ordinary_consumer_sweep/closeout.rs",
        inspection.selected_route_identity_digest().to_string(),
        inspection.selected_family_identity().to_string(),
        inspection.selected_product_identity_digest().to_string(),
        inspection
            .selected_witness_identity_digest()
            .map(str::to_string),
        Some(inspection.proof_chain_digest().to_string()),
        Some(seed.seed_digest().to_string()),
        Some(inspection.residue_chain().residue_digest().to_string()),
        Some(inspection.source_firewall_digest().to_string()),
        None,
        &[
            "selected_route_identity_digest",
            "selected_family_identity",
            "selected_product_identity_digest",
            "selected_witness_identity_digest",
            "proof_chain_digest",
            "seed_digest",
            "residue_digest",
            "source_firewall_digest",
        ],
        "public_proof_inspection",
        "crates/worth-kernel/src/workload_composition/planner_owned_routing/public_facade/current.rs",
    )
}
