use crate::workload_composition::planner_owned_routing::current_worth_touched_graph_conflict_compiled_product_reuse_route_packet;

use super::error::{ReuseFamilyContributorCatalogError, ReuseFamilyContributorCatalogErrorKind};
use super::row::{ReuseFamilyContributorCatalogRow, ReuseFamilyContributorRowKind};

pub(crate) fn current_equivalence_contributor_row(
) -> Result<ReuseFamilyContributorCatalogRow, ReuseFamilyContributorCatalogError> {
    let packet = current_worth_touched_graph_conflict_compiled_product_reuse_route_packet()
        .map_err(|error| {
            ReuseFamilyContributorCatalogError::new(
                ReuseFamilyContributorCatalogErrorKind::CurrentSurfaceUnavailable,
                error.detail(),
            )
        })?;
    ReuseFamilyContributorCatalogRow::new(
        ReuseFamilyContributorRowKind::Equivalence,
        "current_worth_touched_graph_conflict_compiled_product_reuse_route_packet",
        "current_worth_touched_graph_conflict_public_proof_input::{selected_equivalence_family_identity,topology_equivalence_policy_identity_digest,spatial_equivalence_policy_identity_digest}",
        "current_worth_touched_graph_conflict_public_proof_input::compiled_product_reuse_route_packet_identity",
        "current_worth_touched_graph_conflict_selected_route_packet",
        "crates/worth-kernel/src/workload_composition/planner_owned_routing/selected_route/current.rs",
        packet.packet_identity().to_string(),
        packet.topology_selected_family_identity().to_string(),
        packet.topology_selected_product_identity_digest().to_string(),
        packet.topology_selected_equivalence_policy_identity_digest()
            .to_string(),
        packet
            .topology_selected_compatibility_basis_identity_digest()
            .to_string(),
        packet.topology_selected_reuse_basis_identity_digest().to_string(),
        packet.topology_posture(),
        packet
            .topology_reuse_decision_identity_digest()
            .map(str::to_string),
        packet
            .topology_rebuild_denial_identity_digest()
            .map(str::to_string),
        packet.spatial_selected_family_identity().to_string(),
        packet.spatial_selected_product_identity_digest().to_string(),
        packet.spatial_equivalence_policy_identity_digest().to_string(),
        packet
            .spatial_selected_compatibility_basis_identity_digest()
            .to_string(),
        packet.spatial_selected_reuse_basis_identity_digest().to_string(),
        packet.spatial_posture(),
        packet
            .spatial_reuse_decision_identity_digest()
            .map(str::to_string),
        packet
            .spatial_rebuild_denial_identity_digest()
            .map(str::to_string),
        &[
            "topology_selected_family_identity",
            "topology_selected_product_identity_digest",
            "topology_equivalence_policy_identity_digest",
            "topology_selected_compatibility_basis_identity_digest",
            "spatial_selected_family_identity",
            "spatial_selected_product_identity_digest",
            "spatial_equivalence_policy_identity_digest",
            "spatial_selected_compatibility_basis_identity_digest",
        ],
    )
}
